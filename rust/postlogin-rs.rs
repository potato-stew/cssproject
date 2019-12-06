
mod bindings_new;

use std::ptr;
use std::os::raw::*;
use std::convert::TryInto;

use bindings_new::*;

const postlogin_debug : bool = false;

fn default_mystr() -> mystr {
  mystr { p_buf: ptr::null_mut(), alloc_bytes:0, len:0 }
}

fn default_sockaddr() -> sockaddr {
  sockaddr { sa_family: 3, sa_data: [0; 14usize] }
}

#[no_mangle]
unsafe extern "C" fn handle_stor (p_sess: &mut vsf_session) {
  handle_upload_common(p_sess, 0, 0);
}

#[no_mangle]
unsafe extern "C" fn handle_list (p_sess: &mut vsf_session) {
  handle_dir_common(p_sess, 1, 0);
}

#[no_mangle]
unsafe extern "C" fn handle_rnfr (p_sess: &vsf_session) {
  let mut p_statbuf : *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut retval : c_int;

  // Clear old value
  str_free(&p_sess.rnfr_filename_str);
  resolve_tilde(&p_sess.ftp_arg_str, p_sess);

  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryRename);
    str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
    prepend_path_to_filename(&p_sess.log_str);
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  // Does it exist?
  retval = str_stat(& (p_sess.ftp_arg_str), &mut p_statbuf);
  if retval == 0
   {
    // Yes
    str_copy(&p_sess.rnfr_filename_str, &p_sess.ftp_arg_str);
    vsf_cmdio_write(p_sess, FTP_RNFROK, str_to_const_char("Ready for RNTO.\0"));
   }
  else
   {
    vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryRename);
    str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
    prepend_path_to_filename(&p_sess.log_str);
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("RNFR command failed.\0"));
   }

}

#[no_mangle]
unsafe extern "C" fn handle_rest (p_sess: &mut vsf_session) {

//  static struct mystr s_rest_str;
  let s_rest_str = default_mystr();
  let mut val : filesize_t = str_a_to_filesize_t(&p_sess.ftp_arg_str);

  if val < 0
   {
    val = 0;
   }

  p_sess.restart_pos = val;
  str_alloc_text(&s_rest_str, str_to_const_char("Restart position accepted (\0"));
  str_append_filesize_t(&s_rest_str, val);
  str_append_text(&s_rest_str, str_to_const_char(").\0"));
  vsf_cmdio_write_str(p_sess, FTP_RESTOK, &s_rest_str);
}

#[no_mangle]
unsafe extern "C" fn handle_dele (p_sess: &vsf_session) {
  let mut retval : c_int;

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryDelete);
  str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&p_sess.log_str);

  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  retval = str_unlink(&p_sess.ftp_arg_str);
  if retval != 0
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Delete operation failed.\0"));
   }
  else
   {
    vsf_log_do_log(p_sess, 1);
    vsf_cmdio_write(p_sess, FTP_DELEOK, str_to_const_char("Delete operation successful.\0"));
   }

}

#[no_mangle]
unsafe extern "C" fn handle_help (p_sess: &vsf_session) {
  vsf_cmdio_write_hyphen(p_sess, FTP_HELP,str_to_const_char("The following commands are recognized.\0"));
  vsf_cmdio_write_raw(p_sess,str_to_const_char(" ABOR ACCT ALLO APPE CDUP CWD  DELE EPRT EPSV FEAT HELP LIST MDTM MKD\r\n\0"));
  vsf_cmdio_write_raw(p_sess,str_to_const_char(" MODE NLST NOOP OPTS PASS PASV PORT PWD  QUIT REIN REST RETR RMD  RNFR\r\n\0"));
  vsf_cmdio_write_raw(p_sess,str_to_const_char(" RNTO SITE SIZE SMNT STAT STOR STOU STRU SYST TYPE USER XCUP XCWD XMKD\r\n\0"));
  vsf_cmdio_write_raw(p_sess,str_to_const_char(" XPWD XRMD\r\n\0"));
  vsf_cmdio_write(p_sess, FTP_HELP, str_to_const_char("Help OK.\0"));
}

/*
#[no_mangle]
unsafe extern "C" fn handle_port (p_sess: &vsf_session) {
  let mut the_port : c_ushort;
//  let vals = [0 as c_uchar,0,0,0,0,0];
  let mut vals : &[c_uchar] = &mut [0 as c_uchar,0,0,0,0,0];
 
  pasv_cleanup(p_sess);
  port_cleanup(p_sess);

  let mut p_raw : *const c_uchar = vsf_sysutil_parse_uchar_string_sep(&p_sess.ftp_arg_str, ',' as c_char, vals.as_mut_ptr(),
                                             size_of::<[c_uchar; 6]>().try_into().unwrap() );


  if p_raw == ptr::null_mut()
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Illegal PORT command.\0"));
    return;
   }

  let mut tmp = //: &mut vsf_sysutil_sockaddr =
    p_sess.p_port_sockaddr as *mut vsf_sysutil_sockaddr
       as *mut _ as &mut _;// as &mut vsf_sysutil_sockaddr;

  the_port = (((vals[4] << 8) | vals[5])).into();
//  vsf_sysutil_sockaddr_clone(&(p_sess.p_port_sockaddr), p_sess.p_local_addr);
  vsf_sysutil_sockaddr_clone(&(tmp as &vsf_sysutil_sockaddr), p_sess.p_local_addr);
  vsf_sysutil_sockaddr_set_ipv4addr(p_sess.p_port_sockaddr, vals.as_mut_ptr());
  vsf_sysutil_sockaddr_set_port(p_sess.p_port_sockaddr, the_port);
  // SECURITY:
  // 1) Reject requests not connecting to the control socket IP
  // 2) Reject connects to privileged ports
  
  if 0 == tunable_port_promiscuous
   {
    if 0 == vsf_sysutil_sockaddr_addr_equal(p_sess.p_remote_addr,p_sess.p_port_sockaddr) ||
       0 != vsf_sysutil_is_port_reserved(the_port)
     {
      vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Illegal PORT command.\0"));
      port_cleanup(p_sess);
      return;
     }
   }

  vsf_cmdio_write(p_sess, FTP_PORTOK,
                  str_to_const_char("PORT command successful. Consider using PASV.\0"));

}
*/

#[no_mangle]
unsafe extern "C" fn handle_rmd (p_sess: &vsf_session) {

  let mut retval: c_int;

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryRmdir);
  str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&p_sess.log_str);

  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  retval = str_rmdir(&p_sess.ftp_arg_str);
  if retval != 0
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL,
                    str_to_const_char("Remove directory operation failed.\0"));
   }
  else
   {
    vsf_log_do_log(p_sess, 1);
    vsf_cmdio_write(p_sess, FTP_RMDIROK,
                    str_to_const_char("Remove directory operation successful.\0"));
   }

}

#[no_mangle]
unsafe extern "C" fn handle_mkd (p_sess: &vsf_session) {
  let mut retval: c_int;

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryMkdir);
  str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&p_sess.log_str);

  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  // NOTE! Actual permissions will be governed by the tunable umask
  retval = str_mkdir(&p_sess.ftp_arg_str, 0777);
  if retval != 0
  {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL,
                    str_to_const_char("Create directory operation failed.\0"));
    return;
  }
  vsf_log_do_log(p_sess, 1);

   {
//    static struct mystr s_mkd_res;
//    static struct mystr s_tmp_str;
    let s_mkd_res = default_mystr();
    let s_tmp_str = default_mystr();

    str_copy(&s_tmp_str, &p_sess.ftp_arg_str);
    prepend_path_to_filename(&s_tmp_str);
    // Double up double quotes
    str_replace_text(&s_tmp_str, str_to_const_char("\""), str_to_const_char("\"\"\0"));
    // Build result string
    str_alloc_text(&s_mkd_res, str_to_const_char("\"\0"));
    str_append_str(&s_mkd_res, &s_tmp_str);
    str_append_text(&s_mkd_res, str_to_const_char("\" created\0"));
    vsf_cmdio_write_str(p_sess, FTP_MKDIROK, &s_mkd_res);
   }

}

unsafe fn print_char_array_len (a: *const i8, len: u32) {

  print!("\"\0");
  for i in 0..len{
    print!("{:#?}\0",*a.offset(i.try_into().unwrap()));
  }
  print!("\"\0");

}

unsafe fn print_char_array (a: *const i8) {

  print!("\"\0");

  let mut i=0;
  while a.offset(i) != ptr::null_mut() {
    print!("{:#?}\0",*a.offset(i.try_into().unwrap()));
    i = i +1;
  }

  print!("\"\0");
}

#[no_mangle]
unsafe extern "C" fn handle_type (p_sess: &mut vsf_session) {

  str_upper(&p_sess.ftp_arg_str);
  let cmp = str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("I\0"));

/*
  print!("handle_type: cmp={} I:\0",cmp);
  print_char_array(str_to_const_char("I\0"));
  print!(" arg_str:\0");
  print_char_array_len(p_sess.ftp_arg_str.p_buf,p_sess.ftp_arg_str.len);
  println!("\0");
*/

  if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("I\0"  )) ||
     0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("I\0" )) ||
     0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("L8\0" )) ||
     0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("L 8\0"))
   {
    p_sess.is_ascii = 0;
    vsf_cmdio_write(p_sess, FTP_TYPEOK, str_to_const_char("Switching to Binary mode.\0"));
   }
  else if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("A\0"  )) ||
          0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("A N\0"))
   {
    p_sess.is_ascii = 1;
    vsf_cmdio_write(p_sess, FTP_TYPEOK, str_to_const_char("Switching to ASCII mode.\0"));
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Unrecognised TYPE command.\0"));
   }
}

#[no_mangle]
unsafe extern "C" fn handle_stat (p_sess: &vsf_session) {
  vsf_cmdio_write_hyphen(p_sess, FTP_STATOK, str_to_const_char("FTP server status:\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("     Connected to \0"));
  vsf_cmdio_write_raw(p_sess, str_getbuf(&p_sess.remote_ip_str));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("     Logged in as \0"));
  vsf_cmdio_write_raw(p_sess, str_getbuf(&p_sess.user_str));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("     TYPE: \0"));

  if 0 != p_sess.is_ascii
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("ASCII\r\n\0"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("BINARY\r\n\0"));
   }

  if p_sess.bw_rate_max == 0
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     No session bandwidth limit\r\n\0"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Session bandwidth limit in byte/s is \0"));
    vsf_cmdio_write_raw(p_sess, vsf_sysutil_ulong_to_str(p_sess.bw_rate_max.into()));
    vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n\0"));
   }

  if tunable_idle_session_timeout == 0
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     No session timeout\r\n\0"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Session timeout in seconds is \0"));
    vsf_cmdio_write_raw(p_sess,
      vsf_sysutil_ulong_to_str(tunable_idle_session_timeout.into()));
    vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n\0"));
   }

  if 0 != p_sess.control_use_ssl
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Control connection is encrypted\r\n\0")); 
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Control connection is plain text\r\n\0")); 
   }

  if 0 != p_sess.data_use_ssl
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Data connections will be encrypted\r\n\0")); 
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     Data connections will be plain text\r\n\0"));
   }

  if p_sess.num_clients > 0
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("     At session startup, client count was \0"));
    vsf_cmdio_write_raw(p_sess, vsf_sysutil_ulong_to_str(p_sess.num_clients.into()));
    vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n\0"));
   }

  vsf_cmdio_write_raw(p_sess,str_to_const_char("     vsFTPd \0"));
  vsf_cmdio_write_raw(p_sess,str_to_const_char(VSF_VERSION));
  vsf_cmdio_write_raw(p_sess,str_to_const_char(" - secure, fast, stable\r\n\0"));
//  str_to_const_char("     vsFTPd " VSF_VERSION " - secure, fast, stable\r\n\0"));
  vsf_cmdio_write(p_sess, FTP_STATOK, str_to_const_char("End of status\0"));
}

#[no_mangle]
unsafe extern "C" fn handle_stat_file (p_sess: &mut vsf_session) {
  handle_dir_common(p_sess, 1, 1);
}

#[no_mangle]
unsafe extern "C" fn handle_http (p_sess: &mut vsf_session) {
  // Warning: Doesn't respect cmds_allowed etc. because there is currently only
  // one command (GET)!
  // HTTP likely doesn't respect other important FTP options. I don't think
  // logging works.

  if 0 == tunable_download_enable
   {
    bug(str_to_const_char("HTTP needs download - fix your config\0"));
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

  vsf_cmdio_write_raw(p_sess, str_to_const_char("HTTP/1.1 200 OK\r\n\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("Server: vsftpd\r\n\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("Connection: close\r\n\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("X-Frame-Options: SAMEORIGIN\r\n\0"));
  vsf_cmdio_write_raw(p_sess, str_to_const_char("X-Content-Type-Options: nosniff\r\n\0"));
  // Split the path from the HTTP/1.x
  str_split_char(&p_sess.http_get_arg, &p_sess.ftp_arg_str, ' ' as c_char);
  str_copy(&p_sess.ftp_arg_str, &p_sess.http_get_arg);
  str_split_char(&p_sess.http_get_arg, &p_sess.ftp_cmd_str, '.'  as c_char);
  str_upper(&p_sess.ftp_cmd_str);
  if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HTML\0")) ||
     0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HTM\0" ))
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("Content-Type: text/html\r\n\0"));
   }
  else
   {
    vsf_cmdio_write_raw(p_sess, str_to_const_char("Content-Type: dunno\r\n\0"));
   }

  vsf_cmdio_write_raw(p_sess, str_to_const_char("\r\n\0"));
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
unsafe extern "C" fn resolve_tilde (p_str: &mystr, p_sess: &vsf_session) {
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
unsafe extern "C" fn handle_sigurg (p_private: *mut c_void) {
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

  if 0 != str_equal_text(&real_cmd_str, str_to_const_char("ABOR\0"))
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
    vsf_cmdio_write(&(*p_sess), FTP_BADCMD, str_to_const_char("Unknown command.\0"));
   }

  str_free(&async_cmd_str);
  str_free(&async_arg_str);
  str_free(&real_cmd_str);
}

#[no_mangle]
extern "C" fn handle_pwd (p_sess: &mut vsf_session) {
	let mut s_cwd_buf_mangle_str: mystr = default_mystr();
	let mut s_pwd_res_str: mystr = default_mystr();

	unsafe {
    str_getcwd(&s_cwd_buf_mangle_str);

    /* Double up any double-quotes in the pathname! */
	  str_replace_text(&s_cwd_buf_mangle_str, str_to_const_char("\""), str_to_const_char("\"\"\0") );
    str_alloc_text(&s_pwd_res_str, str_to_const_char("\"\0") );
    str_append_str(&s_pwd_res_str, &s_cwd_buf_mangle_str);
    str_append_text(&s_pwd_res_str, str_to_const_char("\" is the current directory\0") );
    vsf_cmdio_write_str(p_sess, FTP_PWDOK, &s_pwd_res_str);

	}
}

#[no_mangle]
extern "C" fn handle_cwd (p_sess: &mut vsf_session) {

  unsafe {
  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  if  0 == vsf_access_check_file(&p_sess.ftp_arg_str) 
  {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0") );
    return;
  }

  let retval = str_chdir(&p_sess.ftp_arg_str);
  if retval == 0
  {
    /* Handle any messages */
    vsf_banner_dir_changed(p_sess, FTP_CWDOK);
    vsf_cmdio_write(p_sess, FTP_CWDOK, str_to_const_char("Directory successfully changed.\0") );
  }
  else
  {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to change directory.\0") );
  }

 }

}

#[no_mangle]
unsafe extern "C" fn handle_pasv (p_sess: &mut vsf_session,is_epsv: c_int) {
  let mut the_port : c_ushort;
  let mut s_pasv_res_str = default_mystr();// mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};

  let mut tmp_sockaddr  = default_sockaddr();// sockaddr { sa_family: 3, sa_data: [0; 14usize] };
  let mut tmp_vsf_sockaddr = vsf_sysutil_sockaddr__bindgen_ty_1 { u_sockaddr: tmp_sockaddr };
  let s_p_sockaddr : vsf_sysutil_sockaddr = vsf_sysutil_sockaddr { u: tmp_vsf_sockaddr };

  let mut is_ipv6: c_int = vsf_sysutil_sockaddr_is_ipv6(p_sess.p_local_addr);

  let cmp = str_isempty(&p_sess.ftp_arg_str);
  println!("cmp={}",cmp);
  if 0 != is_epsv && 0 == str_isempty(&p_sess.ftp_arg_str)
   {
    let mut argval: c_int;
    str_upper(&p_sess.ftp_arg_str);
    if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("ALL\0"))
     {
      p_sess.epsv_all = 1;
      vsf_cmdio_write(p_sess, FTP_EPSVALLOK, str_to_const_char("EPSV ALL ok.\0") );
      return;
     }

    argval = vsf_sysutil_atoi(str_getbuf(&p_sess.ftp_arg_str));
    if argval < 1 || argval > 2 || ( 0 == is_ipv6 && argval == 2 )
     {
      vsf_cmdio_write(p_sess, FTP_EPSVBAD, str_to_const_char("Bad network protocol.\0") );
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
    str_alloc_text(&s_pasv_res_str, str_to_const_char("Entering Extended Passive Mode (|||\0") );
    str_append_ulong(&s_pasv_res_str, the_port.into() );
    str_append_text(&s_pasv_res_str, str_to_const_char("|)\0") );
    vsf_cmdio_write_str(p_sess, FTP_EPSVOK, &s_pasv_res_str);
    return;
   }

  if tunable_pasv_address != ptr::null_mut()
   {
    vsf_sysutil_sockaddr_alloc_ipv4(&&s_p_sockaddr);
    // Report passive address as specified in configuration
    if vsf_sysutil_inet_aton(tunable_pasv_address, &s_p_sockaddr) == 0
     {
      die( str_to_const_char("invalid pasv_address\0") );
     }
   }
  else
   {
    vsf_sysutil_sockaddr_clone(&&s_p_sockaddr, p_sess.p_local_addr);
   }

  str_alloc_text(&s_pasv_res_str, str_to_const_char("Entering Passive Mode (\0") );
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
      str_append_text(&s_pasv_res_str, str_to_const_char("0,0,0,0\0") );
    }
  }
  str_replace_char(&s_pasv_res_str, '.' as c_char, ',' as c_char );
  str_append_text(&s_pasv_res_str, str_to_const_char(",\0") );

  str_append_ulong(&s_pasv_res_str, (the_port >> 8).into() );
//  let shift_the_port : u64 =  the_port >> 8;
//  str_append_ulong(&s_pasv_res_str, shift_the_port);

  str_append_text(&s_pasv_res_str, str_to_const_char(",\0") );
  str_append_ulong(&s_pasv_res_str, (the_port & 255).into() );
  str_append_text(&s_pasv_res_str, str_to_const_char(").\0") );
  vsf_cmdio_write_str(p_sess, FTP_PASVOK, &s_pasv_res_str);
}

#[no_mangle]
unsafe extern "C" fn port_cleanup (p_sess: &vsf_session) {
  vsf_sysutil_sockaddr_clear(&p_sess.p_port_sockaddr);
 }

#[no_mangle]
unsafe extern "C" fn pasv_cleanup (p_sess: &vsf_session) {

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
unsafe extern "C" fn handle_cdup (p_sess: &mut vsf_session) {
  str_alloc_text(&p_sess.ftp_arg_str, str_to_const_char("..\0") );
  handle_cwd(p_sess);
}

#[no_mangle]
unsafe extern "C" fn port_active (p_sess: &vsf_session) -> c_int {
  let mut ret : c_int = 0;

  if p_sess.p_port_sockaddr != ptr::null_mut()
   {
    ret = 1;
    if 0 != pasv_active(p_sess)
     {
      bug( str_to_const_char("port and pasv both active\0") );
     }
   }
  return ret;
}

#[no_mangle]
unsafe extern "C" fn pasv_active (p_sess: &vsf_session ) -> c_int {
  let mut ret : c_int;

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
      bug( str_to_const_char("pasv and port both active\0") );
     }
   }

  return ret;
}

#[no_mangle]
unsafe extern "C" fn data_transfer_checks_ok (p_sess: &mut vsf_session) -> c_int {

  if 0 == pasv_active(p_sess) && 0 == port_active(p_sess)
   {
    vsf_cmdio_write(p_sess, FTP_BADSENDCONN, str_to_const_char("Use PORT or PASV first.\0"));
    return 0;
   }

  if  0 != tunable_ssl_enable && 0 == p_sess.data_use_ssl &&
      ( ( 0 != tunable_force_local_data_ssl && 0 == p_sess.is_anonymous ) ||
        ( 0 != tunable_force_anon_data_ssl  && 0 != p_sess.is_anonymous ) )
   {
    vsf_cmdio_write(p_sess, FTP_NEEDENCRYPT, str_to_const_char("Data connections must be encrypted.\0"));
    return 0;
   }

  return 1;
 }

#[no_mangle]
unsafe extern "C" fn prepend_path_to_filename (p_str: &mystr)
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
unsafe extern "C" fn get_remote_transfer_fd (p_sess: &mut vsf_session, p_status_msg: *const c_char ) -> c_int
 {
  let mut remote_fd: c_int;

  if 0 == pasv_active(p_sess) && 0 == port_active(p_sess)
   {
    bug(str_to_const_char("neither PORT nor PASV active in get_remote_transfer_fd\0"));
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
unsafe extern "C" fn check_abor (p_sess: &mut vsf_session) {
  // If the client sent ABOR, respond to it here
  if 0 != p_sess.abor_received
   {
    p_sess.abor_received = 0;
    vsf_cmdio_write(p_sess, FTP_ABOROK, str_to_const_char("ABOR successful.\0"));
   }

}

#[no_mangle]
unsafe extern "C" fn handle_retr (p_sess: &mut vsf_session, is_http: c_int) {
  let mut s_mark_str:   mystr = default_mystr();// mystr { p_buf: ptr::null_mut(), alloc_bytes: 0, len: 0};
  let mut  s_p_statbuf: *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut trans_ret:    vsf_transfer_ret;// = vsf_transfer_ret { retval:0, transferred: 0};

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
                    str_to_const_char("No support for resume of ASCII transfer.\0"));
    return;
   }

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryDownload);
  str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&p_sess.log_str);

  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  opened_file = str_open(&p_sess.ftp_arg_str, EVSFSysStrOpenMode_kVSFSysStrOpenReadOnly);
  if 0 != vsf_sysutil_retval_is_error(opened_file)
  {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file.\0"));
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
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file.\0"));
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
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Failed to open file.\0"));
    vsf_sysutil_close(opened_file);
    return;
   }

  // Set the download offset (from REST) if any
  if 0 != offset
   {
    vsf_sysutil_lseek_to(opened_file, offset);
   }

  str_alloc_text(&s_mark_str, str_to_const_char("Opening \0"));

  if 0 != tunable_ascii_download_enable && 0 != p_sess.is_ascii
   {
    str_append_text(&s_mark_str, str_to_const_char("ASCII\0"));
    is_ascii = 1;
   }
  else
   {
    str_append_text(&s_mark_str, str_to_const_char("BINARY\0"));
   }

  str_append_text(&s_mark_str, str_to_const_char(" mode data connection for \0"));
  str_append_str(&s_mark_str, &p_sess.ftp_arg_str);
  str_append_text(&s_mark_str, str_to_const_char(" (\0"));
  str_append_filesize_t(&s_mark_str,
                        vsf_sysutil_statbuf_get_size(s_p_statbuf));
  str_append_text(&s_mark_str, str_to_const_char(" bytes).\0"));

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
      vsf_cmdio_write(p_sess, FTP_BADSENDFILE, str_to_const_char("Failure reading local file.\0"));
     }
    else if -2 == trans_ret.retval
     {
      if 0 == p_sess.data_timeout
       {
        vsf_cmdio_write(p_sess, FTP_BADSENDNET,
                        str_to_const_char("Failure writing network stream.\0"));
       }
     }
    else
     {
      vsf_cmdio_write(p_sess, FTP_TRANSFEROK, str_to_const_char("Transfer complete.\0"));
     }

    check_abor(p_sess);
   }

  port_cleanup(p_sess);
  pasv_cleanup(p_sess);
  vsf_sysutil_close(opened_file);

}

#[no_mangle]
unsafe extern "C" fn handle_nlst (p_sess: &mut vsf_session)
 {
  handle_dir_common(p_sess, 0, 0);
 }

#[no_mangle]
unsafe extern "C" fn handle_rnto (p_sess: &vsf_session)
 {
//  static struct mystr s_tmp_str;
  let s_tmp_str = default_mystr();
  let mut retval : c_int;

  /* If we didn't get a RNFR, throw a wobbly */
  if 0 != str_isempty(&p_sess.rnfr_filename_str)
   {
    vsf_cmdio_write(p_sess, FTP_NEEDRNFR,
                    str_to_const_char("RNFR required first.\0"));
    return;
   }

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryRename);
  str_copy(&p_sess.log_str, &p_sess.rnfr_filename_str);
  prepend_path_to_filename(&p_sess.log_str);
  str_append_char(&p_sess.log_str, ' ' as c_char);
  str_copy(&s_tmp_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&s_tmp_str);
  str_append_str(&p_sess.log_str, &s_tmp_str);
  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  /* NOTE - might overwrite destination file. Not a concern because the same
   * could be accomplished with DELE.
   */
  retval = str_rename(&p_sess.rnfr_filename_str, &p_sess.ftp_arg_str);
  /* Clear the RNFR filename; start the two stage process again! */
  str_free(&p_sess.rnfr_filename_str);
  if retval == 0
   {
    vsf_log_do_log(p_sess, 1);
    vsf_cmdio_write(p_sess, FTP_RENAMEOK, str_to_const_char("Rename successful.\0"));
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Rename failed.\0"));
   }

}


#[no_mangle]
unsafe extern "C" fn handle_upload_common (p_sess: &mut vsf_session, is_append: c_int,is_unique: c_int) {

  let mut s_p_statbuf: *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut s_filename = default_mystr();
  let mut p_filename : *mut mystr ;//= ptr::null_mut();// default_mystr();
  let mut trans_ret = vsf_transfer_ret { retval:0, transferred:0 };

  let mut new_file_fd : c_int;
  let mut remote_fd : c_int;
  let mut success : c_int = 0;
  let mut created : c_int;
  let mut do_truncate : c_int = 0;
  let mut offset : filesize_t  = p_sess.restart_pos;

  p_sess.restart_pos = 0;
  if 0 == data_transfer_checks_ok(p_sess)
   {
    return;
   }

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  p_filename = &mut p_sess.ftp_arg_str;// as & _ as *mut _ as *mut mystr;
  if 0 != is_unique
   {
//    get_unique_filename(&mut s_filename, p_filename );//as *mut _ as &mut _ as &mut mystr );
    p_filename = &mut s_filename;
   }

  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryUpload);
  str_copy(&p_sess.log_str, &p_sess.ftp_arg_str);
  prepend_path_to_filename(&p_sess.log_str);
  if 0 == vsf_access_check_file(p_filename)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }
  /* NOTE - actual file permissions will be governed by the tunable umask */
  /* XXX - do we care about race between create and chown() of anonymous
   * upload?
   */

  if 0 != is_unique || ( 0 != p_sess.is_anonymous && 0 == tunable_anon_other_write_enable )
   {
    new_file_fd = str_create_exclusive(p_filename);
   }
  else
   {
    /* For non-anonymous, allow open() to overwrite or append existing files */
    new_file_fd = str_create(p_filename);
    if 0 == is_append && offset == 0
     {
      do_truncate = 1;
     }
   }

  if 0 != vsf_sysutil_retval_is_error(new_file_fd)
   {
    vsf_cmdio_write(p_sess, FTP_UPLOADFAIL, str_to_const_char("Could not create file.\0"));
    return;
   }

  created = 1;
  vsf_sysutil_fstat(new_file_fd, &mut s_p_statbuf);
  if 0 != vsf_sysutil_statbuf_is_regfile(s_p_statbuf)
   {
    /* Now deactive O_NONBLOCK, otherwise we have a problem on DMAPI filesystems
     * such as XFS DMAPI.
     */
    vsf_sysutil_deactivate_noblock(new_file_fd);
   }

  /* Are we required to chown() this file for security? */
  if 0 != p_sess.is_anonymous && 0 != tunable_chown_uploads
   {
    vsf_sysutil_fchmod(new_file_fd, tunable_chown_upload_mode);
    if 0 != tunable_one_process_model
     {
      vsf_one_process_chown_upload(p_sess, new_file_fd);
     }
    else
     {
      vsf_two_process_chown_upload(p_sess, new_file_fd);
     }
   }

  /* Are we required to lock this file? */
  if 0 != tunable_lock_upload_files
   {
    vsf_sysutil_lock_file_write(new_file_fd);
   }

  /* Must truncate the file AFTER locking it! */
  if 0 != do_truncate
   {
    vsf_sysutil_ftruncate(new_file_fd);
    vsf_sysutil_lseek_to(new_file_fd, 0);
   }

  if 0 == is_append && offset != 0
   {
    /* XXX - warning, allows seek past end of file! Check for seek > size? */
    vsf_sysutil_lseek_to(new_file_fd, offset);
   }
  else if 0 != is_append
   {
    vsf_sysutil_lseek_end(new_file_fd);
   }

  if 0 != is_unique
   {
//    struct mystr resp_str = INIT_MYSTR;
    let mut resp_str = default_mystr();
    str_alloc_text(&resp_str, str_to_const_char("FILE: \0"));
    str_append_str(&resp_str, p_filename);
    remote_fd = get_remote_transfer_fd(p_sess, str_getbuf(&resp_str));
    str_free(&resp_str);
   }
  else
   {
    remote_fd = get_remote_transfer_fd(p_sess, str_to_const_char("Ok to send data.\0"));
   }

  if  0 != vsf_sysutil_retval_is_error(remote_fd)
   {
    port_cleanup(p_sess);
    pasv_cleanup(p_sess);
    if 0 != tunable_delete_failed_uploads && 0 != created && 0 == success
     {
      str_unlink(p_filename);
     }
    vsf_sysutil_close(new_file_fd);
   }

  if 0 != tunable_ascii_upload_enable && 0 != p_sess.is_ascii
   {
    trans_ret = vsf_ftpdataio_transfer_file(p_sess, remote_fd,
                                            new_file_fd, 1, 1);
   }
  else
   {
    trans_ret = vsf_ftpdataio_transfer_file(p_sess, remote_fd,
                                            new_file_fd, 1, 0);
   }

  if vsf_ftpdataio_dispose_transfer_fd(p_sess) != 1 && trans_ret.retval == 0
   {
    trans_ret.retval = -2;
   }

  p_sess.transfer_size = trans_ret.transferred;
  if trans_ret.retval == 0
   {
    success = 1;
    vsf_log_do_log(p_sess, 1);
   }

  if trans_ret.retval == -1
   {
    vsf_cmdio_write(p_sess, FTP_BADSENDFILE, str_to_const_char("Failure writing to local file.\0"));
   }
  else if trans_ret.retval == -2
   {
    if 0 == p_sess.data_timeout
     {
      vsf_cmdio_write(p_sess, FTP_BADSENDNET,
                      str_to_const_char("Failure reading network stream.\0"));
     }
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_TRANSFEROK, str_to_const_char("Transfer complete.\0"));
   }

  check_abor(p_sess);
//port_pasv_cleanup_out:
  port_cleanup(p_sess);
  pasv_cleanup(p_sess);
  if 0 != tunable_delete_failed_uploads && 0 != created && 0 == success
   {
    str_unlink(p_filename);
   }
  vsf_sysutil_close(new_file_fd);
}

#[no_mangle]
unsafe extern "C" fn handle_size (p_sess: &vsf_session) {
  /* Note - in ASCII mode, are supposed to return the size after taking into
   * account ASCII linefeed conversions. At least this is what wu-ftpd does in
   * version 2.6.1. Proftpd-1.2.0pre fails to do this.
   * I will not do it because it is a potential I/O DoS.
   */
  let mut s_p_statbuf : *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut retval : c_int;

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0") );
    return;
   }

  retval = str_stat(&p_sess.ftp_arg_str, &mut s_p_statbuf);
  if retval != 0 || 0 == vsf_sysutil_statbuf_is_regfile(s_p_statbuf)
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("Could not get file size.\0") );
   }
  else
   {
    let mut s_size_res_str = default_mystr();

    str_alloc_filesize_t(&s_size_res_str,
                         vsf_sysutil_statbuf_get_size(s_p_statbuf));
    vsf_cmdio_write_str(p_sess, FTP_SIZEOK, &s_size_res_str);
   }
}

#[no_mangle]
unsafe extern "C" fn handle_site (p_sess: &vsf_session) {
//  static struct mystr s_site_args_str;

  let mut s_site_args_str = default_mystr();
  /* What SITE sub-command is it? */
  str_split_char(&p_sess.ftp_arg_str, &s_site_args_str, ' ' as c_char);
  str_upper(&p_sess.ftp_arg_str);
  if 0 != tunable_write_enable &&
     0 != tunable_chmod_enable &&
     0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("CHMOD\0"))
   {
    handle_site_chmod(p_sess, &s_site_args_str);
   }
  else if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("UMASK\0"))
   {
    handle_site_umask(p_sess, &s_site_args_str);
   }
  else if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("HELP\0"))
   {
    if 0 != tunable_write_enable &&
       0 != tunable_chmod_enable
     {
      vsf_cmdio_write(p_sess, FTP_SITEHELP, str_to_const_char( "CHMOD UMASK HELP\0"));
     }
    else
     {
      vsf_cmdio_write(p_sess, FTP_SITEHELP, str_to_const_char("UMASK HELP\0"));
     }
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Unknown SITE command.\0"));
   }
}

#[no_mangle]
unsafe extern "C" fn handle_site_chmod (p_sess: &vsf_session, p_arg_str: &mystr) {
  let mut s_chmod_file_str = default_mystr();
  let mut perms : c_uint;
  let mut retval : c_int;

  if 0 != str_isempty(p_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("SITE CHMOD needs 2 arguments.\0"));
    return;
   }

  str_split_char(p_arg_str, &s_chmod_file_str, ' ' as c_char);
  if 0 != str_isempty(&s_chmod_file_str)
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("SITE CHMOD needs 2 arguments.\0"));
    return;
   }

  resolve_tilde(&s_chmod_file_str, p_sess);
  vsf_log_start_entry(p_sess, EVSFLogEntryType_kVSFLogEntryChmod);
  str_copy(&p_sess.log_str, &s_chmod_file_str);
  prepend_path_to_filename(&p_sess.log_str);
  str_append_char(&p_sess.log_str, ' ' as c_char);
  str_append_str(&p_sess.log_str, p_arg_str);
  if 0 == vsf_access_check_file(&s_chmod_file_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  /* Don't worry - our chmod() implementation only allows 0 - 0777 */
  perms = str_octal_to_uint(p_arg_str);
  retval = str_chmod(&s_chmod_file_str, perms);
  if 0 != vsf_sysutil_retval_is_error(retval)
   {
    vsf_cmdio_write(p_sess, FTP_FILEFAIL, str_to_const_char("SITE CHMOD command failed.\0"));
   }
  else
   {
    vsf_log_do_log(p_sess, 1);
    vsf_cmdio_write(p_sess, FTP_CHMODOK, str_to_const_char("SITE CHMOD command ok.\0"));
   }
}

#[no_mangle]
unsafe extern "C" fn handle_site_umask (p_sess: &vsf_session, p_arg_str: &mystr) {
//  static struct mystr s_umask_resp_str;
  let mut s_umask_resp_str = default_mystr();

  if 0 != str_isempty(p_arg_str)
   {
    /* Empty arg => report current umask */
    str_alloc_text(&s_umask_resp_str, str_to_const_char("Your current UMASK is \0"));
    str_append_text(&s_umask_resp_str,
                    vsf_sysutil_uint_to_octal(vsf_sysutil_get_umask()));
   }
  else
   {
    /* Set current umask */
//    unsigned int new_umask = str_octal_to_uint(p_arg_str);
    let mut new_umask : c_uint = str_octal_to_uint(p_arg_str);
    vsf_sysutil_set_umask(new_umask);
    str_alloc_text(&s_umask_resp_str, str_to_const_char("UMASK set to \0"));
    str_append_text(&s_umask_resp_str,
                    vsf_sysutil_uint_to_octal(vsf_sysutil_get_umask()));
   }
  vsf_cmdio_write_str(p_sess, FTP_UMASKOK, &s_umask_resp_str);
}

#[no_mangle]
unsafe extern "C" fn handle_appe (p_sess: &mut vsf_session) {
  handle_upload_common(p_sess, 1, 0);
}

#[no_mangle]
unsafe extern "C" fn handle_mdtm (p_sess: &vsf_session) {
  let mut s_filename_str = default_mystr();
  let mut s_p_statbuf : *mut vsf_sysutil_statbuf = ptr::null_mut();

  let mut do_write : c_int = 0;
  let mut modtime : c_long = 0;
 
  let mut loc : str_locate_result = str_locate_char(&p_sess.ftp_arg_str, ' ' as c_char);

  let mut retval : c_int = str_stat(&p_sess.ftp_arg_str, &mut s_p_statbuf);
  if 0 != tunable_mdtm_write && retval != 0 && 0 != loc.found &&
      0 != vsf_sysutil_isdigit(str_get_char_at(&p_sess.ftp_arg_str, 0).into())
   {
    if loc.index == 8 || loc.index == 14 ||
        (loc.index > 15 && str_get_char_at(&p_sess.ftp_arg_str, 14) == '.' as c_char)
     {
      do_write = 1;
     }
   }

  if do_write != 0
   {
    str_split_char(&p_sess.ftp_arg_str, &s_filename_str, ' ' as c_char);
    modtime = vsf_sysutil_parse_time(str_getbuf(&p_sess.ftp_arg_str));
    str_copy(&p_sess.ftp_arg_str, &s_filename_str);
   }

  resolve_tilde(&p_sess.ftp_arg_str, p_sess);
  if 0 == vsf_access_check_file(&p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
    return;
   }

  if 0 != do_write && 0 != tunable_write_enable &&
      ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous)
   {
    retval = str_stat(&p_sess.ftp_arg_str, &mut s_p_statbuf);
    if retval != 0 || 0 == vsf_sysutil_statbuf_is_regfile(s_p_statbuf)
     {
      vsf_cmdio_write(p_sess, FTP_FILEFAIL,
                      str_to_const_char("Could not set file modification time.\0"));
     }
    else
     {
      retval = vsf_sysutil_setmodtime(
        str_getbuf(&p_sess.ftp_arg_str), modtime, tunable_use_localtime);
      if retval != 0
       {
        vsf_cmdio_write(p_sess, FTP_FILEFAIL,
                        str_to_const_char("Could not set file modification time.\0"));
       }
      else
       {
        vsf_cmdio_write(p_sess, FTP_MDTMOK,
                        str_to_const_char("File modification time set.\0"));
       }
     }
   }
  else
   {
    if retval != 0 || 0 == vsf_sysutil_statbuf_is_regfile(s_p_statbuf)
     {
      vsf_cmdio_write(p_sess, FTP_FILEFAIL,
                      str_to_const_char("Could not get file modification time.\0"));
     }
    else
     {
//      static struct mystr s_mdtm_res_str;
      let mut s_mdtm_res_str = default_mystr();
      str_alloc_text(&s_mdtm_res_str,
                     vsf_sysutil_statbuf_get_numeric_date(
                       s_p_statbuf, tunable_use_localtime));
      vsf_cmdio_write_str(p_sess, FTP_MDTMOK, &s_mdtm_res_str);
     }
   }
}

#[no_mangle]
unsafe extern "C" fn handle_stou (p_sess: &mut vsf_session) {
  handle_upload_common(p_sess, 0, 1);
}

#[no_mangle]
unsafe extern "C" fn handle_logged_in_user (p_sess: &vsf_session) {
  if 0 != p_sess.is_anonymous
   {
    vsf_cmdio_write(p_sess, FTP_LOGINERR, str_to_const_char("Can't change from guest user.\0"));
   }
  else if 0 != str_equal(&p_sess.user_str, &p_sess.ftp_arg_str)
   {
    vsf_cmdio_write(p_sess, FTP_GIVEPWORD, str_to_const_char("Any password will do.\0"));
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_LOGINERR, str_to_const_char("Can't change to another user.\0"));
   }
}

#[no_mangle]
unsafe extern "C" fn handle_logged_in_pass (p_sess: &vsf_session) {
  vsf_cmdio_write(p_sess, FTP_LOGINOK, str_to_const_char("Already logged in.\0"));
}

#[no_mangle]
unsafe extern "C" fn handle_dir_common (p_sess: &mut vsf_session, full_details: c_int , stat_cmd: c_int ) {
  let mut s_option_str = default_mystr();
  let mut s_filter_str = default_mystr();
  let mut s_dir_name_str = default_mystr();
  let mut s_p_dirstat : *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut dir_allow_read : c_int = 0;
  let mut p_dir : *mut vsf_sysutil_dir = ptr::null_mut();

  let mut retval : c_int = 0;
  let mut use_control : c_int = 0;
  str_empty(&s_option_str);
  str_empty(&s_filter_str);
  /* By default open the current directory */
  str_alloc_text(&s_dir_name_str, str_to_const_char(".\0"));
  if 0 == stat_cmd && 0 == data_transfer_checks_ok(p_sess)
   {
    return;
   }
  /* Do we have an option? Going to be strict here - the option must come
   * first. e.g. "ls -a .." fine, "ls .. -a\0" not fine
   */
  if 0 == str_isempty(&p_sess.ftp_arg_str) &&
      str_get_char_at(&p_sess.ftp_arg_str, 0) == '-' as c_char
   {
    /* Chop off the '-' */
    str_mid_to_end(&p_sess.ftp_arg_str, &s_option_str, 1);
    /* A space will separate options from filter (if any) */
    str_split_char(&s_option_str, &s_filter_str, ' ' as c_char);
   }
  else
   {
    /* The argument, if any, is just a filter */
    str_copy(&s_filter_str, &p_sess.ftp_arg_str);
   }

  if 0 == str_isempty(&s_filter_str)
   {
    resolve_tilde(&s_filter_str, p_sess);
    if 0 == vsf_access_check_file(&s_filter_str)
     {
      vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
      return;
     }
    /* First check - is it an outright directory, as in "ls /pub\0" */
    p_dir = str_opendir(&s_filter_str);
    if p_dir != ptr::null_mut()
     {
      /* Listing a directory! */
      str_copy(&s_dir_name_str, &s_filter_str);
      str_free(&s_filter_str);
     }
    else
     {
      let locate_result : str_locate_result = str_locate_char(&s_filter_str, '/' as c_char);
      if 0 != locate_result.found
       {
        /* Includes a path! Reverse scan for / in the arg, to get the
         * base directory and filter (if any)
         */
        str_copy(&s_dir_name_str, &s_filter_str);
        str_split_char_reverse(&s_dir_name_str, &s_filter_str, '/' as c_char);
        /* If we have e.g. "ls /.message\0", we just ripped off the leading
         * slash because it is the only one!
         */
        if 0 != str_isempty(&s_dir_name_str)
         {
          str_alloc_text(&s_dir_name_str, str_to_const_char("/\0"));
         }
       }
     }
   }

  if p_dir == ptr::null_mut()
   {
    /* NOTE - failure check done below, it's not forgotten */
    p_dir = str_opendir(&s_dir_name_str);
   }
  /* Fine, do it */
  if 0 != stat_cmd
   {
    use_control = 1;
    str_append_char(&s_option_str, 'a' as c_char);
    vsf_cmdio_write_hyphen(p_sess, FTP_STATFILE_OK, str_to_const_char("Status follows:\0"));
   }
  else
   {
    let remote_fd : c_int = get_remote_transfer_fd(p_sess, str_to_const_char("Here comes the directory listing.\0"));
    if 0 != vsf_sysutil_retval_is_error(remote_fd)
     {
//      goto dir_close_out;
      if ptr::null_mut() != p_dir
       {
        vsf_sysutil_closedir(p_dir);
       }

      if 0 == stat_cmd
       {
        port_cleanup(p_sess);
        pasv_cleanup(p_sess);
       }
     }
   }

  if 0 != p_sess.is_anonymous && ptr::null_mut() != p_dir && 0 != tunable_anon_world_readable_only
   {
    vsf_sysutil_dir_stat(p_dir, &mut s_p_dirstat);
    if 0 == vsf_sysutil_statbuf_is_readable_other(s_p_dirstat)
     {
      dir_allow_read = 0;
     }
   }

  if p_dir != ptr::null_mut() && 0 != dir_allow_read
   {
    retval = vsf_ftpdataio_transfer_dir(p_sess, use_control, p_dir,
                                        &s_dir_name_str, &s_option_str,
                                        &s_filter_str, full_details);
   }

  if 0 == stat_cmd
   {
    if vsf_ftpdataio_dispose_transfer_fd(p_sess) != 1 && retval == 0
     {
      retval = -1;
     }
   }

  if 0 != stat_cmd
   {
    vsf_cmdio_write(p_sess, FTP_STATFILE_OK, str_to_const_char("End of status\0"));
   }
  else if retval != 0
   {
    if 0 == p_sess.data_timeout
    {
      vsf_cmdio_write(p_sess, FTP_BADSENDNET,
                      str_to_const_char("Failure writing network stream.\0"));
    }
   }
  else if p_dir == ptr::null_mut() || 0 == dir_allow_read
   {
    vsf_cmdio_write(p_sess, FTP_TRANSFEROK,
                    str_to_const_char("Transfer done (but failed to open directory).\0"));
   }
  else
   {
    vsf_cmdio_write(p_sess, FTP_TRANSFEROK, str_to_const_char("Directory send OK.\0"));
   }

  check_abor(p_sess);

//  dir_close_out:
  if ptr::null_mut() != p_dir
   {
    vsf_sysutil_closedir(p_dir);
   }

  if 0 == stat_cmd
   {
    port_cleanup(p_sess);
    pasv_cleanup(p_sess);
   }

}

#[no_mangle]
#[allow(exceeding_bitshifts)]
unsafe extern "C" fn handle_port (p_sess: &vsf_session) {
  let mut the_port : c_ushort;
  let mut vals : [c_uchar;6] = [0; 6];//0,0,0,0,0];//c_uchar[6];
  let mut p_raw : *const c_uchar = ptr::null_mut();
  pasv_cleanup(p_sess);
  port_cleanup(p_sess);
  p_raw = vsf_sysutil_parse_uchar_string_sep(&p_sess.ftp_arg_str, ',' as c_char, &mut vals[0],6);

  if p_raw == ptr::null_mut()
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Illegal PORT command.\0"));
    return;
   }

  the_port = /*(unsigned short) */ ( ((vals[4] << 8) | vals[5]) ).into();
  vsf_sysutil_sockaddr_clone(&(&*(p_sess.p_port_sockaddr as *mut _)), p_sess.p_local_addr);
  vsf_sysutil_sockaddr_set_ipv4addr(p_sess.p_port_sockaddr, &vals[0]);
  vsf_sysutil_sockaddr_set_port(p_sess.p_port_sockaddr, the_port);
  // SECURITY:
  // 1) Reject requests not connecting to the control socket IP
  // 2) Reject connects to privileged ports
  
  if 0 == tunable_port_promiscuous
   {
    if 0 == vsf_sysutil_sockaddr_addr_equal(p_sess.p_remote_addr,p_sess.p_port_sockaddr) ||
       0 != vsf_sysutil_is_port_reserved(the_port)
     {
      vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Illegal PORT command.\0"));
      port_cleanup(p_sess);
      return;
     }
   }
  vsf_cmdio_write(p_sess, FTP_PORTOK,
                  str_to_const_char("PORT command successful. Consider using PASV.\0"));
}

#[no_mangle]
unsafe extern "C" fn handle_eprt (p_sess: &vsf_session) {
  let s_part1_str = default_mystr();
  let s_part2_str = default_mystr();
  let s_scopeid_str = default_mystr();
  let mut proto : c_int = 0;
  let mut port : c_int = 0;
  let mut p_raw_addr : *const c_uchar = ptr::null_mut();
  let is_ipv6 : c_int = vsf_sysutil_sockaddr_is_ipv6(p_sess.p_local_addr);
  port_cleanup(p_sess);
  pasv_cleanup(p_sess);
  str_copy(&s_part1_str, &p_sess.ftp_arg_str);
  str_split_char(&s_part1_str, &s_part2_str, '|' as c_char);

  if 0 == str_isempty(&s_part1_str)
   {
//    goto bad_eprt;
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Bad EPRT command.\0"));
    return;
   }

  /* Split out the protocol and check it */
  str_split_char(&s_part2_str, &s_part1_str, '|' as c_char);
  proto = str_atoi(&s_part2_str);
  if proto < 1 || proto > 2 || (0 == is_ipv6 && proto == 2)
   {
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Bad EPRT protocol.\0"));
    return;
   }

  /* Split out address and parse it */
  str_split_char(&s_part1_str, &s_part2_str, '|' as c_char);
  if proto == 2
   {
    str_split_char(&s_part1_str, &s_scopeid_str, '%' as c_char);
    p_raw_addr = vsf_sysutil_parse_ipv6(&s_part1_str);
   }
  else
   {
    p_raw_addr = vsf_sysutil_parse_ipv4(&s_part1_str);
   }

  if p_raw_addr == ptr::null_mut()
//  if ptr::null_mut() == p_raw_addr
   {
//    goto bad_eprt;
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Bad EPRT command.\0"));
    return;
   }

  /* Split out port and parse it */
  str_split_char(&s_part2_str, &s_part1_str, '|' as c_char);
  if 0 == str_isempty(&s_part1_str) || 0 != str_isempty(&s_part2_str)
   {
//    goto bad_eprt;
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Bad EPRT command.\0"));
    return;
   }

  port = str_atoi(&s_part2_str);
  if port < 0 || port > 65535
   {
//    goto bad_eprt;
    vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Bad EPRT command.\0"));
    return;
   }

  vsf_sysutil_sockaddr_clone(&(&*(p_sess.p_port_sockaddr as *mut _)), p_sess.p_local_addr);
  if proto == 2
   {
    vsf_sysutil_sockaddr_set_ipv6addr(p_sess.p_port_sockaddr, p_raw_addr);
   }
  else
   {
    vsf_sysutil_sockaddr_set_ipv4addr(p_sess.p_port_sockaddr, p_raw_addr);
   }

  vsf_sysutil_sockaddr_set_port(p_sess.p_port_sockaddr, port.try_into().unwrap());
  /* SECURITY:
   * 1) Reject requests not connecting to the control socket IP
   * 2) Reject connects to privileged ports
   */
  if 0 == tunable_port_promiscuous
   {
    if 0 == vsf_sysutil_sockaddr_addr_equal(p_sess.p_remote_addr,p_sess.p_port_sockaddr) ||
       0 != vsf_sysutil_is_port_reserved(port.try_into().unwrap())
     {
      vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Illegal EPRT command.\0"));
      port_cleanup(p_sess);
      return;
     }
   }
  vsf_cmdio_write(p_sess, FTP_EPRTOK,
                  str_to_const_char("EPRT command successful. Consider using EPSV.\0"));
//  return;
//bad_eprt:
//  vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Bad EPRT command.\0"));
}


#[no_mangle]
unsafe extern "C" fn fnget_unique_filename (p_outstr: &mystr,p_base_str: &mystr) {
  /* Use silly wu-ftpd algorithm for compatibility. It has races of course, if
   * two sessions are using the same file prefix at the same time.
   */

  let mut s_p_statbuf : *mut vsf_sysutil_statbuf = ptr::null_mut();
  let mut s_stou_str = default_mystr();
  let mut suffix : c_uint = 1;
  let mut p_real_base_str : *const mystr = p_base_str;
//  const struct mystr* p_real_base_str = p_base_str;
  let mut retval : c_int = 0;

  if 0 != str_isempty(p_real_base_str)
   {
    str_alloc_text(&s_stou_str, str_to_const_char("STOU\0"));
    p_real_base_str = &s_stou_str;
   }
  else
   {
    /* Do not add any suffix at all if the name is not taken. */
    retval = str_stat(p_real_base_str, &mut s_p_statbuf);
    if 0 != vsf_sysutil_retval_is_error(retval)
     {
       str_copy(p_outstr, p_real_base_str);
       return;
     }
   }

  while true
   {
    str_copy(p_outstr, p_real_base_str);
    str_append_char(p_outstr, '.' as c_char);
    str_append_ulong(p_outstr, suffix.into());
    retval = str_stat(p_outstr, &mut s_p_statbuf);
    if 0 != vsf_sysutil_retval_is_error(retval)
     {
      return;
     }

    suffix = suffix + 1;
   }

}

/*
#[no_mangle]
unsafe extern "C" fn process_post_login (p_sess: &mut vsf_session) {
//  if postlogin_debug { println!("process_post_login\0"); }

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
    bug(str_to_const_char("should not be reached\0"));
   }

  // Don't support async ABOR if we have an SSL channel. The spec says SHOULD
  // NOT, and I think there are synchronization issues between command and
  // data reads.
  if 0 != tunable_async_abor_enable && 0 == p_sess.control_use_ssl
   {
    vsf_sysutil_install_sighandler(EVSFSysUtilSignal_kVSFSysUtilSigURG, Some(handle_sigurg), p_sess as &mut _ as *mut _ as *mut c_void, 0);
    vsf_sysutil_activate_sigurg(VSFTP_COMMAND_FD);
   }

  // Handle any login message
  vsf_banner_dir_changed(p_sess, FTP_LOGINOK);
//  if postlogin_debug { println!("vsf_cmdio_write\0"); }
    vsf_cmdio_write(p_sess, FTP_LOGINOK, str_to_const_char("Login successful.\0"));
  if postlogin_debug { println!("FTP_LOGINOK\0"); }

  while true
   {
    let mut cmd_ok : c_int = 1;
    if 0 != tunable_setproctitle_enable
     {
      vsf_sysutil_setproctitle(str_to_const_char("IDLE\0"));
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
      vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("QUIT\0"))
     {
      vsf_cmdio_write_exit(p_sess, FTP_GOODBYE, str_to_const_char("Goodbye.\0"), 0);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PWD\0" )) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XPWD\0"))
     {
      handle_pwd(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("CWD\0" )) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XCWD\0"))
     {
      handle_cwd(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("CDUP\0")) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XCUP\0"))
     {
      handle_cdup(p_sess);
     }
    else if 0 != tunable_pasv_enable &&
            0 == p_sess.epsv_all &&
             ( 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PASV\0")) ||
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("P@SW\0")) )
     {
      handle_pasv(p_sess, 0);
     }
    else if 0 != tunable_pasv_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPSV\0"))
     {
      handle_pasv(p_sess, 1);
     }
    else if 0 != tunable_download_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RETR\0"))
     {
      handle_retr(p_sess, 0);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("NOOP\0"))
     {
      vsf_cmdio_write(p_sess, FTP_NOOPOK, str_to_const_char("NOOP ok.\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SYST\0"))
     {
      vsf_cmdio_write(p_sess, FTP_SYSTOK, str_to_const_char("UNIX Type: L8\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HELP\0"))
     {
      handle_help(p_sess);
     }
    else if 0 != tunable_dirlist_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("LIST\0"))
     {
      handle_list(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("TYPE\0"))
     {
      handle_type(p_sess);
     }
    else if 0 != tunable_port_enable &&
            0 == p_sess.epsv_all &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PORT\0"))
     {
      handle_port(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_upload_enable || 0 == p_sess.is_anonymous ) &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOR\0"))
     {
      handle_stor(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_mkdir_write_enable || 0 == p_sess.is_anonymous ) &&
             ( 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MKD\0" )) ||
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XMKD\0")) )
     {
      handle_mkd(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous ) &&
             ( 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RMD\0" )) ||
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XRMD\0")))
     {
      handle_rmd(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous ) &&
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("DELE\0"))
     {
      handle_dele(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("REST\0"))
     {
      handle_rest(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous) &&
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNFR\0"))
     {
      handle_rnfr(p_sess);
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous) &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNTO\0"))
     {
      handle_rnto(p_sess);
     }
    else if 0 != tunable_dirlist_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("NLST\0"))
     {
      handle_nlst(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SIZE\0"))
     {
      handle_size(p_sess);
     }
    else if 0 == p_sess.is_anonymous &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SITE\0"))
     {
      handle_site(p_sess);
     }
    // Note - the weird ABOR string is checking for an async ABOR arriving
    // without a SIGURG condition.
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ABOR\0")) ||
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("\0o377\0o364\0o377\0o362ABOR\0"))
     {
      vsf_cmdio_write(p_sess, FTP_ABOR_NOCONN, str_to_const_char("No transfer to ABOR.\0"));
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_other_write_enable || 0 == p_sess.is_anonymous) &&
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("APPE\0"))
     {
      handle_appe(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MDTM\0"))
     {
      handle_mdtm(p_sess);
     }
    else if 0 != tunable_port_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPRT\0"))
     {
      handle_eprt(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STRU\0"))
     {
      str_upper(&p_sess.ftp_arg_str);
      if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("F\0"))
       {
        vsf_cmdio_write(p_sess, FTP_STRUOK, str_to_const_char("Structure set to F.\0"));
       }
      else
       {
        vsf_cmdio_write(p_sess, FTP_BADSTRU, str_to_const_char("Bad STRU command.\0"));
       }
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MODE\0"))
     {
      str_upper(&p_sess.ftp_arg_str);
      if 0 != str_equal_text(&p_sess.ftp_arg_str, str_to_const_char("S\0"))
       {
        vsf_cmdio_write(p_sess, FTP_MODEOK, str_to_const_char("Mode set to S.\0"));
       }
      else
       {
        vsf_cmdio_write(p_sess, FTP_BADMODE, str_to_const_char("Bad MODE command.\0"));
       }
     }
    else if 0 != tunable_write_enable &&
             ( 0 != tunable_anon_upload_enable || 0 == p_sess.is_anonymous ) &&
               0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOU\0"))
     {
      handle_stou(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ALLO\0"))
     {
      vsf_cmdio_write(p_sess, FTP_ALLOOK, str_to_const_char("ALLO command ignored.\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("REIN\0"))
     {
      vsf_cmdio_write(p_sess, FTP_COMMANDNOTIMPL, str_to_const_char("REIN not implemented.\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ACCT\0"))
     {
      vsf_cmdio_write(p_sess, FTP_COMMANDNOTIMPL, str_to_const_char("ACCT not implemented.\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SMNT\0"))
     {
      vsf_cmdio_write(p_sess, FTP_COMMANDNOTIMPL, str_to_const_char("SMNT not implemented.\0"));
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("FEAT\0"))
     {
      handle_feat(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("OPTS\0"))
     {
      handle_opts(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STAT\0")) &&
            0 != str_isempty(&p_sess.ftp_arg_str)
     {
      handle_stat(p_sess);
     }
    else if 0 != tunable_dirlist_enable &&
            0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STAT\0"))
     {
      handle_stat_file(p_sess);
     }
    else if 0 != tunable_ssl_enable && 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PBSZ\0"))
     {
      handle_pbsz(p_sess);
     }
    else if 0 != tunable_ssl_enable && 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PROT\0"))
     {
      handle_prot(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("USER\0"))
     {
      handle_logged_in_user(p_sess);
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PASS\0"))
     {
      handle_logged_in_pass(p_sess);
     }
    else if
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PASV\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PORT\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOR\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("MKD\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XMKD\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RMD\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("XRMD\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("DELE\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNFR\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RNTO\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SITE\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("APPE\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPSV\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("EPRT\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("RETR\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("LIST\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("NLST\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STOU\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ALLO\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("REIN\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("ACCT\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("SMNT\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("FEAT\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("OPTS\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("STAT\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PBSZ\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("PROT\0"))
     {
      vsf_cmdio_write(p_sess, FTP_NOPERM, str_to_const_char("Permission denied.\0"));
     }
    else if 0 != str_isempty(&p_sess.ftp_cmd_str) &&
            0 != str_isempty(&p_sess.ftp_arg_str)
     {
       // Deliberately ignore to avoid NAT device bugs. ProFTPd does the same.
     }
    else if 0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("GET\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("POST\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("HEAD\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("OPTIONS\0")) ||
             0 != str_equal_text(&p_sess.ftp_cmd_str, str_to_const_char("CONNECT\0"))
     {
      vsf_cmdio_write_exit(p_sess, FTP_BADCMD,
                           str_to_const_char("HTTP protocol commands not allowed.\0"), 1);
     }
    else
     {
      vsf_cmdio_write(p_sess, FTP_BADCMD, str_to_const_char("Unknown command.\0"));
     }

    if 0 != vsf_log_entry_pending(p_sess)
     {
      vsf_log_do_log(p_sess, 0);
     }

    if 0 != p_sess.data_timeout
     {
      vsf_cmdio_write_exit(p_sess, FTP_DATA_TIMEOUT,
                           str_to_const_char("Data timeout. Reconnect. Sorry.\0"), 1);
     }

  }
}
*/
