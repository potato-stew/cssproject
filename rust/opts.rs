/*
 * Part of Very Secure FTPd
 * Licence: GPL v2
 * Author: Chris Evans
 * opts.c
 *
 * Routines to handle OPTS.
 */

mod bindings_new;
use bindings_new::*;

use std::os::raw::*;

#[no_mangle]
pub unsafe extern "C" fn handle_opts (mut p_sess: &mut vsf_session ) {
	println!("handle_opts from Rust!");

	str_upper( &p_sess.ftp_arg_str );
	if str_equal_text( &(*p_sess).ftp_arg_str, str_to_const_char("UTF8 ON") ) != 0
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
