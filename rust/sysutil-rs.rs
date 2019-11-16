/*
 * Part of Very Secure FTPd
 * Licence: GPL v2
 * Author: Chris Evans
 * 
 * sysutil.c
 *
 * Routines to make the libc/syscall API more pleasant to use. Long term,
 * more libc/syscalls will go in here to reduce the number of .c files with
 * dependencies on libc or syscalls.
 */
mod bindings_new;
use bindings_new::*;
use bindings_new::SIG_DFL;
use std::mem;
use std::os::raw::*;

use std::os::raw::*;
use std::ptr;
use std::convert::*;
use std::mem::size_of;

/* File locals */
unsafe extern "C" fn vsf_sysutil_alrm_sighandler(signum: c_int)
{
  //(void) signum;
  alarm(1);
}

unsafe extern "C" fn vsf_sysutil_common_sighandler(signum: c_int)
{
    if signum < 0 || signum >= NSIG.try_into().unwrap()     
    {
      /* "cannot happen" */
      return;
    }
    match s_sig_details[signum as usize].sync_sig_handler {
        None => {},
        Some(x) => {
          s_sig_details[signum as usize].pending = 1;
          /* Since this synchronous signal framework has a small race (signal coming
           * in just before we start a blocking call), there's the option to fire an
           * alarm repeatedly until the signal is handled.
           */
          if s_sig_details[signum as usize].use_alarm != 0
          {
            alarm(1);
          }    
      }
  }
}

/* Notes. This signal check is evaluated after potentially blocking system
 * calls, i.e. at highly defined points in the code. Since we only interrupt
 * at these definite locations, the signal handlers can be non-trivial
 * without us having to worry about re-entrancy.
 *
 * We guarantee that a handler for a given signal is not re-entrant. This
 * is taken care of by the "running" flag.
 *
 * This call itself can only be re-entered once we dereference the actual
 * hander function pointer, so we are safe with respect to races modifying
 * the "running" flag.
 */

unsafe extern "C" fn vsf_sysutil_check_pending_actions(context: EVSFSysUtilInterruptContext, retval: c_int, fd: c_int)
 {
   let i: c_uint;
   /* Check the i/o handler before the signal handlers */
   match s_io_handler{
       None => {},
       Some(x) =>{
           if s_io_handler_running == 0 && context == EVSFSysUtilInterruptContext_kVSFSysUtilIO {
            s_io_handler_running = 1;
            (x)(retval, fd, s_p_io_handler_private); 
            s_io_handler_running = 0;
       }
   }
}
   for i in 0..NSIG
   {
     if (s_sig_details[i as usize].pending != 0 && s_sig_details[i as usize].running == 0)
     {
       s_sig_details[i as usize].running = 1;
       if (s_sig_details[i as usize].use_alarm !=0 )
       {
         alarm(0);
       }
       //sync_sig_handler is a pointer and comparison to 0 is actually a comparison with NULL.
       match s_sig_details[i as usize].sync_sig_handler{
           None=>{},
           Some(x)=>{
            s_sig_details[i as usize].pending = 0;
            //check x was dereferenced twice in c file.
            (x)(s_sig_details[i as usize].p_private);
           }
       }
       s_sig_details[i as usize].running = 0; 
     }
   }
}

unsafe extern "C" fn vsf_sysutil_translate_sig(sig: EVSFSysUtilSignal) -> u32
{ 
  let mut realsig: u32 = 0;
  match sig 
  {
    kVSFSysUtilSigALRM =>
      realsig = SIGALRM,
      
    kVSFSysUtilSigTERM =>
      realsig = SIGTERM,
      
    kVSFSysUtilSigCHLD =>
      realsig = SIGCHLD,
      
    kVSFSysUtilSigPIPE =>
      realsig = SIGPIPE,
      
    kVSFSysUtilSigURG =>
      realsig = SIGURG,
      
    kVSFSysUtilSigHUP =>
      realsig = SIGHUP,
      
    _ =>
      unsafe { bug(str_to_const_char("unknown signal in vsf_sysutil_translate_sig")) },
  }
  if realsig < 0 || realsig >= NSIG
  {
    unsafe {bug(str_to_const_char("signal out of range in vsf_sysutil_translate_sig")); }
  }
  return realsig;
}

unsafe extern "C" fn vsf_sysutil_install_sighandler(sig: EVSFSysUtilSignal,
                                handler: vsf_sighandle_t,
                                p_private:*mut c_void,
                               use_alarm: c_int)
{
  let mut realsig: u32 = unsafe{vsf_sysutil_translate_sig(sig)};
  s_sig_details[realsig as usize].p_private = p_private;
  s_sig_details[realsig as usize].sync_sig_handler = handler;
  s_sig_details[realsig as usize].use_alarm = use_alarm;
  vsf_sysutil_set_sighandler(realsig, Some(vsf_sysutil_common_sighandler) );
  
  if use_alarm != 0 && realsig != SIGALRM
  {
    vsf_sysutil_set_sighandler(SIGALRM, Some(vsf_sysutil_alrm_sighandler) );
  }
}

unsafe extern "C" fn vsf_sysutil_default_sig(sig: EVSFSysUtilSignal)
{
  let mut realsig: u32 = vsf_sysutil_translate_sig(sig);
  //SIG_DFL has value 0 from signum-generic.h
  SIG_DFL =  None;//{ Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(0))};
  vsf_sysutil_set_sighandler(realsig, SIG_DFL);
  s_sig_details[realsig as usize].p_private = ptr::null_mut();
  s_sig_details[realsig as usize].sync_sig_handler = None; //Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 :*mut :: std :: os :: raw :: c_void)>(0) );
}

unsafe extern "C" fn vsf_sysutil_install_null_sighandler(sig: EVSFSysUtilSignal)
{
  let mut realsig: u32 = vsf_sysutil_translate_sig(sig);
  //SIG_DFL has value 0 from signum-generic.h
  SIG_DFL = None;// { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(0))};
  vsf_sysutil_set_sighandler(realsig, SIG_DFL);
  s_sig_details[realsig as usize].p_private = ptr::null_mut();
  s_sig_details[realsig as usize].sync_sig_handler = None;// Some( std::mem::transmute::<isize, unsafe extern "C" fn (arg1 :*mut :: std :: os :: raw :: c_void)>(0) );
  vsf_sysutil_set_sighandler(realsig, Some(vsf_sysutil_common_sighandler) );
}

unsafe extern "C" fn vsf_sysutil_install_async_sighandler ( sig: EVSFSysUtilSignal, handler: vsf_async_sighandle_t) {

  let mut realsig: u32 = vsf_sysutil_translate_sig(sig);
  s_sig_details[realsig as usize].p_private = ptr::null_mut();
  s_sig_details[realsig as usize].sync_sig_handler = None;
  vsf_sysutil_block_sig(sig);
  vsf_sysutil_set_sighandler(realsig, handler);
}

unsafe extern "C" fn vsf_sysutil_set_sighandler(sig: c_uint, p_handlefunc: Option <unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)> )
{
  let mut retval: c_int;
  let mut sigact= sigaction:: default();
  vsf_sysutil_memclr(&mut sigact as *mut _ as *mut c_void, size_of::<sigaction>());
  sigact.sa_handler = p_handlefunc;
  retval = sigfillset(&mut sigact.sa_mask);
  if retval != 0
  {
    die(str_to_const_char("sigfillset"));
  }
  retval = sigaction(sig.try_into().unwrap(), &sigact, ptr::null_mut());
  if retval != 0
  {
    die(str_to_const_char("sigaction"));
  }
}

unsafe extern "C" fn vsf_sysutil_block_sig(sig: EVSFSysUtilSignal)
{
  //sigset_t is a struct
  let mut sset= sigset_t :: default();
  let mut retval : c_int;
  let mut realsig: c_int = vsf_sysutil_translate_sig(sig).try_into().unwrap();
  retval = sigemptyset(&mut sset);
  if retval != 0
  {
    die(str_to_const_char("sigemptyset"));
  }
  retval = sigaddset(&mut sset, realsig);
  if retval != 0
  {
    die(str_to_const_char("sigaddset"));
  }
  retval = sigprocmask(SIG_BLOCK.try_into().unwrap(), & sset, ptr::null_mut());
  if retval != 0
  {
    die(str_to_const_char("sigprocmask"));
  }
}

unsafe extern "C" fn vsf_sysutil_unblock_sig(sig: EVSFSysUtilSignal)
{
  //sigset_t is a struct
  let mut sset= sigset_t :: default();
  let mut retval : c_int;
  let mut realsig: c_int = vsf_sysutil_translate_sig(sig).try_into().unwrap();
  retval = sigemptyset(&mut sset);
  if retval != 0
  {
    die(str_to_const_char("sigemptyset"));
  }
  retval = sigaddset(&mut sset, realsig);
  if retval != 0
  {
    die(str_to_const_char("sigaddset"));
  }
  retval = sigprocmask(SIG_UNBLOCK.try_into().unwrap(), & sset, ptr::null_mut());
  if retval != 0
  {
    die(str_to_const_char("sigprocmask"));
  }
}

unsafe extern "C" fn vsf_sysutil_install_io_handler(handler: vsf_context_io_t , p_private:*mut c_void)
{
  match s_io_handler
  {
    None=> {},
    Some(x)=>{
      bug(str_to_const_char("double register of i/o handler"));
    }
  }
  s_io_handler = handler;
  s_p_io_handler_private = p_private;
}

unsafe extern "C" fn vsf_sysutil_uninstall_io_handler()
{
  match s_io_handler
  {
    None=> {},
    Some(x)=>{
    bug(str_to_const_char("no i/o handler to unregister!"));
  }
}
  s_io_handler = None;
  s_p_io_handler_private = ptr::null_mut();
}

fn vsf_sysutil_set_alarm(trigger_seconds: c_uint)
{
  //call to unistd.h
  unsafe{
   alarm(trigger_seconds);
  }
}

fn vsf_sysutil_clear_alarm()
{
  //call to unistd.h
  unsafe{
    vsf_sysutil_set_alarm(0);
  }
}

unsafe extern "C" fn vsf_sysutil_read(fd: c_int, p_buf: *mut c_void, size: c_uint) -> c_int
{
  while true
  {
    //call to unistd.h
    let mut retval: c_int = read(fd, p_buf, size.try_into().unwrap()).try_into().unwrap();
    let mut saved_errno: c_int = (*__errno_location ());
    vsf_sysutil_check_pending_actions(EVSFSysUtilInterruptContext_kVSFSysUtilIO, retval, fd);
    if retval < 0 && saved_errno == EINTR.try_into().unwrap()
    {
      continue;
    }
    return retval;
  }

  return -1;
}


unsafe extern "C" fn vsf_sysutil_write(fd: c_int, p_buf: *mut c_void, size: c_uint) -> c_int
{
  while true
  {
    //call to unistd.h
    let mut retval: c_int = write(fd, p_buf, size.try_into().unwrap()).try_into().unwrap();
    let mut saved_errno: c_int = (*__errno_location ());
    vsf_sysutil_check_pending_actions(EVSFSysUtilInterruptContext_kVSFSysUtilIO, retval, fd);
    if retval < 0 && saved_errno == EINTR.try_into().unwrap()
    {
      continue;
    }
    return retval;
  }
  return -1;  
}

unsafe extern "C" fn vsf_sysutil_read_loop(fd: c_int, p_buf: *mut c_void, mut size: c_uint) -> c_int
{
  let mut retval: c_int;
  let mut num_read: c_int = 0;
  if size > WINT_MAX
  {
    die(str_to_const_char("size too big in vsf_sysutil_read_loop"));
  }
  while true
  {
    let retval: c_int = vsf_sysutil_read(fd, p_buf.offset(num_read.try_into().unwrap()) as *mut _ as *mut c_void, size);
    if retval < 0
    {
      return retval;
    }
    else if retval == 0
    {
      /* Read all we're going to read.. */
      return num_read; 
    }
    let retval: c_uint = retval.try_into().unwrap();
    if retval > size
    {
      die(str_to_const_char("retval too big in vsf_sysutil_read_loop"));
    }
    let retval: c_int = retval.try_into().unwrap();
    num_read += retval;
    let retval: c_uint = retval.try_into().unwrap();
    size = size - retval;
    if (size == 0)
    {
      /* Hit the read target, cool. */
      return num_read;
    }
  }
  return 1;
}

unsafe extern "C" fn vsf_sysutil_write_loop(fd: c_int, p_buf: *mut c_void, mut size: c_uint) -> c_int
{
  let mut retval: c_int;
  let mut num_written: c_int = 0;
  if size > WINT_MAX
  {
    die(str_to_const_char("size too big in vsf_sysutil_write_loop"));
  }
  while true
  {
    retval = vsf_sysutil_write(fd, p_buf.offset(num_written.try_into().unwrap()) as *mut _ as *mut c_void, size);
    if retval < 0
    {
      /* Error */
      return retval;
    }
    else if retval == 0
    {
      /* Written all we're going to write.. */
      return num_written;
    }
    let retval: c_uint = retval.try_into().unwrap();
    if retval > size
    {
      die(str_to_const_char("retval too big in vsf_sysutil_write_loop"));
    }
    let retval: c_int = retval.try_into().unwrap();
    num_written += retval;
    let retval: c_uint = retval.try_into().unwrap();
    size = size - retval;
    if (size == 0)
    {
      /* Hit the write target, cool. */
      return num_written;
    }
  }
  return 1;
}

unsafe extern "C" fn vsf_sysutil_get_file_offset(file_fd: c_int) -> filesize_t
{
  let mut retval: filesize_t = lseek(file_fd, 0, SEEK_CUR.try_into().unwrap());
  if (retval < 0)
  {
    die(str_to_const_char("lseek"));
  }
  return retval;
}

unsafe extern "C" fn vsf_sysutil_lseek_to(fd: c_int, seek_pos: filesize_t )
{
  let mut retval: filesize_t;
  if seek_pos < 0
  {
    die(str_to_const_char("negative seek_pos in vsf_sysutil_lseek_to"));
  }
  retval = lseek(fd, seek_pos, SEEK_SET.try_into().unwrap());
  if retval < 0
  {
    die(str_to_const_char("lseek"));
  }
}

unsafe extern "C" fn vsf_sysutil_lseek_end(fd: c_int)
{
  let mut retval: filesize_t ;
  retval = lseek(fd, 0, SEEK_END.try_into().unwrap());
  if retval < 0
  {
    die(str_to_const_char("lseek"));
  }
}


unsafe extern "C" fn vsf_sysutil_malloc(size: c_uint) -> *mut c_void
{
  let mut p_ret: *mut c_void;
  /* Paranoia - what if we got an integer overflow/underflow? */
  if size == 0 || size > WINT_MAX
  {
    bug(str_to_const_char("zero or big size in vsf_sysutil_malloc"));
  }  
  p_ret = malloc(size as usize);
  if p_ret == ptr::null_mut()
  {
    die(str_to_const_char("malloc"));
  }
  return p_ret;
}

unsafe extern "C" fn vsf_sysutil_realloc(p_ptr: *mut c_void, size: c_uint)-> *mut c_void
{
  let mut p_ret: *mut c_void;
  if size == 0 || size > WINT_MAX
  {
    bug(str_to_const_char("zero or big size in vsf_sysutil_realloc"));
  }
  p_ret = realloc(p_ptr, size as usize);
  if p_ret == ptr::null_mut()
  {
    die(str_to_const_char("realloc"));
  }
  return p_ret;
}

unsafe extern "C" fn vsf_sysutil_free( p_ptr: *mut c_void)
{
  if p_ptr == ptr::null_mut()
  {
    bug(str_to_const_char("vsf_sysutil_free got a null pointer"));
  }
  free(p_ptr);
}

unsafe extern "C" fn vsf_sysutil_getpid() -> c_uint
{
  if s_current_pid == -1
  {
    s_current_pid = vsf_sysutil_getpid_nocache();
  }

  return s_current_pid.try_into().unwrap();
}

unsafe extern "C" fn vsf_sysutil_fork() -> c_int
{
  let mut retval: c_int = vsf_sysutil_fork_failok();
  if retval < 0
  {
    die(str_to_const_char("fork"));
  }
  return retval;
}

unsafe extern "C" fn vsf_sysutil_fork_failok() -> c_int
{
  let mut retval: c_int = vsf_sysutil_fork_failok();
  retval = fork();
  if retval == 0
  {
    vsf_sysutil_post_fork();
  }
  return retval;
}

unsafe extern  "C" fn vsf_sysutil_set_exit_func(exitfunc: exitfunc_t)
{
  s_exit_func = exitfunc;
}

unsafe extern "C" fn vsf_sysutil_exit(exit_code: c_int)
{
  match s_exit_func 
  {
    None=>{},
    Some(x)=>{
      let curr_func: exitfunc_t = s_exit_func;
      /* Prevent recursion */
      s_exit_func = None;
      curr_func;
    }
  }
  _exit(exit_code);

}

unsafe extern "C" fn vsf_sysutil_wait() -> vsf_sysutil_wait_retval
{
  let mut retval= vsf_sysutil_wait_retval::default();
//  vsf_sysutil_memclr(&mut retval as *mut c_void, size_of::<vsf_sysutil_wait_retval>());
  vsf_sysutil_memclr(&mut retval as *mut _ as *mut c_void, size_of::<vsf_sysutil_wait_retval>());
  while true
  {
    let sys_ret: c_int = wait(&mut retval.PRIVATE_HANDS_OFF_exit_status as *mut _ as *mut c_int);
    //call __errno_location funciton
    if sys_ret < 0 && *__errno_location() == EINTR.try_into().unwrap()
    {
      vsf_sysutil_check_pending_actions(EVSFSysUtilInterruptContext_kVSFSysUtilUnknown, 0, 0);
      continue;
    }
    retval.PRIVATE_HANDS_OFF_syscall_retval = sys_ret;
    return retval;
  }
  return retval;
}



unsafe fn vsf_sysutil_memclr(p_dest: *mut c_void, size: usize)
{
  /* Safety */
  if (size == 0)
  {
    return;
  }
  memset(p_dest, 0, size);
}