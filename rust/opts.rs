/*
 * Part of Very Secure FTPd
 * Licence: GPL v2
 * Author: Chris Evans
 * opts.c
 *
 * Routines to handle OPTS.
 */

use std::mem;
use std::ffi::CString;
use std::os::raw::c_char;
use std::str;
use std::os::raw::c_uint;

mod bindings;

use bindings::str_to_const_char;
use bindings::vsf_session;
use bindings::mystr;
use bindings::str_upper;
use bindings::str_equal_text;
use bindings::vsf_cmdio_write;

use bindings::FTP_OPTSOK;
use bindings::FTP_BADOPTS;

#[no_mangle]
pub unsafe extern "C" fn handle_opts (p_sess: &mut vsf_session ) {
	println!("handle_opts from Rust!");

	str_upper( &p_sess.ftp_arg_str );
	if str_equal_text( &p_sess.ftp_arg_str, str_to_const_char("UTF8 ON") ) != 0
	 {
	  vsf_cmdio_write(p_sess, FTP_OPTSOK, str_to_const_char("Always in UTF8 mode.") );
	 }
	else
	 {
	  vsf_cmdio_write(p_sess, FTP_BADOPTS, str_to_const_char("Option not understood.") );
	 }

}

/*
//#include "ftpcodes.h"
//#include "ftpcmdio.h"
//#include "session.h"

void
handle_opts(struct vsf_session* p_sess)
{
  str_upper(&p_sess->ftp_arg_str);
  if (str_equal_text(&p_sess->ftp_arg_str, "UTF8 ON"))
  {
    vsf_cmdio_write(p_sess, FTP_OPTSOK, "Always in UTF8 mode.");
  }
  else
  {
    vsf_cmdio_write(p_sess, FTP_BADOPTS, "Option not understood.");
  }
}
*/
