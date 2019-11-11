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

mod bindings;

use bindings::vsf_session;
use bindings::mystr;
use bindings::str_upper;
use bindings::str_equal_text;
use bindings::vsf_cmdio_write;

use bindings::FTP_OPTSOK;
use bindings::FTP_BADOPTS;

struct Point
 {
  x : i32,
  y : i32
 };

pub fn Translate (p: &Point) {

 (*p).
}

/*
extern "C" {
pub fn str_upper (p_str : * mut mystr) ;
pub fn vsf_cmdio_write (p_sess : * mut vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char) ;
pub fn str_equal_text (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;
}
*/

#[no_mangle]
pub unsafe extern "C" fn handle_opts (p_sess: *mut vsf_session ) {
	str_upper( &(*p_sess).ftp_arg_str );
	println!("handle_opts from Rust!");
	unsafe {
//	str_upper( mem::transmute::<&bindings::mystr,*mut bindings::mystr>(&(*p_sess).ftp_arg_str) );

    let UTF8_ON = CString::new("UTF8 ON").unwrap();
	if str_equal_text( mem::transmute::<&bindings::mystr,*mut bindings::mystr>(&(*p_sess).ftp_arg_str), UTF8_ON.as_ptr() ) != 0
	 {
	  let s = CString::new("Always in UTF8 mode.").unwrap();
	  vsf_cmdio_write(p_sess, FTP_OPTSOK, s.as_ptr());
	 }
	else
	 {
	  let s = CString::new("Option not understood.").unwrap();
	  vsf_cmdio_write(p_sess, FTP_BADOPTS, s.as_ptr());
	 }

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
