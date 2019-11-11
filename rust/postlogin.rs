
mod bindings;

use bindings::str_to_const_char;

use bindings::mystr;

use bindings::str_replace_text;
use bindings::str_alloc_text;
use bindings::str_append_str;
use bindings::str_append_text;
use bindings::vsf_cmdio_write_str;

use bindings::FTP_PWDOK;

/* Private local functions */
/*
static void handle_pwd(struct vsf_session* p_sess);
static void handle_cwd(struct vsf_session* p_sess);
static void handle_pasv(struct vsf_session* p_sess, int is_epsv);
static void handle_retr(struct vsf_session* p_sess, int is_http);
static void handle_cdup(struct vsf_session* p_sess);
static void handle_list(struct vsf_session* p_sess);
static void handle_type(struct vsf_session* p_sess);
static void handle_port(struct vsf_session* p_sess);
static void handle_stor(struct vsf_session* p_sess);
static void handle_mkd(struct vsf_session* p_sess);
static void handle_rmd(struct vsf_session* p_sess);
static void handle_dele(struct vsf_session* p_sess);
static void handle_rest(struct vsf_session* p_sess);
static void handle_rnfr(struct vsf_session* p_sess);
static void handle_rnto(struct vsf_session* p_sess);
static void handle_nlst(struct vsf_session* p_sess);
static void handle_size(struct vsf_session* p_sess);
static void handle_site(struct vsf_session* p_sess);
static void handle_appe(struct vsf_session* p_sess);
static void handle_mdtm(struct vsf_session* p_sess);
static void handle_site_chmod(struct vsf_session* p_sess,
                              struct mystr* p_arg_str);
static void handle_site_umask(struct vsf_session* p_sess,
                              struct mystr* p_arg_str);
static void handle_eprt(struct vsf_session* p_sess);
static void handle_help(struct vsf_session* p_sess);
static void handle_stou(struct vsf_session* p_sess);
static void handle_stat(struct vsf_session* p_sess);
static void handle_stat_file(struct vsf_session* p_sess);
static void handle_logged_in_user(struct vsf_session* p_sess);
static void handle_logged_in_pass(struct vsf_session* p_sess);
static void handle_http(struct vsf_session* p_sess);

static int pasv_active(struct vsf_session* p_sess);
static int port_active(struct vsf_session* p_sess);
static void pasv_cleanup(struct vsf_session* p_sess);
static void port_cleanup(struct vsf_session* p_sess);
static void handle_dir_common(struct vsf_session* p_sess, int full_details,
                              int stat_cmd);
static void prepend_path_to_filename(struct mystr* p_str);
static int get_remote_transfer_fd(struct vsf_session* p_sess,
                                  const char* p_status_msg);
static void check_abor(struct vsf_session* p_sess);
static void handle_sigurg(void* p_private);
static void handle_upload_common(struct vsf_session* p_sess, int is_append,
                                 int is_unique);
static void get_unique_filename(struct mystr* p_outstr,
                                const struct mystr* p_base);
static int data_transfer_checks_ok(struct vsf_session* p_sess);
static void resolve_tilde(struct mystr* p_str, struct vsf_session* p_sess);
*/

fn handle_pwd () {
	mystr s_cwd_buf_mangle_str;
	mystr s_pwd_res_str;
	str_getcwd(&s_cwd_buf_mangle_str);

    /* Double up any double-quotes in the pathname! */
	str_replace_text(&s_cwd_buf_mangle_str, str_to_const_char("\""), str_to_const_char("\"\"") );
    str_alloc_text(&s_pwd_res_str, str_to_const_char("\"") );
    str_append_str(&s_pwd_res_str, &s_cwd_buf_mangle_str);
    str_append_text(&s_pwd_res_str, str_to_const_char("\" is the current directory") );
    vsf_cmdio_write_str(p_sess, FTP_PWDOK, &s_pwd_res_str);
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
