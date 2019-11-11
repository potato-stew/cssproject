
#include <str.h>

//void str_upper(struct mystr* p_str);

//void handle_opts (struct mystr* x) {}

void __attribute__((constructor)) main2 ()
 {
  struct mystr s;
  str_upper(&s);
 }
