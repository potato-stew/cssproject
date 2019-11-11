
use bindings;

pub extern "C" fn str_getcwd_rust(mystr* p_str)  {
  static char* p_getcwd_buf;
  char* p_ret;
  if (p_getcwd_buf == 0)
  {
    vsf_secbuf_alloc(&p_getcwd_buf, VSFTP_PATH_MAX);
  }
  /* In case getcwd() fails */
  str_empty(p_str);
  p_ret = vsf_sysutil_getcwd(p_getcwd_buf, VSFTP_PATH_MAX);
  if (p_ret != 0)
  {
    str_alloc_text(p_str, p_getcwd_buf);
  }
}
