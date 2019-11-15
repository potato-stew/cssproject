
mod bindings_new;

use std::ptr;
use std::os::raw::*;

use bindings_new::*;

extern "C" {
pub fn handle_logged_in_user   (p_sess: &mut vsf_session);
pub fn handle_logged_in_pass   (p_sess: &mut vsf_session);
pub fn handle_stou   (p_sess: &mut vsf_session);
pub fn handle_eprt   (p_sess: &mut vsf_session);
pub fn handle_mdtm   (p_sess: &mut vsf_session);
pub fn handle_appe   (p_sess: &mut vsf_session);
pub fn handle_site   (p_sess: &mut vsf_session);
pub fn handle_nlst  (p_sess: &mut vsf_session);
pub fn handle_rnto   (p_sess: &mut vsf_session);
pub fn handle_rnfr   (p_sess: &mut vsf_session);
pub fn handle_rest   (p_sess: &mut vsf_session);
pub fn handle_dele   (p_sess: &mut vsf_session);
pub fn handle_rmd   (p_sess: &mut vsf_session);
pub fn handle_mkd   (p_sess: &mut vsf_session);
pub fn handle_stor   (p_sess: &mut vsf_session);
pub fn handle_port   (p_sess: &mut vsf_session);
pub fn handle_type   (p_sess: &mut vsf_session);
pub fn handle_help   (p_sess: &mut vsf_session);
pub fn handle_dir_common (p_sess: &mut vsf_session,a: c_int,b: c_int);
}

pub fn default_mystr() -> mystr {
  mystr { p_buf: ptr::null_mut(), alloc_bytes:0, len:0 }
}

pub fn default_sockaddr() -> sockaddr {
  sockaddr { sa_family: 3, sa_data: [0; 14usize] }
}

#[no_mangle]
pub unsafe extern "C" fn handle_stat (p_sess: &vsf_session) {
  vsf_cmdio_write_hyphen(p_sess, FTP_STATOK, str_to_const_char("FTP server status:"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("     Connected to "));
  vsf_cmdio_write_raw(p_sess, str_getbuf(&p_sess.remote_ip_str));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("     Logged in as "));
  vsf_cmdio_write_raw(p_sess, str_getbuf(&p_sess.user_str));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("     TYPE: "));

  if 0 != p_sess.is_ascii
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("ASCII\r\n"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("BINARY\r\n"));
   }

  if p_sess.bw_rate_max == 0
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     No session bandwidth limit\r\n"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Session bandwidth limit in byte/s is "));
    vsf_cmdio_write_raw(p_sess, vsf_sysutil_ulong_to_str(p_sess.bw_rate_max.into()));
    vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n"));
   }

  if tunable_idle_session_timeout == 0
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     No session timeout\r\n"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Session timeout in seconds is "));
    vsf_cmdio_write_raw(p_sess,
      vsf_sysutil_ulong_to_str(tunable_idle_session_timeout.into()));
    vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n"));
   }

  if 0 != p_sess.control_use_ssl
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Control connection is encrypted\r\n")); 
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Control connection is plain text\r\n")); 
   }

  if 0 != p_sess.data_use_ssl
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Data connections will be encrypted\r\n")); 
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Data connections will be plain text\r\n"));
   }

  if p_sess.num_clients > 0
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     At session startup, client count was "));
    vsf_cmdio_write_raw(p_sess, vsf_sysutil_ulong_to_str(p_sess.num_clients.into()));
    vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n"));
   }

  vsf_cmdio_write_raw(p_sess,str_to_const_char("     vsFTPd "));
//  str_to_const_char("     vsFTPd " VSF_VERSION " - secure, fast, stable\r\n"));
//  VSF_VERSION " - secure, fast, stable\r\n"));
//  str_to_const_char(" - secure, fast, stable\r\n"));
  vsf_cmdio_write(p_sess, FTP_STATOK, str_to_const_char("End of status"));
}

#[no_mangle]
pub unsafe extern "C" fn handle_stat_file (p_sess: &mut vsf_session) {
  handle_dir_common(p_sess, 1, 1);
}

#[no_mangle]
pub unsafe extern "C" fn handle_http (p_sess: &mut vsf_session) {
  // Warning: Doesn't respect cmds_allowed etc. because there is currently only
  // one command (GET)!
  // HTTP likely doesn't respect other important FTP options. I don't think
  // logging works.

  if 0 == tunable_download_enable
   {
    bug(str_to_const_char("HTTP needs download - fix your config"));
   }

  // Eat the HTTP headers, which we don't care about.
  loop
   {
    vsf_cmdio_get_cmd_and_arg(p_sess, &p_sess.ftp_cmd_str,
                              &p_sess.ftp_arg_str, 1);

    if 0 == str_isempty(&p_sess.ftp_cmd_str) ||
       0 == str_isempty(&p_sess.ftp_arg_str)
      {
       break;
      }
   }

  vsf_cmdio_write_raw(p_sess, str_to_const_char("HTTP/1.1 200 OK\r\n"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("Server: vsftpd\r\n"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("Connection: close\r\n"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("X-Frame-Options: SAMEORIGIN\r\n"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("X-Content-Type-Options: nosniff\r\n"));
  // Split the path from the HTTP/1.x
  str_split_char(&p_sess.http_get_arg, &p_sess.ftp_arg_str, ' ' as c_char);
  str_copy(&p_sess.ftp_arg_str, &p_sess.http_get_arg);
  str_split_char(&p_sess.http_get_arg, &p_sess.ftp_cmd_str, '.'  as c_char);
  str_upper(&p_sess.ftp_cmd_str);
  if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HTML")) ||
     0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HTM" ))
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("Content-Type: text/html\r\n"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("Content-Type: dunno\r\n"));
   }

  vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n"));
  p_sess.is_ascii = 0;
  p_sess.restart_pos = 0;
  handle_retr(p_sess, 1);

  if 0 != vsf_log_entry_pending(p_sess)
   {
    vsf_log_do_log(p_sess, 0);
   }

  vsf_sysutil_exit(0);
 }


#[no_mangle]
pub unsafe extern "C" fn resolve_tilde (p_str: &mystr, p_sess: &vsf_session) {
  let len : c_uint = str_getlen(p_str);
  if len > 0 && str_get_char_at(p_str, 0) == '~' as c_char
   {
    let mut s_rhs_str = default_mystr();
    if len == 1 || str_get_char_at(p_str, 1) == '/' as c_char
     {
      str_split_char(p_str, &s_rhs_str, '~' as c_char);
      str_copy(p_str, &p_sess.home_str);
      str_append_str(p_str, &s_rhs_str);
     }
    else if 0 != tunable_tilde_user_enable && len > 1
     {
      let mut s_user_str = default_mystr();
      let p_user : *mut vsf_sysutil_user;

      str_copy(&s_rhs_str, p_str);
      str_split_char(&s_rhs_str, &s_user_str, '~' as c_char);
      str_split_char(&s_user_str, &s_rhs_str, '/' as c_char);
      p_user = str_getpwnam(&s_user_str);

      if p_user != ptr::null_mut()
       {
        str_alloc_text(p_str, vsf_sysutil_user_get_homedir(p_user));
        if 0 == str_isempty(&s_rhs_str)
         {
          str_append_char(p_str, '/' as c_char);
          str_append_str(p_str, &s_rhs_str);
         }
       }
     }
   }
 }

#[no_mangle]
pub unsafe extern "C" fn handle_sigurg (p_private: *mut c_void) {
  let async_cmd_str = default_mystr();
  let async_arg_str = default_mystr();
  let real_cmd_str = default_mystr();

  let mut len: c_uint;

  let p_sess: *mut vsf_session = p_private as *mut _ as *mut vsf_session;
//  let p_sess : &mut vsf_session = mem::transmute <*mut vsf_session,&mut vsf_session> (p_sess_temp);

//  let p_sess: &mut vsf_session = mem::transmute <*c_void, &mut vsf_session> (p_private);
  //struct vsf_session*  = (struct vsf_session*) p_private;

  // Did stupid client sent something OOB without a data connection?
  if (*p_sess).data_fd == -1
   {
    return;
   }

  // Get the async command - blocks (use data timeout alarm)
  vsf_cmdio_get_cmd_and_arg(&(*p_sess), &async_cmd_str, &async_arg_str, 0);

  // Chop off first four characters; they are telnet characters. The client
  // should have sent the first two normally and the second two as urgent
  // data.
  len = str_getlen(&async_cmd_str);
  if len >= 4
   {
    str_right(&async_cmd_str, &real_cmd_str, len - 4);
   }

  if 0 != str_equal_text(&real_cmd_str, str_to_const_char("ABOR"))
   {
    (*p_sess).abor_received = 1;
    // This is failok because of a small race condition; the SIGURG might
    // be raised after the data socket is closed, but before data_fd is
    // set to -1.
    vsf_sysutil_shutdown_failok((*p_sess).data_fd);
   }
  else
   {
    // Sorry!
    vsf_cmdio_write(&(*p_sess), FTP_BADCMD, str_to_const_char("Unknown command."));
   }

  str_free(&async_cmd_str);
  str_free(&async_arg_str);
  str_free(&real_cmd_str);
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
  let mut is_ascii: c_int = 0;

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
    vsf_sysutil_close(opened_file);
    return;
   }

  // Now deactive O_NONBLOCK, otherwise we have a problem on DMAPI filesystems
  // such as XFS DMAPI.
  vsf_sysutil_deactivate_noblock(opened_file);
  // Optionally, we'll be paranoid and only serve publicly readable stuff
  if 0 != p_sess.is_anonymous && 0 != tunable_anon_world_readable_only &&
      0 == vsf_sysutil_statbuf_is_readable_other(s_p_statbuf)
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file."));
    vsf_sysutil_close(opened_file);
    return;
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

  let mut goto_port_pasv_cleanup = 0;
  if 0 != is_http
   {
    remote_fd = VSFTP_COMMAND_FD;
   }
  else
   {
    remote_fd = get_remote_transfer_fd(p_sess, str_getbuf(&s_mark_str));
    if 0 != vsf_sysutil_retval_is_error(remote_fd)
     {
      //goto port_pasv_cleanup_out;
      goto_port_pasv_cleanup = 1;
     }
   }

  if 0 == goto_port_pasv_cleanup
   {
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
      vsf_sysutil_close(opened_file);
      return;
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
   }

  port_cleanup(p_sess);
  pasv_cleanup(p_sess);
  vsf_sysutil_close(opened_file);

}

/*
#[no_mangle]
pub unsafe extern "C" fn process_post_login (p_sess: &mut vsf_session) {
  str_getcwd(&p_sess.home_str);

  if 0 != p_sess.is_anonymous
   {
    vsf_sysutil_set_umask(tunable_anon_umask);
    p_sess.bw_rate_max = tunable_anon_max_rate;
   }
  else
   {
    vsf_sysutil_set_umask(tunable_local_umask);
    p_sess.bw_rate_max = tunable_local_max_rate;
   }

  if 0 != p_sess.is_http
   {
    handle_http(p_sess);
    bug(str_to_const_char("should not be reached"));
   }

  // Don't support async ABOR if we have an SSL channel. The spec says SHOULD
  // NOT, and I think there are synchronization issues between command and
  // data reads.
  if 0 != tunable_async_abor_enable && 0 == p_sess.control_use_ssl
   {
    vsf_sysutil_install_sighandler(EVSFSysUtilSignal_kVSFSysUtilSigURG, Some(handle_sigurg), p_sess as &mut _ as *mut c_void, 0);
    vsf_sysutil_activate_sigurg(VSFTP_COMMAND_FD);
   }

  // Handle any login message
  vsf_banner_dir_changed(p_sess, FTP_LOGINOK);
  vsf_cmdio_write(p_sess, FTP_LOGINOK, str_to_const_char("Login successful."));

  while true
   {
    let mut cmd_ok : c_int = 1;
    if 0 != tunable_setproctitle_enable
     {
      vsf_sysutil_setproctitle(str_to_const_char("IDLE"));
     }

    // Blocks
    vsf_cmdio_get_cmd_and_arg(p_sess, &p_sess.ftp_cmd_str,
                              &p_sess.ftp_arg_str, 1);

    if 0 != tunable_setproctitle_enable
     {
      let mut proctitle_str: mystr  = default_mystr();
      str_copy(&proctitle_str, &p_sess.ftp_cmd_str);
      if 0 == str_isempty(&p_sess.ftp_arg_str)
       {
        str_append_char(&proctitle_str, ' ' as c_char);
        str_append_str(&proctitle_str, &p_sess.ftp_arg_str);
       }

      // Suggestion from Solar
      str_replace_unprintable(&proctitle_str, '?' as c_char);
      vsf_sysutil_setproctitle_str(&proctitle_str);
      str_free(&proctitle_str);
     }

    // Test command against the allowed lists..
    if ptr::null() != tunable_cmds_allowed
     {
      let mut s_src_str: mystr = default_mystr();
      let mut s_rhs_str: mystr = default_mystr();
      str_alloc_text(&s_src_str, tunable_cmds_allowed);

      while true
       {
        str_split_char(&s_src_str, &s_rhs_str, ',' as c_char);

        if 0 != str_isempty(&s_src_str)
         {
          cmd_ok = 0;
          break;
         }
        else if 0 != str_equal(&s_src_str, &p_sess.ftp_cmd_str)
         {
          break;
         }

        str_copy(&s_src_str, &s_rhs_str);
       }
     }

    if ptr::null() != tunable_cmds_denied
     {
      let mut s_src_str: mystr = default_mystr();
      let mut s_rhs_str: mystr = default_mystr();
      str_alloc_text(&s_src_str, tunable_cmds_denied);
      while true
       {
        str_split_char(&s_src_str, &s_rhs_str, ',' as c_char);

        if 0 != str_isempty(&s_src_str)
         {
          break;
         }
        else if 0 != str_equal(&s_src_str, &p_sess.ftp_cmd_str)
         {
          cmd_ok = 0;
          break;
         }

        str_copy(&s_src_str, &s_rhs_str);
       }
     }

    if 0 == cmd_ok
     {
      vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied."));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("QUIT"))
     {
      vsf_cmdio_write_exit(p_sess, FTP_GOODBYE, str_to_const_char("Goodbye."), 0);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PWD" )) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XPWD"))
     {
      handle_pwd(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("CWD" )) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XCWD"))
     {
      handle_cwd(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("CDUP")) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XCUP"))
     {
      handle_cdup(p_sess);
     }
    else if 0 != tunable_pasv_enable &&
            0 == p_sess.epsv_all &&
             ( 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PASV")) ||
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("P@SW")) )
     {
      handle_pasv(p_sess, 0);
     }
    else if 0 != tunable_pasv_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPSV"))
     {
      handle_pasv(p_sess, 1);
     }
    else if 0 != tunable_download_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RETR"))
     {
      handle_retr(p_sess, 0);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("NOOP"))
     {
      vsf_cmdio_write(p_sess, FTP_NOOPOK, str_to_const_char("NOOP ok."));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SYST"))
     {
      vsf_cmdio_write(p_sess, FTP_SYSTOK, str_to_const_char("UNIX Type: L8"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HELP"))
     {
      handle_help(p_sess);
     }
    else if 0 != tunable_dirlist_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("LIST"))
     {
      handle_list(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("TYPE"))
     {
      handle_type(p_sess);
     }
    else if 0 != tunable_port_enable &&
            0 == p_sess.epsv_all &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PORT"))
     {
      handle_port(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_upload_enable || 0 == p_sess.is_anonymous ) &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOR"))
     {
      handle_stor(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_mkdir_write_enable || 0 == p_sess.is_anonymous ) &&
             ( 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MKD" )) ||
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XMKD")) )
     {
      handle_mkd(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous ) &&
             ( 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RMD" )) ||
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XRMD")))
     {
      handle_rmd(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous ) &&
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("DELE"))
     {
      handle_dele(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("REST"))
     {
      handle_rest(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous) &&
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNFR"))
     {
      handle_rnfr(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous) &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNTO"))
     {
      handle_rnto(p_sess);
     }
    else if 0 != tunable_dirlist_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("NLST"))
     {
      handle_nlst(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SIZE"))
     {
      handle_size(p_sess);
     }
    else if 0 == p_sess.is_anonymous &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SITE"))
     {
      handle_site(p_sess);
     }
    // Note - the weird ABOR string is checking for an async ABOR arriving
    // without a SIGURG condition.
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ABOR")) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("\377\364\377\362ABOR"))
     {
      vsf_cmdio_write(p_sess, FTP_ABOR_NOCONN, str_to_const_char("No transfer to ABOR."));
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous) &&
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("APPE"))
     {
      handle_appe(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MDTM"))
     {
      handle_mdtm(p_sess);
     }
    else if 0 != tunable_port_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPRT"))
     {
      handle_eprt(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STRU"))
     {
      str_upper(&p_sess.ftp_arg_str);
      if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("F"))
       {
        vsf_cmdio_write(p_sess, FTP_STRUOK, str_to_const_char("Structure set to F."));
       }
      else
       {
        vsf_cmdio_write(p_sess, FTP_BADSTRU, str_to_const_char("Bad STRU command."));
       }
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MODE"))
     {
      str_upper(&p_sess.ftp_arg_str);
      if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("S"))
       {
        vsf_cmdio_write(p_sess, FTP_MODEOK, str_to_const_char("Mode set to S."));
       }
      else
       {
        vsf_cmdio_write(p_sess, FTP_BADMODE, str_to_const_char("Bad MODE command."));
       }
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_upload_enable || 0 == p_sess.is_anonymous ) &&
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOU"))
     {
      handle_stou(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ALLO"))
     {
      vsf_cmdio_write(p_sess, FTP_ALLOOK, str_to_const_char("ALLO command ignored."));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("REIN"))
     {
      vsf_cmdio_write(p_sess, FTP_COMMANDNOTIMPL, str_to_const_char("REIN not implemented."));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ACCT"))
     {
      vsf_cmdio_write(p_sess, FTP_COMMANDNOTIMPL, str_to_const_char("ACCT not implemented."));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SMNT"))
     {
      vsf_cmdio_write(p_sess, FTP_COMMANDNOTIMPL, str_to_const_char("SMNT not implemented."));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("FEAT"))
     {
      handle_feat(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("OPTS"))
     {
      handle_opts(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STAT")) &&
            0 != str_isempty(&p_sess.ftp_arg_str)
     {
      handle_stat(p_sess);
     }
    else if 0 != tunable_dirlist_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STAT"))
     {
      handle_stat_file(p_sess);
     }
    else if 0 != tunable_ssl_enable && 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PBSZ"))
     {
      handle_pbsz(p_sess);
     }
    else if 0 != tunable_ssl_enable && 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PROT"))
     {
      handle_prot(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("USER"))
     {
      handle_logged_in_user(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PASS"))
     {
      handle_logged_in_pass(p_sess);
     }
    else if
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PASV")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PORT")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOR")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MKD")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XMKD")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RMD")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XRMD")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("DELE")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNFR")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNTO")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SITE")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("APPE")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPSV")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPRT")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RETR")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("LIST")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("NLST")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOU")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ALLO")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("REIN")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ACCT")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SMNT")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("FEAT")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("OPTS")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STAT")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PBSZ")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PROT"))
     {
      vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied."));
     }
    else if 0 != str_isempty(&p_sess.ftp_cmd_str) &&
            0 != str_isempty(&p_sess.ftp_arg_str)
     {
       // Deliberately ignore to avoid NAT device bugs. ProFTPd does the same.
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("GET")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("POST")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HEAD")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("OPTIONS")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("CONNECT"))
     {
      vsf_cmdio_write_exit(p_sess, FTP_BADCMD,
                           str_to_const_char("HTTP protocol commands not allowed."), 1);
     }
    else
     {
      vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Unknown command."));
     }

    if 0 != vsf_log_entry_pending(p_sess)
     {
      vsf_log_do_log(p_sess, 0);
     }

    if 0 != p_sess.data_timeout
     {
      vsf_cmdio_write_exit(p_sess, FTP_DATA_TIMEOUT,
                           str_to_const_char("Data timeout. Reconnect. Sorry."), 1);
     }

  }
}
*/

