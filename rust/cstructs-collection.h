/////////////////////////////////////////////////////////////////////////////// parseconf.c

static const char* s_p_saved_filename;

static struct parseconf_bool_setting
{
  const char* p_setting_name;
  int* p_variable;
}
parseconf_bool_array[] =
{
  { "anonymous_enable", &tunable_anonymous_enable },
  { "local_enable", &tunable_local_enable },
  { "pasv_enable", &tunable_pasv_enable },
  { "port_enable", &tunable_port_enable },
  { "chroot_local_user", &tunable_chroot_local_user },
  { "write_enable", &tunable_write_enable },
  { "anon_upload_enable", &tunable_anon_upload_enable },
  { "anon_mkdir_write_enable", &tunable_anon_mkdir_write_enable },
  { "anon_other_write_enable", &tunable_anon_other_write_enable },
  { "chown_uploads", &tunable_chown_uploads },
  { "connect_from_port_20", &tunable_connect_from_port_20 },
  { "xferlog_enable", &tunable_xferlog_enable },
  { "dirmessage_enable", &tunable_dirmessage_enable },
  { "anon_world_readable_only", &tunable_anon_world_readable_only },
  { "async_abor_enable", &tunable_async_abor_enable },
  { "ascii_upload_enable", &tunable_ascii_upload_enable },
  { "ascii_download_enable", &tunable_ascii_download_enable },
  { "one_process_model", &tunable_one_process_model },
  { "xferlog_std_format", &tunable_xferlog_std_format },
  { "pasv_promiscuous", &tunable_pasv_promiscuous },
  { "deny_email_enable", &tunable_deny_email_enable },
  { "chroot_list_enable", &tunable_chroot_list_enable },
  { "setproctitle_enable", &tunable_setproctitle_enable },
  { "text_userdb_names", &tunable_text_userdb_names },
  { "ls_recurse_enable", &tunable_ls_recurse_enable },
  { "log_ftp_protocol", &tunable_log_ftp_protocol },
  { "guest_enable", &tunable_guest_enable },
  { "userlist_enable", &tunable_userlist_enable },
  { "userlist_deny", &tunable_userlist_deny },
  { "use_localtime", &tunable_use_localtime },
  { "check_shell", &tunable_check_shell },
  { "hide_ids", &tunable_hide_ids },
  { "listen", &tunable_listen },
  { "port_promiscuous", &tunable_port_promiscuous },
  { "passwd_chroot_enable", &tunable_passwd_chroot_enable },
  { "no_anon_password", &tunable_no_anon_password },
  { "tcp_wrappers", &tunable_tcp_wrappers },
  { "use_sendfile", &tunable_use_sendfile },
  { "force_dot_files", &tunable_force_dot_files },
  { "listen_ipv6", &tunable_listen_ipv6 },
  { "dual_log_enable", &tunable_dual_log_enable },
  { "syslog_enable", &tunable_syslog_enable },
  { "background", &tunable_background },
  { "virtual_use_local_privs", &tunable_virtual_use_local_privs },
  { "session_support", &tunable_session_support },
  { "download_enable", &tunable_download_enable },
  { "dirlist_enable", &tunable_dirlist_enable },
  { "chmod_enable", &tunable_chmod_enable },
  { "secure_email_list_enable", &tunable_secure_email_list_enable },
  { "run_as_launching_user", &tunable_run_as_launching_user },
  { "no_log_lock", &tunable_no_log_lock },
  { "ssl_enable", &tunable_ssl_enable },
  { "allow_anon_ssl", &tunable_allow_anon_ssl },
  { "force_local_logins_ssl", &tunable_force_local_logins_ssl },
  { "force_local_data_ssl", &tunable_force_local_data_ssl },
  { "ssl_sslv2", &tunable_sslv2 },
  { "ssl_sslv3", &tunable_sslv3 },
  { "ssl_tlsv1", &tunable_tlsv1 },
  { "tilde_user_enable", &tunable_tilde_user_enable },
  { "force_anon_logins_ssl", &tunable_force_anon_logins_ssl },
  { "force_anon_data_ssl", &tunable_force_anon_data_ssl },
  { "mdtm_write", &tunable_mdtm_write },
  { "lock_upload_files", &tunable_lock_upload_files },
  { "pasv_addr_resolve", &tunable_pasv_addr_resolve },
  { "debug_ssl", &tunable_debug_ssl },
  { "require_cert", &tunable_require_cert },
  { "validate_cert", &tunable_validate_cert },
  { "strict_ssl_read_eof", &tunable_strict_ssl_read_eof },
  { "strict_ssl_write_shutdown", &tunable_strict_ssl_write_shutdown },
  { "ssl_request_cert", &tunable_ssl_request_cert },
  { "delete_failed_uploads", &tunable_delete_failed_uploads },
  { "implicit_ssl", &tunable_implicit_ssl },
  { "ptrace_sandbox", &tunable_ptrace_sandbox },
  { "require_ssl_reuse", &tunable_require_ssl_reuse },
  { "isolate", &tunable_isolate },
  { "isolate_network", &tunable_isolate_network },
  { "ftp_enable", &tunable_ftp_enable },
  { "http_enable", &tunable_http_enable },
  { "seccomp_sandbox", &tunable_seccomp_sandbox },
  { "allow_writeable_chroot", &tunable_allow_writeable_chroot },
  { 0, 0 }
};

static struct parseconf_uint_setting
{
  const char* p_setting_name;
  unsigned int* p_variable;
}
parseconf_uint_array[] =
{
  { "accept_timeout", &tunable_accept_timeout },
  { "connect_timeout", &tunable_connect_timeout },
  { "local_umask", &tunable_local_umask },
  { "anon_umask", &tunable_anon_umask },
  { "ftp_data_port", &tunable_ftp_data_port },
  { "idle_session_timeout", &tunable_idle_session_timeout },
  { "data_connection_timeout", &tunable_data_connection_timeout },
  { "pasv_min_port", &tunable_pasv_min_port },
  { "pasv_max_port", &tunable_pasv_max_port },
  { "anon_max_rate", &tunable_anon_max_rate },
  { "local_max_rate", &tunable_local_max_rate },
  { "listen_port", &tunable_listen_port },
  { "max_clients", &tunable_max_clients },
  { "file_open_mode", &tunable_file_open_mode },
  { "max_per_ip", &tunable_max_per_ip },
  { "trans_chunk_size", &tunable_trans_chunk_size },
  { "delay_failed_login", &tunable_delay_failed_login },
  { "delay_successful_login", &tunable_delay_successful_login },
  { "max_login_fails", &tunable_max_login_fails },
  { "chown_upload_mode", &tunable_chown_upload_mode },
  { 0, 0 }
};

static struct parseconf_str_setting
{
  const char* p_setting_name;
  const char** p_variable;
}
parseconf_str_array[] =
{
  { "secure_chroot_dir", &tunable_secure_chroot_dir },
  { "ftp_username", &tunable_ftp_username },
  { "chown_username", &tunable_chown_username },
  { "xferlog_file", &tunable_xferlog_file },
  { "vsftpd_log_file", &tunable_vsftpd_log_file },
  { "message_file", &tunable_message_file },
  { "nopriv_user", &tunable_nopriv_user },
  { "ftpd_banner", &tunable_ftpd_banner },
  { "banned_email_file", &tunable_banned_email_file },
  { "chroot_list_file", &tunable_chroot_list_file },
  { "pam_service_name", &tunable_pam_service_name },
  { "guest_username", &tunable_guest_username },
  { "userlist_file", &tunable_userlist_file },
  { "anon_root", &tunable_anon_root },
  { "local_root", &tunable_local_root },
  { "banner_file", &tunable_banner_file },
  { "pasv_address", &tunable_pasv_address },
  { "listen_address", &tunable_listen_address },
  { "user_config_dir", &tunable_user_config_dir },
  { "listen_address6", &tunable_listen_address6 },
  { "cmds_allowed", &tunable_cmds_allowed },
  { "hide_file", &tunable_hide_file },
  { "deny_file", &tunable_deny_file },
  { "user_sub_token", &tunable_user_sub_token },
  { "email_password_file", &tunable_email_password_file },
  { "rsa_cert_file", &tunable_rsa_cert_file },
  { "dsa_cert_file", &tunable_dsa_cert_file },
  { "ssl_ciphers", &tunable_ssl_ciphers },
  { "rsa_private_key_file", &tunable_rsa_private_key_file },
  { "dsa_private_key_file", &tunable_dsa_private_key_file },
  { "ca_certs_file", &tunable_ca_certs_file },
  { "cmds_denied", &tunable_cmds_denied },
  { 0, 0 }
};


/////////////////////////////////////////////////////////////////////////////// ptracessandbox.c

#if defined(__linux__) && defined(__i386__)

#include <sys/mman.h>
#include <sys/prctl.h>
#include <sys/ptrace.h>
/* For AF_MAX (NPROTO is defined to this) */
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/user.h>
#include <sys/wait.h>
#include <err.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <syslog.h>

#include <asm/unistd.h>

#ifndef __NR_sendfile64
  #define __NR_sendfile64 239
#endif

#ifndef __NR_exit_group
  #define __NR_exit_group 252
#endif

#ifndef __NR_utimes
  #define __NR_utimes 271
#endif

/* For the socketcall() multiplex args. */
#include <linux/net.h>

#ifndef PTRACE_SETOPTIONS
  #define PTRACE_SETOPTIONS 0x4200
#endif

#ifndef PTRACE_O_TRACESYSGOOD
  #define PTRACE_O_TRACESYSGOOD 1
#endif

#ifndef PTRACE_O_TRACEFORK
  #define PTRACE_O_TRACEFORK 2
#endif

#ifndef PTRACE_O_TRACEVFORK
  #define PTRACE_O_TRACEVFORK 4
#endif

#ifndef PTRACE_O_TRACECLONE
  #define PTRACE_O_TRACECLONE 8
#endif

#ifndef O_DIRECT
  #define O_DIRECT 040000
#endif

#define MAX_SYSCALL 300

struct pt_sandbox
{
  int read_event_fd;
  int write_event_fd;
  pid_t pid;
  int is_allowed[MAX_SYSCALL];
  ptrace_sandbox_validator_t validator[MAX_SYSCALL];
  void* validator_arg[MAX_SYSCALL];
  int is_exit;
  struct user_regs_struct regs;
  int is_socketcall_allowed[NPROTO];
  ptrace_sandbox_validator_t socketcall_validator[NPROTO];
  void* socketcall_validator_arg[NPROTO];
};

static int s_sigchld_fd = -1;

#endif /* __linux__ && __i386__ */


/////////////////////////////////////////////////////////////////////////////// seccompsandbox.c

#if defined(__linux__) && defined(__x86_64__)

#include <errno.h>

#include <netinet/in.h>
#include <netinet/tcp.h>

#include <sys/fcntl.h>
#include <sys/mman.h>
#include <sys/prctl.h>
#include <sys/socket.h>
#include <sys/types.h>

#include <linux/filter.h>

#include <asm/unistd.h>

/* #define DEBUG_SIGSYS 1 */

#ifndef PR_SET_SECCOMP
  #define PR_SET_SECCOMP 22
#endif

#ifndef PR_SET_NO_NEW_PRIVS
  #define PR_SET_NO_NEW_PRIVS 38
#endif

#ifndef __NR_openat
  #define __NR_openat 257
#endif

#ifndef O_LARGEFILE
  #define O_LARGEFILE 00100000
#endif

#ifndef O_DIRECTORY
  #define O_DIRECTORY 00200000
#endif

#ifndef O_CLOEXEC
  #define O_CLOEXEC 002000000
#endif

#define kMaxSyscalls 100

#ifdef DEBUG_SIGSYS

#include <signal.h>
#include <string.h>

#endif

static const int kOpenFlags =
    O_CREAT|O_EXCL|O_APPEND|O_NONBLOCK|O_DIRECTORY|O_CLOEXEC|O_LARGEFILE;

static size_t s_syscall_index;
static size_t s_1_arg_validations;
static size_t s_2_arg_validations;
static size_t s_3_arg_validations;
static int s_syscalls[kMaxSyscalls];
static int s_errnos[kMaxSyscalls];
static int s_args_1[kMaxSyscalls];
static int s_vals_1[kMaxSyscalls];
static int s_args_2[kMaxSyscalls];
static int s_vals_2[kMaxSyscalls];
static int s_args_3[kMaxSyscalls];
static int s_vals_3[kMaxSyscalls];

/////////////////////////////////////////////////////////////////////////////// ssl.c

#ifdef VSF_BUILD_SSL

#include <openssl/ssl.h>
#include <openssl/err.h>
#include <openssl/rand.h>
#include <openssl/bio.h>
#include <errno.h>
#include <limits.h>

static int ssl_inited;
static struct mystr debug_str;

#endif

/////////////////////////////////////////////////////////////////////////////// standalone.c

static unsigned int s_children;
static struct hash* s_p_ip_count_hash;
static struct hash* s_p_pid_ip_hash;
static unsigned int s_ipaddr_size;


/////////////////////////////////////////////////////////////////////////////// str.c

#define PRIVATE_HANDS_OFF_p_buf p_buf
#define PRIVATE_HANDS_OFF_len len
#define PRIVATE_HANDS_OFF_alloc_bytes alloc_bytes

/////////////////////////////////////////////////////////////////////////////// strlist.c

#define PRIVATE_HANDS_OFF_alloc_len alloc_len
#define PRIVATE_HANDS_OFF_list_len list_len
#define PRIVATE_HANDS_OFF_p_nodes p_nodes

struct mystr_list_node
{
  struct mystr str;
  struct mystr sort_key_str;
};

static const unsigned int kMaxStrlist = 10 * 1000 * 1000;

static struct mystr s_null_str;


/////////////////////////////////////////////////////////////////////////////// sysdeputil.c

#include "port/porting_junk.h"

#if (defined(__FreeBSD__) && __FreeBSD__ >= 3)
  #define _FILE_OFFSET_BITS 64
  #define _LARGEFILE_SOURCE 1
  #define _LARGEFILE64_SOURCE 1
#endif

/* For INT_MAX */
#include <limits.h>

/* For fd passing */
#include <sys/types.h>
#include <sys/socket.h>
/* For FreeBSD */
#include <sys/param.h>
#include <sys/uio.h>

#include <sys/prctl.h>
#include <signal.h>

/* Configuration.. here are the possibilities */
#undef VSF_SYSDEP_HAVE_CAPABILITIES
#undef VSF_SYSDEP_HAVE_SETKEEPCAPS
#undef VSF_SYSDEP_HAVE_SETPDEATHSIG
#undef VSF_SYSDEP_HAVE_LINUX_SENDFILE
#undef VSF_SYSDEP_HAVE_FREEBSD_SENDFILE
#undef VSF_SYSDEP_HAVE_HPUX_SENDFILE
#undef VSF_SYSDEP_HAVE_AIX_SENDFILE
#undef VSF_SYSDEP_HAVE_SETPROCTITLE
#undef VSF_SYSDEP_TRY_LINUX_SETPROCTITLE_HACK
#undef VSF_SYSDEP_HAVE_HPUX_SETPROCTITLE
#undef VSF_SYSDEP_HAVE_MAP_ANON
#undef VSF_SYSDEP_NEED_OLD_FD_PASSING
#undef VSF_SYSDEP_HAVE_LINUX_CLONE
#ifdef VSF_BUILD_PAM
  #define VSF_SYSDEP_HAVE_PAM
#endif
#define VSF_SYSDEP_HAVE_SHADOW
#define VSF_SYSDEP_HAVE_USERSHELL
#define VSF_SYSDEP_HAVE_LIBCAP
#define VSF_SYSDEP_HAVE_UTMPX

#define __USE_GNU
#include <utmpx.h>

/* BEGIN config */
#if defined(__linux__)
  #include <errno.h>
  #include <syscall.h>
  #define VSF_SYSDEP_HAVE_LINUX_CLONE
  #include <sched.h>
  #ifndef CLONE_NEWPID
    #define CLONE_NEWPID 0x20000000
  #endif
  #ifndef CLONE_NEWIPC
    #define CLONE_NEWIPC 0x08000000
  #endif
  #ifndef CLONE_NEWNET
    #define CLONE_NEWNET 0x40000000
  #endif
  #include <linux/unistd.h>
  #include <errno.h>
  #include <syscall.h>
#endif

#if defined(__linux__) && !defined(__ia64__) && !defined(__s390__)
  #define VSF_SYSDEP_TRY_LINUX_SETPROCTITLE_HACK
  #include <linux/version.h>
  #if defined(LINUX_VERSION_CODE) && defined(KERNEL_VERSION)
    #if (LINUX_VERSION_CODE >= KERNEL_VERSION(2,2,0))
      #define VSF_SYSDEP_HAVE_CAPABILITIES
      #define VSF_SYSDEP_HAVE_LINUX_SENDFILE
      #ifdef PR_SET_KEEPCAPS
        #define VSF_SYSDEP_HAVE_SETKEEPCAPS
      #endif
      #ifdef PR_SET_PDEATHSIG
        #define VSF_SYSDEP_HAVE_SETPDEATHSIG
      #endif
    #endif
  #endif
#endif

#if (defined(__FreeBSD__) && __FreeBSD__ >= 3)
  #define VSF_SYSDEP_HAVE_FREEBSD_SENDFILE
  #define VSF_SYSDEP_HAVE_SETPROCTITLE
#endif

#if defined(__NetBSD__)
  #include <stdlib.h>
  #define VSF_SYSDEP_HAVE_SETPROCTITLE
  #include <sys/param.h>
  #if __NetBSD_Version__ >= 106070000
    #define WTMPX_FILE _PATH_WTMPX
  #else
    #undef VSF_SYSDEP_HAVE_UTMPX
  #endif
#endif

#ifdef __hpux
  #include <sys/socket.h>
  #ifdef SF_DISCONNECT
    #define VSF_SYSDEP_HAVE_HPUX_SENDFILE
  #endif
  #include <sys/param.h>
  #include <sys/pstat.h>
  #ifdef PSTAT_SETCMD
    #define VSF_SYSDEP_HAVE_HPUX_SETPROCTITLE
  #endif
  #undef VSF_SYSDEP_HAVE_UTMPX
#endif

#include <unistd.h>
#include <sys/mman.h>
#ifdef MAP_ANON
  #define VSF_SYSDEP_HAVE_MAP_ANON
#endif

#ifdef __sgi
  #undef VSF_SYSDEP_HAVE_USERSHELL
  #undef VSF_SYSDEP_HAVE_LIBCAP
#endif

#ifdef _AIX
  #undef VSF_SYSDEP_HAVE_USERSHELL
  #undef VSF_SYSDEP_HAVE_LIBCAP
  #undef VSF_SYSDEP_HAVE_UTMPX
  #undef VSF_SYSDEP_HAVE_PAM
  #undef VSF_SYSDEP_HAVE_SHADOW
  #undef VSF_SYSDEP_HAVE_SETPROCTITLE
  #define VSF_SYSDEP_HAVE_AIX_SENDFILE
  #define VSF_SYSDEP_TRY_LINUX_SETPROCTITLE_HACK
  #define VSF_SYSDEP_HAVE_MAP_ANON
#endif

#ifdef __osf__
  #undef VSF_SYSDEP_HAVE_USERSHELL
#endif

#if (defined(__sgi) || defined(__hpux) || defined(__osf__))
  #define VSF_SYSDEP_NEED_OLD_FD_PASSING
#endif

#ifdef __sun
  #define VSF_SYSDEP_HAVE_SOLARIS_SENDFILE
#endif
/* END config */

/* PAM support - we include our own dummy version if the system lacks this */
#include <security/pam_appl.h>

/* No PAM? Try getspnam() with a getpwnam() fallback */
#ifndef VSF_SYSDEP_HAVE_PAM
/* This may hit our own "dummy" include and undef VSF_SYSDEP_HAVE_SHADOW */
#include <shadow.h>
#include <pwd.h>
#include <unistd.h>
#include <crypt.h>
#endif

/* Prefer libcap based capabilities over raw syscall capabilities */
#include <sys/capability.h>

#if defined(VSF_SYSDEP_HAVE_CAPABILITIES) && !defined(VSF_SYSDEP_HAVE_LIBCAP)
#include <linux/unistd.h>
#include <linux/capability.h>
#include <errno.h>
#include <syscall.h>
int capset(cap_user_header_t header, const cap_user_data_t data)
{
  return syscall(__NR_capset, header, data);
}
/* Gross HACK to avoid warnings - linux headers overlap glibc headers */
#undef __NFDBITS
#undef __FDMASK
#endif /* VSF_SYSDEP_HAVE_CAPABILITIES */

#if defined(VSF_SYSDEP_HAVE_LINUX_SENDFILE) || \
    defined(VSF_SYSDEP_HAVE_SOLARIS_SENDFILE)
#include <sys/sendfile.h>
#elif defined(VSF_SYSDEP_HAVE_FREEBSD_SENDFILE)
#include <sys/types.h>
#include <sys/socket.h>
#elif defined(VSF_SYSDEP_HAVE_HPUX_SENDFILE)
#include <sys/socket.h>
#else /* VSF_SYSDEP_HAVE_LINUX_SENDFILE */
#include <unistd.h>
#endif /* VSF_SYSDEP_HAVE_LINUX_SENDFILE */

#ifdef VSF_SYSDEP_HAVE_SETPROCTITLE
#include <sys/types.h>
#include <unistd.h>
#endif

#ifdef VSF_SYSDEP_TRY_LINUX_SETPROCTITLE_HACK
extern char** environ;
static unsigned int s_proctitle_space = 0;
static int s_proctitle_inited = 0;
static char* s_p_proctitle = 0;
#endif

#ifndef VSF_SYSDEP_HAVE_MAP_ANON
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
static int s_zero_fd = -1;
#endif

#if (defined(__sun) || defined(__hpux)) && \
    !defined(LINUX_PAM) && !defined(_OPENPAM)
/* Sun's PAM doesn't use const here, while Linux-PAM and OpenPAM do */
#define lo_const
#else
#define lo_const const
#endif
typedef lo_const void* pam_item_t;

static pam_handle_t* s_pamh;
static struct mystr s_pword_str;
static int pam_conv_func(int nmsg, const struct pam_message** p_msg,
  #define _FILE_OFFSET_BITS 64
  #define _LARGEFILE_SOURCE 1
  #define _LARGEFILE64_SOURCE 1
#endif

/* For INT_MAX */
#include <limits.h>

/* For fd passing */
#include <sys/types.h>
#include <sys/socket.h>
/* For FreeBSD */
#include <sys/param.h>
#include <sys/uio.h>

#include <sys/prctl.h>

/////////////////////////////////////////////////////////////////////////////// sysutil.c
#define PRIVATE_HANDS_OFF_syscall_retval syscall_retval
#define PRIVATE_HANDS_OFF_exit_status exit_status
#include "sysutil.h"
#include "utility.h"
#include "tunables.h"
#include "sysdeputil.h"

/* Activate 64-bit file support on Linux/32bit plus others */
#define _FILE_OFFSET_BITS 64
#define _LARGEFILE_SOURCE 1
#define _LARGEFILE64_SOURCE 1
#define _LARGE_FILES 1

/* For Linux, this adds nothing :-) */
#include "port/porting_junk.h"

#include <signal.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <stdio.h>
#include <dirent.h>
#include <time.h>
#include <arpa/inet.h>
#include <errno.h>
#include <pwd.h>
#include <grp.h>
#include <ctype.h>
#include <sys/wait.h>
#include <sys/time.h>
/* Must be before netinet/ip.h. Found on FreeBSD, Solaris */
#include <netinet/in_systm.h>
#include <netinet/ip.h>
#include <netinet/tcp.h>
#include <limits.h>
#include <syslog.h>
#include <utime.h>
#include <netdb.h>
#include <sys/resource.h>

/* Private variables to this file */
/* Current umask() */
static unsigned int s_current_umask;
/* Cached time */
static struct timeval s_current_time;
/* Current pid */
static int s_current_pid = -1;
/* Exit function */
static exitfunc_t s_exit_func;
/* Difference in timezone from GMT in seconds */
static long s_timezone;

/* Our internal signal handling implementation details */
static struct vsf_sysutil_sig_details
{
  vsf_sighandle_t sync_sig_handler;
  void* p_private;
  volatile sig_atomic_t pending;
  int running;
  int use_alarm;
} s_sig_details[NSIG];

static vsf_context_io_t s_io_handler;
static void* s_p_io_handler_private;
static int s_io_handler_running;

struct vsf_sysutil_sockaddr
{
  union
  {
    struct sockaddr u_sockaddr;
    struct sockaddr_in u_sockaddr_in;
    struct sockaddr_in6 u_sockaddr_in6;
  } u;
};


/////////////////////////////////////////////////////////////////////////////// tcpwarp.c
#include <tcpd.h>

#include <sys/syslog.h>

int deny_severity = LOG_WARNING;
int allow_severity = LOG_INFO;


/////////////////////////////////////////////////////////////////////////////// utility.c
#define DIE_DEBUG

