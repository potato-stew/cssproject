
mod bindings_new;

use std::ptr;
use std::os::raw::*;

use bindings_new::*;

extern "C" {
pub fn resolve_tilde (p_str: &mystr, p_sess: &vsf_session);
}

pub fn default_mystr() -> mystr {
  mystr { p_buf: ptr::null_mut(), alloc_bytes:0, len:0 }
}

pub fn default_sockaddr() -> sockaddr {
  sockaddr { sa_family: 3, sa_data: [0; 14usize] }
}

#[no_mangle]
pub extern "C" fn handle_pwd (p_sess: &mut vsf_session) {
	let mut s_cwd_buf_mangle_str: mystr = default_mystr();
	let mut s_pwd_res_str: mystr = default_mystr();

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
  let mut s_pasv_res_str = default_mystr();// mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};

  let mut tmp_sockaddr  = default_sockaddr();// sockaddr { sa_family: 3, sa_data: [0; 14usize] };
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
  str_replace_char(&s_pasv_res_str, '.' as c_char, ',' as c_char );
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

#[no_mangle]
pub unsafe extern "C" fn data_transfer_checks_ok (p_sess: &mut vsf_session) -> c_int {

  if 0 == pasv_active(p_sess) && 0 == port_active(p_sess)
   {
    vsf_cmdio_write(p_sess, FTP_BADSENDCONN, str_to_const_char("Use PORT or PASV first."));
    return 0;
   }

  if  0 != tunable_ssl_enable && 0 == p_sess.data_use_ssl &&
      ( ( 0 != tunable_force_local_data_ssl && 0 == p_sess.is_anonymous ) ||
        ( 0 != tunable_force_anon_data_ssl  && 0 != p_sess.is_anonymous ) )
   {
    vsf_cmdio_write(p_sess, FTP_NEEDENCRYPT, str_to_const_char("Data connections must be encrypted."));
    return 0;
   }

  return 1;
 }

#[no_mangle]
pub unsafe extern "C" fn prepend_path_to_filename (p_str: &mystr)
{
  let s_tmp_str: mystr = default_mystr();
  // Only prepend current working directory if the incoming filename is
  // relative
  str_empty(&s_tmp_str);
  if 0 != str_isempty(p_str) || str_get_char_at(p_str, 0) != '/' as c_char
  {
    str_getcwd(&s_tmp_str);
    // Careful to not emit // if we are in directory / (common with chroot)
    if 0 != str_isempty(&s_tmp_str) ||
        str_get_char_at(&s_tmp_str, str_getlen(&s_tmp_str) - 1) != '/' as c_char
    {
      str_append_char(&s_tmp_str, '/' as c_char);
    }
  }
  str_append_str(&s_tmp_str, p_str);
  str_copy(p_str, &s_tmp_str);
}

#[no_mangle]
pub unsafe extern "C" fn get_remote_transfer_fd (p_sess: &mut vsf_session, p_status_msg: *const c_char ) -> c_int
 {
  let mut remote_fd: c_int;

  if 0 == pasv_active(p_sess) && 0 == port_active(p_sess)
   {
    bug(str_to_const_char("neither PORT nor PASV active in get_remote_transfer_fd"));
   }

  p_sess.abor_received = 0;
  if 0 != pasv_active(p_sess)
   {
    remote_fd = vsf_ftpdataio_get_pasv_fd(p_sess);
   }
  else
   {
    remote_fd = vsf_ftpdataio_get_port_fd(p_sess);
   }

  if 0 != vsf_sysutil_retval_is_error(remote_fd)
   {
    return remote_fd;
   }

  vsf_cmdio_write(p_sess, FTP_DATACONN, p_status_msg);
  if vsf_ftpdataio_post_mark_connect(p_sess) != 1
   {
    vsf_ftpdataio_dispose_transfer_fd(p_sess);
    return -1;
   }

  return remote_fd;
}

#[no_mangle]
pub unsafe extern "C" fn check_abor (p_sess: &mut vsf_session) {
  // If the client sent ABOR, respond to it here
  if 0 != p_sess.abor_received
   {
    p_sess.abor_received = 0;
    vsf_cmdio_write(p_sess, FTP_ABOROK, str_to_const_char("ABOR successful."));
   }

}


#[no_mangle]
pub unsafe extern "C" fn handle_retr (p_sess: &mut vsf_session, is_http: c_int) {
  let mut s_mark_str:   mystr = default_mystr();// mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};
  let mut  s_p_statbuf: *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut trans_ret:    vsf_transfer_ret = vsf_transfer_ret { retval:0, transferred: 0};

  let remote_fd: c_int;
  let opened_file: c_int;
  let is_ascii: c_int = 0;

  let offset: filesize_t = p_sess.restart_pos;
  p_sess.restart_pos = 0;

  if 0 == is_http && 0 == data_transfer_checks_ok(p_sess)
   {
    return;
   }

  if 0 != p_sess.is_ascii && 0 != offset
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL,
                    str_to_const_char("No support for resume of ASCII transfer."));
    return;
   }

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryDownload);
  str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&p_sess.log_str);

  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied."));
    return;
   }

  opened_file = str_open(&p_sess.ftp_arg_str, EVSFSysStrOpenMode_kVSFSysStrOpenReadOnly);
  if 0 != vsf_sysutil_retval_is_error(opened_file)
  {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file."));
    return;
  }

  // Lock file if required
  if 0 != tunable_lock_upload_files
   {
    vsf_sysutil_lock_file_read(opened_file);
   }

  vsf_sysutil_fstat(opened_file, &mut s_p_statbuf);

  // No games please
  if 0 == vsf_sysutil_statbuf_is_regfile(s_p_statbuf)
  {
    // Note - pretend open failed
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file."));
    // Irritating FireFox does RETR on directories, so avoid logging this
    // very common and noisy case.
    if 0 != vsf_sysutil_statbuf_is_dir(s_p_statbuf)
     {
      vsf_log_clear_entry(p_sess);
     }
    goto file_close_out;
  }

  // Now deactive O_NONBLOCK, otherwise we have a problem on DMAPI filesystems
  // such as XFS DMAPI.
  vsf_sysutil_deactivate_noblock(opened_file);
  // Optionally, we'll be paranoid and only serve publicly readable stuff
  if 0 != p_sess.is_anonymous && 0 != tunable_anon_world_readable_only &&
      0 == vsf_sysutil_statbuf_is_readable_other(s_p_statbuf)
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file."));
    goto file_close_out;
   }

  // Set the download offset (from REST) if any
  if 0 != offset
   {
    vsf_sysutil_lseek_to(opened_file, offset);
   }

  str_alloc_text(&s_mark_str, str_to_const_char("Opening "));

  if 0 != tunable_ascii_download_enable && 0 != p_sess.is_ascii
   {
    str_append_text(&s_mark_str, str_to_const_char("ASCII"));
    is_ascii = 1;
   }
  else
   {
    str_append_text(&s_mark_str, str_to_const_char("BINARY"));
   }

  str_append_text(&s_mark_str, str_to_const_char(" mode data connection for "));
  str_append_str(&s_mark_str, &p_sess.ftp_arg_str);
  str_append_text(&s_mark_str, str_to_const_char(" ("));
  str_append_filesize_t(&s_mark_str,
                        vsf_sysutil_statbuf_get_size(s_p_statbuf));
  str_append_text(&s_mark_str, str_to_const_char(" bytes)."));

  if 0 != is_http
   {
    remote_fd = VSFTP_COMMAND_FD;
   }
  else
   {
    remote_fd = get_remote_transfer_fd(p_sess, str_getbuf(&s_mark_str));
    if 0 != vsf_sysutil_retval_is_error(remote_fd)
     {
      goto port_pasv_cleanup_out;
     }
   }

  trans_ret = vsf_ftpdataio_transfer_file(p_sess, remote_fd,
                                          opened_file, 0, is_ascii);
  if 0 == is_http &&
      1 != vsf_ftpdataio_dispose_transfer_fd(p_sess) &&
      0 == trans_ret.retval
   {
    trans_ret.retval = -2;
   }
  p_sess.transfer_size = trans_ret.transferred;
  // Log _after_ the blocking dispose call, so we get transfer times right
  if 0 == trans_ret.retval
   {
    vsf_log_do_log(p_sess, 1);
   }

  if 0 != is_http
   {
    goto file_close_out;
   }

  // Emit status message _after_ blocking dispose call to avoid buggy FTP
  // clients truncating the transfer.
  if -1 == trans_ret.retval
   {
    vsf_cmdio_write(p_sess, FTP_BADSENDFILE, str_to_const_char("Failure reading local file."));
   }
  else if -2 == trans_ret.retval
   {
    if 0 == p_sess.data_timeout
     {
      vsf_cmdio_write(p_sess, FTP_BADSENDNET,
                      str_to_const_char("Failure writing network stream."));
     }
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_TRANSFEROK, str_to_const_char("Transfer complete."));
   }

  check_abor(p_sess);
//  port_pasv_cleanup_out:
  port_cleanup(p_sess);
  pasv_cleanup(p_sess);
//  file_close_out:
  vsf_sysutil_close(opened_file);
}

