
mod bindings;

use std::ptr;
use std::os::raw::*;

use bindings::*;

extern "C" {
pub fn resolve_tilde (p_str: &mystr, p_sess: &vsf_session);
}

#[no_mangle]
pub extern "C" fn handle_pwd (p_sess: &mut vsf_session) {
	let mut s_cwd_buf_mangle_str: mystr = mystr { PRIVATE_HANDS_OFF_p_buf: ptr::null_mut(), PRIVATE_HANDS_OFF_alloc_bytes: 0, PRIVATE_HANDS_OFF_len: 0};
	let mut s_pwd_res_str: mystr = mystr { PRIVATE_HANDS_OFF_p_buf: ptr::null_mut(), PRIVATE_HANDS_OFF_alloc_bytes: 0, PRIVATE_HANDS_OFF_len: 0};

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
pub extern "C" fn handle_pasv (p_sess: &vsf_session,is_epsv: c_int) {
  let mut the_port : c_ushort;
  let mut s_pasv_res_str : mystr = mystr { PRIVATE_HANDS_OFF_p_buf: ptr::null_mut(), PRIVATE_HANDS_OFF_alloc_bytes: 0, PRIVATE_HANDS_OFF_len: 0};
  let mut s_p_sockaddr : vsf_sysutil_sockaddr = vsf_sysutil_sockaddr { u:{sockaddr: {} } };

  unsafe {
  let mut is_ipv6: c_int = vsf_sysutil_sockaddr_is_ipv6(p_sess.p_local_addr);
  }

  /*
  if (is_epsv && !str_isempty(&p_sess->ftp_arg_str))
  {
    int argval;
    str_upper(&p_sess->ftp_arg_str);
    if (str_equal_text(&p_sess->ftp_arg_str, "ALL"))
    {
      p_sess->epsv_all = 1;
      vsf_cmdio_write(p_sess, FTP_EPSVALLOK, "EPSV ALL ok.");
      return;
    }
    argval = vsf_sysutil_atoi(str_getbuf(&p_sess->ftp_arg_str));
    if (argval < 1 || argval > 2 || (!is_ipv6 && argval == 2))
    {
      vsf_cmdio_write(p_sess, FTP_EPSVBAD, "Bad network protocol.");
      return;
    }
  }
  pasv_cleanup(p_sess);
  port_cleanup(p_sess);
  if (tunable_one_process_model)
  {
    the_port = vsf_one_process_listen(p_sess);
  }
  else
  {
    the_port = vsf_two_process_listen(p_sess);
  }
  if (is_epsv)
  {
    str_alloc_text(&s_pasv_res_str, "Entering Extended Passive Mode (|||");
    str_append_ulong(&s_pasv_res_str, (unsigned long) the_port);
    str_append_text(&s_pasv_res_str, "|)");
    vsf_cmdio_write_str(p_sess, FTP_EPSVOK, &s_pasv_res_str);
    return;
  }
  if (tunable_pasv_address != 0)
  {
    vsf_sysutil_sockaddr_alloc_ipv4(&s_p_sockaddr);
    // Report passive address as specified in configuration
    if (vsf_sysutil_inet_aton(tunable_pasv_address, s_p_sockaddr) == 0)
    {
      die("invalid pasv_address");
    }
  }
  else
  {
    vsf_sysutil_sockaddr_clone(&s_p_sockaddr, p_sess->p_local_addr);
  }
  str_alloc_text(&s_pasv_res_str, "Entering Passive Mode (");
  if (!is_ipv6)
  {
    str_append_text(&s_pasv_res_str, vsf_sysutil_inet_ntop(s_p_sockaddr));
  }
  else
  {
    const void* p_v4addr = vsf_sysutil_sockaddr_ipv6_v4(s_p_sockaddr);
    if (p_v4addr)
    {
      str_append_text(&s_pasv_res_str, vsf_sysutil_inet_ntoa(p_v4addr));
    }
    else
    {
      str_append_text(&s_pasv_res_str, "0,0,0,0");
    }
  }
  str_replace_char(&s_pasv_res_str, '.', ',');
  str_append_text(&s_pasv_res_str, ",");
  str_append_ulong(&s_pasv_res_str, the_port >> 8);
  str_append_text(&s_pasv_res_str, ",");
  str_append_ulong(&s_pasv_res_str, the_port & 255);
  str_append_text(&s_pasv_res_str, ").");
  vsf_cmdio_write_str(p_sess, FTP_PASVOK, &s_pasv_res_str);
  */
}

/*
void
handle_pwd(struct vsf_session* p_sess)
{
  static struct mystr s_cwd_buf_mangle_str;
  static struct mystr s_pwd_res_str;
  str_getcwd(&s_cwd_buf_mangle_str);
  /* Double up any double-quotes in the pathname! */
  str_replace_text(&s_cwd_buf_mangle_str, "\"", "\"\"");
  /* Enclose pathname in quotes */
  str_alloc_text(&s_pwd_res_str, "\"");
  str_append_str(&s_pwd_res_str, &s_cwd_buf_mangle_str);
  str_append_text(&s_pwd_res_str, "\" is the current directory");
  vsf_cmdio_write_str(p_sess, FTP_PWDOK, &s_pwd_res_str);
}
*/
