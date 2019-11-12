
mod bindings_new;

use std::ptr;
use std::os::raw::*;

use bindings_new::*;

extern "C" {
pub fn resolve_tilde (p_str: &mystr, p_sess: &vsf_session);
}

#[no_mangle]
pub extern "C" fn handle_pwd (p_sess: &mut vsf_session) {
	let mut s_cwd_buf_mangle_str: mystr = mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};
	let mut s_pwd_res_str: mystr = mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};

	unsafe {
    str_getcwd(&s_cwd_buf_mangle_str);

    /* Double up any double-quotes in the pathname! */
	  str_replace_text(&s_cwd_buf_mangle_str, str_to_const_char("\""), str_to_const_char("\"\"") );
    str_alloc_text(&s_pwd_res_str, str_to_const_char("\"") );
    str_append_str(&s_pwd_res_str, &s_cwd_buf_mangle_str);
    str_append_text(&s_pwd_res_str, str_to_const_char("\" is the current directory") );
    vsf_cmdio_write_str(p_sess, FTP_PWDOK, &s_pwd_res_str);

	}
}

#[no_mangle]
pub extern "C" fn handle_cwd (p_sess: &mut vsf_session) {

  unsafe {
  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  if ( 0 == vsf_access_check_file(&p_sess.ftp_arg_str) )
  {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.") );
    return;
  }

  let retval = str_chdir(&p_sess.ftp_arg_str);
  if (retval == 0)
  {
    /* Handle any messages */
    vsf_banner_dir_changed(p_sess, FTP_CWDOK);
    vsf_cmdio_write(p_sess, FTP_CWDOK, str_to_const_char("Directory successfully changed.") );
  }
  else
  {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to change directory.") );
  }

 }

}

#[no_mangle]
pub unsafe extern "C" fn handle_pasv (p_sess: &mut vsf_session,is_epsv: c_int) {
  let mut the_port : c_ushort;
  let mut s_pasv_res_str : mystr = mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};

  let mut tmp_sockaddr  = sockaddr { sa_family: 3, sa_data: [0; 14usize] };
  let mut tmp_vsf_sockaddr = vsf_sysutil_sockaddr__bindgen_ty_1 { u_sockaddr: tmp_sockaddr };
  let s_p_sockaddr : vsf_sysutil_sockaddr = vsf_sysutil_sockaddr { u: tmp_vsf_sockaddr };

  let mut is_ipv6: c_int = vsf_sysutil_sockaddr_is_ipv6(p_sess.p_local_addr);

  if 0 != is_epsv && 0 == str_isempty(&p_sess.ftp_arg_str)
   {
    let mut argval: c_int;
    str_upper(&p_sess.ftp_arg_str);
    if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("ALL"))
     {
      p_sess.epsv_all = 1;
      vsf_cmdio_write(p_sess, FTP_EPSVALLOK, str_to_const_char("EPSV ALL ok.") );
      return;
     }

    argval = vsf_sysutil_atoi(str_getbuf(&p_sess.ftp_arg_str));
    if argval < 1 || argval > 2 || ( 0 == is_ipv6 && argval == 2 )
     {
      vsf_cmdio_write(p_sess, FTP_EPSVBAD, str_to_const_char("Bad network protocol.") );
      return;
     }
   }

  pasv_cleanup(p_sess);
  port_cleanup(p_sess);
  if 0 != tunable_one_process_model
   {
    the_port = vsf_one_process_listen(p_sess);
   }
  else
   {
    the_port = vsf_two_process_listen(p_sess);
   }

  if 0 != is_epsv
   {
    str_alloc_text(&s_pasv_res_str, str_to_const_char("Entering Extended Passive Mode (|||") );
    str_append_ulong(&s_pasv_res_str, the_port.into() );
    str_append_text(&s_pasv_res_str, str_to_const_char("|)") );
    vsf_cmdio_write_str(p_sess, FTP_EPSVOK, &s_pasv_res_str);
    return;
   }

  if tunable_pasv_address != ptr::null_mut()
   {
    vsf_sysutil_sockaddr_alloc_ipv4(&&s_p_sockaddr);
    // Report passive address as specified in configuration
    if (vsf_sysutil_inet_aton(tunable_pasv_address, &s_p_sockaddr) == 0)
     {
      die( str_to_const_char("invalid pasv_address") );
     }
   }
  else
   {
    vsf_sysutil_sockaddr_clone(&&s_p_sockaddr, p_sess.p_local_addr);
   }

  str_alloc_text(&s_pasv_res_str, str_to_const_char("Entering Passive Mode (") );
  if 0 == is_ipv6
  {
    str_append_text(&s_pasv_res_str, vsf_sysutil_inet_ntop(&s_p_sockaddr));
  }
  else
  {
    let p_v4addr: *const c_void = vsf_sysutil_sockaddr_ipv6_v4(&s_p_sockaddr);
    if p_v4addr != ptr::null_mut()
    {
      str_append_text(&s_pasv_res_str, vsf_sysutil_inet_ntoa(p_v4addr));
    }
    else
    {
      str_append_text(&s_pasv_res_str, str_to_const_char("0,0,0,0") );
    }
  }
  str_replace_char(&s_pasv_res_str, '.' as i8, ',' as i8 );
  str_append_text(&s_pasv_res_str, str_to_const_char(",") );

  str_append_ulong(&s_pasv_res_str, (the_port >> 8).into() );
//  let shift_the_port : u64 =  the_port >> 8;
//  str_append_ulong(&s_pasv_res_str, shift_the_port);

  str_append_text(&s_pasv_res_str, str_to_const_char(",") );
  str_append_ulong(&s_pasv_res_str, (the_port & 255).into() );
  str_append_text(&s_pasv_res_str, str_to_const_char(").") );
  vsf_cmdio_write_str(p_sess, FTP_PASVOK, &s_pasv_res_str);
}

#[no_mangle]
pub unsafe extern "C" fn port_cleanup (p_sess: &vsf_session) {
  vsf_sysutil_sockaddr_clear(&p_sess.p_port_sockaddr);
 }

#[no_mangle]
pub unsafe extern "C" fn pasv_cleanup (p_sess: &vsf_session) {

  if 0 != tunable_one_process_model
   {
    vsf_one_process_pasv_cleanup(p_sess);
   }
  else
   {
    vsf_two_process_pasv_cleanup(p_sess);
   }

}

#[no_mangle]
pub unsafe extern "C" fn handle_cdup (p_sess: &mut vsf_session) {
  str_alloc_text(&p_sess.ftp_arg_str, str_to_const_char("..") );
  handle_cwd(p_sess);
}

#[no_mangle]
pub unsafe extern "C" fn port_active (p_sess: &vsf_session) -> c_int {
  let mut ret : c_int = 0;

  if p_sess.p_port_sockaddr != ptr::null_mut()
   {
    ret = 1;
    if 0 != pasv_active(p_sess)
     {
      bug( str_to_const_char("port and pasv both active") );
     }
   }
  return ret;
}

#[no_mangle]
pub unsafe extern "C" fn pasv_active (p_sess: &vsf_session ) -> c_int {
  let mut ret : c_int = 0;

  if 0 != tunable_one_process_model
   {
    ret = vsf_one_process_pasv_active(p_sess);
   }
  else
   {
    ret = vsf_two_process_pasv_active(p_sess);
   }

  if 0 != ret
   {
    if 0 != port_active(p_sess)
     {
      bug( str_to_const_char("pasv and port both active") );
     }
   }

  return ret;
}
