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

use std::os::raw::*;
use std::ptr;
use std::convert::*;

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


fn vsf_sysutil_set_sighandler(sig: c_uint, p_handlefunc: unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int) ){}

unsafe extern "C" fn vsf_sysutil_install_sighandler(sig: EVSFSysUtilSignal,
                                handler: vsf_sighandle_t,
                                p_private:*mut c_void,
                               use_alarm: c_int)
{
  let mut realsig: u32 = unsafe{vsf_sysutil_translate_sig(sig)};
  s_sig_details[realsig as usize].p_private = p_private;
  s_sig_details[realsig as usize].sync_sig_handler = handler;
  s_sig_details[realsig as usize].use_alarm = use_alarm;
  vsf_sysutil_set_sighandler(realsig, vsf_sysutil_common_sighandler);
  
  if use_alarm != 0 && realsig != SIGALRM
  {
    vsf_sysutil_set_sighandler(SIGALRM, vsf_sysutil_alrm_sighandler);
  }
}

unsafe extern "C" fn vsf_sysutil_default_sig(sig: EVSFSysUtilSignal)
{
  let mut realsig: u32 = vsf_sysutil_translate_sig(sig);
  SIG_DFL =  { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(-1))};
  match SIG_DFL{
    None=>{},
    Some(x)=>{
        vsf_sysutil_set_sighandler(realsig, x);
    }
  }
  s_sig_details[realsig as usize].p_private = ptr::null_mut();

  s_sig_details[realsig as usize].sync_sig_handler = Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 :*mut :: std :: os :: raw :: c_void)>(0) );;
}
