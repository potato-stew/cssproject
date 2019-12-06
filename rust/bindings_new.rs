use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;


//added own
pub extern "C" fn __WTERMSIG(status: :: std :: os :: raw :: c_int)-> :: std :: os :: raw :: c_int{return ((status) & 0x7f);}

pub extern "C" fn __WEXITSTATUS(status: :: std :: os :: raw :: c_int)-> :: std :: os :: raw :: c_int{return (((status) & 0xff00)>>8);}

pub extern "C" fn __WIFEXITED(status: :: std :: os :: raw :: c_int)-> :: std :: os :: raw :: c_int{return(__WTERMSIG(status)==0).into();}

pub extern "C" fn WIFEXITED(status: :: std :: os :: raw :: c_int)-> :: std :: os :: raw :: c_int{return __WIFEXITED(status);}

pub extern "C" fn WEXITSTATUS(status: :: std :: os :: raw :: c_int)-> :: std :: os :: raw :: c_int{return __WEXITSTATUS(status);}

pub type size_t = :: std :: os :: raw :: c_ulong;

extern "C"  {
  pub fn dirfd(a:*mut DIR)->:: std :: os :: raw :: c_int;
}

pub fn str_to_const_char (s: &str) -> *const c_char {
  return s.as_bytes().as_ptr() as *const _ as *const c_char;
//  return CString::new(s).unwrap().as_ptr();
 }


# [repr ( C )] # [derive ( Copy , Clone , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd )] pub struct __BindgenBitfieldUnit < Storage , Align > {
storage : Storage , align : [Align ; 0] ,
}
 impl < Storage , Align > __BindgenBitfieldUnit < Storage , Align > {
# [inline] pub const fn new (storage : Storage) -> Self {
Self {
storage , align : []
}

}

}
 impl < Storage , Align > __BindgenBitfieldUnit < Storage , Align > where Storage : AsRef < [u8] > + AsMut < [u8] >, {
# [inline] pub fn get_bit (& self , index : usize) -> bool {
debug_assert ! (index / 8 < self . storage . as_ref ( ) . len ( )) ;
 let byte_index = index / 8 ;
 let byte = self . storage . as_ref () [byte_index] ;
 let bit_index = if cfg ! (target_endian = "big") {
7 - (index % 8)
}
 else {
index % 8
}
 ;
 let mask = 1 << bit_index ;
 byte & mask == mask
}
 # [inline] pub fn set_bit (& mut self , index : usize , val : bool) {
debug_assert ! (index / 8 < self . storage . as_ref ( ) . len ( )) ;
 let byte_index = index / 8 ;
 let byte = & mut self . storage . as_mut () [byte_index] ;
 let bit_index = if cfg ! (target_endian = "big") {
7 - (index % 8)
}
 else {
index % 8
}
 ;
 let mask = 1 << bit_index ;
 if val {
* byte |= mask ;

}
 else {
* byte &= ! mask ;

}

}
 # [inline] pub fn get (& self , bit_offset : usize , bit_width : u8) -> u64 {
debug_assert ! (bit_width <= 64) ;
 debug_assert ! (bit_offset / 8 < self . storage . as_ref ( ) . len ( )) ;
 debug_assert ! (( bit_offset + ( bit_width as usize ) ) / 8 <= self . storage . as_ref ( ) . len ( )) ;
 let mut val = 0 ;
 for i in 0 .. (bit_width as usize) {
if self . get_bit (i + bit_offset) {
let index = if cfg ! (target_endian = "big") {
bit_width as usize - 1 - i
}
 else {
i
}
 ;
 val |= 1 << index ;

}

}
 val
}
 # [inline] pub fn set (& mut self , bit_offset : usize , bit_width : u8 , val : u64) {
debug_assert ! (bit_width <= 64) ;
 debug_assert ! (bit_offset / 8 < self . storage . as_ref ( ) . len ( )) ;
 debug_assert ! (( bit_offset + ( bit_width as usize ) ) / 8 <= self . storage . as_ref ( ) . len ( )) ;
 for i in 0 .. (bit_width as usize) {
let mask = 1 << i ;
 let val_bit_is_set = val & mask == mask ;
 let index = if cfg ! (target_endian = "big") {
bit_width as usize - 1 - i
}
 else {
i
}
 ;
 self . set_bit (index + bit_offset , val_bit_is_set) ;

}

}

}
 # [repr ( C )] # [derive ( Default )] pub struct __IncompleteArrayField < T > (:: std :: marker :: PhantomData < T > , [ T ; 0 ]) ;
 impl < T > __IncompleteArrayField < T > {
# [inline] pub const fn new () -> Self {
__IncompleteArrayField (:: std :: marker :: PhantomData , [ ])
}
 # [inline] pub unsafe fn as_ptr (& self) -> * const T {
:: std :: mem :: transmute (self)
}
 # [inline] pub unsafe fn as_mut_ptr (& mut self) -> * mut T {
:: std :: mem :: transmute (self)
}
 # [inline] pub unsafe fn as_slice (& self , len : usize) -> & [T] {
:: std :: slice :: from_raw_parts (self . as_ptr ( ) , len)
}
 # [inline] pub unsafe fn as_mut_slice (& mut self , len : usize) -> & mut [T] {
:: std :: slice :: from_raw_parts_mut (self . as_mut_ptr ( ) , len)
}

}
 impl < T > :: std :: fmt :: Debug for __IncompleteArrayField < T > {
fn fmt (& self , fmt : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result {
fmt . write_str ("__IncompleteArrayField")
}

}
 impl < T > :: std :: clone :: Clone for __IncompleteArrayField < T > {
# [inline] fn clone (& self) -> Self {
Self :: new ()
}

}
 pub const VSFTP_DEFAULT_CONFIG : & 'static [u8 ; 17usize] = b"/etc/vsftpd.conf\0" ;
 pub const VSFTP_COMMAND_FD : i32 = 0 ;
 pub const VSFTP_PASSWORD_MAX : u32 = 128 ;
 pub const VSFTP_USERNAME_MAX : u32 = 128 ;
 pub const VSFTP_MAX_COMMAND_LINE : u32 = 4096 ;
 pub const VSFTP_DATA_BUFSIZE : u32 = 65536 ;
 pub const VSFTP_DIR_BUFSIZE : u32 = 16384 ;
 pub const VSFTP_MATCHITERS_MAX : u32 = 1000 ;
 pub const VSFTP_PATH_MAX : u32 = 4096 ;
 pub const VSFTP_CONF_FILE_MAX : u32 = 100000 ;
 pub const VSFTP_LISTEN_BACKLOG : u32 = 32 ;
 pub const VSFTP_SECURE_UMASK : u32 = 63 ;
 pub const VSFTP_ROOT_UID : u32 = 0 ;
 pub const VSFTP_PRIVSOCK_MAXSTR : u32 = 131072 ;
 pub const VSFTP_AS_LIMIT : u32 = 209715200 ;
 pub const FTP_DATACONN : i32 = 150 ;
 pub const FTP_NOOPOK : i32 = 200 ;
 pub const FTP_TYPEOK : i32 = 200 ;
 pub const FTP_PORTOK : i32 = 200 ;
 pub const FTP_EPRTOK : i32 = 200 ;
 pub const FTP_UMASKOK : i32 = 200 ;
 pub const FTP_CHMODOK : i32 = 200 ;
 pub const FTP_EPSVALLOK : i32 = 200 ;
 pub const FTP_STRUOK : i32 = 200 ;
 pub const FTP_MODEOK : i32 = 200 ;
 pub const FTP_PBSZOK : i32 = 200 ;
 pub const FTP_PROTOK : i32 = 200 ;
 pub const FTP_OPTSOK : i32 = 200 ;
 pub const FTP_ALLOOK : i32 = 202 ;
 pub const FTP_FEAT : i32 = 211 ;
 pub const FTP_STATOK : i32 = 211 ;
 pub const FTP_SIZEOK : i32 = 213 ;
 pub const FTP_MDTMOK : i32 = 213 ;
 pub const FTP_STATFILE_OK : i32 = 213 ;
 pub const FTP_SITEHELP : i32 = 214 ;
 pub const FTP_HELP : i32 = 214 ;
 pub const FTP_SYSTOK : i32 = 215 ;
 pub const FTP_GREET : i32 = 220 ;
 pub const FTP_GOODBYE : i32 = 221 ;
 pub const FTP_ABOR_NOCONN : i32 = 225 ;
 pub const FTP_TRANSFEROK : i32 = 226 ;
 pub const FTP_ABOROK : i32 = 226 ;
 pub const FTP_PASVOK : i32 = 227 ;
 pub const FTP_EPSVOK : i32 = 229 ;
 pub const FTP_LOGINOK : i32 = 230 ;
 pub const FTP_AUTHOK : i32 = 234 ;
 pub const FTP_CWDOK : i32 = 250 ;
 pub const FTP_RMDIROK : i32 = 250 ;
 pub const FTP_DELEOK : i32 = 250 ;
 pub const FTP_RENAMEOK : i32 = 250 ;
 pub const FTP_PWDOK : i32 = 257 ;
 pub const FTP_MKDIROK : i32 = 257 ;
 pub const FTP_GIVEPWORD : i32 = 331 ;
 pub const FTP_RESTOK : i32 = 350 ;
 pub const FTP_RNFROK : i32 = 350 ;
 pub const FTP_IDLE_TIMEOUT : i32 = 421 ;
 pub const FTP_DATA_TIMEOUT : i32 = 421 ;
 pub const FTP_TOO_MANY_USERS : i32 = 421 ;
 pub const FTP_IP_LIMIT : i32 = 421 ;
 pub const FTP_IP_DENY : i32 = 421 ;
 pub const FTP_TLS_FAIL : i32 = 421 ;
 pub const FTP_BADSENDCONN : i32 = 425 ;
 pub const FTP_BADSENDNET : i32 = 426 ;
 pub const FTP_BADSENDFILE : i32 = 451 ;
 pub const FTP_BADCMD : i32 = 500 ;
 pub const FTP_BADOPTS : i32 = 501 ;
 pub const FTP_COMMANDNOTIMPL : i32 = 502 ;
 pub const FTP_NEEDUSER : i32 = 503 ;
 pub const FTP_NEEDRNFR : i32 = 503 ;
 pub const FTP_BADPBSZ : i32 = 503 ;
 pub const FTP_BADPROT : i32 = 503 ;
 pub const FTP_BADSTRU : i32 = 504 ;
 pub const FTP_BADMODE : i32 = 504 ;
 pub const FTP_BADAUTH : i32 = 504 ;
 pub const FTP_NOSUCHPROT : i32 = 504 ;
 pub const FTP_NEEDENCRYPT : i32 = 522 ;
 pub const FTP_EPSVBAD : i32 = 522 ;
 pub const FTP_DATATLSBAD : i32 = 522 ;
 pub const FTP_LOGINERR : i32 = 530 ;
 pub const FTP_NOHANDLEPROT : i32 = 536 ;
 pub const FTP_FILEFAIL : i32 = 550 ;
 pub const FTP_NOPERM : i32 = 550 ;
 pub const FTP_UPLOADFAIL : i32 = 553 ;
 pub const _SYS_TYPES_H : u32 = 1 ;
 pub const _FEATURES_H : u32 = 1 ;
 pub const _ISOC95_SOURCE : u32 = 1 ;
 pub const _ISOC99_SOURCE : u32 = 1 ;
 pub const _ISOC11_SOURCE : u32 = 1 ;
 pub const _POSIX_SOURCE : u32 = 1 ;
 pub const _POSIX_C_SOURCE : u32 = 200809 ;
 pub const _XOPEN_SOURCE : u32 = 700 ;
 pub const _XOPEN_SOURCE_EXTENDED : u32 = 1 ;
 pub const _LARGEFILE64_SOURCE : u32 = 1 ;
 pub const _DEFAULT_SOURCE : u32 = 1 ;
 pub const _ATFILE_SOURCE : u32 = 1 ;
 pub const __USE_ISOC11 : u32 = 1 ;
 pub const __USE_ISOC99 : u32 = 1 ;
 pub const __USE_ISOC95 : u32 = 1 ;
 pub const __USE_ISOCXX11 : u32 = 1 ;
 pub const __USE_POSIX : u32 = 1 ;
 pub const __USE_POSIX2 : u32 = 1 ;
 pub const __USE_POSIX199309 : u32 = 1 ;
 pub const __USE_POSIX199506 : u32 = 1 ;
 pub const __USE_XOPEN2K : u32 = 1 ;
 pub const __USE_XOPEN2K8 : u32 = 1 ;
 pub const __USE_XOPEN : u32 = 1 ;
 pub const __USE_XOPEN_EXTENDED : u32 = 1 ;
 pub const __USE_UNIX98 : u32 = 1 ;
 pub const _LARGEFILE_SOURCE : u32 = 1 ;
 pub const __USE_XOPEN2K8XSI : u32 = 1 ;
 pub const __USE_XOPEN2KXSI : u32 = 1 ;
 pub const __USE_LARGEFILE : u32 = 1 ;
 pub const __USE_LARGEFILE64 : u32 = 1 ;
 pub const __USE_MISC : u32 = 1 ;
 pub const __USE_ATFILE : u32 = 1 ;
 pub const __USE_GNU : u32 = 1 ;
 pub const __USE_FORTIFY_LEVEL : u32 = 0 ;
 pub const __GLIBC_USE_DEPRECATED_GETS : u32 = 1 ;
 pub const _STDC_PREDEF_H : u32 = 1 ;
 pub const __STDC_IEC_559__ : u32 = 1 ;
 pub const __STDC_IEC_559_COMPLEX__ : u32 = 1 ;
 pub const __STDC_ISO_10646__ : u32 = 201706 ;
 pub const __STDC_NO_THREADS__ : u32 = 1 ;
 pub const __GNU_LIBRARY__ : u32 = 6 ;
 pub const __GLIBC__ : u32 = 2 ;
 pub const __GLIBC_MINOR__ : u32 = 27 ;
 pub const _SYS_CDEFS_H : u32 = 1 ;
 pub const __glibc_c99_flexarr_available : u32 = 1 ;
 pub const __WORDSIZE : u32 = 64 ;
 pub const __WORDSIZE_TIME64_COMPAT32 : u32 = 1 ;
 pub const __SYSCALL_WORDSIZE : u32 = 64 ;
 pub const __HAVE_GENERIC_SELECTION : u32 = 0 ;
 pub const _BITS_TYPES_H : u32 = 1 ;
 pub const _BITS_TYPESIZES_H : u32 = 1 ;
 pub const __OFF_T_MATCHES_OFF64_T : u32 = 1 ;
 pub const __INO_T_MATCHES_INO64_T : u32 = 1 ;
 pub const __RLIM_T_MATCHES_RLIM64_T : u32 = 1 ;
 pub const __FD_SETSIZE : u32 = 1024 ;
 pub const __clock_t_defined : u32 = 1 ;
 pub const __clockid_t_defined : u32 = 1 ;
 pub const __time_t_defined : u32 = 1 ;
 pub const __timer_t_defined : u32 = 1 ;
 pub const _BITS_STDINT_INTN_H : u32 = 1 ;
 pub const __BIT_TYPES_DEFINED__ : u32 = 1 ;
 pub const _ENDIAN_H : u32 = 1 ;
 pub const __LITTLE_ENDIAN : u32 = 1234 ;
 pub const __BIG_ENDIAN : u32 = 4321 ;
 pub const __PDP_ENDIAN : u32 = 3412 ;
 pub const __BYTE_ORDER : u32 = 1234 ;
 pub const __FLOAT_WORD_ORDER : u32 = 1234 ;
 pub const LITTLE_ENDIAN : u32 = 1234 ;
 pub const BIG_ENDIAN : u32 = 4321 ;
 pub const PDP_ENDIAN : u32 = 3412 ;
 pub const BYTE_ORDER : u32 = 1234 ;
 pub const _BITS_BYTESWAP_H : u32 = 1 ;
 pub const _BITS_UINTN_IDENTITY_H : u32 = 1 ;
 pub const _SYS_SELECT_H : u32 = 1 ;
 pub const __FD_ZERO_STOS : & 'static [u8 ; 6usize] = b"stosq\0" ;
 pub const __sigset_t_defined : u32 = 1 ;
 pub const __timeval_defined : u32 = 1 ;
 pub const __timespec_defined : u32 = 1 ;
 pub const FD_SETSIZE : u32 = 1024 ;
 pub const _SYS_SYSMACROS_H : u32 = 1 ;
 pub const _BITS_SYSMACROS_H : u32 = 1 ;
 pub const _BITS_PTHREADTYPES_COMMON_H : u32 = 1 ;
 pub const _THREAD_SHARED_TYPES_H : u32 = 1 ;
 pub const _BITS_PTHREADTYPES_ARCH_H : u32 = 1 ;
 pub const __SIZEOF_PTHREAD_MUTEX_T : u32 = 40 ;
 pub const __SIZEOF_PTHREAD_ATTR_T : u32 = 56 ;
 pub const __SIZEOF_PTHREAD_RWLOCK_T : u32 = 56 ;
 pub const __SIZEOF_PTHREAD_BARRIER_T : u32 = 32 ;
 pub const __SIZEOF_PTHREAD_MUTEXATTR_T : u32 = 4 ;
 pub const __SIZEOF_PTHREAD_COND_T : u32 = 48 ;
 pub const __SIZEOF_PTHREAD_CONDATTR_T : u32 = 4 ;
 pub const __SIZEOF_PTHREAD_RWLOCKATTR_T : u32 = 8 ;
 pub const __SIZEOF_PTHREAD_BARRIERATTR_T : u32 = 4 ;
 pub const __PTHREAD_MUTEX_LOCK_ELISION : u32 = 1 ;
 pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND : u32 = 0 ;
 pub const __PTHREAD_MUTEX_USE_UNION : u32 = 0 ;
 pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED : u32 = 1 ;
 pub const __PTHREAD_MUTEX_HAVE_PREV : u32 = 1 ;
 pub const __have_pthread_attr_t : u32 = 1 ;
 pub const _SYS_SOCKET_H : u32 = 1 ;
 pub const __iovec_defined : u32 = 1 ;
 pub const PF_UNSPEC : u32 = 0 ;
 pub const PF_LOCAL : u32 = 1 ;
 pub const PF_UNIX : u32 = 1 ;
 pub const PF_FILE : u32 = 1 ;
 pub const PF_INET : u32 = 2 ;
 pub const PF_AX25 : u32 = 3 ;
 pub const PF_IPX : u32 = 4 ;
 pub const PF_APPLETALK : u32 = 5 ;
 pub const PF_NETROM : u32 = 6 ;
 pub const PF_BRIDGE : u32 = 7 ;
 pub const PF_ATMPVC : u32 = 8 ;
 pub const PF_X25 : u32 = 9 ;
 pub const PF_INET6 : u32 = 10 ;
 pub const PF_ROSE : u32 = 11 ;
 pub const PF_DECnet : u32 = 12 ;
 pub const PF_NETBEUI : u32 = 13 ;
 pub const PF_SECURITY : u32 = 14 ;
 pub const PF_KEY : u32 = 15 ;
 pub const PF_NETLINK : u32 = 16 ;
 pub const PF_ROUTE : u32 = 16 ;
 pub const PF_PACKET : u32 = 17 ;
 pub const PF_ASH : u32 = 18 ;
 pub const PF_ECONET : u32 = 19 ;
 pub const PF_ATMSVC : u32 = 20 ;
 pub const PF_RDS : u32 = 21 ;
 pub const PF_SNA : u32 = 22 ;
 pub const PF_IRDA : u32 = 23 ;
 pub const PF_PPPOX : u32 = 24 ;
 pub const PF_WANPIPE : u32 = 25 ;
 pub const PF_LLC : u32 = 26 ;
 pub const PF_IB : u32 = 27 ;
 pub const PF_MPLS : u32 = 28 ;
 pub const PF_CAN : u32 = 29 ;
 pub const PF_TIPC : u32 = 30 ;
 pub const PF_BLUETOOTH : u32 = 31 ;
 pub const PF_IUCV : u32 = 32 ;
 pub const PF_RXRPC : u32 = 33 ;
 pub const PF_ISDN : u32 = 34 ;
 pub const PF_PHONET : u32 = 35 ;
 pub const PF_IEEE802154 : u32 = 36 ;
 pub const PF_CAIF : u32 = 37 ;
 pub const PF_ALG : u32 = 38 ;
 pub const PF_NFC : u32 = 39 ;
 pub const PF_VSOCK : u32 = 40 ;
 pub const PF_KCM : u32 = 41 ;
 pub const PF_QIPCRTR : u32 = 42 ;
 pub const PF_SMC : u32 = 43 ;
 pub const PF_MAX : u32 = 44 ;
 pub const AF_UNSPEC : u32 = 0 ;
 pub const AF_LOCAL : u32 = 1 ;
 pub const AF_UNIX : u32 = 1 ;
 pub const AF_FILE : u32 = 1 ;
 pub const AF_INET : u32 = 2 ;
 pub const AF_AX25 : u32 = 3 ;
 pub const AF_IPX : u32 = 4 ;
 pub const AF_APPLETALK : u32 = 5 ;
 pub const AF_NETROM : u32 = 6 ;
 pub const AF_BRIDGE : u32 = 7 ;
 pub const AF_ATMPVC : u32 = 8 ;
 pub const AF_X25 : u32 = 9 ;
 pub const AF_INET6 : u32 = 10 ;
 pub const AF_ROSE : u32 = 11 ;
 pub const AF_DECnet : u32 = 12 ;
 pub const AF_NETBEUI : u32 = 13 ;
 pub const AF_SECURITY : u32 = 14 ;
 pub const AF_KEY : u32 = 15 ;
 pub const AF_NETLINK : u32 = 16 ;
 pub const AF_ROUTE : u32 = 16 ;
 pub const AF_PACKET : u32 = 17 ;
 pub const AF_ASH : u32 = 18 ;
 pub const AF_ECONET : u32 = 19 ;
 pub const AF_ATMSVC : u32 = 20 ;
 pub const AF_RDS : u32 = 21 ;
 pub const AF_SNA : u32 = 22 ;
 pub const AF_IRDA : u32 = 23 ;
 pub const AF_PPPOX : u32 = 24 ;
 pub const AF_WANPIPE : u32 = 25 ;
 pub const AF_LLC : u32 = 26 ;
 pub const AF_IB : u32 = 27 ;
 pub const AF_MPLS : u32 = 28 ;
 pub const AF_CAN : u32 = 29 ;
 pub const AF_TIPC : u32 = 30 ;
 pub const AF_BLUETOOTH : u32 = 31 ;
 pub const AF_IUCV : u32 = 32 ;
 pub const AF_RXRPC : u32 = 33 ;
 pub const AF_ISDN : u32 = 34 ;
 pub const AF_PHONET : u32 = 35 ;
 pub const AF_IEEE802154 : u32 = 36 ;
 pub const AF_CAIF : u32 = 37 ;
 pub const AF_ALG : u32 = 38 ;
 pub const AF_NFC : u32 = 39 ;
 pub const AF_VSOCK : u32 = 40 ;
 pub const AF_KCM : u32 = 41 ;
 pub const AF_QIPCRTR : u32 = 42 ;
 pub const AF_SMC : u32 = 43 ;
 pub const AF_MAX : u32 = 44 ;
 pub const SOL_RAW : u32 = 255 ;
 pub const SOL_DECNET : u32 = 261 ;
 pub const SOL_X25 : u32 = 262 ;
 pub const SOL_PACKET : u32 = 263 ;
 pub const SOL_ATM : u32 = 264 ;
 pub const SOL_AAL : u32 = 265 ;
 pub const SOL_IRDA : u32 = 266 ;
 pub const SOL_NETBEUI : u32 = 267 ;
 pub const SOL_LLC : u32 = 268 ;
 pub const SOL_DCCP : u32 = 269 ;
 pub const SOL_NETLINK : u32 = 270 ;
 pub const SOL_TIPC : u32 = 271 ;
 pub const SOL_RXRPC : u32 = 272 ;
 pub const SOL_PPPOL2TP : u32 = 273 ;
 pub const SOL_BLUETOOTH : u32 = 274 ;
 pub const SOL_PNPIPE : u32 = 275 ;
 pub const SOL_RDS : u32 = 276 ;
 pub const SOL_IUCV : u32 = 277 ;
 pub const SOL_CAIF : u32 = 278 ;
 pub const SOL_ALG : u32 = 279 ;
 pub const SOL_NFC : u32 = 280 ;
 pub const SOL_KCM : u32 = 281 ;
 pub const SOL_TLS : u32 = 282 ;
 pub const SOMAXCONN : u32 = 128 ;
 pub const _BITS_SOCKADDR_H : u32 = 1 ;
 pub const _SS_SIZE : u32 = 128 ;
 pub const FIOSETOWN : u32 = 35073 ;
 pub const SIOCSPGRP : u32 = 35074 ;
 pub const FIOGETOWN : u32 = 35075 ;
 pub const SIOCGPGRP : u32 = 35076 ;
 pub const SIOCATMARK : u32 = 35077 ;
 pub const SIOCGSTAMP : u32 = 35078 ;
 pub const SIOCGSTAMPNS : u32 = 35079 ;
 pub const SOL_SOCKET : u32 = 1 ;
 pub const SO_DEBUG : u32 = 1 ;
 pub const SO_REUSEADDR : u32 = 2 ;
 pub const SO_TYPE : u32 = 3 ;
 pub const SO_ERROR : u32 = 4 ;
 pub const SO_DONTROUTE : u32 = 5 ;
 pub const SO_BROADCAST : u32 = 6 ;
 pub const SO_SNDBUF : u32 = 7 ;
 pub const SO_RCVBUF : u32 = 8 ;
 pub const SO_SNDBUFFORCE : u32 = 32 ;
 pub const SO_RCVBUFFORCE : u32 = 33 ;
 pub const SO_KEEPALIVE : u32 = 9 ;
 pub const SO_OOBINLINE : u32 = 10 ;
 pub const SO_NO_CHECK : u32 = 11 ;
 pub const SO_PRIORITY : u32 = 12 ;
 pub const SO_LINGER : u32 = 13 ;
 pub const SO_BSDCOMPAT : u32 = 14 ;
 pub const SO_REUSEPORT : u32 = 15 ;
 pub const SO_PASSCRED : u32 = 16 ;
 pub const SO_PEERCRED : u32 = 17 ;
 pub const SO_RCVLOWAT : u32 = 18 ;
 pub const SO_SNDLOWAT : u32 = 19 ;
 pub const SO_RCVTIMEO : u32 = 20 ;
 pub const SO_SNDTIMEO : u32 = 21 ;
 pub const SO_SECURITY_AUTHENTICATION : u32 = 22 ;
 pub const SO_SECURITY_ENCRYPTION_TRANSPORT : u32 = 23 ;
 pub const SO_SECURITY_ENCRYPTION_NETWORK : u32 = 24 ;
 pub const SO_BINDTODEVICE : u32 = 25 ;
 pub const SO_ATTACH_FILTER : u32 = 26 ;
 pub const SO_DETACH_FILTER : u32 = 27 ;
 pub const SO_GET_FILTER : u32 = 26 ;
 pub const SO_PEERNAME : u32 = 28 ;
 pub const SO_TIMESTAMP : u32 = 29 ;
 pub const SCM_TIMESTAMP : u32 = 29 ;
 pub const SO_ACCEPTCONN : u32 = 30 ;
 pub const SO_PEERSEC : u32 = 31 ;
 pub const SO_PASSSEC : u32 = 34 ;
 pub const SO_TIMESTAMPNS : u32 = 35 ;
 pub const SCM_TIMESTAMPNS : u32 = 35 ;
 pub const SO_MARK : u32 = 36 ;
 pub const SO_TIMESTAMPING : u32 = 37 ;
 pub const SCM_TIMESTAMPING : u32 = 37 ;
 pub const SO_PROTOCOL : u32 = 38 ;
 pub const SO_DOMAIN : u32 = 39 ;
 pub const SO_RXQ_OVFL : u32 = 40 ;
 pub const SO_WIFI_STATUS : u32 = 41 ;
 pub const SCM_WIFI_STATUS : u32 = 41 ;
 pub const SO_PEEK_OFF : u32 = 42 ;
 pub const SO_NOFCS : u32 = 43 ;
 pub const SO_LOCK_FILTER : u32 = 44 ;
 pub const SO_SELECT_ERR_QUEUE : u32 = 45 ;
 pub const SO_BUSY_POLL : u32 = 46 ;
 pub const SO_MAX_PACING_RATE : u32 = 47 ;
 pub const SO_BPF_EXTENSIONS : u32 = 48 ;
 pub const SO_INCOMING_CPU : u32 = 49 ;
 pub const SO_ATTACH_BPF : u32 = 50 ;
 pub const SO_DETACH_BPF : u32 = 27 ;
 pub const SO_ATTACH_REUSEPORT_CBPF : u32 = 51 ;
 pub const SO_ATTACH_REUSEPORT_EBPF : u32 = 52 ;
 pub const SO_CNX_ADVICE : u32 = 53 ;
 pub const SCM_TIMESTAMPING_OPT_STATS : u32 = 54 ;
 pub const SO_MEMINFO : u32 = 55 ;
 pub const SO_INCOMING_NAPI_ID : u32 = 56 ;
 pub const SO_COOKIE : u32 = 57 ;
 pub const SCM_TIMESTAMPING_PKTINFO : u32 = 58 ;
 pub const SO_PEERGROUPS : u32 = 59 ;
 pub const SO_ZEROCOPY : u32 = 60 ;
 pub const __osockaddr_defined : u32 = 1 ;
 pub const _SYS_MMAN_H : u32 = 1 ;
 pub const MAP_32BIT : u32 = 64 ;
 pub const MAP_GROWSDOWN : u32 = 256 ;
 pub const MAP_DENYWRITE : u32 = 2048 ;
 pub const MAP_EXECUTABLE : u32 = 4096 ;
 pub const MAP_LOCKED : u32 = 8192 ;
 pub const MAP_NORESERVE : u32 = 16384 ;
 pub const MAP_POPULATE : u32 = 32768 ;
 pub const MAP_NONBLOCK : u32 = 65536 ;
 pub const MAP_STACK : u32 = 131072 ;
 pub const MAP_HUGETLB : u32 = 262144 ;
 pub const PROT_READ : u32 = 1 ;
 pub const PROT_WRITE : u32 = 2 ;
 pub const PROT_EXEC : u32 = 4 ;
 pub const PROT_NONE : u32 = 0 ;
 pub const PROT_GROWSDOWN : u32 = 16777216 ;
 pub const PROT_GROWSUP : u32 = 33554432 ;
 pub const MAP_SHARED : u32 = 1 ;
 pub const MAP_PRIVATE : u32 = 2 ;
 pub const MAP_TYPE : u32 = 15 ;
 pub const MAP_FIXED : u32 = 16 ;
 pub const MAP_FILE : u32 = 0 ;
 pub const MAP_ANONYMOUS : u32 = 32 ;
 pub const MAP_ANON : u32 = 32 ;
 pub const MAP_HUGE_SHIFT : u32 = 26 ;
 pub const MAP_HUGE_MASK : u32 = 63 ;
 pub const MS_ASYNC : u32 = 1 ;
 pub const MS_SYNC : u32 = 4 ;
 pub const MS_INVALIDATE : u32 = 2 ;
 pub const MREMAP_MAYMOVE : u32 = 1 ;
 pub const MREMAP_FIXED : u32 = 2 ;
 pub const MADV_NORMAL : u32 = 0 ;
 pub const MADV_RANDOM : u32 = 1 ;
 pub const MADV_SEQUENTIAL : u32 = 2 ;
 pub const MADV_WILLNEED : u32 = 3 ;
 pub const MADV_DONTNEED : u32 = 4 ;
 pub const MADV_FREE : u32 = 8 ;
 pub const MADV_REMOVE : u32 = 9 ;
 pub const MADV_DONTFORK : u32 = 10 ;
 pub const MADV_DOFORK : u32 = 11 ;
 pub const MADV_MERGEABLE : u32 = 12 ;
 pub const MADV_UNMERGEABLE : u32 = 13 ;
 pub const MADV_HUGEPAGE : u32 = 14 ;
 pub const MADV_NOHUGEPAGE : u32 = 15 ;
 pub const MADV_DONTDUMP : u32 = 16 ;
 pub const MADV_DODUMP : u32 = 17 ;
 pub const MADV_WIPEONFORK : u32 = 18 ;
 pub const MADV_KEEPONFORK : u32 = 19 ;
 pub const MADV_HWPOISON : u32 = 100 ;
 pub const POSIX_MADV_NORMAL : u32 = 0 ;
 pub const POSIX_MADV_RANDOM : u32 = 1 ;
 pub const POSIX_MADV_SEQUENTIAL : u32 = 2 ;
 pub const POSIX_MADV_WILLNEED : u32 = 3 ;
 pub const POSIX_MADV_DONTNEED : u32 = 4 ;
 pub const MCL_CURRENT : u32 = 1 ;
 pub const MCL_FUTURE : u32 = 2 ;
 pub const MCL_ONFAULT : u32 = 4 ;
 pub const MFD_CLOEXEC : u32 = 1 ;
 pub const MFD_ALLOW_SEALING : u32 = 2 ;
 pub const MFD_HUGETLB : u32 = 4 ;
 pub const MLOCK_ONFAULT : u32 = 1 ;
 pub const PKEY_DISABLE_ACCESS : u32 = 1 ;
 pub const PKEY_DISABLE_WRITE : u32 = 2 ;
 pub const _FILE_OFFSET_BITS : u32 = 64 ;
 pub const PRIV_SOCK_LOGIN : u32 = 1 ;
 pub const PRIV_SOCK_CHOWN : u32 = 2 ;
 pub const PRIV_SOCK_GET_DATA_SOCK : u32 = 3 ;
 pub const PRIV_SOCK_GET_USER_CMD : u32 = 4 ;
 pub const PRIV_SOCK_WRITE_USER_RESP : u32 = 5 ;
 pub const PRIV_SOCK_DO_SSL_HANDSHAKE : u32 = 6 ;
 pub const PRIV_SOCK_DO_SSL_CLOSE : u32 = 7 ;
 pub const PRIV_SOCK_DO_SSL_READ : u32 = 8 ;
 pub const PRIV_SOCK_DO_SSL_WRITE : u32 = 9 ;
 pub const PRIV_SOCK_PASV_CLEANUP : u32 = 10 ;
 pub const PRIV_SOCK_PASV_ACTIVE : u32 = 11 ;
 pub const PRIV_SOCK_PASV_LISTEN : u32 = 12 ;
 pub const PRIV_SOCK_PASV_ACCEPT : u32 = 13 ;
 pub const PRIV_SOCK_RESULT_OK : u32 = 1 ;
 pub const PRIV_SOCK_RESULT_BAD : u32 = 2 ;
 pub const PTRACE_SANDBOX_ERR_DEAD : i32 = - 1 ;
 pub const PTRACE_SANDBOX_ERR_PTRACE : i32 = - 2 ;
 pub const PTRACE_SANDBOX_ERR_WAITPID : i32 = - 3 ;
 pub const PTRACE_SANDBOX_ERR_WAIT_STATUS : i32 = - 4 ;
 pub const PTRACE_SANDBOX_ERR_POLICY_SYSCALL : i32 = - 5 ;
 pub const PTRACE_SANDBOX_ERR_BAD_SYSCALL : i32 = - 6 ;
 pub const PTRACE_SANDBOX_ERR_POLICY_ARGS : i32 = - 7 ;
 pub const PTRACE_SANDBOX_ERR_API_ABUSE_STOPIT : i32 = - 8 ;
 pub const VSF_SECUTIL_OPTION_CHROOT : u32 = 1 ;
 pub const VSF_SECUTIL_OPTION_USE_GROUPS : u32 = 2 ;
 pub const VSF_SECUTIL_OPTION_CHANGE_EUID : u32 = 4 ;
 pub const VSF_SECUTIL_OPTION_NO_FDS : u32 = 8 ;
 pub const VSF_SECUTIL_OPTION_NO_PROCS : u32 = 16 ;
 pub const VSF_SECUTIL_OPTION_ALLOW_WRITEABLE_ROOT : u32 = 32 ;
// pub const VSF_VERSION : & 'static [u8 ; 6usize] = b"3.0.3\0" ;
 pub const VSF_VERSION : &str  = "3.0.3";
 pub const VSFTP_MAX_VISIT_REMEMBER : u32 = 100 ;
 pub const VSFTP_MAX_MSGFILE_SIZE : u32 = 4000 ;
 pub const _NETINET_IN_SYSTM_H : u32 = 1 ;
 pub const _STDINT_H : u32 = 1 ;
 pub const __GLIBC_USE_LIB_EXT2 : u32 = 1 ;
 pub const __GLIBC_USE_IEC_60559_BFP_EXT : u32 = 1 ;
 pub const __GLIBC_USE_IEC_60559_FUNCS_EXT : u32 = 1 ;
 pub const __GLIBC_USE_IEC_60559_TYPES_EXT : u32 = 1 ;
 pub const _BITS_WCHAR_H : u32 = 1 ;
 pub const _BITS_STDINT_UINTN_H : u32 = 1 ;
 pub const INT8_MIN : i32 = - 128 ;
 pub const INT16_MIN : i32 = - 32768 ;
 pub const INT32_MIN : i32 = - 2147483648 ;
 pub const INT8_MAX : u32 = 127 ;
 pub const INT16_MAX : u32 = 32767 ;
 pub const INT32_MAX : u32 = 2147483647 ;
 pub const UINT8_MAX : u32 = 255 ;
 pub const UINT16_MAX : u32 = 65535 ;
 pub const UINT32_MAX : u32 = 4294967295 ;
 pub const INT_LEAST8_MIN : i32 = - 128 ;
 pub const INT_LEAST16_MIN : i32 = - 32768 ;
 pub const INT_LEAST32_MIN : i32 = - 2147483648 ;
 pub const INT_LEAST8_MAX : u32 = 127 ;
 pub const INT_LEAST16_MAX : u32 = 32767 ;
 pub const INT_LEAST32_MAX : u32 = 2147483647 ;
 pub const UINT_LEAST8_MAX : u32 = 255 ;
 pub const UINT_LEAST16_MAX : u32 = 65535 ;
 pub const UINT_LEAST32_MAX : u32 = 4294967295 ;
 pub const INT_FAST8_MIN : i32 = - 128 ;
 pub const INT_FAST16_MIN : i64 = - 9223372036854775808 ;
 pub const INT_FAST32_MIN : i64 = - 9223372036854775808 ;
 pub const INT_FAST8_MAX : u32 = 127 ;
 pub const INT_FAST16_MAX : u64 = 9223372036854775807 ;
 pub const INT_FAST32_MAX : u64 = 9223372036854775807 ;
 pub const UINT_FAST8_MAX : u32 = 255 ;
 pub const UINT_FAST16_MAX : i32 = - 1 ;
 pub const UINT_FAST32_MAX : i32 = - 1 ;
 pub const INTPTR_MIN : i64 = - 9223372036854775808 ;
 pub const INTPTR_MAX : u64 = 9223372036854775807 ;
 pub const UINTPTR_MAX : i32 = - 1 ;
 pub const PTRDIFF_MIN : i64 = - 9223372036854775808 ;
 pub const PTRDIFF_MAX : u64 = 9223372036854775807 ;
 pub const SIG_ATOMIC_MIN : i32 = - 2147483648 ;
 pub const SIG_ATOMIC_MAX : u32 = 2147483647 ;
 pub const SIZE_MAX : i32 = - 1 ;
 pub const WINT_MIN : u32 = 0 ;
 pub const WINT_MAX : u32 = 4294967295 ;
 pub const INT8_WIDTH : u32 = 8 ;
 pub const UINT8_WIDTH : u32 = 8 ;
 pub const INT16_WIDTH : u32 = 16 ;
 pub const UINT16_WIDTH : u32 = 16 ;
 pub const INT32_WIDTH : u32 = 32 ;
 pub const UINT32_WIDTH : u32 = 32 ;
 pub const INT64_WIDTH : u32 = 64 ;
 pub const UINT64_WIDTH : u32 = 64 ;
 pub const INT_LEAST8_WIDTH : u32 = 8 ;
 pub const UINT_LEAST8_WIDTH : u32 = 8 ;
 pub const INT_LEAST16_WIDTH : u32 = 16 ;
 pub const UINT_LEAST16_WIDTH : u32 = 16 ;
 pub const INT_LEAST32_WIDTH : u32 = 32 ;
 pub const UINT_LEAST32_WIDTH : u32 = 32 ;
 pub const INT_LEAST64_WIDTH : u32 = 64 ;
 pub const UINT_LEAST64_WIDTH : u32 = 64 ;
 pub const INT_FAST8_WIDTH : u32 = 8 ;
 pub const UINT_FAST8_WIDTH : u32 = 8 ;
 pub const INT_FAST16_WIDTH : u32 = 64 ;
 pub const UINT_FAST16_WIDTH : u32 = 64 ;
 pub const INT_FAST32_WIDTH : u32 = 64 ;
 pub const UINT_FAST32_WIDTH : u32 = 64 ;
 pub const INT_FAST64_WIDTH : u32 = 64 ;
 pub const UINT_FAST64_WIDTH : u32 = 64 ;
 pub const INTPTR_WIDTH : u32 = 64 ;
 pub const UINTPTR_WIDTH : u32 = 64 ;
 pub const INTMAX_WIDTH : u32 = 64 ;
 pub const UINTMAX_WIDTH : u32 = 64 ;
 pub const PTRDIFF_WIDTH : u32 = 64 ;
 pub const SIG_ATOMIC_WIDTH : u32 = 32 ;
 pub const SIZE_WIDTH : u32 = 64 ;
 pub const WCHAR_WIDTH : u32 = 32 ;
 pub const WINT_WIDTH : u32 = 32 ;
 pub const _NETINET_IN_H : u32 = 1 ;
 pub const __USE_KERNEL_IPV6_DEFS : u32 = 0 ;
 pub const IP_OPTIONS : u32 = 4 ;
 pub const IP_HDRINCL : u32 = 3 ;
 pub const IP_TOS : u32 = 1 ;
 pub const IP_TTL : u32 = 2 ;
 pub const IP_RECVOPTS : u32 = 6 ;
 pub const IP_RETOPTS : u32 = 7 ;
 pub const IP_MULTICAST_IF : u32 = 32 ;
 pub const IP_MULTICAST_TTL : u32 = 33 ;
 pub const IP_MULTICAST_LOOP : u32 = 34 ;
 pub const IP_ADD_MEMBERSHIP : u32 = 35 ;
 pub const IP_DROP_MEMBERSHIP : u32 = 36 ;
 pub const IP_UNBLOCK_SOURCE : u32 = 37 ;
 pub const IP_BLOCK_SOURCE : u32 = 38 ;
 pub const IP_ADD_SOURCE_MEMBERSHIP : u32 = 39 ;
 pub const IP_DROP_SOURCE_MEMBERSHIP : u32 = 40 ;
 pub const IP_MSFILTER : u32 = 41 ;
 pub const MCAST_JOIN_GROUP : u32 = 42 ;
 pub const MCAST_BLOCK_SOURCE : u32 = 43 ;
 pub const MCAST_UNBLOCK_SOURCE : u32 = 44 ;
 pub const MCAST_LEAVE_GROUP : u32 = 45 ;
 pub const MCAST_JOIN_SOURCE_GROUP : u32 = 46 ;
 pub const MCAST_LEAVE_SOURCE_GROUP : u32 = 47 ;
 pub const MCAST_MSFILTER : u32 = 48 ;
 pub const IP_MULTICAST_ALL : u32 = 49 ;
 pub const IP_UNICAST_IF : u32 = 50 ;
 pub const MCAST_EXCLUDE : u32 = 0 ;
 pub const MCAST_INCLUDE : u32 = 1 ;
 pub const IP_ROUTER_ALERT : u32 = 5 ;
 pub const IP_PKTINFO : u32 = 8 ;
 pub const IP_PKTOPTIONS : u32 = 9 ;
 pub const IP_PMTUDISC : u32 = 10 ;
 pub const IP_MTU_DISCOVER : u32 = 10 ;
 pub const IP_RECVERR : u32 = 11 ;
 pub const IP_RECVTTL : u32 = 12 ;
 pub const IP_RECVTOS : u32 = 13 ;
 pub const IP_MTU : u32 = 14 ;
 pub const IP_FREEBIND : u32 = 15 ;
 pub const IP_IPSEC_POLICY : u32 = 16 ;
 pub const IP_XFRM_POLICY : u32 = 17 ;
 pub const IP_PASSSEC : u32 = 18 ;
 pub const IP_TRANSPARENT : u32 = 19 ;
 pub const IP_ORIGDSTADDR : u32 = 20 ;
 pub const IP_RECVORIGDSTADDR : u32 = 20 ;
 pub const IP_MINTTL : u32 = 21 ;
 pub const IP_NODEFRAG : u32 = 22 ;
 pub const IP_CHECKSUM : u32 = 23 ;
 pub const IP_BIND_ADDRESS_NO_PORT : u32 = 24 ;
 pub const IP_RECVFRAGSIZE : u32 = 25 ;
 pub const IP_PMTUDISC_DONT : u32 = 0 ;
 pub const IP_PMTUDISC_WANT : u32 = 1 ;
 pub const IP_PMTUDISC_DO : u32 = 2 ;
 pub const IP_PMTUDISC_PROBE : u32 = 3 ;
 pub const IP_PMTUDISC_INTERFACE : u32 = 4 ;
 pub const IP_PMTUDISC_OMIT : u32 = 5 ;
 pub const SOL_IP : u32 = 0 ;
 pub const IP_DEFAULT_MULTICAST_TTL : u32 = 1 ;
 pub const IP_DEFAULT_MULTICAST_LOOP : u32 = 1 ;
 pub const IP_MAX_MEMBERSHIPS : u32 = 20 ;
 pub const IPV6_ADDRFORM : u32 = 1 ;
 pub const IPV6_2292PKTINFO : u32 = 2 ;
 pub const IPV6_2292HOPOPTS : u32 = 3 ;
 pub const IPV6_2292DSTOPTS : u32 = 4 ;
 pub const IPV6_2292RTHDR : u32 = 5 ;
 pub const IPV6_2292PKTOPTIONS : u32 = 6 ;
 pub const IPV6_CHECKSUM : u32 = 7 ;
 pub const IPV6_2292HOPLIMIT : u32 = 8 ;
 pub const IPV6_NEXTHOP : u32 = 9 ;
 pub const IPV6_AUTHHDR : u32 = 10 ;
 pub const IPV6_UNICAST_HOPS : u32 = 16 ;
 pub const IPV6_MULTICAST_IF : u32 = 17 ;
 pub const IPV6_MULTICAST_HOPS : u32 = 18 ;
 pub const IPV6_MULTICAST_LOOP : u32 = 19 ;
 pub const IPV6_JOIN_GROUP : u32 = 20 ;
 pub const IPV6_LEAVE_GROUP : u32 = 21 ;
 pub const IPV6_ROUTER_ALERT : u32 = 22 ;
 pub const IPV6_MTU_DISCOVER : u32 = 23 ;
 pub const IPV6_MTU : u32 = 24 ;
 pub const IPV6_RECVERR : u32 = 25 ;
 pub const IPV6_V6ONLY : u32 = 26 ;
 pub const IPV6_JOIN_ANYCAST : u32 = 27 ;
 pub const IPV6_LEAVE_ANYCAST : u32 = 28 ;
 pub const IPV6_IPSEC_POLICY : u32 = 34 ;
 pub const IPV6_XFRM_POLICY : u32 = 35 ;
 pub const IPV6_HDRINCL : u32 = 36 ;
 pub const IPV6_RECVPKTINFO : u32 = 49 ;
 pub const IPV6_PKTINFO : u32 = 50 ;
 pub const IPV6_RECVHOPLIMIT : u32 = 51 ;
 pub const IPV6_HOPLIMIT : u32 = 52 ;
 pub const IPV6_RECVHOPOPTS : u32 = 53 ;
 pub const IPV6_HOPOPTS : u32 = 54 ;
 pub const IPV6_RTHDRDSTOPTS : u32 = 55 ;
 pub const IPV6_RECVRTHDR : u32 = 56 ;
 pub const IPV6_RTHDR : u32 = 57 ;
 pub const IPV6_RECVDSTOPTS : u32 = 58 ;
 pub const IPV6_DSTOPTS : u32 = 59 ;
 pub const IPV6_RECVPATHMTU : u32 = 60 ;
 pub const IPV6_PATHMTU : u32 = 61 ;
 pub const IPV6_DONTFRAG : u32 = 62 ;
 pub const IPV6_RECVTCLASS : u32 = 66 ;
 pub const IPV6_TCLASS : u32 = 67 ;
 pub const IPV6_AUTOFLOWLABEL : u32 = 70 ;
 pub const IPV6_ADDR_PREFERENCES : u32 = 72 ;
 pub const IPV6_MINHOPCOUNT : u32 = 73 ;
 pub const IPV6_ORIGDSTADDR : u32 = 74 ;
 pub const IPV6_RECVORIGDSTADDR : u32 = 74 ;
 pub const IPV6_TRANSPARENT : u32 = 75 ;
 pub const IPV6_UNICAST_IF : u32 = 76 ;
 pub const IPV6_RECVFRAGSIZE : u32 = 77 ;
 pub const IPV6_ADD_MEMBERSHIP : u32 = 20 ;
 pub const IPV6_DROP_MEMBERSHIP : u32 = 21 ;
 pub const IPV6_RXHOPOPTS : u32 = 54 ;
 pub const IPV6_RXDSTOPTS : u32 = 59 ;
 pub const IPV6_PMTUDISC_DONT : u32 = 0 ;
 pub const IPV6_PMTUDISC_WANT : u32 = 1 ;
 pub const IPV6_PMTUDISC_DO : u32 = 2 ;
 pub const IPV6_PMTUDISC_PROBE : u32 = 3 ;
 pub const IPV6_PMTUDISC_INTERFACE : u32 = 4 ;
 pub const IPV6_PMTUDISC_OMIT : u32 = 5 ;
 pub const SOL_IPV6 : u32 = 41 ;
 pub const SOL_ICMPV6 : u32 = 58 ;
 pub const IPV6_RTHDR_LOOSE : u32 = 0 ;
 pub const IPV6_RTHDR_STRICT : u32 = 1 ;
 pub const IPV6_RTHDR_TYPE_0 : u32 = 0 ;
 pub const IN_CLASSA_NET : u32 = 4278190080 ;
 pub const IN_CLASSA_NSHIFT : u32 = 24 ;
 pub const IN_CLASSA_HOST : u32 = 16777215 ;
 pub const IN_CLASSA_MAX : u32 = 128 ;
 pub const IN_CLASSB_NET : u32 = 4294901760 ;
 pub const IN_CLASSB_NSHIFT : u32 = 16 ;
 pub const IN_CLASSB_HOST : u32 = 65535 ;
 pub const IN_CLASSB_MAX : u32 = 65536 ;
 pub const IN_CLASSC_NET : u32 = 4294967040 ;
 pub const IN_CLASSC_NSHIFT : u32 = 8 ;
 pub const IN_CLASSC_HOST : u32 = 255 ;
 pub const IN_LOOPBACKNET : u32 = 127 ;
 pub const INET_ADDRSTRLEN : u32 = 16 ;
 pub const INET6_ADDRSTRLEN : u32 = 46 ;
 pub const __NETINET_IP_H : u32 = 1 ;
 pub const IP_RF : u32 = 32768 ;
 pub const IP_DF : u32 = 16384 ;
 pub const IP_MF : u32 = 8192 ;
 pub const IP_OFFMASK : u32 = 8191 ;
 pub const IPVERSION : u32 = 4 ;
 pub const IP_MAXPACKET : u32 = 65535 ;
 pub const IPTOS_ECN_MASK : u32 = 3 ;
 pub const IPTOS_ECN_NOT_ECT : u32 = 0 ;
 pub const IPTOS_ECN_ECT1 : u32 = 1 ;
 pub const IPTOS_ECN_ECT0 : u32 = 2 ;
 pub const IPTOS_ECN_CE : u32 = 3 ;
 pub const IPTOS_DSCP_MASK : u32 = 252 ;
 pub const IPTOS_DSCP_AF11 : u32 = 40 ;
 pub const IPTOS_DSCP_AF12 : u32 = 48 ;
 pub const IPTOS_DSCP_AF13 : u32 = 56 ;
 pub const IPTOS_DSCP_AF21 : u32 = 72 ;
 pub const IPTOS_DSCP_AF22 : u32 = 80 ;
 pub const IPTOS_DSCP_AF23 : u32 = 88 ;
 pub const IPTOS_DSCP_AF31 : u32 = 104 ;
 pub const IPTOS_DSCP_AF32 : u32 = 112 ;
 pub const IPTOS_DSCP_AF33 : u32 = 120 ;
 pub const IPTOS_DSCP_AF41 : u32 = 136 ;
 pub const IPTOS_DSCP_AF42 : u32 = 144 ;
 pub const IPTOS_DSCP_AF43 : u32 = 152 ;
 pub const IPTOS_DSCP_EF : u32 = 184 ;
 pub const IPTOS_CLASS_MASK : u32 = 224 ;
 pub const IPTOS_CLASS_CS0 : u32 = 0 ;
 pub const IPTOS_CLASS_CS1 : u32 = 32 ;
 pub const IPTOS_CLASS_CS2 : u32 = 64 ;
 pub const IPTOS_CLASS_CS3 : u32 = 96 ;
 pub const IPTOS_CLASS_CS4 : u32 = 128 ;
 pub const IPTOS_CLASS_CS5 : u32 = 160 ;
 pub const IPTOS_CLASS_CS6 : u32 = 192 ;
 pub const IPTOS_CLASS_CS7 : u32 = 224 ;
 pub const IPTOS_CLASS_DEFAULT : u32 = 0 ;
 pub const IPTOS_TOS_MASK : u32 = 30 ;
 pub const IPTOS_LOWDELAY : u32 = 16 ;
 pub const IPTOS_THROUGHPUT : u32 = 8 ;
 pub const IPTOS_RELIABILITY : u32 = 4 ;
 pub const IPTOS_LOWCOST : u32 = 2 ;
 pub const IPTOS_MINCOST : u32 = 2 ;
 pub const IPTOS_PREC_MASK : u32 = 224 ;
 pub const IPTOS_PREC_NETCONTROL : u32 = 224 ;
 pub const IPTOS_PREC_INTERNETCONTROL : u32 = 192 ;
 pub const IPTOS_PREC_CRITIC_ECP : u32 = 160 ;
 pub const IPTOS_PREC_FLASHOVERRIDE : u32 = 128 ;
 pub const IPTOS_PREC_FLASH : u32 = 96 ;
 pub const IPTOS_PREC_IMMEDIATE : u32 = 64 ;
 pub const IPTOS_PREC_PRIORITY : u32 = 32 ;
 pub const IPTOS_PREC_ROUTINE : u32 = 0 ;
 pub const IPOPT_COPY : u32 = 128 ;
 pub const IPOPT_CLASS_MASK : u32 = 96 ;
 pub const IPOPT_NUMBER_MASK : u32 = 31 ;
 pub const IPOPT_CONTROL : u32 = 0 ;
 pub const IPOPT_RESERVED1 : u32 = 32 ;
 pub const IPOPT_DEBMEAS : u32 = 64 ;
 pub const IPOPT_MEASUREMENT : u32 = 64 ;
 pub const IPOPT_RESERVED2 : u32 = 96 ;
 pub const IPOPT_EOL : u32 = 0 ;
 pub const IPOPT_END : u32 = 0 ;
 pub const IPOPT_NOP : u32 = 1 ;
 pub const IPOPT_NOOP : u32 = 1 ;
 pub const IPOPT_RR : u32 = 7 ;
 pub const IPOPT_TS : u32 = 68 ;
 pub const IPOPT_TIMESTAMP : u32 = 68 ;
 pub const IPOPT_SECURITY : u32 = 130 ;
 pub const IPOPT_SEC : u32 = 130 ;
 pub const IPOPT_LSRR : u32 = 131 ;
 pub const IPOPT_SATID : u32 = 136 ;
 pub const IPOPT_SID : u32 = 136 ;
 pub const IPOPT_SSRR : u32 = 137 ;
 pub const IPOPT_RA : u32 = 148 ;
 pub const IPOPT_OPTVAL : u32 = 0 ;
 pub const IPOPT_OLEN : u32 = 1 ;
 pub const IPOPT_OFFSET : u32 = 2 ;
 pub const IPOPT_MINOFF : u32 = 4 ;
 pub const MAX_IPOPTLEN : u32 = 40 ;
 pub const IPOPT_TS_TSONLY : u32 = 0 ;
 pub const IPOPT_TS_TSANDADDR : u32 = 1 ;
 pub const IPOPT_TS_PRESPEC : u32 = 3 ;
 pub const IPOPT_SECUR_UNCLASS : u32 = 0 ;
 pub const IPOPT_SECUR_CONFID : u32 = 61749 ;
 pub const IPOPT_SECUR_EFTO : u32 = 30874 ;
 pub const IPOPT_SECUR_MMMM : u32 = 48205 ;
 pub const IPOPT_SECUR_RESTR : u32 = 44819 ;
 pub const IPOPT_SECUR_SECRET : u32 = 55176 ;
 pub const IPOPT_SECUR_TOPSECRET : u32 = 27589 ;
 pub const MAXTTL : u32 = 255 ;
 pub const IPDEFTTL : u32 = 64 ;
 pub const IPFRAGTTL : u32 = 60 ;
 pub const IPTTLDEC : u32 = 1 ;
 pub const IP_MSS : u32 = 576 ;
 pub const _NETINET_TCP_H : u32 = 1 ;
 pub const TCP_NODELAY : u32 = 1 ;
 pub const TCP_MAXSEG : u32 = 2 ;
 pub const TCP_CORK : u32 = 3 ;
 pub const TCP_KEEPIDLE : u32 = 4 ;
 pub const TCP_KEEPINTVL : u32 = 5 ;
 pub const TCP_KEEPCNT : u32 = 6 ;
 pub const TCP_SYNCNT : u32 = 7 ;
 pub const TCP_LINGER2 : u32 = 8 ;
 pub const TCP_DEFER_ACCEPT : u32 = 9 ;
 pub const TCP_WINDOW_CLAMP : u32 = 10 ;
 pub const TCP_INFO : u32 = 11 ;
 pub const TCP_QUICKACK : u32 = 12 ;
 pub const TCP_CONGESTION : u32 = 13 ;
 pub const TCP_MD5SIG : u32 = 14 ;
 pub const TCP_COOKIE_TRANSACTIONS : u32 = 15 ;
 pub const TCP_THIN_LINEAR_TIMEOUTS : u32 = 16 ;
 pub const TCP_THIN_DUPACK : u32 = 17 ;
 pub const TCP_USER_TIMEOUT : u32 = 18 ;
 pub const TCP_REPAIR : u32 = 19 ;
 pub const TCP_REPAIR_QUEUE : u32 = 20 ;
 pub const TCP_QUEUE_SEQ : u32 = 21 ;
 pub const TCP_REPAIR_OPTIONS : u32 = 22 ;
 pub const TCP_FASTOPEN : u32 = 23 ;
 pub const TCP_TIMESTAMP : u32 = 24 ;
 pub const TCP_NOTSENT_LOWAT : u32 = 25 ;
 pub const TCP_CC_INFO : u32 = 26 ;
 pub const TCP_SAVE_SYN : u32 = 27 ;
 pub const TCP_SAVED_SYN : u32 = 28 ;
 pub const TCP_REPAIR_WINDOW : u32 = 29 ;
 pub const TCP_FASTOPEN_CONNECT : u32 = 30 ;
 pub const TCP_ULP : u32 = 31 ;
 pub const TCP_MD5SIG_EXT : u32 = 32 ;
 pub const TH_FIN : u32 = 1 ;
 pub const TH_SYN : u32 = 2 ;
 pub const TH_RST : u32 = 4 ;
 pub const TH_PUSH : u32 = 8 ;
 pub const TH_ACK : u32 = 16 ;
 pub const TH_URG : u32 = 32 ;
 pub const TCPOPT_EOL : u32 = 0 ;
 pub const TCPOPT_NOP : u32 = 1 ;
 pub const TCPOPT_MAXSEG : u32 = 2 ;
 pub const TCPOLEN_MAXSEG : u32 = 4 ;
 pub const TCPOPT_WINDOW : u32 = 3 ;
 pub const TCPOLEN_WINDOW : u32 = 3 ;
 pub const TCPOPT_SACK_PERMITTED : u32 = 4 ;
 pub const TCPOLEN_SACK_PERMITTED : u32 = 2 ;
 pub const TCPOPT_SACK : u32 = 5 ;
 pub const TCPOPT_TIMESTAMP : u32 = 8 ;
 pub const TCPOLEN_TIMESTAMP : u32 = 10 ;
 pub const TCPOLEN_TSTAMP_APPA : u32 = 12 ;
 pub const TCPOPT_TSTAMP_HDR : u32 = 16844810 ;
 pub const TCP_MSS : u32 = 512 ;
 pub const TCP_MAXWIN : u32 = 65535 ;
 pub const TCP_MAX_WINSHIFT : u32 = 14 ;
 pub const SOL_TCP : u32 = 6 ;
 pub const TCPI_OPT_TIMESTAMPS : u32 = 1 ;
 pub const TCPI_OPT_SACK : u32 = 2 ;
 pub const TCPI_OPT_WSCALE : u32 = 4 ;
 pub const TCPI_OPT_ECN : u32 = 8 ;
 pub const TCPI_OPT_ECN_SEEN : u32 = 16 ;
 pub const TCPI_OPT_SYN_DATA : u32 = 32 ;
 pub const TCP_MD5SIG_MAXKEYLEN : u32 = 80 ;
 pub const TCP_MD5SIG_FLAG_PREFIX : u32 = 1 ;
 pub const TCP_COOKIE_MIN : u32 = 8 ;
 pub const TCP_COOKIE_MAX : u32 = 16 ;
 pub const TCP_COOKIE_PAIR_SIZE : u32 = 32 ;
 pub const TCP_COOKIE_IN_ALWAYS : u32 = 1 ;
 pub const TCP_COOKIE_OUT_NEVER : u32 = 2 ;
 pub const TCP_S_DATA_IN : u32 = 4 ;
 pub const TCP_S_DATA_OUT : u32 = 8 ;
 pub const TCP_MSS_DEFAULT : u32 = 536 ;
 pub const TCP_MSS_DESIRED : u32 = 1220 ;
 pub const _ERRNO_H : u32 = 1 ;
 pub const _BITS_ERRNO_H : u32 = 1 ;
 pub const EPERM : u32 = 1 ;
 pub const ENOENT : u32 = 2 ;
 pub const ESRCH : u32 = 3 ;
 pub const EINTR : u32 = 4 ;
 pub const EIO : u32 = 5 ;
 pub const ENXIO : u32 = 6 ;
 pub const E2BIG : u32 = 7 ;
 pub const ENOEXEC : u32 = 8 ;
 pub const EBADF : u32 = 9 ;
 pub const ECHILD : u32 = 10 ;
 pub const EAGAIN : u32 = 11 ;
 pub const ENOMEM : u32 = 12 ;
 pub const EACCES : u32 = 13 ;
 pub const EFAULT : u32 = 14 ;
 pub const ENOTBLK : u32 = 15 ;
 pub const EBUSY : u32 = 16 ;
 pub const EEXIST : u32 = 17 ;
 pub const EXDEV : u32 = 18 ;
 pub const ENODEV : u32 = 19 ;
 pub const ENOTDIR : u32 = 20 ;
 pub const EISDIR : u32 = 21 ;
 pub const EINVAL : u32 = 22 ;
 pub const ENFILE : u32 = 23 ;
 pub const EMFILE : u32 = 24 ;
 pub const ENOTTY : u32 = 25 ;
 pub const ETXTBSY : u32 = 26 ;
 pub const EFBIG : u32 = 27 ;
 pub const ENOSPC : u32 = 28 ;
 pub const ESPIPE : u32 = 29 ;
 pub const EROFS : u32 = 30 ;
 pub const EMLINK : u32 = 31 ;
 pub const EPIPE : u32 = 32 ;
 pub const EDOM : u32 = 33 ;
 pub const ERANGE : u32 = 34 ;
 pub const EDEADLK : u32 = 35 ;
 pub const ENAMETOOLONG : u32 = 36 ;
 pub const ENOLCK : u32 = 37 ;
 pub const ENOSYS : u32 = 38 ;
 pub const ENOTEMPTY : u32 = 39 ;
 pub const ELOOP : u32 = 40 ;
 pub const EWOULDBLOCK : u32 = 11 ;
 pub const ENOMSG : u32 = 42 ;
 pub const EIDRM : u32 = 43 ;
 pub const ECHRNG : u32 = 44 ;
 pub const EL2NSYNC : u32 = 45 ;
 pub const EL3HLT : u32 = 46 ;
 pub const EL3RST : u32 = 47 ;
 pub const ELNRNG : u32 = 48 ;
 pub const EUNATCH : u32 = 49 ;
 pub const ENOCSI : u32 = 50 ;
 pub const EL2HLT : u32 = 51 ;
 pub const EBADE : u32 = 52 ;
 pub const EBADR : u32 = 53 ;
 pub const EXFULL : u32 = 54 ;
 pub const ENOANO : u32 = 55 ;
 pub const EBADRQC : u32 = 56 ;
 pub const EBADSLT : u32 = 57 ;
 pub const EDEADLOCK : u32 = 35 ;
 pub const EBFONT : u32 = 59 ;
 pub const ENOSTR : u32 = 60 ;
 pub const ENODATA : u32 = 61 ;
 pub const ETIME : u32 = 62 ;
 pub const ENOSR : u32 = 63 ;
 pub const ENONET : u32 = 64 ;
 pub const ENOPKG : u32 = 65 ;
 pub const EREMOTE : u32 = 66 ;
 pub const ENOLINK : u32 = 67 ;
 pub const EADV : u32 = 68 ;
 pub const ESRMNT : u32 = 69 ;
 pub const ECOMM : u32 = 70 ;
 pub const EPROTO : u32 = 71 ;
 pub const EMULTIHOP : u32 = 72 ;
 pub const EDOTDOT : u32 = 73 ;
 pub const EBADMSG : u32 = 74 ;
 pub const EOVERFLOW : u32 = 75 ;
 pub const ENOTUNIQ : u32 = 76 ;
 pub const EBADFD : u32 = 77 ;
 pub const EREMCHG : u32 = 78 ;
 pub const ELIBACC : u32 = 79 ;
 pub const ELIBBAD : u32 = 80 ;
 pub const ELIBSCN : u32 = 81 ;
 pub const ELIBMAX : u32 = 82 ;
 pub const ELIBEXEC : u32 = 83 ;
 pub const EILSEQ : u32 = 84 ;
 pub const ERESTART : u32 = 85 ;
 pub const ESTRPIPE : u32 = 86 ;
 pub const EUSERS : u32 = 87 ;
 pub const ENOTSOCK : u32 = 88 ;
 pub const EDESTADDRREQ : u32 = 89 ;
 pub const EMSGSIZE : u32 = 90 ;
 pub const EPROTOTYPE : u32 = 91 ;
 pub const ENOPROTOOPT : u32 = 92 ;
 pub const EPROTONOSUPPORT : u32 = 93 ;
 pub const ESOCKTNOSUPPORT : u32 = 94 ;
 pub const EOPNOTSUPP : u32 = 95 ;
 pub const EPFNOSUPPORT : u32 = 96 ;
 pub const EAFNOSUPPORT : u32 = 97 ;
 pub const EADDRINUSE : u32 = 98 ;
 pub const EADDRNOTAVAIL : u32 = 99 ;
 pub const ENETDOWN : u32 = 100 ;
 pub const ENETUNREACH : u32 = 101 ;
 pub const ENETRESET : u32 = 102 ;
 pub const ECONNABORTED : u32 = 103 ;
 pub const ECONNRESET : u32 = 104 ;
 pub const ENOBUFS : u32 = 105 ;
 pub const EISCONN : u32 = 106 ;
 pub const ENOTCONN : u32 = 107 ;
 pub const ESHUTDOWN : u32 = 108 ;
 pub const ETOOMANYREFS : u32 = 109 ;
 pub const ETIMEDOUT : u32 = 110 ;
 pub const ECONNREFUSED : u32 = 111 ;
 pub const EHOSTDOWN : u32 = 112 ;
 pub const EHOSTUNREACH : u32 = 113 ;
 pub const EALREADY : u32 = 114 ;
 pub const EINPROGRESS : u32 = 115 ;
 pub const ESTALE : u32 = 116 ;
 pub const EUCLEAN : u32 = 117 ;
 pub const ENOTNAM : u32 = 118 ;
 pub const ENAVAIL : u32 = 119 ;
 pub const EISNAM : u32 = 120 ;
 pub const EREMOTEIO : u32 = 121 ;
 pub const EDQUOT : u32 = 122 ;
 pub const ENOMEDIUM : u32 = 123 ;
 pub const EMEDIUMTYPE : u32 = 124 ;
 pub const ECANCELED : u32 = 125 ;
 pub const ENOKEY : u32 = 126 ;
 pub const EKEYEXPIRED : u32 = 127 ;
 pub const EKEYREVOKED : u32 = 128 ;
 pub const EKEYREJECTED : u32 = 129 ;
 pub const EOWNERDEAD : u32 = 130 ;
 pub const ENOTRECOVERABLE : u32 = 131 ;
 pub const ERFKILL : u32 = 132 ;
 pub const EHWPOISON : u32 = 133 ;
 pub const ENOTSUP : u32 = 95 ;
 pub const __error_t_defined : u32 = 1 ;
 pub const _FCNTL_H : u32 = 1 ;
 pub const __O_LARGEFILE : u32 = 0 ;
 pub const F_GETLK64 : u32 = 5 ;
 pub const F_SETLK64 : u32 = 6 ;
 pub const F_SETLKW64 : u32 = 7 ;
 pub const O_ACCMODE : u32 = 3 ;
 pub const O_RDONLY : u32 = 0 ;
 pub const O_WRONLY : i32 = 1 ;
 pub const O_RDWR : u32 = 2 ;
 pub const O_CREAT : i32 = 64 ;
 pub const O_EXCL : i32 = 128 ;
 pub const O_NOCTTY : u32 = 256 ;
 pub const O_TRUNC : u32 = 512 ;
 pub const O_APPEND : i32 = 1024 ;
 pub const O_NONBLOCK : i32 = 2048 ;
 pub const O_NDELAY : u32 = 2048 ;
 pub const O_SYNC : u32 = 1052672 ;
 pub const O_FSYNC : u32 = 1052672 ;
 pub const O_ASYNC : u32 = 8192 ;
 pub const __O_DIRECTORY : u32 = 65536 ;
 pub const __O_NOFOLLOW : u32 = 131072 ;
 pub const __O_CLOEXEC : u32 = 524288 ;
 pub const __O_DIRECT : u32 = 16384 ;
 pub const __O_NOATIME : u32 = 262144 ;
 pub const __O_PATH : u32 = 2097152 ;
 pub const __O_DSYNC : u32 = 4096 ;
 pub const __O_TMPFILE : u32 = 4259840 ;
 pub const F_GETLK : u32 = 5 ;
 pub const F_SETLK : u32 = 6 ;
 pub const F_SETLKW : u32 = 7 ;
 pub const F_OFD_GETLK : u32 = 36 ;
 pub const F_OFD_SETLK : u32 = 37 ;
 pub const F_OFD_SETLKW : u32 = 38 ;
 pub const O_LARGEFILE : u32 = 0 ;
 pub const O_DIRECTORY : u32 = 65536 ;
 pub const O_NOFOLLOW : u32 = 131072 ;
 pub const O_CLOEXEC : u32 = 524288 ;
 pub const O_DIRECT : u32 = 16384 ;
 pub const O_NOATIME : u32 = 262144 ;
 pub const O_PATH : u32 = 2097152 ;
 pub const O_TMPFILE : u32 = 4259840 ;
 pub const O_DSYNC : u32 = 4096 ;
 pub const O_RSYNC : u32 = 1052672 ;
 pub const F_DUPFD : u32 = 0 ;
 pub const F_GETFD : u32 = 1 ;
 pub const F_SETFD : u32 = 2 ;
 pub const F_GETFL : u32 = 3 ;
 pub const F_SETFL : u32 = 4 ;
 pub const __F_SETOWN : u32 = 8 ;
 pub const __F_GETOWN : u32 = 9 ;
 pub const F_SETOWN : u32 = 8 ;
 pub const F_GETOWN : u32 = 9 ;
 pub const __F_SETSIG : u32 = 10 ;
 pub const __F_GETSIG : u32 = 11 ;
 pub const __F_SETOWN_EX : u32 = 15 ;
 pub const __F_GETOWN_EX : u32 = 16 ;
 pub const F_SETSIG : u32 = 10 ;
 pub const F_GETSIG : u32 = 11 ;
 pub const F_SETOWN_EX : u32 = 15 ;
 pub const F_GETOWN_EX : u32 = 16 ;
 pub const F_SETLEASE : u32 = 1024 ;
 pub const F_GETLEASE : u32 = 1025 ;
 pub const F_NOTIFY : u32 = 1026 ;
 pub const F_SETPIPE_SZ : u32 = 1031 ;
 pub const F_GETPIPE_SZ : u32 = 1032 ;
 pub const F_ADD_SEALS : u32 = 1033 ;
 pub const F_GET_SEALS : u32 = 1034 ;
 pub const F_GET_RW_HINT : u32 = 1035 ;
 pub const F_SET_RW_HINT : u32 = 1036 ;
 pub const F_GET_FILE_RW_HINT : u32 = 1037 ;
 pub const F_SET_FILE_RW_HINT : u32 = 1038 ;
 pub const F_DUPFD_CLOEXEC : u32 = 1030 ;
 pub const FD_CLOEXEC : u32 = 1 ;
 pub const F_RDLCK : u32 = 0 ;
 pub const F_WRLCK : u32 = 1 ;
 pub const F_UNLCK : u32 = 2 ;
 pub const F_EXLCK : u32 = 4 ;
 pub const F_SHLCK : u32 = 8 ;
 pub const LOCK_SH : u32 = 1 ;
 pub const LOCK_EX : u32 = 2 ;
 pub const LOCK_NB : u32 = 4 ;
 pub const LOCK_UN : u32 = 8 ;
 pub const LOCK_MAND : u32 = 32 ;
 pub const LOCK_READ : u32 = 64 ;
 pub const LOCK_WRITE : u32 = 128 ;
 pub const LOCK_RW : u32 = 192 ;
 pub const DN_ACCESS : u32 = 1 ;
 pub const DN_MODIFY : u32 = 2 ;
 pub const DN_CREATE : u32 = 4 ;
 pub const DN_DELETE : u32 = 8 ;
 pub const DN_RENAME : u32 = 16 ;
 pub const DN_ATTRIB : u32 = 32 ;
 pub const DN_MULTISHOT : u32 = 2147483648 ;
 pub const F_SEAL_SEAL : u32 = 1 ;
 pub const F_SEAL_SHRINK : u32 = 2 ;
 pub const F_SEAL_GROW : u32 = 4 ;
 pub const F_SEAL_WRITE : u32 = 8 ;
 pub const RWF_WRITE_LIFE_NOT_SET : u32 = 0 ;
 pub const RWH_WRITE_LIFE_NONE : u32 = 1 ;
 pub const RWH_WRITE_LIFE_SHORT : u32 = 2 ;
 pub const RWH_WRITE_LIFE_MEDIUM : u32 = 3 ;
 pub const RWH_WRITE_LIFE_LONG : u32 = 4 ;
 pub const RWH_WRITE_LIFE_EXTREME : u32 = 5 ;
 pub const FAPPEND : u32 = 1024 ;
 pub const FFSYNC : u32 = 1052672 ;
 pub const FASYNC : u32 = 8192 ;
 pub const FNONBLOCK : u32 = 2048 ;
 pub const FNDELAY : u32 = 2048 ;
 pub const __POSIX_FADV_DONTNEED : u32 = 4 ;
 pub const __POSIX_FADV_NOREUSE : u32 = 5 ;
 pub const POSIX_FADV_NORMAL : u32 = 0 ;
 pub const POSIX_FADV_RANDOM : u32 = 1 ;
 pub const POSIX_FADV_SEQUENTIAL : u32 = 2 ;
 pub const POSIX_FADV_WILLNEED : u32 = 3 ;
 pub const POSIX_FADV_DONTNEED : u32 = 4 ;
 pub const POSIX_FADV_NOREUSE : u32 = 5 ;
 pub const SYNC_FILE_RANGE_WAIT_BEFORE : u32 = 1 ;
 pub const SYNC_FILE_RANGE_WRITE : u32 = 2 ;
 pub const SYNC_FILE_RANGE_WAIT_AFTER : u32 = 4 ;
 pub const SPLICE_F_MOVE : u32 = 1 ;
 pub const SPLICE_F_NONBLOCK : u32 = 2 ;
 pub const SPLICE_F_MORE : u32 = 4 ;
 pub const SPLICE_F_GIFT : u32 = 8 ;
 pub const FALLOC_FL_KEEP_SIZE : u32 = 1 ;
 pub const FALLOC_FL_PUNCH_HOLE : u32 = 2 ;
 pub const FALLOC_FL_NO_HIDE_STALE : u32 = 4 ;
 pub const FALLOC_FL_COLLAPSE_RANGE : u32 = 8 ;
 pub const FALLOC_FL_ZERO_RANGE : u32 = 16 ;
 pub const FALLOC_FL_INSERT_RANGE : u32 = 32 ;
 pub const FALLOC_FL_UNSHARE_RANGE : u32 = 64 ;
 pub const MAX_HANDLE_SZ : u32 = 128 ;
 pub const AT_FDCWD : i32 = - 100 ;
 pub const AT_SYMLINK_NOFOLLOW : u32 = 256 ;
 pub const AT_REMOVEDIR : u32 = 512 ;
 pub const AT_SYMLINK_FOLLOW : u32 = 1024 ;
 pub const AT_NO_AUTOMOUNT : u32 = 2048 ;
 pub const AT_EMPTY_PATH : u32 = 4096 ;
 pub const AT_EACCESS : u32 = 512 ;
 pub const _BITS_STAT_H : u32 = 1 ;
 pub const _STAT_VER_KERNEL : u32 = 0 ;
 pub const _STAT_VER_LINUX : u32 = 1 ;
 pub const _MKNOD_VER_LINUX : u32 = 0 ;
 pub const _STAT_VER : u32 = 1 ;
 pub const __S_IFMT : u32 = 61440 ;
 pub const __S_IFDIR : u32 = 16384 ;
 pub const __S_IFCHR : u32 = 8192 ;
 pub const __S_IFBLK : u32 = 24576 ;
 pub const __S_IFREG : u32 = 32768 ;
 pub const __S_IFIFO : u32 = 4096 ;
 pub const __S_IFLNK : u32 = 40960 ;
 pub const __S_IFSOCK : u32 = 49152 ;
 pub const __S_ISUID : u32 = 2048 ;
 pub const __S_ISGID : u32 = 1024 ;
 pub const __S_ISVTX : u32 = 512 ;
 pub const __S_IREAD : u32 = 256 ;
 pub const __S_IWRITE : u32 = 128 ;
 pub const __S_IEXEC : u32 = 64 ;
 pub const UTIME_NOW : u32 = 1073741823 ;
 pub const UTIME_OMIT : u32 = 1073741822 ;
 pub const S_IFMT : u32 = 61440 ;
 pub const S_IFDIR : u32 = 16384 ;
 pub const S_IFCHR : u32 = 8192 ;
 pub const S_IFBLK : u32 = 24576 ;
 pub const S_IFREG : u32 = 32768 ;
 pub const S_IFIFO : u32 = 4096 ;
 pub const S_IFLNK : u32 = 40960 ;
 pub const S_IFSOCK : u32 = 49152 ;
 pub const S_ISUID : u32 = 2048 ;
 pub const S_ISGID : u32 = 1024 ;
 pub const S_ISVTX : u32 = 512 ;
 pub const S_IRUSR : u32 = 256 ;
 pub const S_IWUSR : u32 = 128 ;
 pub const S_IXUSR : u32 = 64 ;
 pub const S_IRWXU : u32 = 448 ;
 pub const S_IRGRP : u32 = 32 ;
 pub const S_IWGRP : u32 = 16 ;
 pub const S_IXGRP : u32 = 8 ;
 pub const S_IRWXG : u32 = 56 ;
 pub const S_IROTH : u32 = 4 ;
 pub const S_IWOTH : u32 = 2 ;
 pub const S_IXOTH : u32 = 1 ;
 pub const S_IRWXO : u32 = 7 ;
 pub const R_OK : u32 = 4 ;
 pub const W_OK : u32 = 2 ;
 pub const X_OK : u32 = 1 ;
 pub const F_OK : u32 = 0 ;
 pub const SEEK_SET : u32 = 0 ;
 pub const SEEK_CUR : u32 = 1 ;
 pub const SEEK_END : u32 = 2 ;
 pub const F_ULOCK : u32 = 0 ;
 pub const F_LOCK : u32 = 1 ;
 pub const F_TLOCK : u32 = 2 ;
 pub const F_TEST : u32 = 3 ;
 pub const _SYS_PRCTL_H : u32 = 1 ;
 pub const __BITS_PER_LONG : u32 = 64 ;
 pub const PR_SET_PDEATHSIG : u32 = 1 ;
 pub const PR_GET_PDEATHSIG : u32 = 2 ;
 pub const PR_GET_DUMPABLE : u32 = 3 ;
 pub const PR_SET_DUMPABLE : u32 = 4 ;
 pub const PR_GET_UNALIGN : u32 = 5 ;
 pub const PR_SET_UNALIGN : u32 = 6 ;
 pub const PR_UNALIGN_NOPRINT : u32 = 1 ;
 pub const PR_UNALIGN_SIGBUS : u32 = 2 ;
 pub const PR_GET_KEEPCAPS : u32 = 7 ;
 pub const PR_SET_KEEPCAPS : u32 = 8 ;
 pub const PR_GET_FPEMU : u32 = 9 ;
 pub const PR_SET_FPEMU : u32 = 10 ;
 pub const PR_FPEMU_NOPRINT : u32 = 1 ;
 pub const PR_FPEMU_SIGFPE : u32 = 2 ;
 pub const PR_GET_FPEXC : u32 = 11 ;
 pub const PR_SET_FPEXC : u32 = 12 ;
 pub const PR_FP_EXC_SW_ENABLE : u32 = 128 ;
 pub const PR_FP_EXC_DIV : u32 = 65536 ;
 pub const PR_FP_EXC_OVF : u32 = 131072 ;
 pub const PR_FP_EXC_UND : u32 = 262144 ;
 pub const PR_FP_EXC_RES : u32 = 524288 ;
 pub const PR_FP_EXC_INV : u32 = 1048576 ;
 pub const PR_FP_EXC_DISABLED : u32 = 0 ;
 pub const PR_FP_EXC_NONRECOV : u32 = 1 ;
 pub const PR_FP_EXC_ASYNC : u32 = 2 ;
 pub const PR_FP_EXC_PRECISE : u32 = 3 ;
 pub const PR_GET_TIMING : u32 = 13 ;
 pub const PR_SET_TIMING : u32 = 14 ;
 pub const PR_TIMING_STATISTICAL : u32 = 0 ;
 pub const PR_TIMING_TIMESTAMP : u32 = 1 ;
 pub const PR_SET_NAME : u32 = 15 ;
 pub const PR_GET_NAME : u32 = 16 ;
 pub const PR_GET_ENDIAN : u32 = 19 ;
 pub const PR_SET_ENDIAN : u32 = 20 ;
 pub const PR_ENDIAN_BIG : u32 = 0 ;
 pub const PR_ENDIAN_LITTLE : u32 = 1 ;
 pub const PR_ENDIAN_PPC_LITTLE : u32 = 2 ;
 pub const PR_GET_SECCOMP : u32 = 21 ;
 pub const PR_SET_SECCOMP : u32 = 22 ;
 pub const PR_CAPBSET_READ : u32 = 23 ;
 pub const PR_CAPBSET_DROP : u32 = 24 ;
 pub const PR_GET_TSC : u32 = 25 ;
 pub const PR_SET_TSC : u32 = 26 ;
 pub const PR_TSC_ENABLE : u32 = 1 ;
 pub const PR_TSC_SIGSEGV : u32 = 2 ;
 pub const PR_GET_SECUREBITS : u32 = 27 ;
 pub const PR_SET_SECUREBITS : u32 = 28 ;
 pub const PR_SET_TIMERSLACK : u32 = 29 ;
 pub const PR_GET_TIMERSLACK : u32 = 30 ;
 pub const PR_TASK_PERF_EVENTS_DISABLE : u32 = 31 ;
 pub const PR_TASK_PERF_EVENTS_ENABLE : u32 = 32 ;
 pub const PR_MCE_KILL : u32 = 33 ;
 pub const PR_MCE_KILL_CLEAR : u32 = 0 ;
 pub const PR_MCE_KILL_SET : u32 = 1 ;
 pub const PR_MCE_KILL_LATE : u32 = 0 ;
 pub const PR_MCE_KILL_EARLY : u32 = 1 ;
 pub const PR_MCE_KILL_DEFAULT : u32 = 2 ;
 pub const PR_MCE_KILL_GET : u32 = 34 ;
 pub const PR_SET_MM : u32 = 35 ;
 pub const PR_SET_MM_START_CODE : u32 = 1 ;
 pub const PR_SET_MM_END_CODE : u32 = 2 ;
 pub const PR_SET_MM_START_DATA : u32 = 3 ;
 pub const PR_SET_MM_END_DATA : u32 = 4 ;
 pub const PR_SET_MM_START_STACK : u32 = 5 ;
 pub const PR_SET_MM_START_BRK : u32 = 6 ;
 pub const PR_SET_MM_BRK : u32 = 7 ;
 pub const PR_SET_MM_ARG_START : u32 = 8 ;
 pub const PR_SET_MM_ARG_END : u32 = 9 ;
 pub const PR_SET_MM_ENV_START : u32 = 10 ;
 pub const PR_SET_MM_ENV_END : u32 = 11 ;
 pub const PR_SET_MM_AUXV : u32 = 12 ;
 pub const PR_SET_MM_EXE_FILE : u32 = 13 ;
 pub const PR_SET_MM_MAP : u32 = 14 ;
 pub const PR_SET_MM_MAP_SIZE : u32 = 15 ;
 pub const PR_SET_PTRACER : u32 = 1499557217 ;
 pub const PR_SET_CHILD_SUBREAPER : u32 = 36 ;
 pub const PR_GET_CHILD_SUBREAPER : u32 = 37 ;
 pub const PR_SET_NO_NEW_PRIVS : u32 = 38 ;
 pub const PR_GET_NO_NEW_PRIVS : u32 = 39 ;
 pub const PR_GET_TID_ADDRESS : u32 = 40 ;
 pub const PR_SET_THP_DISABLE : u32 = 41 ;
 pub const PR_GET_THP_DISABLE : u32 = 42 ;
 pub const PR_MPX_ENABLE_MANAGEMENT : u32 = 43 ;
 pub const PR_MPX_DISABLE_MANAGEMENT : u32 = 44 ;
 pub const PR_SET_FP_MODE : u32 = 45 ;
 pub const PR_GET_FP_MODE : u32 = 46 ;
 pub const PR_FP_MODE_FR : u32 = 1 ;
 pub const PR_FP_MODE_FRE : u32 = 2 ;
 pub const PR_CAP_AMBIENT : u32 = 47 ;
 pub const PR_CAP_AMBIENT_IS_SET : u32 = 1 ;
 pub const PR_CAP_AMBIENT_RAISE : u32 = 2 ;
 pub const PR_CAP_AMBIENT_LOWER : u32 = 3 ;
 pub const PR_CAP_AMBIENT_CLEAR_ALL : u32 = 4 ;
 pub const PR_SVE_SET_VL : u32 = 50 ;
 pub const PR_SVE_SET_VL_ONEXEC : u32 = 262144 ;
 pub const PR_SVE_GET_VL : u32 = 51 ;
 pub const PR_SVE_VL_LEN_MASK : u32 = 65535 ;
 pub const PR_SVE_VL_INHERIT : u32 = 131072 ;
 pub const PR_GET_SPECULATION_CTRL : u32 = 52 ;
 pub const PR_SET_SPECULATION_CTRL : u32 = 53 ;
 pub const PR_SPEC_STORE_BYPASS : u32 = 0 ;
 pub const PR_SPEC_INDIRECT_BRANCH : u32 = 1 ;
 pub const PR_SPEC_NOT_AFFECTED : u32 = 0 ;
 pub const PR_SPEC_PRCTL : u32 = 1 ;
 pub const PR_SPEC_ENABLE : u32 = 2 ;
 pub const PR_SPEC_DISABLE : u32 = 4 ;
 pub const PR_SPEC_FORCE_DISABLE : u32 = 8 ;
 pub const PR_GET_DISPLAY_LSM : u32 = 1000000 ;
 pub const PR_SET_DISPLAY_LSM : u32 = 1000001 ;
 pub const BPF_LD : u32 = 0 ;
 pub const BPF_LDX : u32 = 1 ;
 pub const BPF_ST : u32 = 2 ;
 pub const BPF_STX : u32 = 3 ;
 pub const BPF_ALU : u32 = 4 ;
 pub const BPF_JMP : u32 = 5 ;
 pub const BPF_RET : u32 = 6 ;
 pub const BPF_MISC : u32 = 7 ;
 pub const BPF_W : u32 = 0 ;
 pub const BPF_H : u32 = 8 ;
 pub const BPF_B : u32 = 16 ;
 pub const BPF_IMM : u32 = 0 ;
 pub const BPF_ABS : u32 = 32 ;
 pub const BPF_IND : u32 = 64 ;
 pub const BPF_MEM : u32 = 96 ;
 pub const BPF_LEN : u32 = 128 ;
 pub const BPF_MSH : u32 = 160 ;
 pub const BPF_ADD : u32 = 0 ;
 pub const BPF_SUB : u32 = 16 ;
 pub const BPF_MUL : u32 = 32 ;
 pub const BPF_DIV : u32 = 48 ;
 pub const BPF_OR : u32 = 64 ;
 pub const BPF_AND : u32 = 80 ;
 pub const BPF_LSH : u32 = 96 ;
 pub const BPF_RSH : u32 = 112 ;
 pub const BPF_NEG : u32 = 128 ;
 pub const BPF_MOD : u32 = 144 ;
 pub const BPF_XOR : u32 = 160 ;
 pub const BPF_JA : u32 = 0 ;
 pub const BPF_JEQ : u32 = 16 ;
 pub const BPF_JGT : u32 = 32 ;
 pub const BPF_JGE : u32 = 48 ;
 pub const BPF_JSET : u32 = 64 ;
 pub const BPF_K : u32 = 0 ;
 pub const BPF_X : u32 = 8 ;
 pub const BPF_MAXINSNS : u32 = 4096 ;
 pub const BPF_MAJOR_VERSION : u32 = 1 ;
 pub const BPF_MINOR_VERSION : u32 = 1 ;
 pub const BPF_A : u32 = 16 ;
 pub const BPF_TAX : u32 = 0 ;
 pub const BPF_TXA : u32 = 128 ;
 pub const BPF_MEMWORDS : u32 = 16 ;
 pub const SKF_AD_OFF : i32 = - 4096 ;
 pub const SKF_AD_PROTOCOL : u32 = 0 ;
 pub const SKF_AD_PKTTYPE : u32 = 4 ;
 pub const SKF_AD_IFINDEX : u32 = 8 ;
 pub const SKF_AD_NLATTR : u32 = 12 ;
 pub const SKF_AD_NLATTR_NEST : u32 = 16 ;
 pub const SKF_AD_MARK : u32 = 20 ;
 pub const SKF_AD_QUEUE : u32 = 24 ;
 pub const SKF_AD_HATYPE : u32 = 28 ;
 pub const SKF_AD_RXHASH : u32 = 32 ;
 pub const SKF_AD_CPU : u32 = 36 ;
 pub const SKF_AD_ALU_XOR_X : u32 = 40 ;
 pub const SKF_AD_VLAN_TAG : u32 = 44 ;
 pub const SKF_AD_VLAN_TAG_PRESENT : u32 = 48 ;
 pub const SKF_AD_PAY_OFFSET : u32 = 52 ;
 pub const SKF_AD_RANDOM : u32 = 56 ;
 pub const SKF_AD_VLAN_TPID : u32 = 60 ;
 pub const SKF_AD_MAX : u32 = 64 ;
 pub const SKF_NET_OFF : i32 = - 1048576 ;
 pub const SKF_LL_OFF : i32 = - 2097152 ;
 pub const BPF_NET_OFF : i32 = - 1048576 ;
 pub const BPF_LL_OFF : i32 = - 2097152 ;
 pub const __X32_SYSCALL_BIT : u32 = 1073741824 ;
 pub const _ASM_X86_UNISTD_64_H : u32 = 1 ;
 pub const __NR_read : u32 = 0 ;
 pub const __NR_write : u32 = 1 ;
 pub const __NR_open : u32 = 2 ;
 pub const __NR_close : u32 = 3 ;
 pub const __NR_stat : u32 = 4 ;
 pub const __NR_fstat : u32 = 5 ;
 pub const __NR_lstat : u32 = 6 ;
 pub const __NR_poll : u32 = 7 ;
 pub const __NR_lseek : u32 = 8 ;
 pub const __NR_mmap : u32 = 9 ;
 pub const __NR_mprotect : u32 = 10 ;
 pub const __NR_munmap : u32 = 11 ;
 pub const __NR_brk : u32 = 12 ;
 pub const __NR_rt_sigaction : u32 = 13 ;
 pub const __NR_rt_sigprocmask : u32 = 14 ;
 pub const __NR_rt_sigreturn : u32 = 15 ;
 pub const __NR_ioctl : u32 = 16 ;
 pub const __NR_pread64 : u32 = 17 ;
 pub const __NR_pwrite64 : u32 = 18 ;
 pub const __NR_readv : u32 = 19 ;
 pub const __NR_writev : u32 = 20 ;
 pub const __NR_access : u32 = 21 ;
 pub const __NR_pipe : u32 = 22 ;
 pub const __NR_select : u32 = 23 ;
 pub const __NR_sched_yield : u32 = 24 ;
 pub const __NR_mremap : u32 = 25 ;
 pub const __NR_msync : u32 = 26 ;
 pub const __NR_mincore : u32 = 27 ;
 pub const __NR_madvise : u32 = 28 ;
 pub const __NR_shmget : u32 = 29 ;
 pub const __NR_shmat : u32 = 30 ;
 pub const __NR_shmctl : u32 = 31 ;
 pub const __NR_dup : u32 = 32 ;
 pub const __NR_dup2 : u32 = 33 ;
 pub const __NR_pause : u32 = 34 ;
 pub const __NR_nanosleep : u32 = 35 ;
 pub const __NR_getitimer : u32 = 36 ;
 pub const __NR_alarm : u32 = 37 ;
 pub const __NR_setitimer : u32 = 38 ;
 pub const __NR_getpid : u32 = 39 ;
 pub const __NR_sendfile : u32 = 40 ;
 pub const __NR_socket : u32 = 41 ;
 pub const __NR_connect : u32 = 42 ;
 pub const __NR_accept : u32 = 43 ;
 pub const __NR_sendto : u32 = 44 ;
 pub const __NR_recvfrom : u32 = 45 ;
 pub const __NR_sendmsg : u32 = 46 ;
 pub const __NR_recvmsg : u32 = 47 ;
 pub const __NR_shutdown : u32 = 48 ;
 pub const __NR_bind : u32 = 49 ;
 pub const __NR_listen : u32 = 50 ;
 pub const __NR_getsockname : u32 = 51 ;
 pub const __NR_getpeername : u32 = 52 ;
 pub const __NR_socketpair : u32 = 53 ;
 pub const __NR_setsockopt : u32 = 54 ;
 pub const __NR_getsockopt : u32 = 55 ;
 pub const __NR_clone : u32 = 56 ;
 pub const __NR_fork : u32 = 57 ;
 pub const __NR_vfork : u32 = 58 ;
 pub const __NR_execve : u32 = 59 ;
 pub const __NR_exit : u32 = 60 ;
 pub const __NR_wait4 : u32 = 61 ;
 pub const __NR_kill : u32 = 62 ;
 pub const __NR_uname : u32 = 63 ;
 pub const __NR_semget : u32 = 64 ;
 pub const __NR_semop : u32 = 65 ;
 pub const __NR_semctl : u32 = 66 ;
 pub const __NR_shmdt : u32 = 67 ;
 pub const __NR_msgget : u32 = 68 ;
 pub const __NR_msgsnd : u32 = 69 ;
 pub const __NR_msgrcv : u32 = 70 ;
 pub const __NR_msgctl : u32 = 71 ;
 pub const __NR_fcntl : u32 = 72 ;
 pub const __NR_flock : u32 = 73 ;
 pub const __NR_fsync : u32 = 74 ;
 pub const __NR_fdatasync : u32 = 75 ;
 pub const __NR_truncate : u32 = 76 ;
 pub const __NR_ftruncate : u32 = 77 ;
 pub const __NR_getdents : u32 = 78 ;
 pub const __NR_getcwd : u32 = 79 ;
 pub const __NR_chdir : u32 = 80 ;
 pub const __NR_fchdir : u32 = 81 ;
 pub const __NR_rename : u32 = 82 ;
 pub const __NR_mkdir : u32 = 83 ;
 pub const __NR_rmdir : u32 = 84 ;
 pub const __NR_creat : u32 = 85 ;
 pub const __NR_link : u32 = 86 ;
 pub const __NR_unlink : u32 = 87 ;
 pub const __NR_symlink : u32 = 88 ;
 pub const __NR_readlink : u32 = 89 ;
 pub const __NR_chmod : u32 = 90 ;
 pub const __NR_fchmod : u32 = 91 ;
 pub const __NR_chown : u32 = 92 ;
 pub const __NR_fchown : u32 = 93 ;
 pub const __NR_lchown : u32 = 94 ;
 pub const __NR_umask : u32 = 95 ;
 pub const __NR_gettimeofday : u32 = 96 ;
 pub const __NR_getrlimit : u32 = 97 ;
 pub const __NR_getrusage : u32 = 98 ;
 pub const __NR_sysinfo : u32 = 99 ;
 pub const __NR_times : u32 = 100 ;
 pub const __NR_ptrace : u32 = 101 ;
 pub const __NR_getuid : u32 = 102 ;
 pub const __NR_syslog : u32 = 103 ;
 pub const __NR_getgid : u32 = 104 ;
 pub const __NR_setuid : u32 = 105 ;
 pub const __NR_setgid : u32 = 106 ;
 pub const __NR_geteuid : u32 = 107 ;
 pub const __NR_getegid : u32 = 108 ;
 pub const __NR_setpgid : u32 = 109 ;
 pub const __NR_getppid : u32 = 110 ;
 pub const __NR_getpgrp : u32 = 111 ;
 pub const __NR_setsid : u32 = 112 ;
 pub const __NR_setreuid : u32 = 113 ;
 pub const __NR_setregid : u32 = 114 ;
 pub const __NR_getgroups : u32 = 115 ;
 pub const __NR_setgroups : u32 = 116 ;
 pub const __NR_setresuid : u32 = 117 ;
 pub const __NR_getresuid : u32 = 118 ;
 pub const __NR_setresgid : u32 = 119 ;
 pub const __NR_getresgid : u32 = 120 ;
 pub const __NR_getpgid : u32 = 121 ;
 pub const __NR_setfsuid : u32 = 122 ;
 pub const __NR_setfsgid : u32 = 123 ;
 pub const __NR_getsid : u32 = 124 ;
 pub const __NR_capget : u32 = 125 ;
 pub const __NR_capset : u32 = 126 ;
 pub const __NR_rt_sigpending : u32 = 127 ;
 pub const __NR_rt_sigtimedwait : u32 = 128 ;
 pub const __NR_rt_sigqueueinfo : u32 = 129 ;
 pub const __NR_rt_sigsuspend : u32 = 130 ;
 pub const __NR_sigaltstack : u32 = 131 ;
 pub const __NR_utime : u32 = 132 ;
 pub const __NR_mknod : u32 = 133 ;
 pub const __NR_uselib : u32 = 134 ;
 pub const __NR_personality : u32 = 135 ;
 pub const __NR_ustat : u32 = 136 ;
 pub const __NR_statfs : u32 = 137 ;
 pub const __NR_fstatfs : u32 = 138 ;
 pub const __NR_sysfs : u32 = 139 ;
 pub const __NR_getpriority : u32 = 140 ;
 pub const __NR_setpriority : u32 = 141 ;
 pub const __NR_sched_setparam : u32 = 142 ;
 pub const __NR_sched_getparam : u32 = 143 ;
 pub const __NR_sched_setscheduler : u32 = 144 ;
 pub const __NR_sched_getscheduler : u32 = 145 ;
 pub const __NR_sched_get_priority_max : u32 = 146 ;
 pub const __NR_sched_get_priority_min : u32 = 147 ;
 pub const __NR_sched_rr_get_interval : u32 = 148 ;
 pub const __NR_mlock : u32 = 149 ;
 pub const __NR_munlock : u32 = 150 ;
 pub const __NR_mlockall : u32 = 151 ;
 pub const __NR_munlockall : u32 = 152 ;
 pub const __NR_vhangup : u32 = 153 ;
 pub const __NR_modify_ldt : u32 = 154 ;
 pub const __NR_pivot_root : u32 = 155 ;
 pub const __NR__sysctl : u32 = 156 ;
 pub const __NR_prctl : u32 = 157 ;
 pub const __NR_arch_prctl : u32 = 158 ;
 pub const __NR_adjtimex : u32 = 159 ;
 pub const __NR_setrlimit : u32 = 160 ;
 pub const __NR_chroot : u32 = 161 ;
 pub const __NR_sync : u32 = 162 ;
 pub const __NR_acct : u32 = 163 ;
 pub const __NR_settimeofday : u32 = 164 ;
 pub const __NR_mount : u32 = 165 ;
 pub const __NR_umount2 : u32 = 166 ;
 pub const __NR_swapon : u32 = 167 ;
 pub const __NR_swapoff : u32 = 168 ;
 pub const __NR_reboot : u32 = 169 ;
 pub const __NR_sethostname : u32 = 170 ;
 pub const __NR_setdomainname : u32 = 171 ;
 pub const __NR_iopl : u32 = 172 ;
 pub const __NR_ioperm : u32 = 173 ;
 pub const __NR_create_module : u32 = 174 ;
 pub const __NR_init_module : u32 = 175 ;
 pub const __NR_delete_module : u32 = 176 ;
 pub const __NR_get_kernel_syms : u32 = 177 ;
 pub const __NR_query_module : u32 = 178 ;
 pub const __NR_quotactl : u32 = 179 ;
 pub const __NR_nfsservctl : u32 = 180 ;
 pub const __NR_getpmsg : u32 = 181 ;
 pub const __NR_putpmsg : u32 = 182 ;
 pub const __NR_afs_syscall : u32 = 183 ;
 pub const __NR_tuxcall : u32 = 184 ;
 pub const __NR_security : u32 = 185 ;
 pub const __NR_gettid : u32 = 186 ;
 pub const __NR_readahead : u32 = 187 ;
 pub const __NR_setxattr : u32 = 188 ;
 pub const __NR_lsetxattr : u32 = 189 ;
 pub const __NR_fsetxattr : u32 = 190 ;
 pub const __NR_getxattr : u32 = 191 ;
 pub const __NR_lgetxattr : u32 = 192 ;
 pub const __NR_fgetxattr : u32 = 193 ;
 pub const __NR_listxattr : u32 = 194 ;
 pub const __NR_llistxattr : u32 = 195 ;
 pub const __NR_flistxattr : u32 = 196 ;
 pub const __NR_removexattr : u32 = 197 ;
 pub const __NR_lremovexattr : u32 = 198 ;
 pub const __NR_fremovexattr : u32 = 199 ;
 pub const __NR_tkill : u32 = 200 ;
 pub const __NR_time : u32 = 201 ;
 pub const __NR_futex : u32 = 202 ;
 pub const __NR_sched_setaffinity : u32 = 203 ;
 pub const __NR_sched_getaffinity : u32 = 204 ;
 pub const __NR_set_thread_area : u32 = 205 ;
 pub const __NR_io_setup : u32 = 206 ;
 pub const __NR_io_destroy : u32 = 207 ;
 pub const __NR_io_getevents : u32 = 208 ;
 pub const __NR_io_submit : u32 = 209 ;
 pub const __NR_io_cancel : u32 = 210 ;
 pub const __NR_get_thread_area : u32 = 211 ;
 pub const __NR_lookup_dcookie : u32 = 212 ;
 pub const __NR_epoll_create : u32 = 213 ;
 pub const __NR_epoll_ctl_old : u32 = 214 ;
 pub const __NR_epoll_wait_old : u32 = 215 ;
 pub const __NR_remap_file_pages : u32 = 216 ;
 pub const __NR_getdents64 : u32 = 217 ;
 pub const __NR_set_tid_address : u32 = 218 ;
 pub const __NR_restart_syscall : u32 = 219 ;
 pub const __NR_semtimedop : u32 = 220 ;
 pub const __NR_fadvise64 : u32 = 221 ;
 pub const __NR_timer_create : u32 = 222 ;
 pub const __NR_timer_settime : u32 = 223 ;
 pub const __NR_timer_gettime : u32 = 224 ;
 pub const __NR_timer_getoverrun : u32 = 225 ;
 pub const __NR_timer_delete : u32 = 226 ;
 pub const __NR_clock_settime : u32 = 227 ;
 pub const __NR_clock_gettime : u32 = 228 ;
 pub const __NR_clock_getres : u32 = 229 ;
 pub const __NR_clock_nanosleep : u32 = 230 ;
 pub const __NR_exit_group : u32 = 231 ;
 pub const __NR_epoll_wait : u32 = 232 ;
 pub const __NR_epoll_ctl : u32 = 233 ;
 pub const __NR_tgkill : u32 = 234 ;
 pub const __NR_utimes : u32 = 235 ;
 pub const __NR_vserver : u32 = 236 ;
 pub const __NR_mbind : u32 = 237 ;
 pub const __NR_set_mempolicy : u32 = 238 ;
 pub const __NR_get_mempolicy : u32 = 239 ;
 pub const __NR_mq_open : u32 = 240 ;
 pub const __NR_mq_unlink : u32 = 241 ;
 pub const __NR_mq_timedsend : u32 = 242 ;
 pub const __NR_mq_timedreceive : u32 = 243 ;
 pub const __NR_mq_notify : u32 = 244 ;
 pub const __NR_mq_getsetattr : u32 = 245 ;
 pub const __NR_kexec_load : u32 = 246 ;
 pub const __NR_waitid : u32 = 247 ;
 pub const __NR_add_key : u32 = 248 ;
 pub const __NR_request_key : u32 = 249 ;
 pub const __NR_keyctl : u32 = 250 ;
 pub const __NR_ioprio_set : u32 = 251 ;
 pub const __NR_ioprio_get : u32 = 252 ;
 pub const __NR_inotify_init : u32 = 253 ;
 pub const __NR_inotify_add_watch : u32 = 254 ;
 pub const __NR_inotify_rm_watch : u32 = 255 ;
 pub const __NR_migrate_pages : u32 = 256 ;
 pub const __NR_openat : u32 = 257 ;
 pub const __NR_mkdirat : u32 = 258 ;
 pub const __NR_mknodat : u32 = 259 ;
 pub const __NR_fchownat : u32 = 260 ;
 pub const __NR_futimesat : u32 = 261 ;
 pub const __NR_newfstatat : u32 = 262 ;
 pub const __NR_unlinkat : u32 = 263 ;
 pub const __NR_renameat : u32 = 264 ;
 pub const __NR_linkat : u32 = 265 ;
 pub const __NR_symlinkat : u32 = 266 ;
 pub const __NR_readlinkat : u32 = 267 ;
 pub const __NR_fchmodat : u32 = 268 ;
 pub const __NR_faccessat : u32 = 269 ;
 pub const __NR_pselect6 : u32 = 270 ;
 pub const __NR_ppoll : u32 = 271 ;
 pub const __NR_unshare : u32 = 272 ;
 pub const __NR_set_robust_list : u32 = 273 ;
 pub const __NR_get_robust_list : u32 = 274 ;
 pub const __NR_splice : u32 = 275 ;
 pub const __NR_tee : u32 = 276 ;
 pub const __NR_sync_file_range : u32 = 277 ;
 pub const __NR_vmsplice : u32 = 278 ;
 pub const __NR_move_pages : u32 = 279 ;
 pub const __NR_utimensat : u32 = 280 ;
 pub const __NR_epoll_pwait : u32 = 281 ;
 pub const __NR_signalfd : u32 = 282 ;
 pub const __NR_timerfd_create : u32 = 283 ;
 pub const __NR_eventfd : u32 = 284 ;
 pub const __NR_fallocate : u32 = 285 ;
 pub const __NR_timerfd_settime : u32 = 286 ;
 pub const __NR_timerfd_gettime : u32 = 287 ;
 pub const __NR_accept4 : u32 = 288 ;
 pub const __NR_signalfd4 : u32 = 289 ;
 pub const __NR_eventfd2 : u32 = 290 ;
 pub const __NR_epoll_create1 : u32 = 291 ;
 pub const __NR_dup3 : u32 = 292 ;
 pub const __NR_pipe2 : u32 = 293 ;
 pub const __NR_inotify_init1 : u32 = 294 ;
 pub const __NR_preadv : u32 = 295 ;
 pub const __NR_pwritev : u32 = 296 ;
 pub const __NR_rt_tgsigqueueinfo : u32 = 297 ;
 pub const __NR_perf_event_open : u32 = 298 ;
 pub const __NR_recvmmsg : u32 = 299 ;
 pub const __NR_fanotify_init : u32 = 300 ;
 pub const __NR_fanotify_mark : u32 = 301 ;
 pub const __NR_prlimit64 : u32 = 302 ;
 pub const __NR_name_to_handle_at : u32 = 303 ;
 pub const __NR_open_by_handle_at : u32 = 304 ;
 pub const __NR_clock_adjtime : u32 = 305 ;
 pub const __NR_syncfs : u32 = 306 ;
 pub const __NR_sendmmsg : u32 = 307 ;
 pub const __NR_setns : u32 = 308 ;
 pub const __NR_getcpu : u32 = 309 ;
 pub const __NR_process_vm_readv : u32 = 310 ;
 pub const __NR_process_vm_writev : u32 = 311 ;
 pub const __NR_kcmp : u32 = 312 ;
 pub const __NR_finit_module : u32 = 313 ;
 pub const __NR_sched_setattr : u32 = 314 ;
 pub const __NR_sched_getattr : u32 = 315 ;
 pub const __NR_renameat2 : u32 = 316 ;
 pub const __NR_seccomp : u32 = 317 ;
 pub const __NR_getrandom : u32 = 318 ;
 pub const __NR_memfd_create : u32 = 319 ;
 pub const __NR_kexec_file_load : u32 = 320 ;
 pub const __NR_bpf : u32 = 321 ;
 pub const __NR_execveat : u32 = 322 ;
 pub const __NR_userfaultfd : u32 = 323 ;
 pub const __NR_membarrier : u32 = 324 ;
 pub const __NR_mlock2 : u32 = 325 ;
 pub const __NR_copy_file_range : u32 = 326 ;
 pub const __NR_preadv2 : u32 = 327 ;
 pub const __NR_pwritev2 : u32 = 328 ;
 pub const __NR_pkey_mprotect : u32 = 329 ;
 pub const __NR_pkey_alloc : u32 = 330 ;
 pub const __NR_pkey_free : u32 = 331 ;
 pub const __NR_statx : u32 = 332 ;
 pub const kMaxSyscalls : u32 = 100 ;
 pub const _LIBC_LIMITS_H_ : u32 = 1 ;
 pub const MB_LEN_MAX : u32 = 16 ;
 pub const CHAR_WIDTH : u32 = 8 ;
 pub const SCHAR_WIDTH : u32 = 8 ;
 pub const UCHAR_WIDTH : u32 = 8 ;
 pub const SHRT_WIDTH : u32 = 16 ;
 pub const USHRT_WIDTH : u32 = 16 ;
 pub const INT_WIDTH : u32 = 32 ;
 pub const UINT_WIDTH : u32 = 32 ;
 pub const LONG_WIDTH : u32 = 64 ;
 pub const ULONG_WIDTH : u32 = 64 ;
 pub const LLONG_WIDTH : u32 = 64 ;
 pub const ULLONG_WIDTH : u32 = 64 ;
 pub const _BITS_POSIX1_LIM_H : u32 = 1 ;
 pub const _POSIX_AIO_LISTIO_MAX : u32 = 2 ;
 pub const _POSIX_AIO_MAX : u32 = 1 ;
 pub const _POSIX_ARG_MAX : u32 = 4096 ;
 pub const _POSIX_CHILD_MAX : u32 = 25 ;
 pub const _POSIX_DELAYTIMER_MAX : u32 = 32 ;
 pub const _POSIX_HOST_NAME_MAX : u32 = 255 ;
 pub const _POSIX_LINK_MAX : u32 = 8 ;
 pub const _POSIX_LOGIN_NAME_MAX : u32 = 9 ;
 pub const _POSIX_MAX_CANON : u32 = 255 ;
 pub const _POSIX_MAX_INPUT : u32 = 255 ;
 pub const _POSIX_MQ_OPEN_MAX : u32 = 8 ;
 pub const _POSIX_MQ_PRIO_MAX : u32 = 32 ;
 pub const _POSIX_NAME_MAX : u32 = 14 ;
 pub const _POSIX_NGROUPS_MAX : u32 = 8 ;
 pub const _POSIX_OPEN_MAX : u32 = 20 ;
 pub const _POSIX_FD_SETSIZE : u32 = 20 ;
 pub const _POSIX_PATH_MAX : u32 = 256 ;
 pub const _POSIX_PIPE_BUF : u32 = 512 ;
 pub const _POSIX_RE_DUP_MAX : u32 = 255 ;
 pub const _POSIX_RTSIG_MAX : u32 = 8 ;
 pub const _POSIX_SEM_NSEMS_MAX : u32 = 256 ;
 pub const _POSIX_SEM_VALUE_MAX : u32 = 32767 ;
 pub const _POSIX_SIGQUEUE_MAX : u32 = 32 ;
 pub const _POSIX_SSIZE_MAX : u32 = 32767 ;
 pub const _POSIX_STREAM_MAX : u32 = 8 ;
 pub const _POSIX_SYMLINK_MAX : u32 = 255 ;
 pub const _POSIX_SYMLOOP_MAX : u32 = 8 ;
 pub const _POSIX_TIMER_MAX : u32 = 32 ;
 pub const _POSIX_TTY_NAME_MAX : u32 = 9 ;
 pub const _POSIX_TZNAME_MAX : u32 = 6 ;
 pub const _POSIX_QLIMIT : u32 = 1 ;
 pub const _POSIX_HIWAT : u32 = 512 ;
 pub const _POSIX_UIO_MAXIOV : u32 = 16 ;
 pub const _POSIX_CLOCKRES_MIN : u32 = 20000000 ;
 pub const NR_OPEN : u32 = 1024 ;
 pub const NGROUPS_MAX : u32 = 65536 ;
 pub const ARG_MAX : u32 = 131072 ;
 pub const LINK_MAX : u32 = 127 ;
 pub const MAX_CANON : u32 = 255 ;
 pub const MAX_INPUT : u32 = 255 ;
 pub const NAME_MAX : u32 = 255 ;
 pub const PATH_MAX : u32 = 4096 ;
 pub const PIPE_BUF : u32 = 4096 ;
 pub const XATTR_NAME_MAX : u32 = 255 ;
 pub const XATTR_SIZE_MAX : u32 = 65536 ;
 pub const XATTR_LIST_MAX : u32 = 65536 ;
 pub const RTSIG_MAX : u32 = 32 ;
 pub const _POSIX_THREAD_KEYS_MAX : u32 = 128 ;
 pub const PTHREAD_KEYS_MAX : u32 = 1024 ;
 pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS : u32 = 4 ;
 pub const PTHREAD_DESTRUCTOR_ITERATIONS : u32 = 4 ;
 pub const _POSIX_THREAD_THREADS_MAX : u32 = 64 ;
 pub const AIO_PRIO_DELTA_MAX : u32 = 20 ;
 pub const PTHREAD_STACK_MIN : u32 = 16384 ;
 pub const DELAYTIMER_MAX : u32 = 2147483647 ;
 pub const TTY_NAME_MAX : u32 = 32 ;
 pub const LOGIN_NAME_MAX : u32 = 256 ;
 pub const HOST_NAME_MAX : u32 = 64 ;
 pub const MQ_PRIO_MAX : u32 = 32768 ;
 pub const SEM_VALUE_MAX : u32 = 2147483647 ;
 pub const _BITS_POSIX2_LIM_H : u32 = 1 ;
 pub const _POSIX2_BC_BASE_MAX : u32 = 99 ;
 pub const _POSIX2_BC_DIM_MAX : u32 = 2048 ;
 pub const _POSIX2_BC_SCALE_MAX : u32 = 99 ;
 pub const _POSIX2_BC_STRING_MAX : u32 = 1000 ;
 pub const _POSIX2_COLL_WEIGHTS_MAX : u32 = 2 ;
 pub const _POSIX2_EXPR_NEST_MAX : u32 = 32 ;
 pub const _POSIX2_LINE_MAX : u32 = 2048 ;
 pub const _POSIX2_RE_DUP_MAX : u32 = 255 ;
 pub const _POSIX2_CHARCLASS_NAME_MAX : u32 = 14 ;
 pub const BC_BASE_MAX : u32 = 99 ;
 pub const BC_DIM_MAX : u32 = 2048 ;
 pub const BC_SCALE_MAX : u32 = 99 ;
 pub const BC_STRING_MAX : u32 = 1000 ;
 pub const COLL_WEIGHTS_MAX : u32 = 255 ;
 pub const EXPR_NEST_MAX : u32 = 32 ;
 pub const LINE_MAX : u32 = 2048 ;
 pub const CHARCLASS_NAME_MAX : u32 = 2048 ;
 pub const RE_DUP_MAX : u32 = 32767 ;
 pub const _XOPEN_LIM_H : u32 = 1 ;
 pub const _XOPEN_IOV_MAX : u32 = 16 ;
 pub const _BITS_UIO_LIM_H : u32 = 1 ;
 pub const __IOV_MAX : u32 = 1024 ;
 pub const IOV_MAX : u32 = 1024 ;
 pub const NL_ARGMAX : u32 = 4096 ;
 pub const NL_LANGMAX : u32 = 2048 ;
 pub const NZERO : u32 = 20 ;
 pub const WORD_BIT : u32 = 32 ;
 pub const LONG_BIT : u32 = 64 ;
 pub const _SYS_PARAM_H : u32 = 1 ;
 pub const _BITS_SIGNUM_H : u32 = 1 ;
 pub const _BITS_SIGNUM_GENERIC_H : u32 = 1 ;
 pub const SIGINT : u32 = 2 ;
 pub const SIGILL : u32 = 4 ;
 pub const SIGABRT : u32 = 6 ;
 pub const SIGFPE : u32 = 8 ;
 pub const SIGSEGV : u32 = 11 ;
 pub const SIGTERM : u32 = 15 ;
 pub const SIGHUP : u32 = 1 ;
 pub const SIGQUIT : u32 = 3 ;
 pub const SIGTRAP : u32 = 5 ;
 pub const SIGKILL : u32 = 9 ;
 pub const SIGBUS : u32 = 10 ;
 pub const SIGSYS : u32 = 12 ;
 pub const SIGPIPE : u32 = 13 ;
 pub const SIGALRM : u32 = 14 ;
 pub const SIGURG : u32 = 16 ;
 pub const SIGSTOP : u32 = 17 ;
 pub const SIGTSTP : u32 = 18 ;
 pub const SIGCONT : u32 = 19 ;
 pub const SIGCHLD : u32 = 20 ;
 pub const SIGTTIN : u32 = 21 ;
 pub const SIGTTOU : u32 = 22 ;
 pub const SIGPOLL : u32 = 23 ;
 pub const SIGXCPU : u32 = 24 ;
 pub const SIGXFSZ : u32 = 25 ;
 pub const SIGVTALRM : u32 = 26 ;
 pub const SIGPROF : u32 = 27 ;
 pub const SIGUSR1 : u32 = 30 ;
 pub const SIGUSR2 : u32 = 31 ;
 pub const SIGWINCH : u32 = 28 ;
 pub const SIGIO : u32 = 23 ;
 pub const SIGIOT : u32 = 6 ;
 pub const SIGCLD : u32 = 20 ;
 pub const __SIGRTMIN : u32 = 32 ;
 pub const __SIGRTMAX : u32 = 32 ;
 pub const _NSIG : u32 = 33 ;
 pub const SIGSTKFLT : u32 = 16 ;
 pub const SIGPWR : u32 = 30 ;
 pub const __sig_atomic_t_defined : u32 = 1 ;
 pub const __siginfo_t_defined : u32 = 1 ;
 pub const __SI_MAX_SIZE : u32 = 128 ;
 pub const _BITS_SIGINFO_ARCH_H : u32 = 1 ;
 pub const __SI_ERRNO_THEN_CODE : u32 = 1 ;
 pub const __SI_HAVE_SIGSYS : u32 = 1 ;
 pub const _BITS_SIGINFO_CONSTS_H : u32 = 1 ;
 pub const __SI_ASYNCIO_AFTER_SIGIO : u32 = 1 ;
 pub const _BITS_SIGINFO_CONSTS_ARCH_H : u32 = 1 ;
 pub const __sigevent_t_defined : u32 = 1 ;
 pub const __SIGEV_MAX_SIZE : u32 = 64 ;
 pub const _BITS_SIGEVENT_CONSTS_H : u32 = 1 ;
 pub const NSIG : u32 = 33 ;
 pub const SA_NOCLDSTOP : u32 = 1 ;
 pub const SA_NOCLDWAIT : u32 = 2 ;
 pub const SA_SIGINFO : u32 = 4 ;
 pub const SA_ONSTACK : u32 = 134217728 ;
 pub const SA_RESTART : u32 = 268435456 ;
 pub const SA_NODEFER : u32 = 1073741824 ;
 pub const SA_RESETHAND : u32 = 2147483648 ;
 pub const SA_INTERRUPT : u32 = 536870912 ;
 pub const SA_NOMASK : u32 = 1073741824 ;
 pub const SA_ONESHOT : u32 = 2147483648 ;
 pub const SA_STACK : u32 = 134217728 ;
 pub const SIG_BLOCK : u32 = 0 ;
 pub const SIG_UNBLOCK : u32 = 1 ;
 pub const SIG_SETMASK : u32 = 2 ;
 pub const _BITS_SIGCONTEXT_H : u32 = 1 ;
 pub const FP_XSTATE_MAGIC1 : u32 = 1179670611 ;
 pub const FP_XSTATE_MAGIC2 : u32 = 1179670597 ;
 pub const __stack_t_defined : u32 = 1 ;
 pub const _SYS_UCONTEXT_H : u32 = 1 ;
 pub const __NGREG : u32 = 23 ;
 pub const NGREG : u32 = 23 ;
 pub const _BITS_SIGSTACK_H : u32 = 1 ;
 pub const MINSIGSTKSZ : u32 = 2048 ;
 pub const SIGSTKSZ : u32 = 8192 ;
 pub const _BITS_SS_FLAGS_H : u32 = 1 ;
 pub const __sigstack_defined : u32 = 1 ;
 pub const _BITS_SIGTHREAD_H : u32 = 1 ;
 pub const HZ : u32 = 100 ;
 pub const EXEC_PAGESIZE : u32 = 4096 ;
 pub const NOGROUP : i32 = - 1 ;
 pub const MAXHOSTNAMELEN : u32 = 64 ;
 pub const MAXSYMLINKS : u32 = 20 ;
 pub const NOFILE : u32 = 256 ;
 pub const NCARGS : u32 = 131072 ;
 pub const NGROUPS : u32 = 65536 ;
 pub const CANBSIZ : u32 = 255 ;
 pub const MAXPATHLEN : u32 = 4096 ;
 pub const DEV_BSIZE : u32 = 512 ;
 pub const _SYS_UIO_H : u32 = 1 ;
 pub const UIO_MAXIOV : u32 = 1024 ;
 pub const _BITS_UIO_EXT_H : u32 = 1 ;
 pub const RWF_HIPRI : u32 = 1 ;
 pub const RWF_DSYNC : u32 = 2 ;
 pub const RWF_SYNC : u32 = 4 ;
 pub const RWF_NOWAIT : u32 = 8 ;
 pub const _UTMPX_H : u32 = 1 ;
 pub const _SYS_TIME_H : u32 = 1 ;
 pub const _PATH_DEFPATH : & 'static [u8 ; 14usize] = b"/usr/bin:/bin\0" ;
 pub const _PATH_STDPATH : & 'static [u8 ; 30usize] = b"/usr/bin:/bin:/usr/sbin:/sbin\0" ;
 pub const _PATH_BSHELL : & 'static [u8 ; 8usize] = b"/bin/sh\0" ;
 pub const _PATH_CONSOLE : & 'static [u8 ; 13usize] = b"/dev/console\0" ;
 pub const _PATH_CSHELL : & 'static [u8 ; 9usize] = b"/bin/csh\0" ;
 pub const _PATH_DEVDB : & 'static [u8 ; 16usize] = b"/var/run/dev.db\0" ;
 pub const _PATH_DEVNULL : & 'static [u8 ; 10usize] = b"/dev/null\0" ;
 pub const _PATH_DRUM : & 'static [u8 ; 10usize] = b"/dev/drum\0" ;
 pub const _PATH_GSHADOW : & 'static [u8 ; 13usize] = b"/etc/gshadow\0" ;
 pub const _PATH_KLOG : & 'static [u8 ; 11usize] = b"/proc/kmsg\0" ;
 pub const _PATH_KMEM : & 'static [u8 ; 10usize] = b"/dev/kmem\0" ;
 pub const _PATH_LASTLOG : & 'static [u8 ; 17usize] = b"/var/log/lastlog\0" ;
 pub const _PATH_MAILDIR : & 'static [u8 ; 10usize] = b"/var/mail\0" ;
 pub const _PATH_MAN : & 'static [u8 ; 15usize] = b"/usr/share/man\0" ;
 pub const _PATH_MEM : & 'static [u8 ; 9usize] = b"/dev/mem\0" ;
 pub const _PATH_MNTTAB : & 'static [u8 ; 11usize] = b"/etc/fstab\0" ;
 pub const _PATH_MOUNTED : & 'static [u8 ; 10usize] = b"/etc/mtab\0" ;
 pub const _PATH_NOLOGIN : & 'static [u8 ; 13usize] = b"/etc/nologin\0" ;
 pub const _PATH_PRESERVE : & 'static [u8 ; 9usize] = b"/var/lib\0" ;
 pub const _PATH_RWHODIR : & 'static [u8 ; 16usize] = b"/var/spool/rwho\0" ;
 pub const _PATH_SENDMAIL : & 'static [u8 ; 19usize] = b"/usr/sbin/sendmail\0" ;
 pub const _PATH_SHADOW : & 'static [u8 ; 12usize] = b"/etc/shadow\0" ;
 pub const _PATH_SHELLS : & 'static [u8 ; 12usize] = b"/etc/shells\0" ;
 pub const _PATH_TTY : & 'static [u8 ; 9usize] = b"/dev/tty\0" ;
 pub const _PATH_UNIX : & 'static [u8 ; 14usize] = b"/boot/vmlinux\0" ;
 pub const _PATH_UTMP : & 'static [u8 ; 14usize] = b"/var/run/utmp\0" ;
 pub const _PATH_VI : & 'static [u8 ; 12usize] = b"/usr/bin/vi\0" ;
 pub const _PATH_WTMP : & 'static [u8 ; 14usize] = b"/var/log/wtmp\0" ;
 pub const _PATH_DEV : & 'static [u8 ; 6usize] = b"/dev/\0" ;
 pub const _PATH_TMP : & 'static [u8 ; 6usize] = b"/tmp/\0" ;
 pub const _PATH_VARDB : & 'static [u8 ; 15usize] = b"/var/lib/misc/\0" ;
 pub const _PATH_VARRUN : & 'static [u8 ; 10usize] = b"/var/run/\0" ;
 pub const _PATH_VARTMP : & 'static [u8 ; 10usize] = b"/var/tmp/\0" ;
 pub const _PATH_UTMPX : & 'static [u8 ; 14usize] = b"/var/run/utmp\0" ;
 pub const _PATH_WTMPX : & 'static [u8 ; 14usize] = b"/var/log/wtmp\0" ;
 pub const __UT_LINESIZE : u32 = 32 ;
 pub const __UT_NAMESIZE : u32 = 32 ;
 pub const __UT_HOSTSIZE : u32 = 256 ;
 pub const EMPTY : u32 = 0 ;
 pub const RUN_LVL : u32 = 1 ;
 pub const BOOT_TIME : u32 = 2 ;
 pub const NEW_TIME : u32 = 3 ;
 pub const OLD_TIME : u32 = 4 ;
 pub const INIT_PROCESS : u32 = 5 ;
 pub const LOGIN_PROCESS : u32 = 6 ;
 pub const USER_PROCESS : u32 = 7 ;
 pub const DEAD_PROCESS : u32 = 8 ;
 pub const ACCOUNTING : u32 = 9 ;
 pub const UTMPX_FILE : & 'static [u8 ; 14usize] = b"/var/run/utmp\0" ;
 pub const UTMPX_FILENAME : & 'static [u8 ; 14usize] = b"/var/run/utmp\0" ;
 pub const WTMPX_FILE : & 'static [u8 ; 14usize] = b"/var/log/wtmp\0" ;
 pub const WTMPX_FILENAME : & 'static [u8 ; 14usize] = b"/var/log/wtmp\0" ;
 pub const _SYSCALL_H : u32 = 1 ;
 pub const __GLIBC_LINUX_VERSION_CODE : u32 = 265728 ;
 pub const SYS__sysctl : u32 = 156 ;
 pub const SYS_accept : u32 = 43 ;
 pub const SYS_accept4 : u32 = 288 ;
 pub const SYS_access : u32 = 21 ;
 pub const SYS_acct : u32 = 163 ;
 pub const SYS_add_key : u32 = 248 ;
 pub const SYS_adjtimex : u32 = 159 ;
 pub const SYS_afs_syscall : u32 = 183 ;
 pub const SYS_alarm : u32 = 37 ;
 pub const SYS_arch_prctl : u32 = 158 ;
 pub const SYS_bind : u32 = 49 ;
 pub const SYS_bpf : u32 = 321 ;
 pub const SYS_brk : u32 = 12 ;
 pub const SYS_capget : u32 = 125 ;
 pub const SYS_capset : u32 = 126 ;
 pub const SYS_chdir : u32 = 80 ;
 pub const SYS_chmod : u32 = 90 ;
 pub const SYS_chown : u32 = 92 ;
 pub const SYS_chroot : u32 = 161 ;
 pub const SYS_clock_adjtime : u32 = 305 ;
 pub const SYS_clock_getres : u32 = 229 ;
 pub const SYS_clock_gettime : u32 = 228 ;
 pub const SYS_clock_nanosleep : u32 = 230 ;
 pub const SYS_clock_settime : u32 = 227 ;
 pub const SYS_clone : u32 = 56 ;
 pub const SYS_close : u32 = 3 ;
 pub const SYS_connect : u32 = 42 ;
 pub const SYS_copy_file_range : u32 = 326 ;
 pub const SYS_creat : u32 = 85 ;
 pub const SYS_create_module : u32 = 174 ;
 pub const SYS_delete_module : u32 = 176 ;
 pub const SYS_dup : u32 = 32 ;
 pub const SYS_dup2 : u32 = 33 ;
 pub const SYS_dup3 : u32 = 292 ;
 pub const SYS_epoll_create : u32 = 213 ;
 pub const SYS_epoll_create1 : u32 = 291 ;
 pub const SYS_epoll_ctl : u32 = 233 ;
 pub const SYS_epoll_ctl_old : u32 = 214 ;
 pub const SYS_epoll_pwait : u32 = 281 ;
 pub const SYS_epoll_wait : u32 = 232 ;
 pub const SYS_epoll_wait_old : u32 = 215 ;
 pub const SYS_eventfd : u32 = 284 ;
 pub const SYS_eventfd2 : u32 = 290 ;
 pub const SYS_execve : u32 = 59 ;
 pub const SYS_execveat : u32 = 322 ;
 pub const SYS_exit : u32 = 60 ;
 pub const SYS_exit_group : u32 = 231 ;
 pub const SYS_faccessat : u32 = 269 ;
 pub const SYS_fadvise64 : u32 = 221 ;
 pub const SYS_fallocate : u32 = 285 ;
 pub const SYS_fanotify_init : u32 = 300 ;
 pub const SYS_fanotify_mark : u32 = 301 ;
 pub const SYS_fchdir : u32 = 81 ;
 pub const SYS_fchmod : u32 = 91 ;
 pub const SYS_fchmodat : u32 = 268 ;
 pub const SYS_fchown : u32 = 93 ;
 pub const SYS_fchownat : u32 = 260 ;
 pub const SYS_fcntl : u32 = 72 ;
 pub const SYS_fdatasync : u32 = 75 ;
 pub const SYS_fgetxattr : u32 = 193 ;
 pub const SYS_finit_module : u32 = 313 ;
 pub const SYS_flistxattr : u32 = 196 ;
 pub const SYS_flock : u32 = 73 ;
 pub const SYS_fork : u32 = 57 ;
 pub const SYS_fremovexattr : u32 = 199 ;
 pub const SYS_fsetxattr : u32 = 190 ;
 pub const SYS_fstat : u32 = 5 ;
 pub const SYS_fstatfs : u32 = 138 ;
 pub const SYS_fsync : u32 = 74 ;
 pub const SYS_ftruncate : u32 = 77 ;
 pub const SYS_futex : u32 = 202 ;
 pub const SYS_futimesat : u32 = 261 ;
 pub const SYS_get_kernel_syms : u32 = 177 ;
 pub const SYS_get_mempolicy : u32 = 239 ;
 pub const SYS_get_robust_list : u32 = 274 ;
 pub const SYS_get_thread_area : u32 = 211 ;
 pub const SYS_getcpu : u32 = 309 ;
 pub const SYS_getcwd : u32 = 79 ;
 pub const SYS_getdents : u32 = 78 ;
 pub const SYS_getdents64 : u32 = 217 ;
 pub const SYS_getegid : u32 = 108 ;
 pub const SYS_geteuid : u32 = 107 ;
 pub const SYS_getgid : u32 = 104 ;
 pub const SYS_getgroups : u32 = 115 ;
 pub const SYS_getitimer : u32 = 36 ;
 pub const SYS_getpeername : u32 = 52 ;
 pub const SYS_getpgid : u32 = 121 ;
 pub const SYS_getpgrp : u32 = 111 ;
 pub const SYS_getpid : u32 = 39 ;
 pub const SYS_getpmsg : u32 = 181 ;
 pub const SYS_getppid : u32 = 110 ;
 pub const SYS_getpriority : u32 = 140 ;
 pub const SYS_getrandom : u32 = 318 ;
 pub const SYS_getresgid : u32 = 120 ;
 pub const SYS_getresuid : u32 = 118 ;
 pub const SYS_getrlimit : u32 = 97 ;
 pub const SYS_getrusage : u32 = 98 ;
 pub const SYS_getsid : u32 = 124 ;
 pub const SYS_getsockname : u32 = 51 ;
 pub const SYS_getsockopt : u32 = 55 ;
 pub const SYS_gettid : u32 = 186 ;
 pub const SYS_gettimeofday : u32 = 96 ;
 pub const SYS_getuid : u32 = 102 ;
 pub const SYS_getxattr : u32 = 191 ;
 pub const SYS_init_module : u32 = 175 ;
 pub const SYS_inotify_add_watch : u32 = 254 ;
 pub const SYS_inotify_init : u32 = 253 ;
 pub const SYS_inotify_init1 : u32 = 294 ;
 pub const SYS_inotify_rm_watch : u32 = 255 ;
 pub const SYS_io_cancel : u32 = 210 ;
 pub const SYS_io_destroy : u32 = 207 ;
 pub const SYS_io_getevents : u32 = 208 ;
 pub const SYS_io_setup : u32 = 206 ;
 pub const SYS_io_submit : u32 = 209 ;
 pub const SYS_ioctl : u32 = 16 ;
 pub const SYS_ioperm : u32 = 173 ;
 pub const SYS_iopl : u32 = 172 ;
 pub const SYS_ioprio_get : u32 = 252 ;
 pub const SYS_ioprio_set : u32 = 251 ;
 pub const SYS_kcmp : u32 = 312 ;
 pub const SYS_kexec_file_load : u32 = 320 ;
 pub const SYS_kexec_load : u32 = 246 ;
 pub const SYS_keyctl : u32 = 250 ;
 pub const SYS_kill : u32 = 62 ;
 pub const SYS_lchown : u32 = 94 ;
 pub const SYS_lgetxattr : u32 = 192 ;
 pub const SYS_link : u32 = 86 ;
 pub const SYS_linkat : u32 = 265 ;
 pub const SYS_listen : u32 = 50 ;
 pub const SYS_listxattr : u32 = 194 ;
 pub const SYS_llistxattr : u32 = 195 ;
 pub const SYS_lookup_dcookie : u32 = 212 ;
 pub const SYS_lremovexattr : u32 = 198 ;
 pub const SYS_lseek : u32 = 8 ;
 pub const SYS_lsetxattr : u32 = 189 ;
 pub const SYS_lstat : u32 = 6 ;
 pub const SYS_madvise : u32 = 28 ;
 pub const SYS_mbind : u32 = 237 ;
 pub const SYS_membarrier : u32 = 324 ;
 pub const SYS_memfd_create : u32 = 319 ;
 pub const SYS_migrate_pages : u32 = 256 ;
 pub const SYS_mincore : u32 = 27 ;
 pub const SYS_mkdir : u32 = 83 ;
 pub const SYS_mkdirat : u32 = 258 ;
 pub const SYS_mknod : u32 = 133 ;
 pub const SYS_mknodat : u32 = 259 ;
 pub const SYS_mlock : u32 = 149 ;
 pub const SYS_mlock2 : u32 = 325 ;
 pub const SYS_mlockall : u32 = 151 ;
 pub const SYS_mmap : u32 = 9 ;
 pub const SYS_modify_ldt : u32 = 154 ;
 pub const SYS_mount : u32 = 165 ;
 pub const SYS_move_pages : u32 = 279 ;
 pub const SYS_mprotect : u32 = 10 ;
 pub const SYS_mq_getsetattr : u32 = 245 ;
 pub const SYS_mq_notify : u32 = 244 ;
 pub const SYS_mq_open : u32 = 240 ;
 pub const SYS_mq_timedreceive : u32 = 243 ;
 pub const SYS_mq_timedsend : u32 = 242 ;
 pub const SYS_mq_unlink : u32 = 241 ;
 pub const SYS_mremap : u32 = 25 ;
 pub const SYS_msgctl : u32 = 71 ;
 pub const SYS_msgget : u32 = 68 ;
 pub const SYS_msgrcv : u32 = 70 ;
 pub const SYS_msgsnd : u32 = 69 ;
 pub const SYS_msync : u32 = 26 ;
 pub const SYS_munlock : u32 = 150 ;
 pub const SYS_munlockall : u32 = 152 ;
 pub const SYS_munmap : u32 = 11 ;
 pub const SYS_name_to_handle_at : u32 = 303 ;
 pub const SYS_nanosleep : u32 = 35 ;
 pub const SYS_newfstatat : u32 = 262 ;
 pub const SYS_nfsservctl : u32 = 180 ;
 pub const SYS_open : u32 = 2 ;
 pub const SYS_open_by_handle_at : u32 = 304 ;
 pub const SYS_openat : u32 = 257 ;
 pub const SYS_pause : u32 = 34 ;
 pub const SYS_perf_event_open : u32 = 298 ;
 pub const SYS_personality : u32 = 135 ;
 pub const SYS_pipe : u32 = 22 ;
 pub const SYS_pipe2 : u32 = 293 ;
 pub const SYS_pivot_root : u32 = 155 ;
 pub const SYS_pkey_alloc : u32 = 330 ;
 pub const SYS_pkey_free : u32 = 331 ;
 pub const SYS_pkey_mprotect : u32 = 329 ;
 pub const SYS_poll : u32 = 7 ;
 pub const SYS_ppoll : u32 = 271 ;
 pub const SYS_prctl : u32 = 157 ;
 pub const SYS_pread64 : u32 = 17 ;
 pub const SYS_preadv : u32 = 295 ;
 pub const SYS_preadv2 : u32 = 327 ;
 pub const SYS_prlimit64 : u32 = 302 ;
 pub const SYS_process_vm_readv : u32 = 310 ;
 pub const SYS_process_vm_writev : u32 = 311 ;
 pub const SYS_pselect6 : u32 = 270 ;
 pub const SYS_ptrace : u32 = 101 ;
 pub const SYS_putpmsg : u32 = 182 ;
 pub const SYS_pwrite64 : u32 = 18 ;
 pub const SYS_pwritev : u32 = 296 ;
 pub const SYS_pwritev2 : u32 = 328 ;
 pub const SYS_query_module : u32 = 178 ;
 pub const SYS_quotactl : u32 = 179 ;
 pub const SYS_read : u32 = 0 ;
 pub const SYS_readahead : u32 = 187 ;
 pub const SYS_readlink : u32 = 89 ;
 pub const SYS_readlinkat : u32 = 267 ;
 pub const SYS_readv : u32 = 19 ;
 pub const SYS_reboot : u32 = 169 ;
 pub const SYS_recvfrom : u32 = 45 ;
 pub const SYS_recvmmsg : u32 = 299 ;
 pub const SYS_recvmsg : u32 = 47 ;
 pub const SYS_remap_file_pages : u32 = 216 ;
 pub const SYS_removexattr : u32 = 197 ;
 pub const SYS_rename : u32 = 82 ;
 pub const SYS_renameat : u32 = 264 ;
 pub const SYS_renameat2 : u32 = 316 ;
 pub const SYS_request_key : u32 = 249 ;
 pub const SYS_restart_syscall : u32 = 219 ;
 pub const SYS_rmdir : u32 = 84 ;
 pub const SYS_rt_sigaction : u32 = 13 ;
 pub const SYS_rt_sigpending : u32 = 127 ;
 pub const SYS_rt_sigprocmask : u32 = 14 ;
 pub const SYS_rt_sigqueueinfo : u32 = 129 ;
 pub const SYS_rt_sigreturn : u32 = 15 ;
 pub const SYS_rt_sigsuspend : u32 = 130 ;
 pub const SYS_rt_sigtimedwait : u32 = 128 ;
 pub const SYS_rt_tgsigqueueinfo : u32 = 297 ;
 pub const SYS_sched_get_priority_max : u32 = 146 ;
 pub const SYS_sched_get_priority_min : u32 = 147 ;
 pub const SYS_sched_getaffinity : u32 = 204 ;
 pub const SYS_sched_getattr : u32 = 315 ;
 pub const SYS_sched_getparam : u32 = 143 ;
 pub const SYS_sched_getscheduler : u32 = 145 ;
 pub const SYS_sched_rr_get_interval : u32 = 148 ;
 pub const SYS_sched_setaffinity : u32 = 203 ;
 pub const SYS_sched_setattr : u32 = 314 ;
 pub const SYS_sched_setparam : u32 = 142 ;
 pub const SYS_sched_setscheduler : u32 = 144 ;
 pub const SYS_sched_yield : u32 = 24 ;
 pub const SYS_seccomp : u32 = 317 ;
 pub const SYS_security : u32 = 185 ;
 pub const SYS_select : u32 = 23 ;
 pub const SYS_semctl : u32 = 66 ;
 pub const SYS_semget : u32 = 64 ;
 pub const SYS_semop : u32 = 65 ;
 pub const SYS_semtimedop : u32 = 220 ;
 pub const SYS_sendfile : u32 = 40 ;
 pub const SYS_sendmmsg : u32 = 307 ;
 pub const SYS_sendmsg : u32 = 46 ;
 pub const SYS_sendto : u32 = 44 ;
 pub const SYS_set_mempolicy : u32 = 238 ;
 pub const SYS_set_robust_list : u32 = 273 ;
 pub const SYS_set_thread_area : u32 = 205 ;
 pub const SYS_set_tid_address : u32 = 218 ;
 pub const SYS_setdomainname : u32 = 171 ;
 pub const SYS_setfsgid : u32 = 123 ;
 pub const SYS_setfsuid : u32 = 122 ;
 pub const SYS_setgid : u32 = 106 ;
 pub const SYS_setgroups : u32 = 116 ;
 pub const SYS_sethostname : u32 = 170 ;
 pub const SYS_setitimer : u32 = 38 ;
 pub const SYS_setns : u32 = 308 ;
 pub const SYS_setpgid : u32 = 109 ;
 pub const SYS_setpriority : u32 = 141 ;
 pub const SYS_setregid : u32 = 114 ;
 pub const SYS_setresgid : u32 = 119 ;
 pub const SYS_setresuid : u32 = 117 ;
 pub const SYS_setreuid : u32 = 113 ;
 pub const SYS_setrlimit : u32 = 160 ;
 pub const SYS_setsid : u32 = 112 ;
 pub const SYS_setsockopt : u32 = 54 ;
 pub const SYS_settimeofday : u32 = 164 ;
 pub const SYS_setuid : u32 = 105 ;
 pub const SYS_setxattr : u32 = 188 ;
 pub const SYS_shmat : u32 = 30 ;
 pub const SYS_shmctl : u32 = 31 ;
 pub const SYS_shmdt : u32 = 67 ;
 pub const SYS_shmget : u32 = 29 ;
 pub const SYS_shutdown : u32 = 48 ;
 pub const SYS_sigaltstack : u32 = 131 ;
 pub const SYS_signalfd : u32 = 282 ;
 pub const SYS_signalfd4 : u32 = 289 ;
 pub const SYS_socket : u32 = 41 ;
 pub const SYS_socketpair : u32 = 53 ;
 pub const SYS_splice : u32 = 275 ;
 pub const SYS_stat : u32 = 4 ;
 pub const SYS_statfs : u32 = 137 ;
 pub const SYS_statx : u32 = 332 ;
 pub const SYS_swapoff : u32 = 168 ;
 pub const SYS_swapon : u32 = 167 ;
 pub const SYS_symlink : u32 = 88 ;
 pub const SYS_symlinkat : u32 = 266 ;
 pub const SYS_sync : u32 = 162 ;
 pub const SYS_sync_file_range : u32 = 277 ;
 pub const SYS_syncfs : u32 = 306 ;
 pub const SYS_sysfs : u32 = 139 ;
 pub const SYS_sysinfo : u32 = 99 ;
 pub const SYS_syslog : u32 = 103 ;
 pub const SYS_tee : u32 = 276 ;
 pub const SYS_tgkill : u32 = 234 ;
 pub const SYS_time : u32 = 201 ;
 pub const SYS_timer_create : u32 = 222 ;
 pub const SYS_timer_delete : u32 = 226 ;
 pub const SYS_timer_getoverrun : u32 = 225 ;
 pub const SYS_timer_gettime : u32 = 224 ;
 pub const SYS_timer_settime : u32 = 223 ;
 pub const SYS_timerfd_create : u32 = 283 ;
 pub const SYS_timerfd_gettime : u32 = 287 ;
 pub const SYS_timerfd_settime : u32 = 286 ;
 pub const SYS_times : u32 = 100 ;
 pub const SYS_tkill : u32 = 200 ;
 pub const SYS_truncate : u32 = 76 ;
 pub const SYS_tuxcall : u32 = 184 ;
 pub const SYS_umask : u32 = 95 ;
 pub const SYS_umount2 : u32 = 166 ;
 pub const SYS_uname : u32 = 63 ;
 pub const SYS_unlink : u32 = 87 ;
 pub const SYS_unlinkat : u32 = 263 ;
 pub const SYS_unshare : u32 = 272 ;
 pub const SYS_uselib : u32 = 134 ;
 pub const SYS_userfaultfd : u32 = 323 ;
 pub const SYS_ustat : u32 = 136 ;
 pub const SYS_utime : u32 = 132 ;
 pub const SYS_utimensat : u32 = 280 ;
 pub const SYS_utimes : u32 = 235 ;
 pub const SYS_vfork : u32 = 58 ;
 pub const SYS_vhangup : u32 = 153 ;
 pub const SYS_vmsplice : u32 = 278 ;
 pub const SYS_vserver : u32 = 236 ;
 pub const SYS_wait4 : u32 = 61 ;
 pub const SYS_waitid : u32 = 247 ;
 pub const SYS_write : u32 = 1 ;
 pub const SYS_writev : u32 = 20 ;
 pub const _SCHED_H : u32 = 1 ;
 pub const _BITS_SCHED_H : u32 = 1 ;
 pub const SCHED_OTHER : u32 = 0 ;
 pub const SCHED_FIFO : u32 = 1 ;
 pub const SCHED_RR : u32 = 2 ;
 pub const SCHED_BATCH : u32 = 3 ;
 pub const SCHED_ISO : u32 = 4 ;
 pub const SCHED_IDLE : u32 = 5 ;
 pub const SCHED_DEADLINE : u32 = 6 ;
 pub const SCHED_RESET_ON_FORK : u32 = 1073741824 ;
 pub const CSIGNAL : u32 = 255 ;
 pub const CLONE_VM : u32 = 256 ;
 pub const CLONE_FS : u32 = 512 ;
 pub const CLONE_FILES : u32 = 1024 ;
 pub const CLONE_SIGHAND : u32 = 2048 ;
 pub const CLONE_PTRACE : u32 = 8192 ;
 pub const CLONE_VFORK : u32 = 16384 ;
 pub const CLONE_PARENT : u32 = 32768 ;
 pub const CLONE_THREAD : u32 = 65536 ;
 pub const CLONE_NEWNS : u32 = 131072 ;
 pub const CLONE_SYSVSEM : u32 = 262144 ;
 pub const CLONE_SETTLS : u32 = 524288 ;
 pub const CLONE_PARENT_SETTID : u32 = 1048576 ;
 pub const CLONE_CHILD_CLEARTID : u32 = 2097152 ;
 pub const CLONE_DETACHED : u32 = 4194304 ;
 pub const CLONE_UNTRACED : u32 = 8388608 ;
 pub const CLONE_CHILD_SETTID : u32 = 16777216 ;
 pub const CLONE_NEWCGROUP : u32 = 33554432 ;
 pub const CLONE_NEWUTS : u32 = 67108864 ;
 pub const CLONE_NEWIPC : u32 = 134217728 ;
 pub const CLONE_NEWUSER : u32 = 268435456 ;
 pub const CLONE_NEWPID : u32 = 536870912 ;
 pub const CLONE_NEWNET : u32 = 1073741824 ;
 pub const CLONE_IO : u32 = 2147483648 ;
 pub const _BITS_CPU_SET_H : u32 = 1 ;
 pub const __CPU_SETSIZE : u32 = 1024 ;
 pub const CPU_SETSIZE : u32 = 1024 ;
 pub const LINUX_VERSION_CODE : u32 = 266002 ;
 pub const _UNISTD_H : u32 = 1 ;
 pub const _POSIX_VERSION : u32 = 200809 ;
 pub const __POSIX2_THIS_VERSION : u32 = 200809 ;
 pub const _POSIX2_VERSION : u32 = 200809 ;
 pub const _POSIX2_C_VERSION : u32 = 200809 ;
 pub const _POSIX2_C_BIND : u32 = 200809 ;
 pub const _POSIX2_C_DEV : u32 = 200809 ;
 pub const _POSIX2_SW_DEV : u32 = 200809 ;
 pub const _POSIX2_LOCALEDEF : u32 = 200809 ;
 pub const _XOPEN_VERSION : u32 = 700 ;
 pub const _XOPEN_XCU_VERSION : u32 = 4 ;
 pub const _XOPEN_XPG2 : u32 = 1 ;
 pub const _XOPEN_XPG3 : u32 = 1 ;
 pub const _XOPEN_XPG4 : u32 = 1 ;
 pub const _XOPEN_UNIX : u32 = 1 ;
 pub const _XOPEN_CRYPT : u32 = 1 ;
 pub const _XOPEN_ENH_I18N : u32 = 1 ;
 pub const _XOPEN_LEGACY : u32 = 1 ;
 pub const _BITS_POSIX_OPT_H : u32 = 1 ;
 pub const _POSIX_JOB_CONTROL : u32 = 1 ;
 pub const _POSIX_SAVED_IDS : u32 = 1 ;
 pub const _POSIX_PRIORITY_SCHEDULING : u32 = 200809 ;
 pub const _POSIX_SYNCHRONIZED_IO : u32 = 200809 ;
 pub const _POSIX_FSYNC : u32 = 200809 ;
 pub const _POSIX_MAPPED_FILES : u32 = 200809 ;
 pub const _POSIX_MEMLOCK : u32 = 200809 ;
 pub const _POSIX_MEMLOCK_RANGE : u32 = 200809 ;
 pub const _POSIX_MEMORY_PROTECTION : u32 = 200809 ;
 pub const _POSIX_CHOWN_RESTRICTED : u32 = 0 ;
 pub const _POSIX_VDISABLE : u8 = 0u8 ;
 pub const _POSIX_NO_TRUNC : u32 = 1 ;
 pub const _XOPEN_REALTIME : u32 = 1 ;
 pub const _XOPEN_REALTIME_THREADS : u32 = 1 ;
 pub const _XOPEN_SHM : u32 = 1 ;
 pub const _POSIX_THREADS : u32 = 200809 ;
 pub const _POSIX_REENTRANT_FUNCTIONS : u32 = 1 ;
 pub const _POSIX_THREAD_SAFE_FUNCTIONS : u32 = 200809 ;
 pub const _POSIX_THREAD_PRIORITY_SCHEDULING : u32 = 200809 ;
 pub const _POSIX_THREAD_ATTR_STACKSIZE : u32 = 200809 ;
 pub const _POSIX_THREAD_ATTR_STACKADDR : u32 = 200809 ;
 pub const _POSIX_THREAD_PRIO_INHERIT : u32 = 200809 ;
 pub const _POSIX_THREAD_PRIO_PROTECT : u32 = 200809 ;
 pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT : u32 = 200809 ;
 pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT : i32 = - 1 ;
 pub const _POSIX_SEMAPHORES : u32 = 200809 ;
 pub const _POSIX_REALTIME_SIGNALS : u32 = 200809 ;
 pub const _POSIX_ASYNCHRONOUS_IO : u32 = 200809 ;
 pub const _POSIX_ASYNC_IO : u32 = 1 ;
 pub const _LFS_ASYNCHRONOUS_IO : u32 = 1 ;
 pub const _POSIX_PRIORITIZED_IO : u32 = 200809 ;
 pub const _LFS64_ASYNCHRONOUS_IO : u32 = 1 ;
 pub const _LFS_LARGEFILE : u32 = 1 ;
 pub const _LFS64_LARGEFILE : u32 = 1 ;
 pub const _LFS64_STDIO : u32 = 1 ;
 pub const _POSIX_SHARED_MEMORY_OBJECTS : u32 = 200809 ;
 pub const _POSIX_CPUTIME : u32 = 0 ;
 pub const _POSIX_THREAD_CPUTIME : u32 = 0 ;
 pub const _POSIX_REGEXP : u32 = 1 ;
 pub const _POSIX_READER_WRITER_LOCKS : u32 = 200809 ;
 pub const _POSIX_SHELL : u32 = 1 ;
 pub const _POSIX_TIMEOUTS : u32 = 200809 ;
 pub const _POSIX_SPIN_LOCKS : u32 = 200809 ;
 pub const _POSIX_SPAWN : u32 = 200809 ;
 pub const _POSIX_TIMERS : u32 = 200809 ;
 pub const _POSIX_BARRIERS : u32 = 200809 ;
 pub const _POSIX_MESSAGE_PASSING : u32 = 200809 ;
 pub const _POSIX_THREAD_PROCESS_SHARED : u32 = 200809 ;
 pub const _POSIX_MONOTONIC_CLOCK : u32 = 0 ;
 pub const _POSIX_CLOCK_SELECTION : u32 = 200809 ;
 pub const _POSIX_ADVISORY_INFO : u32 = 200809 ;
 pub const _POSIX_IPV6 : u32 = 200809 ;
 pub const _POSIX_RAW_SOCKETS : u32 = 200809 ;
 pub const _POSIX2_CHAR_TERM : u32 = 200809 ;
 pub const _POSIX_SPORADIC_SERVER : i32 = - 1 ;
 pub const _POSIX_THREAD_SPORADIC_SERVER : i32 = - 1 ;
 pub const _POSIX_TRACE : i32 = - 1 ;
 pub const _POSIX_TRACE_EVENT_FILTER : i32 = - 1 ;
 pub const _POSIX_TRACE_INHERIT : i32 = - 1 ;
 pub const _POSIX_TRACE_LOG : i32 = - 1 ;
 pub const _POSIX_TYPED_MEMORY_OBJECTS : i32 = - 1 ;
 pub const _POSIX_V7_LPBIG_OFFBIG : i32 = - 1 ;
 pub const _POSIX_V6_LPBIG_OFFBIG : i32 = - 1 ;
 pub const _XBS5_LPBIG_OFFBIG : i32 = - 1 ;
 pub const _POSIX_V7_LP64_OFF64 : u32 = 1 ;
 pub const _POSIX_V6_LP64_OFF64 : u32 = 1 ;
 pub const _XBS5_LP64_OFF64 : u32 = 1 ;
 pub const __ILP32_OFF32_CFLAGS : & 'static [u8 ; 5usize] = b"-m32\0" ;
 pub const __ILP32_OFF32_LDFLAGS : & 'static [u8 ; 5usize] = b"-m32\0" ;
 pub const __ILP32_OFFBIG_CFLAGS : & 'static [u8 ; 48usize] = b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0" ;
 pub const __ILP32_OFFBIG_LDFLAGS : & 'static [u8 ; 5usize] = b"-m32\0" ;
 pub const __LP64_OFF64_CFLAGS : & 'static [u8 ; 5usize] = b"-m64\0" ;
 pub const __LP64_OFF64_LDFLAGS : & 'static [u8 ; 5usize] = b"-m64\0" ;
 pub const STDIN_FILENO : u32 = 0 ;
 pub const STDOUT_FILENO : u32 = 1 ;
 pub const STDERR_FILENO : u32 = 2 ;
 pub const SEEK_DATA : u32 = 3 ;
 pub const SEEK_HOLE : u32 = 4 ;
 pub const L_SET : u32 = 0 ;
 pub const L_INCR : u32 = 1 ;
 pub const L_XTND : u32 = 2 ;
 pub const _GETOPT_POSIX_H : u32 = 1 ;
 pub const _GETOPT_CORE_H : u32 = 1 ;
 pub const _SYS_SENDFILE_H : u32 = 1 ;
 pub const _LARGE_FILES : u32 = 1 ;
 pub const _STRING_H : u32 = 1 ;
 pub const _BITS_TYPES_LOCALE_T_H : u32 = 1 ;
 pub const _BITS_TYPES___LOCALE_T_H : u32 = 1 ;
 pub const _STRINGS_H : u32 = 1 ;
 pub const _GLIBCXX_STDLIB_H : u32 = 1 ;
 pub const _GLIBCXX_CXX_CONFIG_H : u32 = 1 ;
 pub const _GLIBCXX_RELEASE : u32 = 7 ;
 pub const __GLIBCXX__ : u32 = 20181206 ;
 pub const _GLIBCXX_HAVE_ATTRIBUTE_VISIBILITY : u32 = 1 ;
 pub const _GLIBCXX_USE_DEPRECATED : u32 = 1 ;
 pub const _GLIBCXX_EXTERN_TEMPLATE : u32 = 1 ;
 pub const _GLIBCXX_USE_DUAL_ABI : u32 = 1 ;
 pub const _GLIBCXX_USE_CXX11_ABI : u32 = 1 ;
 pub const _GLIBCXX_INLINE_VERSION : u32 = 0 ;
 pub const _GLIBCXX_USE_ALLOCATOR_NEW : u32 = 1 ;
 pub const _GLIBCXX_OS_DEFINES : u32 = 1 ;
 pub const __NO_CTYPE : u32 = 1 ;
 pub const _GLIBCXX_CPU_DEFINES : u32 = 1 ;
 pub const _GLIBCXX_FAST_MATH : u32 = 0 ;
 pub const _GLIBCXX_USE_FLOAT128 : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ACOSF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ACOSL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ALIGNED_ALLOC : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ASINF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ASINL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_AS_SYMVER_DIRECTIVE : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ATAN2F : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ATAN2L : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ATANF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ATANL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_AT_QUICK_EXIT : u32 = 1 ;
 pub const _GLIBCXX_HAVE_CEILF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_CEILL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_COMPLEX_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_COSF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_COSHF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_COSHL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_COSL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_DIRENT_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_DLFCN_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EBADMSG : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ECANCELED : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ECHILD : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EIDRM : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENDIAN_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENODATA : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENOLINK : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENOSPC : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENOSR : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENOSTR : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENOTRECOVERABLE : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ENOTSUP : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EOVERFLOW : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EOWNERDEAD : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EPERM : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EPROTO : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ETIME : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ETIMEDOUT : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ETXTBSY : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EWOULDBLOCK : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EXCEPTION_PTR_SINCE_GCC46 : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EXECINFO_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EXPF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_EXPL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FABSF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FABSL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FCNTL_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FENV_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FINITE : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FINITEF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FINITEL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FLOAT_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FLOORF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FLOORL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FMODF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FMODL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FREXPF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_FREXPL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_GETIPINFO : u32 = 1 ;
 pub const _GLIBCXX_HAVE_GETS : u32 = 1 ;
 pub const _GLIBCXX_HAVE_HYPOT : u32 = 1 ;
 pub const _GLIBCXX_HAVE_HYPOTF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_HYPOTL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ICONV : u32 = 1 ;
 pub const _GLIBCXX_HAVE_INT64_T : u32 = 1 ;
 pub const _GLIBCXX_HAVE_INT64_T_LONG : u32 = 1 ;
 pub const _GLIBCXX_HAVE_INTTYPES_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ISINFF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ISINFL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ISNANF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ISNANL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_ISWBLANK : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LC_MESSAGES : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LDEXPF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LDEXPL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LIBINTL_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LIMIT_AS : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LIMIT_DATA : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LIMIT_FSIZE : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LIMIT_RSS : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LIMIT_VMEM : u32 = 0 ;
 pub const _GLIBCXX_HAVE_LINUX_FUTEX : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LOCALE_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LOG10F : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LOG10L : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LOGF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_LOGL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_MBSTATE_T : u32 = 1 ;
 pub const _GLIBCXX_HAVE_MEMALIGN : u32 = 1 ;
 pub const _GLIBCXX_HAVE_MEMORY_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_MODF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_MODFF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_MODFL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_POLL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_POSIX_MEMALIGN : u32 = 1 ;
 pub const _GLIBCXX_HAVE_POWF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_POWL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_QUICK_EXIT : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SETENV : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINCOS : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINCOSF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINCOSL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINHF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINHL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SINL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SQRTF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SQRTL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STDALIGN_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STDBOOL_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STDINT_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STDLIB_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRERROR_L : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRERROR_R : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRINGS_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRING_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRTOF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRTOLD : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRUCT_DIRENT_D_TYPE : u32 = 1 ;
 pub const _GLIBCXX_HAVE_STRXFRM_L : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYMVER_SYMBOL_RENAMING_RUNTIME_SUPPORT : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_IOCTL_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_IPC_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_PARAM_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_RESOURCE_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_SDT_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_SEM_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_STATVFS_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_STAT_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_SYSINFO_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_TIME_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_TYPES_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_SYS_UIO_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_S_ISREG : u32 = 1 ;
 pub const _GLIBCXX_HAVE_TANF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_TANHF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_TANHL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_TANL : u32 = 1 ;
 pub const _GLIBCXX_HAVE_TGMATH_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_TLS : u32 = 1 ;
 pub const _GLIBCXX_HAVE_UCHAR_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_UNISTD_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_UTIME_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_VFWSCANF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_VSWSCANF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_VWSCANF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_WCHAR_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_WCSTOF : u32 = 1 ;
 pub const _GLIBCXX_HAVE_WCTYPE_H : u32 = 1 ;
 pub const _GLIBCXX_HAVE_WRITEV : u32 = 1 ;
 pub const _GLIBCXX_HAVE___CXA_THREAD_ATEXIT_IMPL : u32 = 1 ;
 pub const LT_OBJDIR : & 'static [u8 ; 7usize] = b".libs/\0" ;
 pub const _GLIBCXX_PACKAGE_BUGREPORT : & 'static [u8 ; 1usize] = b"\0" ;
 pub const _GLIBCXX_PACKAGE_NAME : & 'static [u8 ; 15usize] = b"package-unused\0" ;
 pub const _GLIBCXX_PACKAGE_STRING : & 'static [u8 ; 30usize] = b"package-unused version-unused\0" ;
 pub const _GLIBCXX_PACKAGE_TARNAME : & 'static [u8 ; 10usize] = b"libstdc++\0" ;
 pub const _GLIBCXX_PACKAGE_URL : & 'static [u8 ; 1usize] = b"\0" ;
 pub const _GLIBCXX_PACKAGE__GLIBCXX_VERSION : & 'static [u8 ; 15usize] = b"version-unused\0" ;
 pub const STDC_HEADERS : u32 = 1 ;
 pub const _GLIBCXX11_USE_C99_COMPLEX : u32 = 1 ;
 pub const _GLIBCXX11_USE_C99_MATH : u32 = 1 ;
 pub const _GLIBCXX11_USE_C99_STDIO : u32 = 1 ;
 pub const _GLIBCXX11_USE_C99_STDLIB : u32 = 1 ;
 pub const _GLIBCXX11_USE_C99_WCHAR : u32 = 1 ;
 pub const _GLIBCXX98_USE_C99_COMPLEX : u32 = 1 ;
 pub const _GLIBCXX98_USE_C99_MATH : u32 = 1 ;
 pub const _GLIBCXX98_USE_C99_STDIO : u32 = 1 ;
 pub const _GLIBCXX98_USE_C99_STDLIB : u32 = 1 ;
 pub const _GLIBCXX98_USE_C99_WCHAR : u32 = 1 ;
 pub const _GLIBCXX_ATOMIC_BUILTINS : u32 = 1 ;
 pub const _GLIBCXX_FULLY_DYNAMIC_STRING : u32 = 0 ;
 pub const _GLIBCXX_HAS_GTHREADS : u32 = 1 ;
 pub const _GLIBCXX_HOSTED : u32 = 1 ;
 pub const _GLIBCXX_RES_LIMITS : u32 = 1 ;
 pub const _GLIBCXX_STDIO_EOF : i32 = - 1 ;
 pub const _GLIBCXX_STDIO_SEEK_CUR : u32 = 1 ;
 pub const _GLIBCXX_STDIO_SEEK_END : u32 = 2 ;
 pub const _GLIBCXX_SYMVER : u32 = 1 ;
 pub const _GLIBCXX_SYMVER_GNU : u32 = 1 ;
 pub const _GLIBCXX_USE_C11_UCHAR_CXX11 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_COMPLEX_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_CTYPE_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_FENV_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_INTTYPES_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_INTTYPES_WCHAR_T_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_MATH_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_C99_STDINT_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_CLOCK_MONOTONIC : u32 = 1 ;
 pub const _GLIBCXX_USE_CLOCK_REALTIME : u32 = 1 ;
 pub const _GLIBCXX_USE_DECIMAL_FLOAT : u32 = 1 ;
 pub const _GLIBCXX_USE_FCHMOD : u32 = 1 ;
 pub const _GLIBCXX_USE_FCHMODAT : u32 = 1 ;
 pub const _GLIBCXX_USE_GETTIMEOFDAY : u32 = 1 ;
 pub const _GLIBCXX_USE_GET_NPROCS : u32 = 1 ;
 pub const _GLIBCXX_USE_INT128 : u32 = 1 ;
 pub const _GLIBCXX_USE_LFS : u32 = 1 ;
 pub const _GLIBCXX_USE_LONG_LONG : u32 = 1 ;
 pub const _GLIBCXX_USE_NANOSLEEP : u32 = 1 ;
 pub const _GLIBCXX_USE_NLS : u32 = 1 ;
 pub const _GLIBCXX_USE_PTHREAD_RWLOCK_T : u32 = 1 ;
 pub const _GLIBCXX_USE_RANDOM_TR1 : u32 = 1 ;
 pub const _GLIBCXX_USE_REALPATH : u32 = 1 ;
 pub const _GLIBCXX_USE_SCHED_YIELD : u32 = 1 ;
 pub const _GLIBCXX_USE_SC_NPROCESSORS_ONLN : u32 = 1 ;
 pub const _GLIBCXX_USE_SENDFILE : u32 = 1 ;
 pub const _GLIBCXX_USE_ST_MTIM : u32 = 1 ;
 pub const _GLIBCXX_USE_TMPNAM : u32 = 1 ;
 pub const _GLIBCXX_USE_UTIMENSAT : u32 = 1 ;
 pub const _GLIBCXX_USE_WCHAR_T : u32 = 1 ;
 pub const _GLIBCXX_VERBOSE : u32 = 1 ;
 pub const _GLIBCXX_X86_RDRAND : u32 = 1 ;
 pub const _GTHREAD_USE_MUTEX_TIMEDLOCK : u32 = 1 ;
 pub const _GLIBCXX_CSTDLIB : u32 = 1 ;
 pub const _STDLIB_H : u32 = 1 ;
 pub const WNOHANG : u32 = 1 ;
 pub const WUNTRACED : u32 = 2 ;
 pub const WSTOPPED : u32 = 2 ;
 pub const WEXITED : u32 = 4 ;
 pub const WCONTINUED : u32 = 8 ;
 pub const WNOWAIT : u32 = 16777216 ;
 pub const __WNOTHREAD : u32 = 536870912 ;
 pub const __WALL : u32 = 1073741824 ;
 pub const __WCLONE : u32 = 2147483648 ;
 pub const __ENUM_IDTYPE_T : u32 = 1 ;
 pub const __W_CONTINUED : u32 = 65535 ;
 pub const __WCOREFLAG : u32 = 128 ;
 pub const __HAVE_FLOAT128 : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT128 : u32 = 0 ;
 pub const __HAVE_FLOAT64X : u32 = 1 ;
 pub const __HAVE_FLOAT64X_LONG_DOUBLE : u32 = 1 ;
 pub const __HAVE_FLOAT16 : u32 = 0 ;
 pub const __HAVE_FLOAT32 : u32 = 1 ;
 pub const __HAVE_FLOAT64 : u32 = 1 ;
 pub const __HAVE_FLOAT32X : u32 = 1 ;
 pub const __HAVE_FLOAT128X : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT16 : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT32 : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT64 : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT32X : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT64X : u32 = 0 ;
 pub const __HAVE_DISTINCT_FLOAT128X : u32 = 0 ;
 pub const __HAVE_FLOATN_NOT_TYPEDEF : u32 = 0 ;
 pub const __ldiv_t_defined : u32 = 1 ;
 pub const __lldiv_t_defined : u32 = 1 ;
 pub const RAND_MAX : u32 = 2147483647 ;
 pub const EXIT_FAILURE : u32 = 1 ;
 pub const EXIT_SUCCESS : u32 = 0 ;
 pub const _ALLOCA_H : u32 = 1 ;
 pub const _SYS_STAT_H : u32 = 1 ;
 pub const S_IREAD : u32 = 256 ;
 pub const S_IWRITE : u32 = 128 ;
 pub const S_IEXEC : u32 = 64 ;
 pub const ACCESSPERMS : u32 = 511 ;
 pub const ALLPERMS : u32 = 4095 ;
 pub const DEFFILEMODE : u32 = 438 ;
 pub const S_BLKSIZE : u32 = 512 ;
 pub const _MKNOD_VER : u32 = 0 ;
 pub const _STDIO_H : u32 = 1 ;
 pub const ____FILE_defined : u32 = 1 ;
 pub const __FILE_defined : u32 = 1 ;
 pub const _BITS_LIBIO_H : u32 = 1 ;
 pub const _BITS_G_CONFIG_H : u32 = 1 ;
 pub const ____mbstate_t_defined : u32 = 1 ;
 pub const _G_HAVE_MMAP : u32 = 1 ;
 pub const _G_HAVE_MREMAP : u32 = 1 ;
 pub const _G_IO_IO_FILE_VERSION : u32 = 131073 ;
 pub const _G_BUFSIZ : u32 = 8192 ;
 pub const _IO_BUFSIZ : u32 = 8192 ;
 pub const __GNUC_VA_LIST : u32 = 1 ;
 pub const _IO_UNIFIED_JUMPTABLES : u32 = 1 ;
 pub const EOF : i32 = - 1 ;
 pub const _IOS_INPUT : u32 = 1 ;
 pub const _IOS_OUTPUT : u32 = 2 ;
 pub const _IOS_ATEND : u32 = 4 ;
 pub const _IOS_APPEND : u32 = 8 ;
 pub const _IOS_TRUNC : u32 = 16 ;
 pub const _IOS_NOCREATE : u32 = 32 ;
 pub const _IOS_NOREPLACE : u32 = 64 ;
 pub const _IOS_BIN : u32 = 128 ;
 pub const _IO_MAGIC : u32 = 4222418944 ;
 pub const _OLD_STDIO_MAGIC : u32 = 4206624768 ;
 pub const _IO_MAGIC_MASK : u32 = 4294901760 ;
 pub const _IO_USER_BUF : u32 = 1 ;
 pub const _IO_UNBUFFERED : u32 = 2 ;
 pub const _IO_NO_READS : u32 = 4 ;
 pub const _IO_NO_WRITES : u32 = 8 ;
 pub const _IO_EOF_SEEN : u32 = 16 ;
 pub const _IO_ERR_SEEN : u32 = 32 ;
 pub const _IO_DELETE_DONT_CLOSE : u32 = 64 ;
 pub const _IO_LINKED : u32 = 128 ;
 pub const _IO_IN_BACKUP : u32 = 256 ;
 pub const _IO_LINE_BUF : u32 = 512 ;
 pub const _IO_TIED_PUT_GET : u32 = 1024 ;
 pub const _IO_CURRENTLY_PUTTING : u32 = 2048 ;
 pub const _IO_IS_APPENDING : u32 = 4096 ;
 pub const _IO_IS_FILEBUF : u32 = 8192 ;
 pub const _IO_BAD_SEEN : u32 = 16384 ;
 pub const _IO_USER_LOCK : u32 = 32768 ;
 pub const _IO_FLAGS2_MMAP : u32 = 1 ;
 pub const _IO_FLAGS2_NOTCANCEL : u32 = 2 ;
 pub const _IO_FLAGS2_USER_WBUF : u32 = 8 ;
 pub const _IO_SKIPWS : u32 = 1 ;
 pub const _IO_LEFT : u32 = 2 ;
 pub const _IO_RIGHT : u32 = 4 ;
 pub const _IO_INTERNAL : u32 = 8 ;
 pub const _IO_DEC : u32 = 16 ;
 pub const _IO_OCT : u32 = 32 ;
 pub const _IO_HEX : u32 = 64 ;
 pub const _IO_SHOWBASE : u32 = 128 ;
 pub const _IO_SHOWPOINT : u32 = 256 ;
 pub const _IO_UPPERCASE : u32 = 512 ;
 pub const _IO_SHOWPOS : u32 = 1024 ;
 pub const _IO_SCIENTIFIC : u32 = 2048 ;
 pub const _IO_FIXED : u32 = 4096 ;
 pub const _IO_UNITBUF : u32 = 8192 ;
 pub const _IO_STDIO : u32 = 16384 ;
 pub const _IO_DONT_CLOSE : u32 = 32768 ;
 pub const _IO_BOOLALPHA : u32 = 65536 ;
 pub const _IOFBF : u32 = 0 ;
 pub const _IOLBF : u32 = 1 ;
 pub const _IONBF : u32 = 2 ;
 pub const BUFSIZ : u32 = 8192 ;
 pub const P_tmpdir : & 'static [u8 ; 5usize] = b"/tmp\0" ;
 pub const _BITS_STDIO_LIM_H : u32 = 1 ;
 pub const L_tmpnam : u32 = 20 ;
 pub const TMP_MAX : u32 = 238328 ;
 pub const FILENAME_MAX : u32 = 4096 ;
 pub const L_ctermid : u32 = 9 ;
 pub const L_cuserid : u32 = 9 ;
 pub const FOPEN_MAX : u32 = 16 ;
 pub const _DIRENT_H : u32 = 1 ;
 pub const _DIRENT_MATCHES_DIRENT64 : u32 = 1 ;
 pub const MAXNAMLEN : u32 = 255 ;
 pub const _TIME_H : u32 = 1 ;
 pub const _BITS_TIME_H : u32 = 1 ;
 pub const CLOCK_REALTIME : u32 = 0 ;
 pub const CLOCK_MONOTONIC : u32 = 1 ;
 pub const CLOCK_PROCESS_CPUTIME_ID : u32 = 2 ;
 pub const CLOCK_THREAD_CPUTIME_ID : u32 = 3 ;
 pub const CLOCK_MONOTONIC_RAW : u32 = 4 ;
 pub const CLOCK_REALTIME_COARSE : u32 = 5 ;
 pub const CLOCK_MONOTONIC_COARSE : u32 = 6 ;
 pub const CLOCK_BOOTTIME : u32 = 7 ;
 pub const CLOCK_REALTIME_ALARM : u32 = 8 ;
 pub const CLOCK_BOOTTIME_ALARM : u32 = 9 ;
 pub const CLOCK_TAI : u32 = 11 ;
 pub const TIMER_ABSTIME : u32 = 1 ;
 pub const _BITS_TIMEX_H : u32 = 1 ;
 pub const ADJ_OFFSET : u32 = 1 ;
 pub const ADJ_FREQUENCY : u32 = 2 ;
 pub const ADJ_MAXERROR : u32 = 4 ;
 pub const ADJ_ESTERROR : u32 = 8 ;
 pub const ADJ_STATUS : u32 = 16 ;
 pub const ADJ_TIMECONST : u32 = 32 ;
 pub const ADJ_TAI : u32 = 128 ;
 pub const ADJ_SETOFFSET : u32 = 256 ;
 pub const ADJ_MICRO : u32 = 4096 ;
 pub const ADJ_NANO : u32 = 8192 ;
 pub const ADJ_TICK : u32 = 16384 ;
 pub const ADJ_OFFSET_SINGLESHOT : u32 = 32769 ;
 pub const ADJ_OFFSET_SS_READ : u32 = 40961 ;
 pub const MOD_OFFSET : u32 = 1 ;
 pub const MOD_FREQUENCY : u32 = 2 ;
 pub const MOD_MAXERROR : u32 = 4 ;
 pub const MOD_ESTERROR : u32 = 8 ;
 pub const MOD_STATUS : u32 = 16 ;
 pub const MOD_TIMECONST : u32 = 32 ;
 pub const MOD_CLKB : u32 = 16384 ;
 pub const MOD_CLKA : u32 = 32769 ;
 pub const MOD_TAI : u32 = 128 ;
 pub const MOD_MICRO : u32 = 4096 ;
 pub const MOD_NANO : u32 = 8192 ;
 pub const STA_PLL : u32 = 1 ;
 pub const STA_PPSFREQ : u32 = 2 ;
 pub const STA_PPSTIME : u32 = 4 ;
 pub const STA_FLL : u32 = 8 ;
 pub const STA_INS : u32 = 16 ;
 pub const STA_DEL : u32 = 32 ;
 pub const STA_UNSYNC : u32 = 64 ;
 pub const STA_FREQHOLD : u32 = 128 ;
 pub const STA_PPSSIGNAL : u32 = 256 ;
 pub const STA_PPSJITTER : u32 = 512 ;
 pub const STA_PPSWANDER : u32 = 1024 ;
 pub const STA_PPSERROR : u32 = 2048 ;
 pub const STA_CLOCKERR : u32 = 4096 ;
 pub const STA_NANO : u32 = 8192 ;
 pub const STA_MODE : u32 = 16384 ;
 pub const STA_CLK : u32 = 32768 ;
 pub const STA_RONLY : u32 = 65280 ;
 pub const __struct_tm_defined : u32 = 1 ;
 pub const __itimerspec_defined : u32 = 1 ;
 pub const TIME_UTC : u32 = 1 ;
 pub const _ARPA_INET_H : u32 = 1 ;
 pub const _PWD_H : u32 = 1 ;
 pub const NSS_BUFLEN_PASSWD : u32 = 1024 ;
 pub const _GRP_H : u32 = 1 ;
 pub const NSS_BUFLEN_GROUP : u32 = 1024 ;
 pub const _CTYPE_H : u32 = 1 ;
 pub const _SYS_WAIT_H : u32 = 1 ;
 pub const WCOREFLAG : u32 = 128 ;
 pub const WAIT_ANY : i32 = - 1 ;
 pub const WAIT_MYPGRP : u32 = 0 ;
 pub const _SYS_SYSLOG_H : u32 = 1 ;
 pub const _BITS_SYSLOG_PATH_H : u32 = 1 ;
 pub const _PATH_LOG : & 'static [u8 ; 9usize] = b"/dev/log\0" ;
 pub const LOG_EMERG : u32 = 0 ;
 pub const LOG_ALERT : u32 = 1 ;
 pub const LOG_CRIT : u32 = 2 ;
 pub const LOG_ERR : u32 = 3 ;
 pub const LOG_WARNING : u32 = 4 ;
 pub const LOG_NOTICE : u32 = 5 ;
 pub const LOG_INFO : u32 = 6 ;
 pub const LOG_DEBUG : u32 = 7 ;
 pub const LOG_PRIMASK : u32 = 7 ;
 pub const LOG_KERN : u32 = 0 ;
 pub const LOG_USER : u32 = 8 ;
 pub const LOG_MAIL : u32 = 16 ;
 pub const LOG_DAEMON : u32 = 24 ;
 pub const LOG_AUTH : u32 = 32 ;
 pub const LOG_SYSLOG : u32 = 40 ;
 pub const LOG_LPR : u32 = 48 ;
 pub const LOG_NEWS : u32 = 56 ;
 pub const LOG_UUCP : u32 = 64 ;
 pub const LOG_CRON : u32 = 72 ;
 pub const LOG_AUTHPRIV : u32 = 80 ;
 pub const LOG_FTP : u32 = 88 ;
 pub const LOG_LOCAL0 : u32 = 128 ;
 pub const LOG_LOCAL1 : u32 = 136 ;
 pub const LOG_LOCAL2 : u32 = 144 ;
 pub const LOG_LOCAL3 : u32 = 152 ;
 pub const LOG_LOCAL4 : u32 = 160 ;
 pub const LOG_LOCAL5 : u32 = 168 ;
 pub const LOG_LOCAL6 : u32 = 176 ;
 pub const LOG_LOCAL7 : u32 = 184 ;
 pub const LOG_NFACILITIES : u32 = 24 ;
 pub const LOG_FACMASK : u32 = 1016 ;
 pub const LOG_PID : u32 = 1 ;
 pub const LOG_CONS : u32 = 2 ;
 pub const LOG_ODELAY : u32 = 4 ;
 pub const LOG_NDELAY : u32 = 8 ;
 pub const LOG_NOWAIT : u32 = 16 ;
 pub const LOG_PERROR : u32 = 32 ;
 pub const _UTIME_H : u32 = 1 ;
 pub const _NETDB_H : u32 = 1 ;
 pub const _RPC_NETDB_H : u32 = 1 ;
 pub const _PATH_HEQUIV : & 'static [u8 ; 17usize] = b"/etc/hosts.equiv\0" ;
 pub const _PATH_HOSTS : & 'static [u8 ; 11usize] = b"/etc/hosts\0" ;
 pub const _PATH_NETWORKS : & 'static [u8 ; 14usize] = b"/etc/networks\0" ;
 pub const _PATH_NSSWITCH_CONF : & 'static [u8 ; 19usize] = b"/etc/nsswitch.conf\0" ;
 pub const _PATH_PROTOCOLS : & 'static [u8 ; 15usize] = b"/etc/protocols\0" ;
 pub const _PATH_SERVICES : & 'static [u8 ; 14usize] = b"/etc/services\0" ;
 pub const HOST_NOT_FOUND : u32 = 1 ;
 pub const TRY_AGAIN : u32 = 2 ;
 pub const NO_RECOVERY : u32 = 3 ;
 pub const NO_DATA : u32 = 4 ;
 pub const NETDB_INTERNAL : i32 = - 1 ;
 pub const NETDB_SUCCESS : u32 = 0 ;
 pub const NO_ADDRESS : u32 = 4 ;
// pub const IPPORT_RESERVED : u32 = 1024 ;
 pub const SCOPE_DELIMITER : u8 = 37u8 ;
 pub const GAI_WAIT : u32 = 0 ;
 pub const GAI_NOWAIT : u32 = 1 ;
 pub const AI_PASSIVE : u32 = 1 ;
 pub const AI_CANONNAME : u32 = 2 ;
 pub const AI_NUMERICHOST : u32 = 4 ;
 pub const AI_V4MAPPED : u32 = 8 ;
 pub const AI_ALL : u32 = 16 ;
 pub const AI_ADDRCONFIG : u32 = 32 ;
 pub const AI_IDN : u32 = 64 ;
 pub const AI_CANONIDN : u32 = 128 ;
 pub const AI_IDN_ALLOW_UNASSIGNED : u32 = 256 ;
 pub const AI_IDN_USE_STD3_ASCII_RULES : u32 = 512 ;
 pub const AI_NUMERICSERV : u32 = 1024 ;
 pub const EAI_BADFLAGS : i32 = - 1 ;
 pub const EAI_NONAME : i32 = - 2 ;
 pub const EAI_AGAIN : i32 = - 3 ;
 pub const EAI_FAIL : i32 = - 4 ;
 pub const EAI_FAMILY : i32 = - 6 ;
 pub const EAI_SOCKTYPE : i32 = - 7 ;
 pub const EAI_SERVICE : i32 = - 8 ;
 pub const EAI_MEMORY : i32 = - 10 ;
 pub const EAI_SYSTEM : i32 = - 11 ;
 pub const EAI_OVERFLOW : i32 = - 12 ;
 pub const EAI_NODATA : i32 = - 5 ;
 pub const EAI_ADDRFAMILY : i32 = - 9 ;
 pub const EAI_INPROGRESS : i32 = - 100 ;
 pub const EAI_CANCELED : i32 = - 101 ;
 pub const EAI_NOTCANCELED : i32 = - 102 ;
 pub const EAI_ALLDONE : i32 = - 103 ;
 pub const EAI_INTR : i32 = - 104 ;
 pub const EAI_IDN_ENCODE : i32 = - 105 ;
 pub const NI_MAXHOST : u32 = 1025 ;
 pub const NI_MAXSERV : u32 = 32 ;
 pub const NI_NUMERICHOST : u32 = 1 ;
 pub const NI_NUMERICSERV : u32 = 2 ;
 pub const NI_NOFQDN : u32 = 4 ;
 pub const NI_NAMEREQD : u32 = 8 ;
 pub const NI_DGRAM : u32 = 16 ;
 pub const NI_IDN : u32 = 32 ;
 pub const NI_IDN_ALLOW_UNASSIGNED : u32 = 64 ;
 pub const NI_IDN_USE_STD3_ASCII_RULES : u32 = 128 ;
 pub const _SYS_RESOURCE_H : u32 = 1 ;
 pub const RLIM64_INFINITY : i32 = - 1 ;
 pub const __rusage_defined : u32 = 1 ;
 pub const PRIO_MIN : i32 = - 20 ;
 pub const PRIO_MAX : u32 = 20 ;
 extern "C" {
////# [link_name = "\u{1}_Z21vsf_access_check_filePK5mystr"] 

pub fn vsf_access_check_file (p_filename_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
////# [link_name = "\u{1}_Z29vsf_access_check_file_visiblePK5mystr"] 

pub fn vsf_access_check_file_visible (p_filename_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ascii_to_bin_ret {
pub stored : :: std :: os :: raw :: c_uint , pub last_was_cr : :: std :: os :: raw :: c_int , pub p_buf : * mut :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_ascii_to_bin_ret () {
assert_eq ! (:: std :: mem :: size_of :: < ascii_to_bin_ret > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( ascii_to_bin_ret ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ascii_to_bin_ret > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( ascii_to_bin_ret ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ascii_to_bin_ret > ( ) ) ) . stored as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ascii_to_bin_ret ) , "::" , stringify ! ( stored ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ascii_to_bin_ret > ( ) ) ) . last_was_cr as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ascii_to_bin_ret ) , "::" , stringify ! ( last_was_cr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ascii_to_bin_ret > ( ) ) ) . p_buf as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ascii_to_bin_ret ) , "::" , stringify ! ( p_buf ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_ascii_ascii_to_binPcji"] 
pub fn vsf_ascii_ascii_to_bin (p_in : * mut :: std :: os :: raw :: c_char , in_len : :: std :: os :: raw :: c_uint , prev_cr : :: std :: os :: raw :: c_int) -> ascii_to_bin_ret ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct bin_to_ascii_ret {
pub stored : :: std :: os :: raw :: c_uint , pub last_was_cr : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_bin_to_ascii_ret () {
assert_eq ! (:: std :: mem :: size_of :: < bin_to_ascii_ret > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( bin_to_ascii_ret ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < bin_to_ascii_ret > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( bin_to_ascii_ret ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < bin_to_ascii_ret > ( ) ) ) . stored as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( bin_to_ascii_ret ) , "::" , stringify ! ( stored ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < bin_to_ascii_ret > ( ) ) ) . last_was_cr as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( bin_to_ascii_ret ) , "::" , stringify ! ( last_was_cr ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_ascii_bin_to_asciiPKcPcji"] 
pub fn vsf_ascii_bin_to_ascii (p_in : * const :: std :: os :: raw :: c_char , p_out : * mut :: std :: os :: raw :: c_char , in_len : :: std :: os :: raw :: c_uint , prev_cr : :: std :: os :: raw :: c_int) -> bin_to_ascii_ret ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_banner_dir_changedP11vsf_sessioni"] 
pub fn vsf_banner_dir_changed (p_sess : &vsf_session , ftpcode : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_banner_writeP11vsf_sessionP5mystri"] 
pub fn vsf_banner_write (p_sess : &vsf_session , p_str : &mystr , ftpcode : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11handle_featP11vsf_session"] 
pub fn handle_feat (p_sess : &vsf_session) ;

}
 pub type filesize_t = :: std :: os :: raw :: c_longlong ;
 extern "C" {
//# [link_name = "\u{1}_Z12str_filereadP5mystrPKcj"] 
pub fn str_fileread (p_str : &mystr , p_filename : * const :: std :: os :: raw :: c_char , maxsize : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_cmdio_sock_setupv"] 
pub fn vsf_cmdio_sock_setup () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15vsf_cmdio_writeP11vsf_sessioniPKc"] 
pub fn vsf_cmdio_write (p_sess : *const vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char) ;

}

/*pub fn vsf_cmdio_write_rust (p_sess : *mut vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char) {
//	vsf_cmdio_write(p_sess as *const _  as & _ as &vsf_session ,status,p_text);
    if p_sess != ptr::null_mut() {
	vsf_cmdio_write(p_sess as *const _  as & _ as &vsf_session ,status,p_text);    	
    }
}
*/
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_cmdio_write_hyphenP11vsf_sessioniPKc"] 
pub fn vsf_cmdio_write_hyphen (p_sess : &vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_cmdio_write_rawP11vsf_sessionPKc"] 
pub fn vsf_cmdio_write_raw (p_sess : &vsf_session , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_cmdio_write_exitP11vsf_sessioniPKci"] 
pub fn vsf_cmdio_write_exit (p_sess : &vsf_session , status : :: std :: os :: raw :: c_int , p_text : * const :: std :: os :: raw :: c_char , exit_val : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_cmdio_write_strP11vsf_sessioniPK5mystr"] 
pub fn vsf_cmdio_write_str (p_sess : &vsf_session , status : :: std :: os :: raw :: c_int , p_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_cmdio_write_str_hyphenP11vsf_sessioniPK5mystr"] 
pub fn vsf_cmdio_write_str_hyphen (p_sess : &vsf_session , status : :: std :: os :: raw :: c_int , p_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_cmdio_set_alarmP11vsf_session"] 
pub fn vsf_cmdio_set_alarm (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_cmdio_get_cmd_and_argP11vsf_sessionP5mystrS2_i"] 
pub fn vsf_cmdio_get_cmd_and_arg (p_sess : &vsf_session , p_cmd_str : &mystr , p_arg_str : &mystr , set_alarm : :: std :: os :: raw :: c_int) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_dir {
_unused : [u8 ; 0] ,
}
 extern "C" {
//# [link_name = "\u{1}_Z33vsf_ftpdataio_dispose_transfer_fdP11vsf_session"] 
pub fn vsf_ftpdataio_dispose_transfer_fd (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_ftpdataio_get_pasv_fdP11vsf_session"] 
pub fn vsf_ftpdataio_get_pasv_fd (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_ftpdataio_get_port_fdP11vsf_session"] 
pub fn vsf_ftpdataio_get_port_fd (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_ftpdataio_post_mark_connectP11vsf_session"] 
pub fn vsf_ftpdataio_post_mark_connect (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_transfer_ret {
pub retval : :: std :: os :: raw :: c_int ,
pub transferred : filesize_t ,
}
 # [test] fn bindgen_test_layout_vsf_transfer_ret () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_transfer_ret > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( vsf_transfer_ret ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_transfer_ret > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( vsf_transfer_ret ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_transfer_ret > ( ) ) ) . retval as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_transfer_ret ) , "::" , stringify ! ( retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_transfer_ret > ( ) ) ) . transferred as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( vsf_transfer_ret ) , "::" , stringify ! ( transferred ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_ftpdataio_transfer_fileP11vsf_sessioniiii"] 
pub fn vsf_ftpdataio_transfer_file (p_sess : &vsf_session , remote_fd : :: std :: os :: raw :: c_int , file_fd : :: std :: os :: raw :: c_int , is_recv : :: std :: os :: raw :: c_int , is_ascii : :: std :: os :: raw :: c_int) -> vsf_transfer_ret ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_ftpdataio_transfer_dirP11vsf_sessioniP15vsf_sysutil_dirPK5mystrS5_S5_i"] 
pub fn vsf_ftpdataio_transfer_dir (p_sess : &vsf_session , is_control : :: std :: os :: raw :: c_int , p_dir : * mut vsf_sysutil_dir , p_base_dir_str : * const mystr , p_option_str : * const mystr , p_filter_str : * const mystr , is_verbose : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct pt_sandbox {
_unused : [u8 ; 0] ,
}
 extern "C" {
//# [link_name = "\u{1}_Z12policy_setupP10pt_sandboxPK11vsf_session"] 
pub fn policy_setup (p_sandbox : * mut pt_sandbox , p_sess : * const vsf_session) ;

}
 pub type hashfunc_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_uint , arg2 : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_uint > ;
 extern "C" {
//# [link_name = "\u{1}_Z10hash_allocjjjPFjjPvE"] 
pub fn hash_alloc (buckets : :: std :: os :: raw :: c_uint , key_size : :: std :: os :: raw :: c_uint , value_size : :: std :: os :: raw :: c_uint , hash_func : hashfunc_t) -> * mut hash ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17hash_lookup_entryP4hashPv"] 
pub fn hash_lookup_entry (p_hash : * mut hash , p_key : * mut :: std :: os :: raw :: c_void) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14hash_add_entryP4hashPvS1_"] 
pub fn hash_add_entry (p_hash : * mut hash , p_key : * mut :: std :: os :: raw :: c_void , p_value : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15hash_free_entryP4hashPv"] 
pub fn hash_free_entry (p_hash : * mut hash , p_key : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_parse_ipv6PK5mystr"] 
pub fn vsf_sysutil_parse_ipv6 (p_str : * const mystr) -> * const :: std :: os :: raw :: c_uchar ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_parse_ipv4PK5mystr"] 
pub fn vsf_sysutil_parse_ipv4 (p_str : * const mystr) -> * const :: std :: os :: raw :: c_uchar ;

}
 extern "C" {
//# [link_name = "\u{1}_Z34vsf_sysutil_parse_uchar_string_sepPK5mystrcPhj"] 
pub fn vsf_sysutil_parse_uchar_string_sep (p_str : * const mystr , sep : :: std :: os :: raw :: c_char , p_items : * mut :: std :: os :: raw :: c_uchar , items : :: std :: os :: raw :: c_uint) -> * const :: std :: os :: raw :: c_uchar ;

}
 pub const EVSFLogEntryType_kVSFLogEntryNull : EVSFLogEntryType = 1 ;
 pub const EVSFLogEntryType_kVSFLogEntryDownload : EVSFLogEntryType = 2 ;
 pub const EVSFLogEntryType_kVSFLogEntryUpload : EVSFLogEntryType = 3 ;
 pub const EVSFLogEntryType_kVSFLogEntryMkdir : EVSFLogEntryType = 4 ;
 pub const EVSFLogEntryType_kVSFLogEntryLogin : EVSFLogEntryType = 5 ;
 pub const EVSFLogEntryType_kVSFLogEntryFTPInput : EVSFLogEntryType = 6 ;
 pub const EVSFLogEntryType_kVSFLogEntryFTPOutput : EVSFLogEntryType = 7 ;
 pub const EVSFLogEntryType_kVSFLogEntryConnection : EVSFLogEntryType = 8 ;
 pub const EVSFLogEntryType_kVSFLogEntryDelete : EVSFLogEntryType = 9 ;
 pub const EVSFLogEntryType_kVSFLogEntryRename : EVSFLogEntryType = 10 ;
 pub const EVSFLogEntryType_kVSFLogEntryRmdir : EVSFLogEntryType = 11 ;
 pub const EVSFLogEntryType_kVSFLogEntryChmod : EVSFLogEntryType = 12 ;
 pub const EVSFLogEntryType_kVSFLogEntryDebug : EVSFLogEntryType = 13 ;
 pub type EVSFLogEntryType = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z12vsf_log_initP11vsf_session"] 
pub fn vsf_log_init (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_log_start_entryP11vsf_session16EVSFLogEntryType"] 
pub fn vsf_log_start_entry (p_sess : &vsf_session , what : EVSFLogEntryType) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_log_entry_pendingP11vsf_session"] 
pub fn vsf_log_entry_pending (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_log_clear_entryP11vsf_session"] 
pub fn vsf_log_clear_entry (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14vsf_log_do_logP11vsf_sessioni"] 
pub fn vsf_log_do_log (p_sess : &vsf_session , succeeded : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z12vsf_log_lineP11vsf_session16EVSFLogEntryTypeP5mystr"] 
pub fn vsf_log_line (p_sess : &vsf_session , what : EVSFLogEntryType , p_str : &mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_ls_populate_dir_listP10mystr_listS0_P15vsf_sysutil_dirPK5mystrS5_S5_i"] 
pub fn vsf_ls_populate_dir_list (p_list : &mystr_list , p_subdir_list : &mystr_list , p_dir : * mut vsf_sysutil_dir , p_base_dir_str : * const mystr , p_option_str : * const mystr , p_filter_str : * const mystr , is_verbose : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_filename_passes_filterPK5mystrS1_Pj"] 
pub fn vsf_filename_passes_filter (p_filename_str : * const mystr , p_filter_str : * const mystr , iters : * mut :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 pub type str_netfd_read_t = :: std :: option :: Option < unsafe extern "C" fn (p_sess : &vsf_session , arg1 : * mut :: std :: os :: raw :: c_char , arg2 : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int > ;
 extern "C" {
//# [link_name = "\u{1}_Z15str_netfd_allocP11vsf_sessionP5mystrcPcjPFiS0_S3_jES5_"] 
pub fn str_netfd_alloc (p_sess : &vsf_session , p_str : &mystr , term : :: std :: os :: raw :: c_char , p_readbuf : * mut :: std :: os :: raw :: c_char , maxlen : :: std :: os :: raw :: c_uint , p_peekfunc : str_netfd_read_t , p_readfunc : str_netfd_read_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_netfd_readP5mystrij"] 
pub fn str_netfd_read (p_str : &mystr , fd : :: std :: os :: raw :: c_int , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_netfd_writePK5mystri"] 
pub fn str_netfd_write (p_str : * const mystr , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_one_process_startP11vsf_session"] 
pub fn vsf_one_process_start (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_one_process_loginP11vsf_sessionPK5mystr"] 
pub fn vsf_one_process_login (p_sess : &vsf_session , p_pass_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z34vsf_one_process_get_priv_data_sockP11vsf_session"] 
pub fn vsf_one_process_get_priv_data_sock (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_one_process_pasv_cleanupP11vsf_session"] 
pub fn vsf_one_process_pasv_cleanup (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_one_process_pasv_activeP11vsf_session"] 
pub fn vsf_one_process_pasv_active (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_one_process_listenP11vsf_session"] 
pub fn vsf_one_process_listen (p_sess : &vsf_session) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_one_process_get_pasv_fdP11vsf_session"] 
pub fn vsf_one_process_get_pasv_fd (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_one_process_chown_uploadP11vsf_sessioni"] 
pub fn vsf_one_process_chown_upload (p_sess : &vsf_session , fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11handle_optsP11vsf_session"] 
pub fn handle_opts (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_parseconf_load_filePKci"] 
pub fn vsf_parseconf_load_file (p_filename : * const :: std :: os :: raw :: c_char , errs_fatal : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_parseconf_load_settingPKci"] 
pub fn vsf_parseconf_load_setting (p_setting : * const :: std :: os :: raw :: c_char , errs_fatal : :: std :: os :: raw :: c_int) ;

}
 pub type __u_char = :: std :: os :: raw :: c_uchar ;
 pub type __u_short = :: std :: os :: raw :: c_ushort ;
 pub type __u_int = :: std :: os :: raw :: c_uint ;
 pub type __u_long = :: std :: os :: raw :: c_ulong ;
 pub type __int8_t = :: std :: os :: raw :: c_schar ;
 pub type __uint8_t = :: std :: os :: raw :: c_uchar ;
 pub type __int16_t = :: std :: os :: raw :: c_short ;
 pub type __uint16_t = :: std :: os :: raw :: c_ushort ;
 pub type __int32_t = :: std :: os :: raw :: c_int ;
 pub type __uint32_t = :: std :: os :: raw :: c_uint ;
 pub type __int64_t = :: std :: os :: raw :: c_long ;
 pub type __uint64_t = :: std :: os :: raw :: c_ulong ;
 pub type __quad_t = :: std :: os :: raw :: c_long ;
 pub type __u_quad_t = :: std :: os :: raw :: c_ulong ;
 pub type __intmax_t = :: std :: os :: raw :: c_long ;
 pub type __uintmax_t = :: std :: os :: raw :: c_ulong ;
 pub type __dev_t = :: std :: os :: raw :: c_ulong ;
 pub type __uid_t = :: std :: os :: raw :: c_uint ;
 pub type __gid_t = :: std :: os :: raw :: c_uint ;
 pub type __ino_t = :: std :: os :: raw :: c_ulong ;
 pub type __ino64_t = :: std :: os :: raw :: c_ulong ;
 pub type __mode_t = :: std :: os :: raw :: c_uint ;
 pub type __nlink_t = :: std :: os :: raw :: c_ulong ;
 pub type __off_t = :: std :: os :: raw :: c_long ;
 pub type __off64_t = :: std :: os :: raw :: c_long ;
 pub type __pid_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __fsid_t {
pub __val : [:: std :: os :: raw :: c_int ; 2usize] ,
}
 # [test] fn bindgen_test_layout___fsid_t () {
assert_eq ! (:: std :: mem :: size_of :: < __fsid_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __fsid_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __fsid_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __fsid_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __fsid_t > ( ) ) ) . __val as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __fsid_t ) , "::" , stringify ! ( __val ) )) ;

}
 pub type __clock_t = :: std :: os :: raw :: c_long ;
 pub type __rlim_t = :: std :: os :: raw :: c_ulong ;
 pub type __rlim64_t = :: std :: os :: raw :: c_ulong ;
 pub type __id_t = :: std :: os :: raw :: c_uint ;
 pub type __time_t = :: std :: os :: raw :: c_long ;
 pub type __useconds_t = :: std :: os :: raw :: c_uint ;
 pub type __suseconds_t = :: std :: os :: raw :: c_long ;
 pub type __daddr_t = :: std :: os :: raw :: c_int ;
 pub type __key_t = :: std :: os :: raw :: c_int ;
 pub type __clockid_t = :: std :: os :: raw :: c_int ;
 pub type __timer_t = * mut :: std :: os :: raw :: c_void ;
 pub type __blksize_t = :: std :: os :: raw :: c_long ;
 pub type __blkcnt_t = :: std :: os :: raw :: c_long ;
 pub type __blkcnt64_t = :: std :: os :: raw :: c_long ;
 pub type __fsblkcnt_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsblkcnt64_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsfilcnt_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsfilcnt64_t = :: std :: os :: raw :: c_ulong ;
 pub type __fsword_t = :: std :: os :: raw :: c_long ;
 pub type __ssize_t = :: std :: os :: raw :: c_long ;
 pub type __syscall_slong_t = :: std :: os :: raw :: c_long ;
 pub type __syscall_ulong_t = :: std :: os :: raw :: c_ulong ;
 pub type __loff_t = __off64_t ;
 pub type __caddr_t = * mut :: std :: os :: raw :: c_char ;
 pub type __intptr_t = :: std :: os :: raw :: c_long ;
 pub type __socklen_t = :: std :: os :: raw :: c_uint ;
 pub type __sig_atomic_t = :: std :: os :: raw :: c_int ;
 pub type u_char = __u_char ;
 pub type u_short = __u_short ;
 pub type u_int = __u_int ;
 pub type u_long = __u_long ;
 pub type quad_t = __quad_t ;
 pub type u_quad_t = __u_quad_t ;
 pub type fsid_t = __fsid_t ;
 pub type loff_t = __loff_t ;
 pub type ino_t = __ino_t ;
 pub type ino64_t = __ino64_t ;
 pub type dev_t = __dev_t ;
 pub type gid_t = __gid_t ;
 pub type mode_t = __mode_t ;
 pub type nlink_t = __nlink_t ;
 pub type uid_t = __uid_t ;
 pub type off_t = __off_t ;
 pub type off64_t = __off64_t ;
 pub type pid_t = __pid_t ;
 pub type id_t = __id_t ;
 pub type daddr_t = __daddr_t ;
 pub type caddr_t = __caddr_t ;
 pub type key_t = __key_t ;
 pub type clock_t = __clock_t ;
 pub type clockid_t = __clockid_t ;
 pub type time_t = __time_t ;
 pub type timer_t = __timer_t ;
 pub type useconds_t = __useconds_t ;
 pub type suseconds_t = __suseconds_t ;
 pub type ulong = :: std :: os :: raw :: c_ulong ;
 pub type ushort = :: std :: os :: raw :: c_ushort ;
 pub type uint = :: std :: os :: raw :: c_uint ;
 pub type u_int8_t = :: std :: os :: raw :: c_uchar ;
 pub type u_int16_t = :: std :: os :: raw :: c_ushort ;
 pub type u_int32_t = :: std :: os :: raw :: c_uint ;
 pub type u_int64_t = :: std :: os :: raw :: c_ulong ;
 pub type register_t = :: std :: os :: raw :: c_long ;
 # [repr ( C )] # [derive ( Default, Debug , Copy , Clone )] pub struct __sigset_t {
pub __val : [:: std :: os :: raw :: c_ulong ; 16usize] ,
}
 # [test] fn bindgen_test_layout___sigset_t () {
assert_eq ! (:: std :: mem :: size_of :: < __sigset_t > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( __sigset_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __sigset_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __sigset_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __sigset_t > ( ) ) ) . __val as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __sigset_t ) , "::" , stringify ! ( __val ) )) ;

}
 pub type sigset_t = __sigset_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct timeval {
pub tv_sec : __time_t , pub tv_usec : __suseconds_t ,
}
 # [test] fn bindgen_test_layout_timeval () {
assert_eq ! (:: std :: mem :: size_of :: < timeval > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( timeval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timeval > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( timeval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timeval > ( ) ) ) . tv_sec as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timeval ) , "::" , stringify ! ( tv_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timeval > ( ) ) ) . tv_usec as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( timeval ) , "::" , stringify ! ( tv_usec ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct timespec {
pub tv_sec : __time_t , pub tv_nsec : __syscall_slong_t ,
}
 # [test] fn bindgen_test_layout_timespec () {
assert_eq ! (:: std :: mem :: size_of :: < timespec > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( timespec ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timespec > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( timespec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timespec > ( ) ) ) . tv_sec as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timespec ) , "::" , stringify ! ( tv_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timespec > ( ) ) ) . tv_nsec as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( timespec ) , "::" , stringify ! ( tv_nsec ) )) ;

}
 pub type __fd_mask = :: std :: os :: raw :: c_long ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct fd_set {
pub fds_bits : [__fd_mask ; 16usize] ,
}
 # [test] fn bindgen_test_layout_fd_set () {
assert_eq ! (:: std :: mem :: size_of :: < fd_set > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( fd_set ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < fd_set > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( fd_set ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < fd_set > ( ) ) ) . fds_bits as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( fd_set ) , "::" , stringify ! ( fds_bits ) )) ;

}
 pub type fd_mask = __fd_mask ;
 extern "C" {
pub fn select (__nfds : :: std :: os :: raw :: c_int , __readfds : * mut fd_set , __writefds : * mut fd_set , __exceptfds : * mut fd_set , __timeout : * mut timeval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pselect (__nfds : :: std :: os :: raw :: c_int , __readfds : * mut fd_set , __writefds : * mut fd_set , __exceptfds : * mut fd_set , __timeout : * const timespec , __sigmask : * const __sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gnu_dev_major (__dev : __dev_t) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn gnu_dev_minor (__dev : __dev_t) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn gnu_dev_makedev (__major : :: std :: os :: raw :: c_uint , __minor : :: std :: os :: raw :: c_uint) -> __dev_t ;

}
 pub type blksize_t = __blksize_t ;
 pub type blkcnt_t = __blkcnt_t ;
 pub type fsblkcnt_t = __fsblkcnt_t ;
 pub type fsfilcnt_t = __fsfilcnt_t ;
 pub type blkcnt64_t = __blkcnt64_t ;
 pub type fsblkcnt64_t = __fsblkcnt64_t ;
 pub type fsfilcnt64_t = __fsfilcnt64_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_rwlock_arch_t {
pub __readers : :: std :: os :: raw :: c_uint , pub __writers : :: std :: os :: raw :: c_uint , pub __wrphase_futex : :: std :: os :: raw :: c_uint , pub __writers_futex : :: std :: os :: raw :: c_uint , pub __pad3 : :: std :: os :: raw :: c_uint , pub __pad4 : :: std :: os :: raw :: c_uint , pub __cur_writer : :: std :: os :: raw :: c_int , pub __shared : :: std :: os :: raw :: c_int , pub __rwelision : :: std :: os :: raw :: c_schar , pub __pad1 : [:: std :: os :: raw :: c_uchar ; 7usize] , pub __pad2 : :: std :: os :: raw :: c_ulong , pub __flags : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout___pthread_rwlock_arch_t () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_rwlock_arch_t > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( __pthread_rwlock_arch_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_rwlock_arch_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_rwlock_arch_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __readers as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __readers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __writers as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __writers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __wrphase_futex as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __wrphase_futex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __writers_futex as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __writers_futex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad3 as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad3 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad4 as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad4 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __cur_writer as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __cur_writer ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __shared as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __shared ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __rwelision as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __rwelision ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad1 as * const _ as usize } , 33usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __pad2 as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __pad2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_rwlock_arch_t > ( ) ) ) . __flags as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_rwlock_arch_t ) , "::" , stringify ! ( __flags ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_internal_list {
pub __prev : * mut __pthread_internal_list , pub __next : * mut __pthread_internal_list ,
}
 # [test] fn bindgen_test_layout___pthread_internal_list () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_internal_list > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( __pthread_internal_list ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_internal_list > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_internal_list ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_internal_list > ( ) ) ) . __prev as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_internal_list ) , "::" , stringify ! ( __prev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_internal_list > ( ) ) ) . __next as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_internal_list ) , "::" , stringify ! ( __next ) )) ;

}
 pub type __pthread_list_t = __pthread_internal_list ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_mutex_s {
pub __lock : :: std :: os :: raw :: c_int , pub __count : :: std :: os :: raw :: c_uint , pub __owner : :: std :: os :: raw :: c_int , pub __nusers : :: std :: os :: raw :: c_uint , pub __kind : :: std :: os :: raw :: c_int , pub __spins : :: std :: os :: raw :: c_short , pub __elision : :: std :: os :: raw :: c_short , pub __list : __pthread_list_t ,
}
 # [test] fn bindgen_test_layout___pthread_mutex_s () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_mutex_s > ( ) , 40usize , concat ! ( "Size of: " , stringify ! ( __pthread_mutex_s ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_mutex_s > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_mutex_s ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __lock as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __lock ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __count as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __count ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __owner as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __owner ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __nusers as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __nusers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __kind as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __kind ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __spins as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __spins ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __elision as * const _ as usize } , 22usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __elision ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_mutex_s > ( ) ) ) . __list as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_mutex_s ) , "::" , stringify ! ( __list ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct __pthread_cond_s {
pub __bindgen_anon_1 : __pthread_cond_s__bindgen_ty_1 , pub __bindgen_anon_2 : __pthread_cond_s__bindgen_ty_2 , pub __g_refs : [:: std :: os :: raw :: c_uint ; 2usize] , pub __g_size : [:: std :: os :: raw :: c_uint ; 2usize] , pub __g1_orig_size : :: std :: os :: raw :: c_uint , pub __wrefs : :: std :: os :: raw :: c_uint , pub __g_signals : [:: std :: os :: raw :: c_uint ; 2usize] ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union __pthread_cond_s__bindgen_ty_1 {
pub __wseq : :: std :: os :: raw :: c_ulonglong , pub __wseq32 : __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 , _bindgen_union_align : u64 ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
pub __low : :: std :: os :: raw :: c_uint , pub __high : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . __low as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( __low ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . __high as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( __high ) )) ;

}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1 > ( ) ) ) . __wseq as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) , "::" , stringify ! ( __wseq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_1 > ( ) ) ) . __wseq32 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_1 ) , "::" , stringify ! ( __wseq32 ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union __pthread_cond_s__bindgen_ty_2 {
pub __g1_start : :: std :: os :: raw :: c_ulonglong , pub __g1_start32 : __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 , _bindgen_union_align : u64 ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
pub __low : :: std :: os :: raw :: c_uint , pub __high : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) ) ) . __low as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) , "::" , stringify ! ( __low ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 > ( ) ) ) . __high as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ) , "::" , stringify ! ( __high ) )) ;

}
 # [test] fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2 () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2 > ( ) ) ) . __g1_start as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) , "::" , stringify ! ( __g1_start ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s__bindgen_ty_2 > ( ) ) ) . __g1_start32 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s__bindgen_ty_2 ) , "::" , stringify ! ( __g1_start32 ) )) ;

}
 # [test] fn bindgen_test_layout___pthread_cond_s () {
assert_eq ! (:: std :: mem :: size_of :: < __pthread_cond_s > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( __pthread_cond_s ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __pthread_cond_s > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __pthread_cond_s ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g_refs as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g_refs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g_size as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g1_orig_size as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g1_orig_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __wrefs as * const _ as usize } , 36usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __wrefs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __pthread_cond_s > ( ) ) ) . __g_signals as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( __pthread_cond_s ) , "::" , stringify ! ( __g_signals ) )) ;

}
 pub type pthread_t = :: std :: os :: raw :: c_ulong ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_mutexattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 4usize] , pub __align : :: std :: os :: raw :: c_int , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout_pthread_mutexattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_mutexattr_t > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( pthread_mutexattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_mutexattr_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( pthread_mutexattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutexattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutexattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutexattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutexattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_condattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 4usize] , pub __align : :: std :: os :: raw :: c_int , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout_pthread_condattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_condattr_t > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( pthread_condattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_condattr_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( pthread_condattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_condattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_condattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_condattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_condattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 pub type pthread_key_t = :: std :: os :: raw :: c_uint ;
 pub type pthread_once_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_attr_t {
pub __size : [:: std :: os :: raw :: c_char ; 56usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 7usize] ,
}
 # [test] fn bindgen_test_layout_pthread_attr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_attr_t > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( pthread_attr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_attr_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_attr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_attr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_attr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_attr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_attr_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_mutex_t {
pub __data : __pthread_mutex_s , pub __size : [:: std :: os :: raw :: c_char ; 40usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 5usize] ,
}
 # [test] fn bindgen_test_layout_pthread_mutex_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_mutex_t > ( ) , 40usize , concat ! ( "Size of: " , stringify ! ( pthread_mutex_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_mutex_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_mutex_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutex_t > ( ) ) ) . __data as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutex_t ) , "::" , stringify ! ( __data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutex_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutex_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_mutex_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_mutex_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_cond_t {
pub __data : __pthread_cond_s , pub __size : [:: std :: os :: raw :: c_char ; 48usize] , pub __align : :: std :: os :: raw :: c_longlong , _bindgen_union_align : [u64 ; 6usize] ,
}
 # [test] fn bindgen_test_layout_pthread_cond_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_cond_t > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( pthread_cond_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_cond_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_cond_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_cond_t > ( ) ) ) . __data as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_cond_t ) , "::" , stringify ! ( __data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_cond_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_cond_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_cond_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_cond_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_rwlock_t {
pub __data : __pthread_rwlock_arch_t , pub __size : [:: std :: os :: raw :: c_char ; 56usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 7usize] ,
}
 # [test] fn bindgen_test_layout_pthread_rwlock_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_rwlock_t > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( pthread_rwlock_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_rwlock_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_rwlock_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlock_t > ( ) ) ) . __data as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlock_t ) , "::" , stringify ! ( __data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlock_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlock_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlock_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlock_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_rwlockattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 8usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_pthread_rwlockattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_rwlockattr_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( pthread_rwlockattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_rwlockattr_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_rwlockattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlockattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlockattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_rwlockattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_rwlockattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 pub type pthread_spinlock_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_barrier_t {
pub __size : [:: std :: os :: raw :: c_char ; 32usize] , pub __align : :: std :: os :: raw :: c_long , _bindgen_union_align : [u64 ; 4usize] ,
}
 # [test] fn bindgen_test_layout_pthread_barrier_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_barrier_t > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( pthread_barrier_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_barrier_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( pthread_barrier_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrier_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrier_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrier_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrier_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union pthread_barrierattr_t {
pub __size : [:: std :: os :: raw :: c_char ; 4usize] , pub __align : :: std :: os :: raw :: c_int , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout_pthread_barrierattr_t () {
assert_eq ! (:: std :: mem :: size_of :: < pthread_barrierattr_t > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( pthread_barrierattr_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < pthread_barrierattr_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( pthread_barrierattr_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrierattr_t > ( ) ) ) . __size as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrierattr_t ) , "::" , stringify ! ( __size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < pthread_barrierattr_t > ( ) ) ) . __align as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( pthread_barrierattr_t ) , "::" , stringify ! ( __align ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct iovec {
pub iov_base : * mut :: std :: os :: raw :: c_void , pub iov_len : usize ,
}
 # [test] fn bindgen_test_layout_iovec () {
assert_eq ! (:: std :: mem :: size_of :: < iovec > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( iovec ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < iovec > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( iovec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iovec > ( ) ) ) . iov_base as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( iovec ) , "::" , stringify ! ( iov_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iovec > ( ) ) ) . iov_len as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( iovec ) , "::" , stringify ! ( iov_len ) )) ;

}
 pub type socklen_t = __socklen_t ;
 pub const __socket_type_SOCK_STREAM : __socket_type = 1 ;
 pub const __socket_type_SOCK_DGRAM : __socket_type = 2 ;
 pub const __socket_type_SOCK_RAW : __socket_type = 3 ;
 pub const __socket_type_SOCK_RDM : __socket_type = 4 ;
 pub const __socket_type_SOCK_SEQPACKET : __socket_type = 5 ;
 pub const __socket_type_SOCK_DCCP : __socket_type = 6 ;
 pub const __socket_type_SOCK_PACKET : __socket_type = 10 ;
 pub const __socket_type_SOCK_CLOEXEC : __socket_type = 524288 ;
 pub const __socket_type_SOCK_NONBLOCK : __socket_type = 2048 ;
 pub type __socket_type = u32 ;
 pub type sa_family_t = :: std :: os :: raw :: c_ushort ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sockaddr {
pub sa_family : sa_family_t , pub sa_data : [:: std :: os :: raw :: c_char ; 14usize] ,
}
 # [test] fn bindgen_test_layout_sockaddr () {
assert_eq ! (:: std :: mem :: size_of :: < sockaddr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( sockaddr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sockaddr > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( sockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr > ( ) ) ) . sa_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr ) , "::" , stringify ! ( sa_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr > ( ) ) ) . sa_data as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr ) , "::" , stringify ! ( sa_data ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct sockaddr_storage {
pub ss_family : sa_family_t , pub __ss_padding : [:: std :: os :: raw :: c_char ; 118usize] , pub __ss_align : :: std :: os :: raw :: c_ulong ,
}
 # [test] fn bindgen_test_layout_sockaddr_storage () {
assert_eq ! (:: std :: mem :: size_of :: < sockaddr_storage > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( sockaddr_storage ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sockaddr_storage > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sockaddr_storage ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_storage > ( ) ) ) . ss_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_storage ) , "::" , stringify ! ( ss_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_storage > ( ) ) ) . __ss_padding as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_storage ) , "::" , stringify ! ( __ss_padding ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_storage > ( ) ) ) . __ss_align as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_storage ) , "::" , stringify ! ( __ss_align ) )) ;

}
 pub const MSG_OOB : _bindgen_ty_1 = 1 ;
 pub const MSG_PEEK : _bindgen_ty_1 = 2 ;
 pub const MSG_DONTROUTE : _bindgen_ty_1 = 4 ;
 pub const MSG_TRYHARD : _bindgen_ty_1 = 4 ;
 pub const MSG_CTRUNC : _bindgen_ty_1 = 8 ;
 pub const MSG_PROXY : _bindgen_ty_1 = 16 ;
 pub const MSG_TRUNC : _bindgen_ty_1 = 32 ;
 pub const MSG_DONTWAIT : _bindgen_ty_1 = 64 ;
 pub const MSG_EOR : _bindgen_ty_1 = 128 ;
 pub const MSG_WAITALL : _bindgen_ty_1 = 256 ;
 pub const MSG_FIN : _bindgen_ty_1 = 512 ;
 pub const MSG_SYN : _bindgen_ty_1 = 1024 ;
 pub const MSG_CONFIRM : _bindgen_ty_1 = 2048 ;
 pub const MSG_RST : _bindgen_ty_1 = 4096 ;
 pub const MSG_ERRQUEUE : _bindgen_ty_1 = 8192 ;
 pub const MSG_NOSIGNAL : _bindgen_ty_1 = 16384 ;
 pub const MSG_MORE : _bindgen_ty_1 = 32768 ;
 pub const MSG_WAITFORONE : _bindgen_ty_1 = 65536 ;
 pub const MSG_BATCH : _bindgen_ty_1 = 262144 ;
 pub const MSG_ZEROCOPY : _bindgen_ty_1 = 67108864 ;
 pub const MSG_FASTOPEN : _bindgen_ty_1 = 536870912 ;
 pub const MSG_CMSG_CLOEXEC : _bindgen_ty_1 = 1073741824 ;
 pub type _bindgen_ty_1 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct msghdr {
pub msg_name : * mut :: std :: os :: raw :: c_void , pub msg_namelen : socklen_t , pub msg_iov : * mut iovec , pub msg_iovlen : usize , pub msg_control : * mut :: std :: os :: raw :: c_void , pub msg_controllen : usize , pub msg_flags : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_msghdr () {
assert_eq ! (:: std :: mem :: size_of :: < msghdr > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( msghdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < msghdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( msghdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_namelen as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_namelen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_iov as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_iov ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_iovlen as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_iovlen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_control as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_control ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_controllen as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_controllen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < msghdr > ( ) ) ) . msg_flags as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( msghdr ) , "::" , stringify ! ( msg_flags ) )) ;

}
 # [repr ( C )] # [derive ( Debug )] pub struct cmsghdr {
pub cmsg_len : usize , pub cmsg_level : :: std :: os :: raw :: c_int , pub cmsg_type : :: std :: os :: raw :: c_int , pub __cmsg_data : __IncompleteArrayField < :: std :: os :: raw :: c_uchar > ,
}
 # [test] fn bindgen_test_layout_cmsghdr () {
assert_eq ! (:: std :: mem :: size_of :: < cmsghdr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( cmsghdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < cmsghdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( cmsghdr ) )) ;

}
 extern "C" {
pub fn __cmsg_nxthdr (__mhdr : * mut msghdr , __cmsg : * mut cmsghdr) -> * mut cmsghdr ;

}
 pub const SCM_RIGHTS : _bindgen_ty_2 = 1 ;
 pub const SCM_CREDENTIALS : _bindgen_ty_2 = 2 ;
 pub type _bindgen_ty_2 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ucred {
pub pid : pid_t , pub uid : uid_t , pub gid : gid_t ,
}
 # [test] fn bindgen_test_layout_ucred () {
assert_eq ! (:: std :: mem :: size_of :: < ucred > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( ucred ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ucred > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ucred ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucred > ( ) ) ) . pid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ucred ) , "::" , stringify ! ( pid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucred > ( ) ) ) . uid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ucred ) , "::" , stringify ! ( uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucred > ( ) ) ) . gid as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ucred ) , "::" , stringify ! ( gid ) )) ;

}
 # [repr ( C )] # [derive (Default, Debug , Copy , Clone )] pub struct linger {
pub l_onoff : :: std :: os :: raw :: c_int , pub l_linger : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_linger () {
assert_eq ! (:: std :: mem :: size_of :: < linger > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( linger ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < linger > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( linger ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < linger > ( ) ) ) . l_onoff as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( linger ) , "::" , stringify ! ( l_onoff ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < linger > ( ) ) ) . l_linger as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( linger ) , "::" , stringify ! ( l_linger ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct osockaddr {
pub sa_family : :: std :: os :: raw :: c_ushort , pub sa_data : [:: std :: os :: raw :: c_uchar ; 14usize] ,
}
 # [test] fn bindgen_test_layout_osockaddr () {
assert_eq ! (:: std :: mem :: size_of :: < osockaddr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( osockaddr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < osockaddr > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( osockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < osockaddr > ( ) ) ) . sa_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( osockaddr ) , "::" , stringify ! ( sa_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < osockaddr > ( ) ) ) . sa_data as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( osockaddr ) , "::" , stringify ! ( sa_data ) )) ;

}
 pub const SHUT_RD : _bindgen_ty_3 = 0 ;
 pub const SHUT_WR : _bindgen_ty_3 = 1 ;
 pub const SHUT_RDWR : _bindgen_ty_3 = 2 ;
 pub type _bindgen_ty_3 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mmsghdr {
pub msg_hdr : msghdr , pub msg_len : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_mmsghdr () {
assert_eq ! (:: std :: mem :: size_of :: < mmsghdr > ( ) , 64usize , concat ! ( "Size of: " , stringify ! ( mmsghdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mmsghdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mmsghdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mmsghdr > ( ) ) ) . msg_hdr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mmsghdr ) , "::" , stringify ! ( msg_hdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mmsghdr > ( ) ) ) . msg_len as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( mmsghdr ) , "::" , stringify ! ( msg_len ) )) ;

}
 extern "C" {
pub fn socket (__domain : :: std :: os :: raw :: c_int , __type : :: std :: os :: raw :: c_int , __protocol : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn socketpair (__domain : :: std :: os :: raw :: c_int , __type : :: std :: os :: raw :: c_int , __protocol : :: std :: os :: raw :: c_int , __fds : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn bind (__fd : :: std :: os :: raw :: c_int , __addr : * const sockaddr , __len : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getsockname (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __len : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn connect (__fd : :: std :: os :: raw :: c_int , __addr : * const sockaddr , __len : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpeername (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __len : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn send (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn recv (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn sendto (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int , __addr : * const sockaddr , __addr_len : socklen_t) -> isize ;

}
 extern "C" {
pub fn recvfrom (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __n : usize , __flags : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __addr_len : * mut socklen_t) -> isize ;

}
 extern "C" {
pub fn sendmsg (__fd : :: std :: os :: raw :: c_int , __message : * const msghdr , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn sendmmsg (__fd : :: std :: os :: raw :: c_int , __vmessages : * mut mmsghdr , __vlen : :: std :: os :: raw :: c_uint , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn recvmsg (__fd : :: std :: os :: raw :: c_int , __message : * mut msghdr , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn recvmmsg (__fd : :: std :: os :: raw :: c_int , __vmessages : * mut mmsghdr , __vlen : :: std :: os :: raw :: c_uint , __flags : :: std :: os :: raw :: c_int , __tmo : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getsockopt (__fd : :: std :: os :: raw :: c_int , __level : :: std :: os :: raw :: c_int , __optname : :: std :: os :: raw :: c_int , __optval : * mut :: std :: os :: raw :: c_void , __optlen : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setsockopt (__fd : :: std :: os :: raw :: c_int , __level : :: std :: os :: raw :: c_int , __optname : :: std :: os :: raw :: c_int , __optval : * const :: std :: os :: raw :: c_void , __optlen : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn listen (__fd : :: std :: os :: raw :: c_int , __n : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn accept (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __addr_len : * mut socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn accept4 (__fd : :: std :: os :: raw :: c_int , __addr : * mut sockaddr , __addr_len : * mut socklen_t , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn shutdown (__fd : :: std :: os :: raw :: c_int , __how : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sockatmark (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isfdtype (__fd : :: std :: os :: raw :: c_int , __fdtype : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn memfd_create (__name : * const :: std :: os :: raw :: c_char , __flags : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mlock2 (__addr : * const :: std :: os :: raw :: c_void , __length : usize , __flags : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_alloc (__flags : :: std :: os :: raw :: c_uint , __access_rights : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_set (__key : :: std :: os :: raw :: c_int , __access_rights : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_get (__key : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_free (__key : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pkey_mprotect (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int , __pkey : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mmap (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int , __fd : :: std :: os :: raw :: c_int , __offset : __off_t) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn mmap64 (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int , __fd : :: std :: os :: raw :: c_int , __offset : __off64_t) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn munmap (__addr : * mut :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mprotect (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __prot : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn msync (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn madvise (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __advice : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn posix_madvise (__addr : * mut :: std :: os :: raw :: c_void , __len : usize , __advice : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mlock (__addr : * const :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn munlock (__addr : * const :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mlockall (__flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn munlockall () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mincore (__start : * mut :: std :: os :: raw :: c_void , __len : usize , __vec : * mut :: std :: os :: raw :: c_uchar) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mremap (__addr : * mut :: std :: os :: raw :: c_void , __old_len : usize , __new_len : usize , __flags : :: std :: os :: raw :: c_int , ...) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn remap_file_pages (__start : * mut :: std :: os :: raw :: c_void , __size : usize , __prot : :: std :: os :: raw :: c_int , __pgoff : usize , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn shm_open (__name : * const :: std :: os :: raw :: c_char , __oflag : :: std :: os :: raw :: c_int , __mode : mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn shm_unlink (__name : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18process_post_loginP11vsf_session"] 
pub fn process_post_login (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_priv_parent_postloginP11vsf_session"] 
pub fn vsf_priv_parent_postlogin (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15init_connectionP11vsf_session"] 
pub fn init_connection (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_privop_get_ftp_port_sockP11vsf_sessionti"] 
pub fn vsf_privop_get_ftp_port_sock (p_sess : &vsf_session , remote_port : :: std :: os :: raw :: c_ushort , use_port_sockaddr : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_privop_pasv_cleanupP11vsf_session"] 
pub fn vsf_privop_pasv_cleanup (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_privop_pasv_listenP11vsf_session"] 
pub fn vsf_privop_pasv_listen (p_sess : &vsf_session) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_privop_pasv_activeP11vsf_session"] 
pub fn vsf_privop_pasv_active (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_privop_accept_pasvP11vsf_session"] 
pub fn vsf_privop_accept_pasv (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_privop_do_file_chownP11vsf_sessioni"] 
pub fn vsf_privop_do_file_chown (p_sess : &vsf_session , fd : :: std :: os :: raw :: c_int) ;

}
 pub const EVSFPrivopLoginResult_kVSFLoginNull : EVSFPrivopLoginResult = 0 ;
 pub const EVSFPrivopLoginResult_kVSFLoginFail : EVSFPrivopLoginResult = 1 ;
 pub const EVSFPrivopLoginResult_kVSFLoginAnon : EVSFPrivopLoginResult = 2 ;
 pub const EVSFPrivopLoginResult_kVSFLoginReal : EVSFPrivopLoginResult = 3 ;
 pub type EVSFPrivopLoginResult = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_privop_do_loginP11vsf_sessionPK5mystr"] 
pub fn vsf_privop_do_login (p_sess : &vsf_session , p_pass_str : * const mystr) -> EVSFPrivopLoginResult ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14priv_sock_initP11vsf_session"] 
pub fn priv_sock_init (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15priv_sock_closeP11vsf_session"] 
pub fn priv_sock_close (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28priv_sock_set_parent_contextP11vsf_session"] 
pub fn priv_sock_set_parent_context (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27priv_sock_set_child_contextP11vsf_session"] 
pub fn priv_sock_set_child_context (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18priv_sock_send_cmdic"] 
pub fn priv_sock_send_cmd (fd : :: std :: os :: raw :: c_int , cmd : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18priv_sock_send_striPK5mystr"] 
pub fn priv_sock_send_str (fd : :: std :: os :: raw :: c_int , p_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18priv_sock_send_bufiPKcj"] 
pub fn priv_sock_send_buf (fd : :: std :: os :: raw :: c_int , p_buf : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18priv_sock_recv_bufiPcj"] 
pub fn priv_sock_recv_buf (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20priv_sock_get_resulti"] 
pub fn priv_sock_get_result (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17priv_sock_get_cmdi"] 
pub fn priv_sock_get_cmd (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17priv_sock_get_striP5mystr"] 
pub fn priv_sock_get_str (fd : :: std :: os :: raw :: c_int , p_dest : &mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21priv_sock_send_resultic"] 
pub fn priv_sock_send_result (fd : :: std :: os :: raw :: c_int , res : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17priv_sock_send_fdii"] 
pub fn priv_sock_send_fd (fd : :: std :: os :: raw :: c_int , send_fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17priv_sock_recv_fdi"] 
pub fn priv_sock_recv_fd (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18priv_sock_send_intii"] 
pub fn priv_sock_send_int (fd : :: std :: os :: raw :: c_int , the_int : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17priv_sock_get_inti"] 
pub fn priv_sock_get_int (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 pub type ptrace_sandbox_validator_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : * mut pt_sandbox , arg2 : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int > ;
 extern "C" {
//# [link_name = "\u{1}_Z20ptrace_sandbox_allocv"] 
pub fn ptrace_sandbox_alloc () -> * mut pt_sandbox ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19ptrace_sandbox_freeP10pt_sandbox"] 
pub fn ptrace_sandbox_free (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29ptrace_sandbox_launch_processP10pt_sandboxPFvPvES1_"] 
pub fn ptrace_sandbox_launch_process (p_sandbox : * mut pt_sandbox , p_func : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut :: std :: os :: raw :: c_void ) > , p_arg : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_run_processesP10pt_sandbox"] 
pub fn ptrace_sandbox_run_processes (p_sandbox : * mut pt_sandbox) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29ptrace_sandbox_kill_processesP10pt_sandbox"] 
pub fn ptrace_sandbox_kill_processes (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22ptrace_sandbox_get_argP10pt_sandboxiPm"] 
pub fn ptrace_sandbox_get_arg (p_sandbox : * mut pt_sandbox , arg : :: std :: os :: raw :: c_int , p_out : * mut :: std :: os :: raw :: c_ulong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33ptrace_sandbox_get_socketcall_argP10pt_sandboxiPm"] 
pub fn ptrace_sandbox_get_socketcall_arg (p_sandbox : * mut pt_sandbox , arg : :: std :: os :: raw :: c_int , p_out : * mut :: std :: os :: raw :: c_ulong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23ptrace_sandbox_get_longP10pt_sandboxmPm"] 
pub fn ptrace_sandbox_get_long (p_sandbox : * mut pt_sandbox , ptr : :: std :: os :: raw :: c_ulong , p_out : * mut :: std :: os :: raw :: c_ulong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22ptrace_sandbox_get_bufP10pt_sandboxmmPv"] 
pub fn ptrace_sandbox_get_buf (p_sandbox : * mut pt_sandbox , ptr : :: std :: os :: raw :: c_ulong , len : :: std :: os :: raw :: c_ulong , p_buf : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_attach_pointv"] 
pub fn ptrace_sandbox_attach_point () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_exitP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_exit (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_readP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_read (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_writeP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_write (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31ptrace_sandbox_permit_sigactionP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_sigaction (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_alarmP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_alarm (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32ptrace_sandbox_permit_query_timeP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_query_time (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_mmapP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_mmap (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30ptrace_sandbox_permit_mprotectP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_mprotect (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32ptrace_sandbox_permit_file_statsP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_file_stats (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30ptrace_sandbox_permit_fd_statsP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_fd_stats (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_getcwdP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_getcwd (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_chdirP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_chdir (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_umaskP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_umask (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_openP10pt_sandboxi"] 
pub fn ptrace_sandbox_permit_open (p_sandbox : * mut pt_sandbox , writeable : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_closeP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_close (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30ptrace_sandbox_permit_getdentsP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_getdents (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_fcntlP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_fcntl (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30ptrace_sandbox_permit_sendfileP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_sendfile (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_seekP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_seek (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_selectP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_select (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_unlinkP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_unlink (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_mkdirP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_mkdir (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_rmdirP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_rmdir (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_renameP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_rename (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_utimeP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_utime (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31ptrace_sandbox_permit_sigreturnP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_sigreturn (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_recvP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_recv (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30ptrace_sandbox_permit_readlinkP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_readlink (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25ptrace_sandbox_permit_brkP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_brk (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_sleepP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_sleep (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_fchmodP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_fchmod (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27ptrace_sandbox_permit_chmodP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_chmod (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_fchownP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_fchown (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_mremapP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_mremap (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31ptrace_sandbox_permit_ftruncateP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_ftruncate (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_socketP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_socket (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z35ptrace_sandbox_set_socket_validatorP10pt_sandboxPFiS0_PvES1_"] 
pub fn ptrace_sandbox_set_socket_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26ptrace_sandbox_permit_bindP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_bind (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33ptrace_sandbox_set_bind_validatorP10pt_sandboxPFiS0_PvES1_"] 
pub fn ptrace_sandbox_set_bind_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29ptrace_sandbox_permit_connectP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_connect (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z36ptrace_sandbox_set_connect_validatorP10pt_sandboxPFiS0_PvES1_"] 
pub fn ptrace_sandbox_set_connect_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_listenP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_listen (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28ptrace_sandbox_permit_acceptP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_accept (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32ptrace_sandbox_permit_setsockoptP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_setsockopt (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z39ptrace_sandbox_set_setsockopt_validatorP10pt_sandboxPFiS0_PvES1_"] 
pub fn ptrace_sandbox_set_setsockopt_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32ptrace_sandbox_permit_getsockoptP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_getsockopt (p_sandbox : * mut pt_sandbox) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z39ptrace_sandbox_set_getsockopt_validatorP10pt_sandboxPFiS0_PvES1_"] 
pub fn ptrace_sandbox_set_getsockopt_validator (p_sandbox : * mut pt_sandbox , val : ptrace_sandbox_validator_t , p_arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30ptrace_sandbox_permit_shutdownP10pt_sandbox"] 
pub fn ptrace_sandbox_permit_shutdown (p_sandbox : * mut pt_sandbox) ;

}
 pub const EVSFRWTarget_kVSFRWControl : EVSFRWTarget = 1 ;
 pub const EVSFRWTarget_kVSFRWData : EVSFRWTarget = 2 ;
 pub type EVSFRWTarget = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z13ftp_write_strPK11vsf_sessionPK5mystr12EVSFRWTarget"] 
pub fn ftp_write_str (p_sess : * const vsf_session , p_str : * const mystr , target : EVSFRWTarget) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z13ftp_read_dataP11vsf_sessionPcj"] 
pub fn ftp_read_data (p_sess : &vsf_session , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14ftp_write_dataPK11vsf_sessionPKcj"] 
pub fn ftp_write_data (p_sess : * const vsf_session , p_buf : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11ftp_getlineP11vsf_sessionP5mystrPc"] 
pub fn ftp_getline (p_sess : &vsf_session , p_str : &mystr , p_buf : * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_secbuf_allocPPcj"] 
pub fn vsf_secbuf_alloc (p_ptr : * mut * mut :: std :: os :: raw :: c_char , size : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15vsf_secbuf_freePPc"] 
pub fn vsf_secbuf_free (p_ptr : * mut * mut :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20seccomp_sandbox_initv"] 
pub fn seccomp_sandbox_init () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30seccomp_sandbox_setup_preloginPK11vsf_session"] 
pub fn seccomp_sandbox_setup_prelogin (p_sess : * const vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31seccomp_sandbox_setup_postloginPK11vsf_session"] 
pub fn seccomp_sandbox_setup_postlogin (p_sess : * const vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z38seccomp_sandbox_setup_postlogin_brokerv"] 
pub fn seccomp_sandbox_setup_postlogin_broker () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24seccomp_sandbox_lockdownv"] 
pub fn seccomp_sandbox_lockdown () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_secutil_change_credentialsPK5mystrS1_S1_jj"] 
pub fn vsf_secutil_change_credentials (p_user_str : * const mystr , p_dir_str : * const mystr , p_ext_dir_str : * const mystr , caps : :: std :: os :: raw :: c_uint , options : :: std :: os :: raw :: c_uint) ;

}

 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mystr {
pub p_buf : * mut :: std :: os :: raw :: c_char ,
pub len : :: std :: os :: raw :: c_uint ,
pub alloc_bytes : :: std :: os :: raw :: c_uint ,
}

 # [test] fn bindgen_test_layout_mystr () {
assert_eq ! (:: std :: mem :: size_of :: < mystr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( mystr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mystr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mystr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr > ( ) ) ) . p_buf as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mystr ) , "::" , stringify ! ( p_buf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr > ( ) ) ) . len as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( mystr ) , "::" , stringify ! ( len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr > ( ) ) ) . alloc_bytes as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( mystr ) , "::" , stringify ! ( alloc_bytes ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26private_str_alloc_memchunkP5mystrPKcj"] 
pub fn private_str_alloc_memchunk (p_str : &mystr , p_src : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_alloc_textP5mystrPKc"] 
pub fn str_alloc_text (p_str : &mystr , p_src : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18str_alloc_alt_termP5mystrPKcc"] 
pub fn str_alloc_alt_term (p_str : &mystr , p_src : * const :: std :: os :: raw :: c_char , term : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_alloc_ulongP5mystrm"] 
pub fn str_alloc_ulong (p_str : &mystr , the_ulong : :: std :: os :: raw :: c_ulong) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20str_alloc_filesize_tP5mystrx"] 
pub fn str_alloc_filesize_t (p_str : &mystr , the_filesize : filesize_t) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_copyP5mystrPKS_"] 
pub fn str_copy (p_dest : &mystr , p_src : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_strdupPK5mystr"] 
pub fn str_strdup (p_str : * const mystr) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_emptyP5mystr"] 
pub fn str_empty (p_str : &mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_freeP5mystr"] 
pub fn str_free (p_str : &mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_truncP5mystrj"] 
pub fn str_trunc (p_str : &mystr , trunc_len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11str_reserveP5mystrj"] 
pub fn str_reserve (p_str : &mystr , res_len : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11str_isemptyPK5mystr"] 
pub fn str_isempty (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_getlenPK5mystr"] 
pub fn str_getlen (p_str : * const mystr) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_getbufPK5mystr"] 
pub fn str_getbuf (p_str : * const mystr) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_strcmpPK5mystrS1_"] 
pub fn str_strcmp (p_str1 : * const mystr , p_str2 : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_equalPK5mystrS1_"] 
pub fn str_equal (p_str1 : * const mystr , p_str2 : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_equal_textPK5mystrPKc"] 
pub fn str_equal_text (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_append_strP5mystrPKS_"] 
pub fn str_append_str (p_str : &mystr , p_other : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_append_textP5mystrPKc"] 
pub fn str_append_text (p_str : &mystr , p_src : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16str_append_ulongP5mystrm"] 
pub fn str_append_ulong (p_str : &mystr , the_long : :: std :: os :: raw :: c_ulong) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21str_append_filesize_tP5mystrx"] 
pub fn str_append_filesize_t (p_str : &mystr , the_filesize : filesize_t) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_append_charP5mystrc"] 
pub fn str_append_char (p_str : &mystr , the_char : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17str_append_doubleP5mystrd"] 
pub fn str_append_double (p_str : &mystr , the_double : f64) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_upperP5mystr"] 
//pub fn str_upper (p_str : &mystr) ;
pub fn str_upper (p_str : &mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_rpadP5mystrj"] 
pub fn str_rpad (p_str : &mystr , min_width : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_lpadP5mystrj"] 
pub fn str_lpad (p_str : &mystr , min_width : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16str_replace_charP5mystrcc"] 
pub fn str_replace_char (p_str : &mystr , from : :: std :: os :: raw :: c_char , to : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16str_replace_textP5mystrPKcS2_"] 
pub fn str_replace_text (p_str : &mystr , p_from : * const :: std :: os :: raw :: c_char , p_to : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_split_charP5mystrS0_c"] 
pub fn str_split_char (p_src : &mystr , p_rhs : &mystr , c : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22str_split_char_reverseP5mystrS0_c"] 
pub fn str_split_char_reverse (p_src : &mystr , p_rhs : &mystr , c : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_split_textP5mystrS0_PKc"] 
pub fn str_split_text (p_src : &mystr , p_rhs : &mystr , p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22str_split_text_reverseP5mystrS0_PKc"] 
pub fn str_split_text_reverse (p_src : &mystr , p_rhs : &mystr , p_text : * const :: std :: os :: raw :: c_char) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct str_locate_result {
pub found : :: std :: os :: raw :: c_int , pub index : :: std :: os :: raw :: c_uint , pub char_found : :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_str_locate_result () {
assert_eq ! (:: std :: mem :: size_of :: < str_locate_result > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( str_locate_result ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < str_locate_result > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( str_locate_result ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < str_locate_result > ( ) ) ) . found as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( str_locate_result ) , "::" , stringify ! ( found ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < str_locate_result > ( ) ) ) . index as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( str_locate_result ) , "::" , stringify ! ( index ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < str_locate_result > ( ) ) ) . char_found as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( str_locate_result ) , "::" , stringify ! ( char_found ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_locate_charPK5mystrc"] 
pub fn str_locate_char (p_str : * const mystr , look_char : :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_locate_strPK5mystrS1_"] 
pub fn str_locate_str (p_str : * const mystr , p_look_str : * const mystr) -> str_locate_result ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22str_locate_str_reversePK5mystrS1_"] 
pub fn str_locate_str_reverse (p_str : * const mystr , p_look_str : * const mystr) -> str_locate_result ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_locate_textPK5mystrPKc"] 
pub fn str_locate_text (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23str_locate_text_reversePK5mystrPKc"] 
pub fn str_locate_text_reverse (p_str : * const mystr , p_text : * const :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16str_locate_charsPK5mystrPKc"] 
pub fn str_locate_chars (p_str : * const mystr , p_chars : * const :: std :: os :: raw :: c_char) -> str_locate_result ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_leftPK5mystrPS_j"] 
pub fn str_left (p_str : * const mystr , p_out : &mystr , chars : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_rightPK5mystrPS_j"] 
pub fn str_right (p_str : * const mystr , p_out : &mystr , chars : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_mid_to_endPK5mystrPS_j"] 
pub fn str_mid_to_end (p_str : * const mystr , p_out : &mystr , indexx : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_get_char_atPK5mystrj"] 
pub fn str_get_char_at (p_str : * const mystr , indexx : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18str_contains_spacePK5mystr"] 
pub fn str_contains_space (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z13str_all_spacePK5mystr"] 
pub fn str_all_space (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24str_contains_unprintablePK5mystr"] 
pub fn str_contains_unprintable (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23str_replace_unprintableP5mystrc"] 
pub fn str_replace_unprintable (p_str : &mystr , new_char : :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_atoiPK5mystr"] 
pub fn str_atoi (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19str_a_to_filesize_tPK5mystr"] 
pub fn str_a_to_filesize_t (p_str : * const mystr) -> filesize_t ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17str_octal_to_uintPK5mystr"] 
pub fn str_octal_to_uint (p_str : * const mystr) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11str_getlinePK5mystrPS_Pj"] 
pub fn str_getline (p_str : * const mystr , p_line_str : &mystr , p_pos : * mut :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17str_contains_linePK5mystrS1_"] 
pub fn str_contains_line (p_str : * const mystr , p_line_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_session {
pub p_local_addr : * mut vsf_sysutil_sockaddr ,
pub p_remote_addr : * mut vsf_sysutil_sockaddr ,
pub p_control_line_buf : * mut :: std :: os :: raw :: c_char ,
pub idle_timeout :  :: std :: os :: raw :: c_int ,
pub data_timeout :  :: std :: os :: raw :: c_int ,
pub pasv_listen_fd :  :: std :: os :: raw :: c_int ,
pub p_port_sockaddr : * mut vsf_sysutil_sockaddr ,
pub data_fd :  :: std :: os :: raw :: c_int ,
pub data_progress :  :: std :: os :: raw :: c_int ,
pub bw_rate_max :  :: std :: os :: raw :: c_uint ,
pub bw_send_start_sec :  :: std :: os :: raw :: c_long ,
pub bw_send_start_usec :  :: std :: os :: raw :: c_long ,
pub is_anonymous : :: std  :: os :: raw :: c_int ,
pub is_guest : :: std :: os :: raw :: c_int ,
pub user_str :  mystr ,
pub anon_pass_str :  mystr ,
pub restart_pos :  filesize_t ,
 pub is_ascii :  :: std :: os :: raw :: c_int ,
 pub rnfr_filename_str :  mystr ,
 pub abor_received :  :: std :: os :: raw :: c_int ,
 pub epsv_all :  :: std :: os :: raw :: c_int ,
 pub is_http :  :: std :: os :: raw :: c_int ,
 pub http_get_arg : mystr ,
 pub p_visited_dir_list : *mut mystr_list ,
 pub anon_ftp_uid :  :: std :: os :: raw :: c_int ,
 pub guest_user_uid :  :: std :: os :: raw :: c_int ,
 pub anon_upload_chown_uid :  :: std :: os :: raw :: c_int ,
 pub banned_email_str :  mystr ,
 pub email_passwords_str :  mystr ,
 pub userlist_str :  mystr ,
 pub banner_str :  mystr ,
 pub tcp_wrapper_ok :  :: std :: os :: raw :: c_int ,
 pub xferlog_fd :  :: std :: os :: raw :: c_int ,
 pub vsftpd_log_fd :  :: std :: os :: raw :: c_int ,
 pub remote_ip_str :  mystr ,
 pub log_type :  :: std :: os :: raw :: c_ulong ,
 pub log_start_sec :  :: std :: os :: raw :: c_long ,
 pub log_start_usec :  :: std :: os :: raw :: c_long ,
 pub log_str :  mystr ,
 pub transfer_size :  filesize_t ,
 pub ftp_cmd_str :  mystr ,
 pub ftp_arg_str :   mystr ,
 pub parent_fd :   :: std :: os :: raw :: c_int ,
 pub child_fd :   :: std :: os :: raw :: c_int ,
 pub num_clients :   :: std :: os :: raw :: c_uint ,
 pub num_this_ip :   :: std :: os :: raw :: c_uint ,
 pub home_str :   mystr ,
 pub control_use_ssl :   :: std :: os :: raw :: c_int ,
 pub data_use_ssl : :: std :: os :: raw :: c_int ,
 pub p_ssl_ctx : *  mut  :: std :: os :: raw :: c_void ,
 pub p_control_ssl : *  mut  :: std :: os :: raw :: c_void ,
 pub p_data_ssl : * mut   :: std :: os :: raw :: c_void ,
 pub control_cert_digest :   mystr ,
 pub ssl_slave_active :   :: std :: os :: raw :: c_int ,
 pub ssl_slave_fd :   :: std :: os :: raw :: c_int ,
 pub ssl_consumer_fd :   :: std :: os :: raw :: c_int ,
 pub login_fails :   :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_vsf_session () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_session > ( ) , 480usize , concat ! ( "Size of: " , stringify ! ( vsf_session ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_session > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( vsf_session ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_local_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_local_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_remote_addr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_remote_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_control_line_buf as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_control_line_buf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . idle_timeout as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( idle_timeout ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_timeout as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_timeout ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . pasv_listen_fd as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( pasv_listen_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_port_sockaddr as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_port_sockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_fd as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_progress as * const _ as usize } , 52usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_progress ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . bw_rate_max as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( bw_rate_max ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . bw_send_start_sec as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( bw_send_start_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . bw_send_start_usec as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( bw_send_start_usec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_anonymous as * const _ as usize } , 80usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_anonymous ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_guest as * const _ as usize } , 84usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_guest ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . user_str as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( user_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . anon_pass_str as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( anon_pass_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . restart_pos as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( restart_pos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_ascii as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_ascii ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . rnfr_filename_str as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( rnfr_filename_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . abor_received as * const _ as usize } , 152usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( abor_received ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . epsv_all as * const _ as usize } , 156usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( epsv_all ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . is_http as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( is_http ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . http_get_arg as * const _ as usize } , 168usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( http_get_arg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_visited_dir_list as * const _ as usize } , 184usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_visited_dir_list ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . anon_ftp_uid as * const _ as usize } , 192usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( anon_ftp_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . guest_user_uid as * const _ as usize } , 196usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( guest_user_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . anon_upload_chown_uid as * const _ as usize } , 200usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( anon_upload_chown_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . banned_email_str as * const _ as usize } , 208usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( banned_email_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . email_passwords_str as * const _ as usize } , 224usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( email_passwords_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . userlist_str as * const _ as usize } , 240usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( userlist_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . banner_str as * const _ as usize } , 256usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( banner_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . tcp_wrapper_ok as * const _ as usize } , 272usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( tcp_wrapper_ok ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . xferlog_fd as * const _ as usize } , 276usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( xferlog_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . vsftpd_log_fd as * const _ as usize } , 280usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( vsftpd_log_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . remote_ip_str as * const _ as usize } , 288usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( remote_ip_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_type as * const _ as usize } , 304usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_start_sec as * const _ as usize } , 312usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_start_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_start_usec as * const _ as usize } , 320usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_start_usec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . log_str as * const _ as usize } , 328usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( log_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . transfer_size as * const _ as usize } , 344usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( transfer_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ftp_cmd_str as * const _ as usize } , 352usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ftp_cmd_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ftp_arg_str as * const _ as usize } , 368usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ftp_arg_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . parent_fd as * const _ as usize } , 384usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( parent_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . child_fd as * const _ as usize } , 388usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( child_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . num_clients as * const _ as usize } , 392usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( num_clients ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . num_this_ip as * const _ as usize } , 396usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( num_this_ip ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . home_str as * const _ as usize } , 400usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( home_str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . control_use_ssl as * const _ as usize } , 416usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( control_use_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . data_use_ssl as * const _ as usize } , 420usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( data_use_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_ssl_ctx as * const _ as usize } , 424usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_ssl_ctx ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_control_ssl as * const _ as usize } , 432usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_control_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . p_data_ssl as * const _ as usize } , 440usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( p_data_ssl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . control_cert_digest as * const _ as usize } , 448usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( control_cert_digest ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ssl_slave_active as * const _ as usize } , 464usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ssl_slave_active ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ssl_slave_fd as * const _ as usize } , 468usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ssl_slave_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . ssl_consumer_fd as * const _ as usize } , 472usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( ssl_consumer_fd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_session > ( ) ) ) . login_fails as * const _ as usize } , 476usize , concat ! ( "Offset of field: " , stringify ! ( vsf_session ) , "::" , stringify ! ( login_fails ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8ssl_readP11vsf_sessionPvPcj"] 
pub fn ssl_read (p_sess : &vsf_session , p_ssl : * mut :: std :: os :: raw :: c_void , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8ssl_peekP11vsf_sessionPvPcj"] 
pub fn ssl_peek (p_sess : &vsf_session , p_ssl : * mut :: std :: os :: raw :: c_void , p_buf : * mut :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9ssl_writePvPKcj"] 
pub fn ssl_write (p_ssl : * mut :: std :: os :: raw :: c_void , p_buf : * const :: std :: os :: raw :: c_char , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z13ssl_write_strPvPK5mystr"] 
pub fn ssl_write_str (p_ssl : * mut :: std :: os :: raw :: c_void , p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17ssl_read_into_strP11vsf_sessionPvP5mystr"] 
pub fn ssl_read_into_str (p_sess : &vsf_session , p_ssl : * mut :: std :: os :: raw :: c_void , p_str : &mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8ssl_initP11vsf_session"] 
pub fn ssl_init (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10ssl_acceptP11vsf_sessioni"] 
pub fn ssl_accept (p_sess : &vsf_session , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14ssl_data_closeP11vsf_session"] 
pub fn ssl_data_close (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21ssl_comm_channel_initP11vsf_session"] 
pub fn ssl_comm_channel_init (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z37ssl_comm_channel_set_consumer_contextP11vsf_session"] 
pub fn ssl_comm_channel_set_consumer_context (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z37ssl_comm_channel_set_producer_contextP11vsf_session"] 
pub fn ssl_comm_channel_set_producer_context (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11handle_authP11vsf_session"] 
pub fn handle_auth (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11handle_pbszP11vsf_session"] 
pub fn handle_pbsz (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11handle_protP11vsf_session"] 
pub fn handle_prot (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21ssl_control_handshakeP11vsf_session"] 
pub fn ssl_control_handshake (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15ssl_add_entropyP11vsf_session"] 
pub fn ssl_add_entropy (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9ssl_slaveP11vsf_session"] 
pub fn ssl_slave (p_sess : &vsf_session) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_client_launch {
pub num_children : :: std :: os :: raw :: c_uint , pub num_this_ip : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_vsf_client_launch () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_client_launch > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( vsf_client_launch ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_client_launch > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_client_launch ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_client_launch > ( ) ) ) . num_children as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_client_launch ) , "::" , stringify ! ( num_children ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_client_launch > ( ) ) ) . num_this_ip as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( vsf_client_launch ) , "::" , stringify ! ( num_this_ip ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_standalone_mainv"] 
pub fn vsf_standalone_main () -> vsf_client_launch ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mystr_list {
pub alloc_len : :: std :: os :: raw :: c_uint,
pub list_len : :: std :: os :: raw :: c_uint ,
pub p_nodes : *mut mystr_list_node ,
}
 # [test] fn bindgen_test_layout_mystr_list () {
assert_eq ! (:: std :: mem :: size_of :: < mystr_list > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( mystr_list ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mystr_list > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mystr_list ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list > ( ) ) ) . alloc_len as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list ) , "::" , stringify ! ( alloc_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list > ( ) ) ) . list_len as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list ) , "::" , stringify ! ( list_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list > ( ) ) ) . p_nodes as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list ) , "::" , stringify ! ( p_nodes ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z13str_list_freeP10mystr_list"] 
pub fn str_list_free (p_list : &mystr_list) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z12str_list_addP10mystr_listPK5mystrS3_"] 
pub fn str_list_add (p_list : &mystr_list , p_str : * const mystr , p_sort_key_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z13str_list_sortP10mystr_listi"] 
pub fn str_list_sort (p_list : &mystr_list , reverse : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19str_list_get_lengthPK10mystr_list"] 
pub fn str_list_get_length (p_list : * const mystr_list) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21str_list_contains_strPK10mystr_listPK5mystr"] 
pub fn str_list_contains_str (p_list : * const mystr_list , p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17str_list_get_pstrPK10mystr_listj"] 
pub fn str_list_get_pstr (p_list : * const mystr_list , indexx : :: std :: os :: raw :: c_uint) -> * const mystr ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysdep_check_authP5mystrPKS_S2_"] 
pub fn vsf_sysdep_check_auth (p_user : &mystr , p_pass : * const mystr , p_remote_host : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysdep_has_capabilitiesv"] 
pub fn vsf_sysdep_has_capabilities () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z39vsf_sysdep_has_capabilities_as_non_rootv"] 
pub fn vsf_sysdep_has_capabilities_as_non_root () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysdep_keep_capabilitiesv"] 
pub fn vsf_sysdep_keep_capabilities () ;

}
 pub const ESysdepCapabilities_kCapabilityCAP_CHOWN : ESysdepCapabilities = 1 ;
 pub const ESysdepCapabilities_kCapabilityCAP_NET_BIND_SERVICE : ESysdepCapabilities = 2 ;
 pub type ESysdepCapabilities = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysdep_adopt_capabilitiesj"] 
pub fn vsf_sysdep_adopt_capabilities (caps : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_sendfileiiPxxj"] 
pub fn vsf_sysutil_sendfile (out_fd : :: std :: os :: raw :: c_int , in_fd : :: std :: os :: raw :: c_int , p_offset : * mut filesize_t , num_send : filesize_t , max_chunk : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_setproctitle_initiPPKc"] 
pub fn vsf_sysutil_setproctitle_init (argc : :: std :: os :: raw :: c_int , argv : * mut * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_setproctitlePKc"] 
pub fn vsf_sysutil_setproctitle (p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_setproctitle_strPK5mystr"] 
pub fn vsf_sysutil_setproctitle_str (p_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32vsf_sysutil_set_proctitle_prefixPK5mystr"] 
pub fn vsf_sysutil_set_proctitle_prefix (p_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_map_anon_pages_initv"] 
pub fn vsf_sysutil_map_anon_pages_init () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_map_anon_pagesj"] 
pub fn vsf_sysutil_map_anon_pages (length : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_send_fdii"] 
pub fn vsf_sysutil_send_fd (sock_fd : :: std :: os :: raw :: c_int , send_fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_recv_fdi"] 
pub fn vsf_sysutil_recv_fd (sock_fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_set_die_if_parent_diesv"] 
pub fn vsf_set_die_if_parent_dies () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_set_term_if_parent_diesv"] 
pub fn vsf_set_term_if_parent_dies () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_fork_isolate_failokv"] 
pub fn vsf_sysutil_fork_isolate_failok () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z35vsf_sysutil_fork_isolate_all_failokv"] 
pub fn vsf_sysutil_fork_isolate_all_failok () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_fork_newnetv"] 
pub fn vsf_sysutil_fork_newnet () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_getpid_nocachev"] 
pub fn vsf_sysutil_getpid_nocache () -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_statbuf {
_unused : [u8 ; 0] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_user {
_unused : [u8 ; 0] ,
}
 extern "C" {
//# [link_name = "\u{1}_Z10str_getcwdP5mystr"] 
pub fn str_getcwd (p_str : &mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z12str_readlinkP5mystrPKS_"] 
pub fn str_readlink (p_str : &mystr , p_filename_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z14str_write_loopPK5mystri"] 
pub fn str_write_loop (p_str : * const mystr , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z13str_read_loopP5mystri"] 
pub fn str_read_loop (p_str : &mystr , fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_mkdirPK5mystrj"] 
pub fn str_mkdir (p_str : * const mystr , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_rmdirPK5mystr"] 
pub fn str_rmdir (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_unlinkPK5mystr"] 
pub fn str_unlink (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_chdirPK5mystr"] 
pub fn str_chdir (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 pub const EVSFSysStrOpenMode_kVSFSysStrOpenUnknown : EVSFSysStrOpenMode = 0 ;
 pub const EVSFSysStrOpenMode_kVSFSysStrOpenReadOnly : EVSFSysStrOpenMode = 1 ;
 pub type EVSFSysStrOpenMode = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z8str_openPK5mystr18EVSFSysStrOpenMode"] 
pub fn str_open (p_str : * const mystr , mode : EVSFSysStrOpenMode) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_createPK5mystr"] 
pub fn str_create (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20str_create_exclusivePK5mystr"] 
pub fn str_create_exclusive (p_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_chmodPK5mystrj"] 
pub fn str_chmod (p_str : * const mystr , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8str_statPK5mystrPP19vsf_sysutil_statbuf"] 
pub fn str_stat (p_str : * const mystr , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z9str_lstatPK5mystrPP19vsf_sysutil_statbuf"] 
pub fn str_lstat (p_str : * const mystr , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_renamePK5mystrS1_"] 
pub fn str_rename (p_from_str : * const mystr , p_to_str : * const mystr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z11str_opendirPK5mystr"] 
pub fn str_opendir (p_str : * const mystr) -> * mut vsf_sysutil_dir ;

}
 extern "C" {
//# [link_name = "\u{1}_Z15str_next_direntP5mystrP15vsf_sysutil_dir"] 
pub fn str_next_dirent (p_filename_str : &mystr , p_dir : * mut vsf_sysutil_dir) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z12str_getpwnamPK5mystr"] 
pub fn str_getpwnam (p_user_str : * const mystr) -> * mut vsf_sysutil_user ;

}
 extern "C" {
//# [link_name = "\u{1}_Z10str_syslogPK5mystri"] 
pub fn str_syslog (p_str : * const mystr , severe : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_retval_is_errori"] 
pub fn vsf_sysutil_retval_is_error (retval : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 pub const EVSFSysUtilError_kVSFSysUtilErrUnknown : EVSFSysUtilError = 1 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrADDRINUSE : EVSFSysUtilError = 2 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrNOSYS : EVSFSysUtilError = 3 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrINTR : EVSFSysUtilError = 4 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrINVAL : EVSFSysUtilError = 5 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrOPNOTSUPP : EVSFSysUtilError = 6 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrACCES : EVSFSysUtilError = 7 ;
 pub const EVSFSysUtilError_kVSFSysUtilErrNOENT : EVSFSysUtilError = 8 ;
 pub type EVSFSysUtilError = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_get_errorv"] 
pub fn vsf_sysutil_get_error () -> EVSFSysUtilError ;

}
 pub const EVSFSysUtilSignal_kVSFSysUtilSigALRM : EVSFSysUtilSignal = 1 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigTERM : EVSFSysUtilSignal = 2 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigCHLD : EVSFSysUtilSignal = 3 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigPIPE : EVSFSysUtilSignal = 4 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigURG : EVSFSysUtilSignal = 5 ;
 pub const EVSFSysUtilSignal_kVSFSysUtilSigHUP : EVSFSysUtilSignal = 6 ;
 pub type EVSFSysUtilSignal = u32 ;
 pub const EVSFSysUtilInterruptContext_kVSFSysUtilUnknown : EVSFSysUtilInterruptContext = 0 ;
 pub const EVSFSysUtilInterruptContext_kVSFSysUtilIO : EVSFSysUtilInterruptContext = 1 ;
 pub type EVSFSysUtilInterruptContext = u32 ;
 pub type vsf_sighandle_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : * mut :: std :: os :: raw :: c_void) > ;
 pub type vsf_async_sighandle_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int) > ;
 pub type vsf_context_io_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int , arg2 : :: std :: os :: raw :: c_int , arg3 : * mut :: std :: os :: raw :: c_void) > ;
 extern "C" {
//# [link_name = "\u{1}_Z35vsf_sysutil_install_null_sighandler17EVSFSysUtilSignal"] 
pub fn vsf_sysutil_install_null_sighandler (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_install_sighandler17EVSFSysUtilSignalPFvPvES0_i"] 
pub fn vsf_sysutil_install_sighandler (arg1 : EVSFSysUtilSignal , handler : vsf_sighandle_t , p_private : * mut :: std :: os :: raw :: c_void , use_alarm : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z36vsf_sysutil_install_async_sighandler17EVSFSysUtilSignalPFviE"] 
pub fn vsf_sysutil_install_async_sighandler (sig : EVSFSysUtilSignal , handler : vsf_async_sighandle_t) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_default_sig17EVSFSysUtilSignal"] 
pub fn vsf_sysutil_default_sig (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_install_io_handlerPFviiPvES_"] 
pub fn vsf_sysutil_install_io_handler (handler : vsf_context_io_t , p_private : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32vsf_sysutil_uninstall_io_handlerv"] 
pub fn vsf_sysutil_uninstall_io_handler () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33vsf_sysutil_check_pending_actions27EVSFSysUtilInterruptContextii"] 
pub fn vsf_sysutil_check_pending_actions (context : EVSFSysUtilInterruptContext , retval : :: std :: os :: raw :: c_int , fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_block_sig17EVSFSysUtilSignal"] 
pub fn vsf_sysutil_block_sig (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_unblock_sig17EVSFSysUtilSignal"] 
pub fn vsf_sysutil_unblock_sig (sig : EVSFSysUtilSignal) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_set_alarmj"] 
pub fn vsf_sysutil_set_alarm (trigger_seconds : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_clear_alarmv"] 
pub fn vsf_sysutil_clear_alarm () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_getcwdPcj"] 
pub fn vsf_sysutil_getcwd (p_dest : * mut :: std :: os :: raw :: c_char , buf_size : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_mkdirPKcj"] 
pub fn vsf_sysutil_mkdir (p_dirname : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_rmdirPKc"] 
pub fn vsf_sysutil_rmdir (p_dirname : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_chdirPKc"] 
pub fn vsf_sysutil_chdir (p_dirname : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_renamePKcS0_"] 
pub fn vsf_sysutil_rename (p_from : * const :: std :: os :: raw :: c_char , p_to : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_opendirPKc"] 
pub fn vsf_sysutil_opendir (p_dirname : * const :: std :: os :: raw :: c_char) -> * mut vsf_sysutil_dir ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_closedirP15vsf_sysutil_dir"] 
pub fn vsf_sysutil_closedir (p_dir : * mut vsf_sysutil_dir) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_next_direntP15vsf_sysutil_dir"] 
pub fn vsf_sysutil_next_dirent (p_dir : * mut vsf_sysutil_dir) -> * const :: std :: os :: raw :: c_char ;

}
 pub const EVSFSysUtilOpenMode_kVSFSysUtilOpenReadOnly : EVSFSysUtilOpenMode = 1 ;
 pub const EVSFSysUtilOpenMode_kVSFSysUtilOpenWriteOnly : EVSFSysUtilOpenMode = 2 ;
 pub const EVSFSysUtilOpenMode_kVSFSysUtilOpenReadWrite : EVSFSysUtilOpenMode = 3 ;
 pub type EVSFSysUtilOpenMode = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_open_filePKc19EVSFSysUtilOpenMode"] 
pub fn vsf_sysutil_open_file (p_filename : * const :: std :: os :: raw :: c_char , arg1 : EVSFSysUtilOpenMode) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33vsf_sysutil_create_file_exclusivePKc"] 
pub fn vsf_sysutil_create_file_exclusive (p_filename : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z38vsf_sysutil_create_or_open_file_appendPKcj"] 
pub fn vsf_sysutil_create_or_open_file_append (p_filename : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_create_or_open_filePKcj"] 
pub fn vsf_sysutil_create_or_open_file (p_filename : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_dupfd2ii"] 
pub fn vsf_sysutil_dupfd2 (old_fd : :: std :: os :: raw :: c_int , new_fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_closei"] 
pub fn vsf_sysutil_close (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_close_failoki"] 
pub fn vsf_sysutil_close_failok (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_unlinkPKc"] 
pub fn vsf_sysutil_unlink (p_dead : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_write_accessPKc"] 
pub fn vsf_sysutil_write_access (p_filename : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_ftruncatei"] 
pub fn vsf_sysutil_ftruncate (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_lseek_toix"] 
pub fn vsf_sysutil_lseek_to (fd : :: std :: os :: raw :: c_int , seek_pos : filesize_t) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_lseek_endi"] 
pub fn vsf_sysutil_lseek_end (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_get_file_offseti"] 
pub fn vsf_sysutil_get_file_offset (file_fd : :: std :: os :: raw :: c_int) -> filesize_t ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_readiPvj"] 
pub fn vsf_sysutil_read (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_writeiPKvj"] 
pub fn vsf_sysutil_write (fd : :: std :: os :: raw :: c_int , p_buf : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_read_loopiPvj"] 
pub fn vsf_sysutil_read_loop (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_write_loopiPKvj"] 
pub fn vsf_sysutil_write_loop (fd : :: std :: os :: raw :: c_int , p_buf : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_statPKcPP19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_stat (p_name : * const :: std :: os :: raw :: c_char , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_lstatPKcPP19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_lstat (p_name : * const :: std :: os :: raw :: c_char , p_ptr : * mut * mut vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_fstatiPP19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_fstat (fd : :: std :: os :: raw :: c_int , p_ptr : * mut * mut vsf_sysutil_statbuf) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_dir_statPK15vsf_sysutil_dirPP19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_dir_stat (p_dir : * const vsf_sysutil_dir , p_ptr : * mut * mut vsf_sysutil_statbuf) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_statbuf_is_regfilePK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_is_regfile (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_statbuf_is_symlinkPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_is_symlink (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_statbuf_is_socketPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_is_socket (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_statbuf_is_dirPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_is_dir (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_statbuf_get_sizePK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_get_size (p_stat : * const vsf_sysutil_statbuf) -> filesize_t ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_statbuf_get_permsPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_get_perms (p_stat : * const vsf_sysutil_statbuf) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_statbuf_get_datePK19vsf_sysutil_statbufil"] 
pub fn vsf_sysutil_statbuf_get_date (p_stat : * const vsf_sysutil_statbuf , use_localtime : :: std :: os :: raw :: c_int , curr_time : :: std :: os :: raw :: c_long) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z36vsf_sysutil_statbuf_get_numeric_datePK19vsf_sysutil_statbufi"] 
pub fn vsf_sysutil_statbuf_get_numeric_date (p_stat : * const vsf_sysutil_statbuf , use_localtime : :: std :: os :: raw :: c_int) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_statbuf_get_linksPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_get_links (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_statbuf_get_uidPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_get_uid (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_statbuf_get_gidPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_get_gid (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z37vsf_sysutil_statbuf_is_readable_otherPK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_is_readable_other (p_stat : * const vsf_sysutil_statbuf) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z37vsf_sysutil_statbuf_get_sortkey_mtimePK19vsf_sysutil_statbuf"] 
pub fn vsf_sysutil_statbuf_get_sortkey_mtime (p_stat : * const vsf_sysutil_statbuf) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_chmodPKcj"] 
pub fn vsf_sysutil_chmod (p_filename : * const :: std :: os :: raw :: c_char , mode : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_fchowniii"] 
pub fn vsf_sysutil_fchown (fd : :: std :: os :: raw :: c_int , uid : :: std :: os :: raw :: c_int , gid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_fchmodij"] 
pub fn vsf_sysutil_fchmod (fd : :: std :: os :: raw :: c_int , mode : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_readlinkPKcPcj"] 
pub fn vsf_sysutil_readlink (p_filename : * const :: std :: os :: raw :: c_char , p_dest : * mut :: std :: os :: raw :: c_char , bufsiz : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_lock_file_writei"] 
pub fn vsf_sysutil_lock_file_write (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_lock_file_readi"] 
pub fn vsf_sysutil_lock_file_read (fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_unlock_filei"] 
pub fn vsf_sysutil_unlock_file (fd : :: std :: os :: raw :: c_int) ;

}
 pub const EVSFSysUtilMapPermission_kVSFSysUtilMapProtReadOnly : EVSFSysUtilMapPermission = 1 ;
 pub const EVSFSysUtilMapPermission_kVSFSysUtilMapProtNone : EVSFSysUtilMapPermission = 2 ;
 pub type EVSFSysUtilMapPermission = u32 ;
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_memprotectPvj24EVSFSysUtilMapPermission"] 
pub fn vsf_sysutil_memprotect (p_addr : * mut :: std :: os :: raw :: c_void , len : :: std :: os :: raw :: c_uint , perm : EVSFSysUtilMapPermission) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_memunmapPvj"] 
pub fn vsf_sysutil_memunmap (p_start : * mut :: std :: os :: raw :: c_void , length : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_mallocj"] 
pub fn vsf_sysutil_malloc (size : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_reallocPvj"] 
pub fn vsf_sysutil_realloc (p_ptr : * mut :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_freePv"] 
pub fn vsf_sysutil_free (p_ptr : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_getpidv"] 
pub fn vsf_sysutil_getpid () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_post_forkv"] 
pub fn vsf_sysutil_post_fork () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_forkv"] 
pub fn vsf_sysutil_fork () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_fork_failokv"] 
pub fn vsf_sysutil_fork_failok () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_exiti"] 
pub fn vsf_sysutil_exit (exit_code : :: std :: os :: raw :: c_int) ;

}
 # [repr ( C )] # [derive ( Default, Debug , Copy , Clone )] pub struct vsf_sysutil_wait_retval {
pub PRIVATE_HANDS_OFF_syscall_retval : :: std :: os :: raw :: c_int , 
pub PRIVATE_HANDS_OFF_exit_status : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_vsf_sysutil_wait_retval () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_wait_retval > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_wait_retval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_wait_retval > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_wait_retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_wait_retval > ( ) ) ) . PRIVATE_HANDS_OFF_syscall_retval as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_wait_retval ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_syscall_retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_wait_retval > ( ) ) ) . PRIVATE_HANDS_OFF_exit_status as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_wait_retval ) , "::" , stringify ! ( PRIVATE_HANDS_OFF_exit_status ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_waitv"] 
pub fn vsf_sysutil_wait () -> vsf_sysutil_wait_retval ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_wait_reap_onev"] 
pub fn vsf_sysutil_wait_reap_one () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_wait_get_retvalPK23vsf_sysutil_wait_retval"] 
pub fn vsf_sysutil_wait_get_retval (p_waitret : * const vsf_sysutil_wait_retval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32vsf_sysutil_wait_exited_normallyPK23vsf_sysutil_wait_retval"] 
pub fn vsf_sysutil_wait_exited_normally (p_waitret : * const vsf_sysutil_wait_retval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_wait_get_exitcodePK23vsf_sysutil_wait_retval"] 
pub fn vsf_sysutil_wait_get_exitcode (p_waitret : * const vsf_sysutil_wait_retval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_strlenPKc"] 
pub fn vsf_sysutil_strlen (p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_strdupPKc"] 
pub fn vsf_sysutil_strdup (p_str : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_memclrPvj"] 
pub fn vsf_sysutil_memclr (p_dest : * mut :: std :: os :: raw :: c_void , s: usize) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_memcpyPvPKvj"] 
pub fn vsf_sysutil_memcpy (p_dest : * mut :: std :: os :: raw :: c_void , p_src : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_strcpyPcPKcj"] 
pub fn vsf_sysutil_strcpy (p_dest : * mut :: std :: os :: raw :: c_char , p_src : * const :: std :: os :: raw :: c_char , maxsize : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_memcmpPKvS0_j"] 
pub fn vsf_sysutil_memcmp (p_src1 : * const :: std :: os :: raw :: c_void , p_src2 : * const :: std :: os :: raw :: c_void , size : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_strcmpPKcS0_"] 
pub fn vsf_sysutil_strcmp (p_src1 : * const :: std :: os :: raw :: c_char , p_src2 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_atoiPKc"] 
pub fn vsf_sysutil_atoi (p_str : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_a_to_filesize_tPKc"] 
pub fn vsf_sysutil_a_to_filesize_t (p_str : * const :: std :: os :: raw :: c_char) -> filesize_t ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_ulong_to_strm"] 
pub fn vsf_sysutil_ulong_to_str (the_ulong : :: std :: os :: raw :: c_ulong) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_filesize_t_to_strx"] 
pub fn vsf_sysutil_filesize_t_to_str (the_filesize : filesize_t) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_double_to_strd"] 
pub fn vsf_sysutil_double_to_str (the_double : f64) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_uint_to_octalj"] 
pub fn vsf_sysutil_uint_to_octal (the_uint : :: std :: os :: raw :: c_uint) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_octal_to_uintPKc"] 
pub fn vsf_sysutil_octal_to_uint (p_str : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_toupperi"] 
pub fn vsf_sysutil_toupper (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_isspacei"] 
pub fn vsf_sysutil_isspace (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_isprinti"] 
pub fn vsf_sysutil_isprint (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_isalnumi"] 
pub fn vsf_sysutil_isalnum (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_isdigiti"] 
pub fn vsf_sysutil_isdigit (the_char : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Default,Debug , Copy , Clone )] pub struct vsf_sysutil_socketpair_retval {
pub socket_one : :: std :: os :: raw :: c_int , pub socket_two : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_vsf_sysutil_socketpair_retval () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_socketpair_retval > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_socketpair_retval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_socketpair_retval > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_socketpair_retval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_socketpair_retval > ( ) ) ) . socket_one as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_socketpair_retval ) , "::" , stringify ! ( socket_one ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_socketpair_retval > ( ) ) ) . socket_two as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_socketpair_retval ) , "::" , stringify ! ( socket_two ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_sockaddr_allocPP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_alloc (p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_sockaddr_clearPP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_clear (p_sockptr : &*mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_sockaddr_alloc_ipv4PP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_alloc_ipv4 (p_sockptr : && vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_sockaddr_alloc_ipv6PP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_alloc_ipv6 (p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_sockaddr_clonePP20vsf_sysutil_sockaddrPKS_"] 
pub fn vsf_sysutil_sockaddr_clone (p_sockptr : &&vsf_sysutil_sockaddr , p_src : * const vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_sockaddr_addr_equalPK20vsf_sysutil_sockaddrS1_"] 
pub fn vsf_sysutil_sockaddr_addr_equal (p1 : * const vsf_sysutil_sockaddr , p2 : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_is_ipv6PK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_is_ipv6 (p_sockaddr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33vsf_sysutil_sockaddr_set_ipv4addrP20vsf_sysutil_sockaddrPKh"] 
pub fn vsf_sysutil_sockaddr_set_ipv4addr (p_sockptr : * mut vsf_sysutil_sockaddr , p_raw : * const :: std :: os :: raw :: c_uchar) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33vsf_sysutil_sockaddr_set_ipv6addrP20vsf_sysutil_sockaddrPKh"] 
pub fn vsf_sysutil_sockaddr_set_ipv6addr (p_sockptr : * mut vsf_sysutil_sockaddr , p_raw : * const :: std :: os :: raw :: c_uchar) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_set_anyP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_set_any (p_sockaddr : * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_sockaddr_get_portPK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_get_port (p_sockptr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_sockaddr_set_portP20vsf_sysutil_sockaddrt"] 
pub fn vsf_sysutil_sockaddr_set_port (p_sockptr : * mut vsf_sysutil_sockaddr , the_port : :: std :: os :: raw :: c_ushort) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_is_port_reservedt"] 
pub fn vsf_sysutil_is_port_reserved (port : :: std :: os :: raw :: c_ushort) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_get_ipsockPK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_get_ipsock (p_sockaddr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_get_ipaddr_sizev"] 
pub fn vsf_sysutil_get_ipaddr_size () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z33vsf_sysutil_sockaddr_get_raw_addrP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_get_raw_addr (p_sockaddr : * mut vsf_sysutil_sockaddr) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_ipv6_v4PK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_ipv6_v4 (p_sockaddr : &vsf_sysutil_sockaddr) -> * const :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_sockaddr_ipv4_v6PK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_sockaddr_ipv4_v6 (p_sockaddr : * const vsf_sysutil_sockaddr) -> * const :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_get_ipv4_sockv"] 
pub fn vsf_sysutil_get_ipv4_sock () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_get_ipv6_sockv"] 
pub fn vsf_sysutil_get_ipv6_sock () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z34vsf_sysutil_unix_stream_socketpairv"] 
pub fn vsf_sysutil_unix_stream_socketpair () -> vsf_sysutil_socketpair_retval ;

}
 extern "C" {
//# [link_name = "\u{1}_Z16vsf_sysutil_bindiPK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_bind (fd : :: std :: os :: raw :: c_int , p_sockptr : * const vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_listenij"] 
pub fn vsf_sysutil_listen (fd : :: std :: os :: raw :: c_int , backlog : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_getsocknameiPP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_getsockname (fd : :: std :: os :: raw :: c_int , p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_getpeernameiPP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_getpeername (fd : :: std :: os :: raw :: c_int , p_sockptr : * mut * mut vsf_sysutil_sockaddr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_accept_timeoutiP20vsf_sysutil_sockaddrj"] 
pub fn vsf_sysutil_accept_timeout (fd : :: std :: os :: raw :: c_int , p_sockaddr : * mut vsf_sysutil_sockaddr , wait_seconds : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_connect_timeoutiPK20vsf_sysutil_sockaddrj"] 
pub fn vsf_sysutil_connect_timeout (fd : :: std :: os :: raw :: c_int , p_sockaddr : * const vsf_sysutil_sockaddr , wait_seconds : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_dns_resolvePP20vsf_sysutil_sockaddrPKc"] 
pub fn vsf_sysutil_dns_resolve (p_sockptr : * mut * mut vsf_sysutil_sockaddr , p_name : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_activate_keepalivei"] 
pub fn vsf_sysutil_activate_keepalive (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32vsf_sysutil_set_iptos_throughputi"] 
pub fn vsf_sysutil_set_iptos_throughput (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_activate_reuseaddri"] 
pub fn vsf_sysutil_activate_reuseaddr (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_set_nodelayi"] 
pub fn vsf_sysutil_set_nodelay (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_activate_sigurgi"] 
pub fn vsf_sysutil_activate_sigurg (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_activate_oobinlinei"] 
pub fn vsf_sysutil_activate_oobinline (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_activate_lingeri"] 
pub fn vsf_sysutil_activate_linger (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z36vsf_sysutil_deactivate_linger_failoki"] 
pub fn vsf_sysutil_deactivate_linger_failok (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_activate_noblocki"] 
pub fn vsf_sysutil_activate_noblock (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z30vsf_sysutil_deactivate_noblocki"] 
pub fn vsf_sysutil_deactivate_noblock (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_shutdown_failoki"] 
pub fn vsf_sysutil_shutdown_failok (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z32vsf_sysutil_shutdown_read_failoki"] 
pub fn vsf_sysutil_shutdown_read_failok (fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_recv_peekiPvj"] 
pub fn vsf_sysutil_recv_peek (fd : :: std :: os :: raw :: c_int , p_buf : * mut :: std :: os :: raw :: c_void , len : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_inet_ntopPK20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_inet_ntop (p_sockptr : & vsf_sysutil_sockaddr) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_inet_ntoaPKv"] 
pub fn vsf_sysutil_inet_ntoa (p_raw_addr : * const :: std :: os :: raw :: c_void) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_inet_atonPKcP20vsf_sysutil_sockaddr"] 
pub fn vsf_sysutil_inet_aton (p_text : * const :: std :: os :: raw :: c_char , p_addr : &vsf_sysutil_sockaddr) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_group {
_unused : [u8 ; 0] ,
}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_getpwuidi"] 
pub fn vsf_sysutil_getpwuid (uid : :: std :: os :: raw :: c_int) -> * mut vsf_sysutil_user ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_getpwnamPKc"] 
pub fn vsf_sysutil_getpwnam (p_user : * const :: std :: os :: raw :: c_char) -> * mut vsf_sysutil_user ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_user_getnamePK16vsf_sysutil_user"] 
pub fn vsf_sysutil_user_getname (p_user : * const vsf_sysutil_user) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_user_get_homedirPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_user_get_homedir (p_user : * const vsf_sysutil_user) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_user_getuidPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_user_getuid (p_user : * const vsf_sysutil_user) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_user_getgidPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_user_getgid (p_user : * const vsf_sysutil_user) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_getgrgidi"] 
pub fn vsf_sysutil_getgrgid (gid : :: std :: os :: raw :: c_int) -> * mut vsf_sysutil_group ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_group_getnamePK17vsf_sysutil_group"] 
pub fn vsf_sysutil_group_getname (p_group : * const vsf_sysutil_group) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z23vsf_sysutil_getpagesizev"] 
pub fn vsf_sysutil_getpagesize () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_get_random_bytev"] 
pub fn vsf_sysutil_get_random_byte () -> :: std :: os :: raw :: c_uchar ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_get_umaskv"] 
pub fn vsf_sysutil_get_umask () -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_sysutil_set_umaskj"] 
pub fn vsf_sysutil_set_umask (umask : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_make_session_leaderv"] 
pub fn vsf_sysutil_make_session_leader () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z31vsf_sysutil_reopen_standard_fdsv"] 
pub fn vsf_sysutil_reopen_standard_fds () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_tzsetv"] 
pub fn vsf_sysutil_tzset () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_sysutil_get_current_datev"] 
pub fn vsf_sysutil_get_current_date () -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_qsortPvjjPFiPKvS1_E"] 
pub fn vsf_sysutil_qsort (p_base : * mut :: std :: os :: raw :: c_void , num_elem : :: std :: os :: raw :: c_uint , elem_size : :: std :: os :: raw :: c_uint , p_compar : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * const :: std :: os :: raw :: c_void , arg2 : * const :: std :: os :: raw :: c_void ) -> :: std :: os :: raw :: c_int >) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_getenvPKc"] 
pub fn vsf_sysutil_getenv (p_var : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 pub type exitfunc_t = :: std :: option :: Option < unsafe extern "C" fn () > ;
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_set_exit_funcPFvvE"] 
pub fn vsf_sysutil_set_exit_func (exitfunc : exitfunc_t) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_getuidv"] 
pub fn vsf_sysutil_getuid () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_openlogi"] 
pub fn vsf_sysutil_openlog (force : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_syslogPKci"] 
pub fn vsf_sysutil_syslog (p_text : * const :: std :: os :: raw :: c_char , severe : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z20vsf_sysutil_closelogv"] 
pub fn vsf_sysutil_closelog () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_running_as_rootv"] 
pub fn vsf_sysutil_running_as_root () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_setuidPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_setuid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_setgidPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_setgid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_setuid_numerici"] 
pub fn vsf_sysutil_setuid_numeric (uid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z26vsf_sysutil_setgid_numerici"] 
pub fn vsf_sysutil_setgid_numeric (gid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_geteuidv"] 
pub fn vsf_sysutil_geteuid () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_getegidv"] 
pub fn vsf_sysutil_getegid () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_seteuidPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_seteuid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z19vsf_sysutil_setegidPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_setegid (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_seteuid_numerici"] 
pub fn vsf_sysutil_seteuid_numeric (uid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_sysutil_setegid_numerici"] 
pub fn vsf_sysutil_setegid_numeric (gid : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z29vsf_sysutil_clear_supp_groupsv"] 
pub fn vsf_sysutil_clear_supp_groups () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_initgroupsPK16vsf_sysutil_user"] 
pub fn vsf_sysutil_initgroups (p_user : * const vsf_sysutil_user) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_sysutil_chrootPKc"] 
pub fn vsf_sysutil_chroot (p_root_path : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_get_time_secv"] 
pub fn vsf_sysutil_get_time_sec () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
//# [link_name = "\u{1}_Z25vsf_sysutil_get_time_usecv"] 
pub fn vsf_sysutil_get_time_usec () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_parse_timePKc"] 
pub fn vsf_sysutil_parse_time (p_text : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
//# [link_name = "\u{1}_Z17vsf_sysutil_sleepd"] 
pub fn vsf_sysutil_sleep (seconds : f64) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_setmodtimePKcli"] 
pub fn vsf_sysutil_setmodtime (p_file : * const :: std :: os :: raw :: c_char , the_time : :: std :: os :: raw :: c_long , is_localtime : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z35vsf_sysutil_set_address_space_limitm"] 
pub fn vsf_sysutil_set_address_space_limit (bytes : :: std :: os :: raw :: c_ulong) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_sysutil_set_no_fdsv"] 
pub fn vsf_sysutil_set_no_fds () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z24vsf_sysutil_set_no_procsv"] 
pub fn vsf_sysutil_set_no_procs () ;

}
 extern "C" {
//# [link_name = "\u{1}_Z18vsf_tcp_wrapper_oki"] 
pub fn vsf_tcp_wrapper_ok (remote_fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22tunables_load_defaultsv"] 
pub fn tunables_load_defaults () ;

}
 extern "C" {
pub static mut tunable_anonymous_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_local_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_pasv_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_port_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chroot_local_user : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_write_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_upload_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_mkdir_write_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_other_write_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chown_uploads : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_connect_from_port_20 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_xferlog_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_dirmessage_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_anon_world_readable_only : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_async_abor_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ascii_upload_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ascii_download_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_one_process_model : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_xferlog_std_format : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_pasv_promiscuous : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_deny_email_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chroot_list_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_setproctitle_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_text_userdb_names : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ls_recurse_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_log_ftp_protocol : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_guest_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_userlist_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_userlist_deny : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_use_localtime : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_check_shell : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_hide_ids : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_listen : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_port_promiscuous : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_passwd_chroot_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_no_anon_password : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_tcp_wrappers : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_use_sendfile : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_dot_files : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_listen_ipv6 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_dual_log_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_syslog_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_background : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_virtual_use_local_privs : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_session_support : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_download_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_dirlist_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_chmod_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_secure_email_list_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_run_as_launching_user : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_no_log_lock : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ssl_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_allow_anon_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_local_logins_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_local_data_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_sslv2 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_sslv3 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_tlsv1 : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_tilde_user_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_anon_logins_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_force_anon_data_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_mdtm_write : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_lock_upload_files : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_pasv_addr_resolve : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_debug_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_require_cert : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_validate_cert : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_strict_ssl_read_eof : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_strict_ssl_write_shutdown : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ssl_request_cert : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_delete_failed_uploads : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_implicit_ssl : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ptrace_sandbox : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_require_ssl_reuse : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_isolate : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_isolate_network : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_ftp_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_http_enable : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_seccomp_sandbox : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_allow_writeable_chroot : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut tunable_accept_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_connect_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_local_umask : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_anon_umask : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_ftp_data_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_idle_session_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_data_connection_timeout : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_pasv_min_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_pasv_max_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_anon_max_rate : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_local_max_rate : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_listen_port : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_max_clients : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_file_open_mode : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_max_per_ip : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_trans_chunk_size : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_delay_failed_login : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_delay_successful_login : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_max_login_fails : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_chown_upload_mode : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub static mut tunable_secure_chroot_dir : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ftp_username : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_chown_username : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_xferlog_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_vsftpd_log_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_message_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_nopriv_user : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ftpd_banner : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_banned_email_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_chroot_list_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_pam_service_name : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_guest_username : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_userlist_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_anon_root : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_local_root : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_banner_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_pasv_address : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_listen_address : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_user_config_dir : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_listen_address6 : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_cmds_allowed : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_hide_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_deny_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_user_sub_token : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_email_password_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_rsa_cert_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_dsa_cert_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ssl_ciphers : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_rsa_private_key_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_dsa_private_key_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_ca_certs_file : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut tunable_cmds_denied : * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_two_process_startP11vsf_session"] 
pub fn vsf_two_process_start (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z21vsf_two_process_loginP11vsf_sessionPK5mystr"] 
pub fn vsf_two_process_login (p_sess : &vsf_session , p_pass_str : * const mystr) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z34vsf_two_process_get_priv_data_sockP11vsf_session"] 
pub fn vsf_two_process_get_priv_data_sock (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_two_process_pasv_cleanupP11vsf_session"] 
pub fn vsf_two_process_pasv_cleanup (p_sess : &vsf_session) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_two_process_pasv_activeP11vsf_session"] 
pub fn vsf_two_process_pasv_active (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z22vsf_two_process_listenP11vsf_session"] 
pub fn vsf_two_process_listen (p_sess : &vsf_session) -> :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
//# [link_name = "\u{1}_Z27vsf_two_process_get_pasv_fdP11vsf_session"] 
pub fn vsf_two_process_get_pasv_fd (p_sess : &vsf_session) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_Z28vsf_two_process_chown_uploadP11vsf_sessioni"] 
pub fn vsf_two_process_chown_upload (p_sess : &vsf_session , fd : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z3diePKc"] 
pub fn die (p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z4die2PKcS0_"] 
pub fn die2 (p_text1 : * const :: std :: os :: raw :: c_char , p_text2 : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z3bugPKc"] 
pub fn bug (p_text : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}_Z8vsf_exitPKc"] 
pub fn vsf_exit (p_text : * const :: std :: os :: raw :: c_char) ;

}
 pub type int_least8_t = :: std :: os :: raw :: c_schar ;
 pub type int_least16_t = :: std :: os :: raw :: c_short ;
 pub type int_least32_t = :: std :: os :: raw :: c_int ;
 pub type int_least64_t = :: std :: os :: raw :: c_long ;
 pub type uint_least8_t = :: std :: os :: raw :: c_uchar ;
 pub type uint_least16_t = :: std :: os :: raw :: c_ushort ;
 pub type uint_least32_t = :: std :: os :: raw :: c_uint ;
 pub type uint_least64_t = :: std :: os :: raw :: c_ulong ;
 pub type int_fast8_t = :: std :: os :: raw :: c_schar ;
 pub type int_fast16_t = :: std :: os :: raw :: c_long ;
 pub type int_fast32_t = :: std :: os :: raw :: c_long ;
 pub type int_fast64_t = :: std :: os :: raw :: c_long ;
 pub type uint_fast8_t = :: std :: os :: raw :: c_uchar ;
 pub type uint_fast16_t = :: std :: os :: raw :: c_ulong ;
 pub type uint_fast32_t = :: std :: os :: raw :: c_ulong ;
 pub type uint_fast64_t = :: std :: os :: raw :: c_ulong ;
 pub type intmax_t = __intmax_t ;
 pub type uintmax_t = __uintmax_t ;
 pub type n_short = u16 ;
 pub type n_long = u32 ;
 pub type n_time = u32 ;
 pub type in_addr_t = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct in_addr {
pub s_addr : in_addr_t ,
}
 # [test] fn bindgen_test_layout_in_addr () {
assert_eq ! (:: std :: mem :: size_of :: < in_addr > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( in_addr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < in_addr > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( in_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in_addr > ( ) ) ) . s_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in_addr ) , "::" , stringify ! ( s_addr ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct ip_opts {
pub ip_dst : in_addr , pub ip_opts : [:: std :: os :: raw :: c_char ; 40usize] ,
}
 # [test] fn bindgen_test_layout_ip_opts () {
assert_eq ! (:: std :: mem :: size_of :: < ip_opts > ( ) , 44usize , concat ! ( "Size of: " , stringify ! ( ip_opts ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip_opts > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip_opts ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_opts > ( ) ) ) . ip_dst as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip_opts ) , "::" , stringify ! ( ip_dst ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_opts > ( ) ) ) . ip_opts as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip_opts ) , "::" , stringify ! ( ip_opts ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ip_mreqn {
pub imr_multiaddr : in_addr , pub imr_address : in_addr , pub imr_ifindex : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_ip_mreqn () {
assert_eq ! (:: std :: mem :: size_of :: < ip_mreqn > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( ip_mreqn ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip_mreqn > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip_mreqn ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreqn > ( ) ) ) . imr_multiaddr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreqn ) , "::" , stringify ! ( imr_multiaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreqn > ( ) ) ) . imr_address as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreqn ) , "::" , stringify ! ( imr_address ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreqn > ( ) ) ) . imr_ifindex as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreqn ) , "::" , stringify ! ( imr_ifindex ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct in_pktinfo {
pub ipi_ifindex : :: std :: os :: raw :: c_int , pub ipi_spec_dst : in_addr , pub ipi_addr : in_addr ,
}
 # [test] fn bindgen_test_layout_in_pktinfo () {
assert_eq ! (:: std :: mem :: size_of :: < in_pktinfo > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( in_pktinfo ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < in_pktinfo > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( in_pktinfo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in_pktinfo > ( ) ) ) . ipi_ifindex as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in_pktinfo ) , "::" , stringify ! ( ipi_ifindex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in_pktinfo > ( ) ) ) . ipi_spec_dst as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( in_pktinfo ) , "::" , stringify ! ( ipi_spec_dst ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in_pktinfo > ( ) ) ) . ipi_addr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( in_pktinfo ) , "::" , stringify ! ( ipi_addr ) )) ;

}
 pub const IPPROTO_IP : _bindgen_ty_4 = 0 ;
 pub const IPPROTO_ICMP : _bindgen_ty_4 = 1 ;
 pub const IPPROTO_IGMP : _bindgen_ty_4 = 2 ;
 pub const IPPROTO_IPIP : _bindgen_ty_4 = 4 ;
 pub const IPPROTO_TCP : _bindgen_ty_4 = 6 ;
 pub const IPPROTO_EGP : _bindgen_ty_4 = 8 ;
 pub const IPPROTO_PUP : _bindgen_ty_4 = 12 ;
 pub const IPPROTO_UDP : _bindgen_ty_4 = 17 ;
 pub const IPPROTO_IDP : _bindgen_ty_4 = 22 ;
 pub const IPPROTO_TP : _bindgen_ty_4 = 29 ;
 pub const IPPROTO_DCCP : _bindgen_ty_4 = 33 ;
 pub const IPPROTO_IPV6 : _bindgen_ty_4 = 41 ;
 pub const IPPROTO_RSVP : _bindgen_ty_4 = 46 ;
 pub const IPPROTO_GRE : _bindgen_ty_4 = 47 ;
 pub const IPPROTO_ESP : _bindgen_ty_4 = 50 ;
 pub const IPPROTO_AH : _bindgen_ty_4 = 51 ;
 pub const IPPROTO_MTP : _bindgen_ty_4 = 92 ;
 pub const IPPROTO_BEETPH : _bindgen_ty_4 = 94 ;
 pub const IPPROTO_ENCAP : _bindgen_ty_4 = 98 ;
 pub const IPPROTO_PIM : _bindgen_ty_4 = 103 ;
 pub const IPPROTO_COMP : _bindgen_ty_4 = 108 ;
 pub const IPPROTO_SCTP : _bindgen_ty_4 = 132 ;
 pub const IPPROTO_UDPLITE : _bindgen_ty_4 = 136 ;
 pub const IPPROTO_MPLS : _bindgen_ty_4 = 137 ;
 pub const IPPROTO_RAW : _bindgen_ty_4 = 255 ;
 pub const IPPROTO_MAX : _bindgen_ty_4 = 256 ;
 pub type _bindgen_ty_4 = u32 ;
 pub const IPPROTO_HOPOPTS : _bindgen_ty_5 = 0 ;
 pub const IPPROTO_ROUTING : _bindgen_ty_5 = 43 ;
 pub const IPPROTO_FRAGMENT : _bindgen_ty_5 = 44 ;
 pub const IPPROTO_ICMPV6 : _bindgen_ty_5 = 58 ;
 pub const IPPROTO_NONE : _bindgen_ty_5 = 59 ;
 pub const IPPROTO_DSTOPTS : _bindgen_ty_5 = 60 ;
 pub const IPPROTO_MH : _bindgen_ty_5 = 135 ;
 pub type _bindgen_ty_5 = u32 ;
 pub type in_port_t = u16 ;
 pub const IPPORT_ECHO : _bindgen_ty_6 = 7 ;
 pub const IPPORT_DISCARD : _bindgen_ty_6 = 9 ;
 pub const IPPORT_SYSTAT : _bindgen_ty_6 = 11 ;
 pub const IPPORT_DAYTIME : _bindgen_ty_6 = 13 ;
 pub const IPPORT_NETSTAT : _bindgen_ty_6 = 15 ;
 pub const IPPORT_FTP : _bindgen_ty_6 = 21 ;
 pub const IPPORT_TELNET : _bindgen_ty_6 = 23 ;
 pub const IPPORT_SMTP : _bindgen_ty_6 = 25 ;
 pub const IPPORT_TIMESERVER : _bindgen_ty_6 = 37 ;
 pub const IPPORT_NAMESERVER : _bindgen_ty_6 = 42 ;
 pub const IPPORT_WHOIS : _bindgen_ty_6 = 43 ;
 pub const IPPORT_MTP : _bindgen_ty_6 = 57 ;
 pub const IPPORT_TFTP : _bindgen_ty_6 = 69 ;
 pub const IPPORT_RJE : _bindgen_ty_6 = 77 ;
 pub const IPPORT_FINGER : _bindgen_ty_6 = 79 ;
 pub const IPPORT_TTYLINK : _bindgen_ty_6 = 87 ;
 pub const IPPORT_SUPDUP : _bindgen_ty_6 = 95 ;
 pub const IPPORT_EXECSERVER : _bindgen_ty_6 = 512 ;
 pub const IPPORT_LOGINSERVER : _bindgen_ty_6 = 513 ;
 pub const IPPORT_CMDSERVER : _bindgen_ty_6 = 514 ;
 pub const IPPORT_EFSSERVER : _bindgen_ty_6 = 520 ;
 pub const IPPORT_BIFFUDP : _bindgen_ty_6 = 512 ;
 pub const IPPORT_WHOSERVER : _bindgen_ty_6 = 513 ;
 pub const IPPORT_ROUTESERVER : _bindgen_ty_6 = 520 ;
 pub const IPPORT_RESERVED : _bindgen_ty_6 = 1024 ;
 pub const IPPORT_USERRESERVED : _bindgen_ty_6 = 5000 ;
 pub type _bindgen_ty_6 = u32 ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct in6_addr {
pub __in6_u : in6_addr__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union in6_addr__bindgen_ty_1 {
pub __u6_addr8 : [u8 ; 16usize] , pub __u6_addr16 : [u16 ; 8usize] , pub __u6_addr32 : [u32 ; 4usize] , _bindgen_union_align : [u32 ; 4usize] ,
}
 # [test] fn bindgen_test_layout_in6_addr__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < in6_addr__bindgen_ty_1 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( in6_addr__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < in6_addr__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( in6_addr__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in6_addr__bindgen_ty_1 > ( ) ) ) . __u6_addr8 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in6_addr__bindgen_ty_1 ) , "::" , stringify ! ( __u6_addr8 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in6_addr__bindgen_ty_1 > ( ) ) ) . __u6_addr16 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in6_addr__bindgen_ty_1 ) , "::" , stringify ! ( __u6_addr16 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in6_addr__bindgen_ty_1 > ( ) ) ) . __u6_addr32 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in6_addr__bindgen_ty_1 ) , "::" , stringify ! ( __u6_addr32 ) )) ;

}
 # [test] fn bindgen_test_layout_in6_addr () {
assert_eq ! (:: std :: mem :: size_of :: < in6_addr > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( in6_addr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < in6_addr > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( in6_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in6_addr > ( ) ) ) . __in6_u as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in6_addr ) , "::" , stringify ! ( __in6_u ) )) ;

}
 extern "C" {
pub static in6addr_any : in6_addr ;

}
 extern "C" {
pub static in6addr_loopback : in6_addr ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sockaddr_in {
pub sin_family : sa_family_t , pub sin_port : in_port_t , pub sin_addr : in_addr , pub sin_zero : [:: std :: os :: raw :: c_uchar ; 8usize] ,
}
 # [test] fn bindgen_test_layout_sockaddr_in () {
assert_eq ! (:: std :: mem :: size_of :: < sockaddr_in > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( sockaddr_in ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sockaddr_in > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( sockaddr_in ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in > ( ) ) ) . sin_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in ) , "::" , stringify ! ( sin_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in > ( ) ) ) . sin_port as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in ) , "::" , stringify ! ( sin_port ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in > ( ) ) ) . sin_addr as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in ) , "::" , stringify ! ( sin_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in > ( ) ) ) . sin_zero as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in ) , "::" , stringify ! ( sin_zero ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct sockaddr_in6 {
pub sin6_family : sa_family_t , pub sin6_port : in_port_t , pub sin6_flowinfo : u32 , pub sin6_addr : in6_addr , pub sin6_scope_id : u32 ,
}
 # [test] fn bindgen_test_layout_sockaddr_in6 () {
assert_eq ! (:: std :: mem :: size_of :: < sockaddr_in6 > ( ) , 28usize , concat ! ( "Size of: " , stringify ! ( sockaddr_in6 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sockaddr_in6 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( sockaddr_in6 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in6 > ( ) ) ) . sin6_family as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in6 ) , "::" , stringify ! ( sin6_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in6 > ( ) ) ) . sin6_port as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in6 ) , "::" , stringify ! ( sin6_port ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in6 > ( ) ) ) . sin6_flowinfo as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in6 ) , "::" , stringify ! ( sin6_flowinfo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in6 > ( ) ) ) . sin6_addr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in6 ) , "::" , stringify ! ( sin6_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sockaddr_in6 > ( ) ) ) . sin6_scope_id as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( sockaddr_in6 ) , "::" , stringify ! ( sin6_scope_id ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ip_mreq {
pub imr_multiaddr : in_addr , pub imr_interface : in_addr ,
}
 # [test] fn bindgen_test_layout_ip_mreq () {
assert_eq ! (:: std :: mem :: size_of :: < ip_mreq > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( ip_mreq ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip_mreq > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip_mreq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreq > ( ) ) ) . imr_multiaddr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreq ) , "::" , stringify ! ( imr_multiaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreq > ( ) ) ) . imr_interface as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreq ) , "::" , stringify ! ( imr_interface ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ip_mreq_source {
pub imr_multiaddr : in_addr , pub imr_interface : in_addr , pub imr_sourceaddr : in_addr ,
}
 # [test] fn bindgen_test_layout_ip_mreq_source () {
assert_eq ! (:: std :: mem :: size_of :: < ip_mreq_source > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( ip_mreq_source ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip_mreq_source > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip_mreq_source ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreq_source > ( ) ) ) . imr_multiaddr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreq_source ) , "::" , stringify ! ( imr_multiaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreq_source > ( ) ) ) . imr_interface as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreq_source ) , "::" , stringify ! ( imr_interface ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_mreq_source > ( ) ) ) . imr_sourceaddr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ip_mreq_source ) , "::" , stringify ! ( imr_sourceaddr ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct ipv6_mreq {
pub ipv6mr_multiaddr : in6_addr , pub ipv6mr_interface : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_ipv6_mreq () {
assert_eq ! (:: std :: mem :: size_of :: < ipv6_mreq > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( ipv6_mreq ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ipv6_mreq > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ipv6_mreq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ipv6_mreq > ( ) ) ) . ipv6mr_multiaddr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ipv6_mreq ) , "::" , stringify ! ( ipv6mr_multiaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ipv6_mreq > ( ) ) ) . ipv6mr_interface as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( ipv6_mreq ) , "::" , stringify ! ( ipv6mr_interface ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct group_req {
pub gr_interface : u32 , pub gr_group : sockaddr_storage ,
}
 # [test] fn bindgen_test_layout_group_req () {
assert_eq ! (:: std :: mem :: size_of :: < group_req > ( ) , 136usize , concat ! ( "Size of: " , stringify ! ( group_req ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < group_req > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( group_req ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_req > ( ) ) ) . gr_interface as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( group_req ) , "::" , stringify ! ( gr_interface ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_req > ( ) ) ) . gr_group as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( group_req ) , "::" , stringify ! ( gr_group ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct group_source_req {
pub gsr_interface : u32 , pub gsr_group : sockaddr_storage , pub gsr_source : sockaddr_storage ,
}
 # [test] fn bindgen_test_layout_group_source_req () {
assert_eq ! (:: std :: mem :: size_of :: < group_source_req > ( ) , 264usize , concat ! ( "Size of: " , stringify ! ( group_source_req ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < group_source_req > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( group_source_req ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_source_req > ( ) ) ) . gsr_interface as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( group_source_req ) , "::" , stringify ! ( gsr_interface ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_source_req > ( ) ) ) . gsr_group as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( group_source_req ) , "::" , stringify ! ( gsr_group ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_source_req > ( ) ) ) . gsr_source as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( group_source_req ) , "::" , stringify ! ( gsr_source ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ip_msfilter {
pub imsf_multiaddr : in_addr , pub imsf_interface : in_addr , pub imsf_fmode : u32 , pub imsf_numsrc : u32 , pub imsf_slist : [in_addr ; 1usize] ,
}
 # [test] fn bindgen_test_layout_ip_msfilter () {
assert_eq ! (:: std :: mem :: size_of :: < ip_msfilter > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( ip_msfilter ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip_msfilter > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip_msfilter ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_msfilter > ( ) ) ) . imsf_multiaddr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip_msfilter ) , "::" , stringify ! ( imsf_multiaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_msfilter > ( ) ) ) . imsf_interface as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip_msfilter ) , "::" , stringify ! ( imsf_interface ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_msfilter > ( ) ) ) . imsf_fmode as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ip_msfilter ) , "::" , stringify ! ( imsf_fmode ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_msfilter > ( ) ) ) . imsf_numsrc as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( ip_msfilter ) , "::" , stringify ! ( imsf_numsrc ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_msfilter > ( ) ) ) . imsf_slist as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( ip_msfilter ) , "::" , stringify ! ( imsf_slist ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct group_filter {
pub gf_interface : u32 , pub gf_group : sockaddr_storage , pub gf_fmode : u32 , pub gf_numsrc : u32 , pub gf_slist : [sockaddr_storage ; 1usize] ,
}
 # [test] fn bindgen_test_layout_group_filter () {
assert_eq ! (:: std :: mem :: size_of :: < group_filter > ( ) , 272usize , concat ! ( "Size of: " , stringify ! ( group_filter ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < group_filter > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( group_filter ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_filter > ( ) ) ) . gf_interface as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( group_filter ) , "::" , stringify ! ( gf_interface ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_filter > ( ) ) ) . gf_group as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( group_filter ) , "::" , stringify ! ( gf_group ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_filter > ( ) ) ) . gf_fmode as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( group_filter ) , "::" , stringify ! ( gf_fmode ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_filter > ( ) ) ) . gf_numsrc as * const _ as usize } , 140usize , concat ! ( "Offset of field: " , stringify ! ( group_filter ) , "::" , stringify ! ( gf_numsrc ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group_filter > ( ) ) ) . gf_slist as * const _ as usize } , 144usize , concat ! ( "Offset of field: " , stringify ! ( group_filter ) , "::" , stringify ! ( gf_slist ) )) ;

}
 extern "C" {
pub fn ntohl (__netlong : u32) -> u32 ;

}
 extern "C" {
pub fn ntohs (__netshort : u16) -> u16 ;

}
 extern "C" {
pub fn htonl (__hostlong : u32) -> u32 ;

}
 extern "C" {
pub fn htons (__hostshort : u16) -> u16 ;

}
 extern "C" {
pub fn bindresvport (__sockfd : :: std :: os :: raw :: c_int , __sock_in : * mut sockaddr_in) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn bindresvport6 (__sockfd : :: std :: os :: raw :: c_int , __sock_in : * mut sockaddr_in6) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct in6_pktinfo {
pub ipi6_addr : in6_addr , pub ipi6_ifindex : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_in6_pktinfo () {
assert_eq ! (:: std :: mem :: size_of :: < in6_pktinfo > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( in6_pktinfo ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < in6_pktinfo > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( in6_pktinfo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in6_pktinfo > ( ) ) ) . ipi6_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( in6_pktinfo ) , "::" , stringify ! ( ipi6_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < in6_pktinfo > ( ) ) ) . ipi6_ifindex as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( in6_pktinfo ) , "::" , stringify ! ( ipi6_ifindex ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct ip6_mtuinfo {
pub ip6m_addr : sockaddr_in6 , pub ip6m_mtu : u32 ,
}
 # [test] fn bindgen_test_layout_ip6_mtuinfo () {
assert_eq ! (:: std :: mem :: size_of :: < ip6_mtuinfo > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( ip6_mtuinfo ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip6_mtuinfo > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip6_mtuinfo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip6_mtuinfo > ( ) ) ) . ip6m_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip6_mtuinfo ) , "::" , stringify ! ( ip6m_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip6_mtuinfo > ( ) ) ) . ip6m_mtu as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( ip6_mtuinfo ) , "::" , stringify ! ( ip6m_mtu ) )) ;

}
 extern "C" {
pub fn inet6_option_space (__nbytes : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_option_init (__bp : * mut :: std :: os :: raw :: c_void , __cmsgp : * mut * mut cmsghdr , __type : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_option_append (__cmsg : * mut cmsghdr , __typep : * const u8 , __multx : :: std :: os :: raw :: c_int , __plusy : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_option_alloc (__cmsg : * mut cmsghdr , __datalen : :: std :: os :: raw :: c_int , __multx : :: std :: os :: raw :: c_int , __plusy : :: std :: os :: raw :: c_int) -> * mut u8 ;

}
 extern "C" {
pub fn inet6_option_next (__cmsg : * const cmsghdr , __tptrp : * mut * mut u8) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_option_find (__cmsg : * const cmsghdr , __tptrp : * mut * mut u8 , __type : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_init (__extbuf : * mut :: std :: os :: raw :: c_void , __extlen : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_append (__extbuf : * mut :: std :: os :: raw :: c_void , __extlen : socklen_t , __offset : :: std :: os :: raw :: c_int , __type : u8 , __len : socklen_t , __align : u8 , __databufp : * mut * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_finish (__extbuf : * mut :: std :: os :: raw :: c_void , __extlen : socklen_t , __offset : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_set_val (__databuf : * mut :: std :: os :: raw :: c_void , __offset : :: std :: os :: raw :: c_int , __val : * mut :: std :: os :: raw :: c_void , __vallen : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_next (__extbuf : * mut :: std :: os :: raw :: c_void , __extlen : socklen_t , __offset : :: std :: os :: raw :: c_int , __typep : * mut u8 , __lenp : * mut socklen_t , __databufp : * mut * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_find (__extbuf : * mut :: std :: os :: raw :: c_void , __extlen : socklen_t , __offset : :: std :: os :: raw :: c_int , __type : u8 , __lenp : * mut socklen_t , __databufp : * mut * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_opt_get_val (__databuf : * mut :: std :: os :: raw :: c_void , __offset : :: std :: os :: raw :: c_int , __val : * mut :: std :: os :: raw :: c_void , __vallen : socklen_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_rth_space (__type : :: std :: os :: raw :: c_int , __segments : :: std :: os :: raw :: c_int) -> socklen_t ;

}
 extern "C" {
pub fn inet6_rth_init (__bp : * mut :: std :: os :: raw :: c_void , __bp_len : socklen_t , __type : :: std :: os :: raw :: c_int , __segments : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn inet6_rth_add (__bp : * mut :: std :: os :: raw :: c_void , __addr : * const in6_addr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_rth_reverse (__in : * const :: std :: os :: raw :: c_void , __out : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_rth_segments (__bp : * const :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet6_rth_getaddr (__bp : * const :: std :: os :: raw :: c_void , __index : :: std :: os :: raw :: c_int) -> * mut in6_addr ;

}
 extern "C" {
pub fn getipv4sourcefilter (__s : :: std :: os :: raw :: c_int , __interface_addr : in_addr , __group : in_addr , __fmode : * mut u32 , __numsrc : * mut u32 , __slist : * mut in_addr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setipv4sourcefilter (__s : :: std :: os :: raw :: c_int , __interface_addr : in_addr , __group : in_addr , __fmode : u32 , __numsrc : u32 , __slist : * const in_addr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getsourcefilter (__s : :: std :: os :: raw :: c_int , __interface_addr : u32 , __group : * const sockaddr , __grouplen : socklen_t , __fmode : * mut u32 , __numsrc : * mut u32 , __slist : * mut sockaddr_storage) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setsourcefilter (__s : :: std :: os :: raw :: c_int , __interface_addr : u32 , __group : * const sockaddr , __grouplen : socklen_t , __fmode : u32 , __numsrc : u32 , __slist : * const sockaddr_storage) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct timestamp {
pub len : u8 , pub ptr : u8 , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > , pub data : [u32 ; 9usize] ,
}
 # [test] fn bindgen_test_layout_timestamp () {
assert_eq ! (:: std :: mem :: size_of :: < timestamp > ( ) , 40usize , concat ! ( "Size of: " , stringify ! ( timestamp ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timestamp > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( timestamp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timestamp > ( ) ) ) . len as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timestamp ) , "::" , stringify ! ( len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timestamp > ( ) ) ) . ptr as * const _ as usize } , 1usize , concat ! ( "Offset of field: " , stringify ! ( timestamp ) , "::" , stringify ! ( ptr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timestamp > ( ) ) ) . data as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( timestamp ) , "::" , stringify ! ( data ) )) ;

}
 impl timestamp {
# [inline] pub fn flags (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_flags (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn overflow (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_overflow (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (flags : :: std :: os :: raw :: c_uint , overflow : :: std :: os :: raw :: c_uint) -> __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let flags : u32 = unsafe { :: std :: mem :: transmute ( flags ) } ; flags as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let overflow : u32 = unsafe { :: std :: mem :: transmute ( overflow ) } ; overflow as u64 }) ;
 __bindgen_bitfield_unit
}

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct iphdr {
pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > , pub tos : u8 , pub tot_len : u16 , pub id : u16 , pub frag_off : u16 , pub ttl : u8 , pub protocol : u8 , pub check : u16 , pub saddr : u32 , pub daddr : u32 ,
}
 # [test] fn bindgen_test_layout_iphdr () {
assert_eq ! (:: std :: mem :: size_of :: < iphdr > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( iphdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < iphdr > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( iphdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . tos as * const _ as usize } , 1usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( tos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . tot_len as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( tot_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . id as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( id ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . frag_off as * const _ as usize } , 6usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( frag_off ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . ttl as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( ttl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . protocol as * const _ as usize } , 9usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( protocol ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . check as * const _ as usize } , 10usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( check ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . saddr as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( saddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < iphdr > ( ) ) ) . daddr as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( iphdr ) , "::" , stringify ! ( daddr ) )) ;

}
 impl iphdr {
# [inline] pub fn ihl (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_ihl (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn version (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_version (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (ihl : :: std :: os :: raw :: c_uint , version : :: std :: os :: raw :: c_uint) -> __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let ihl : u32 = unsafe { :: std :: mem :: transmute ( ihl ) } ; ihl as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let version : u32 = unsafe { :: std :: mem :: transmute ( version ) } ; version as u64 }) ;
 __bindgen_bitfield_unit
}

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ip {
pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > , pub ip_tos : u8 , pub ip_len : :: std :: os :: raw :: c_ushort , pub ip_id : :: std :: os :: raw :: c_ushort , pub ip_off : :: std :: os :: raw :: c_ushort , pub ip_ttl : u8 , pub ip_p : u8 , pub ip_sum : :: std :: os :: raw :: c_ushort , pub ip_src : in_addr , pub ip_dst : in_addr ,
}
 # [test] fn bindgen_test_layout_ip () {
assert_eq ! (:: std :: mem :: size_of :: < ip > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( ip ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_tos as * const _ as usize } , 1usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_tos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_len as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_id as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_id ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_off as * const _ as usize } , 6usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_off ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_ttl as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_ttl ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_p as * const _ as usize } , 9usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_p ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_sum as * const _ as usize } , 10usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_sum ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_src as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_src ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip > ( ) ) ) . ip_dst as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( ip ) , "::" , stringify ! ( ip_dst ) )) ;

}
 impl ip {
# [inline] pub fn ip_hl (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_ip_hl (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn ip_v (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_ip_v (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (ip_hl : :: std :: os :: raw :: c_uint , ip_v : :: std :: os :: raw :: c_uint) -> __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let ip_hl : u32 = unsafe { :: std :: mem :: transmute ( ip_hl ) } ; ip_hl as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let ip_v : u32 = unsafe { :: std :: mem :: transmute ( ip_v ) } ; ip_v as u64 }) ;
 __bindgen_bitfield_unit
}

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ip_timestamp {
pub ipt_code : u8 , pub ipt_len : u8 , pub ipt_ptr : u8 , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > , pub data : [u32 ; 9usize] ,
}
 # [test] fn bindgen_test_layout_ip_timestamp () {
assert_eq ! (:: std :: mem :: size_of :: < ip_timestamp > ( ) , 40usize , concat ! ( "Size of: " , stringify ! ( ip_timestamp ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ip_timestamp > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( ip_timestamp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_timestamp > ( ) ) ) . ipt_code as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ip_timestamp ) , "::" , stringify ! ( ipt_code ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_timestamp > ( ) ) ) . ipt_len as * const _ as usize } , 1usize , concat ! ( "Offset of field: " , stringify ! ( ip_timestamp ) , "::" , stringify ! ( ipt_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_timestamp > ( ) ) ) . ipt_ptr as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( ip_timestamp ) , "::" , stringify ! ( ipt_ptr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ip_timestamp > ( ) ) ) . data as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( ip_timestamp ) , "::" , stringify ! ( data ) )) ;

}
 impl ip_timestamp {
# [inline] pub fn ipt_flg (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_ipt_flg (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn ipt_oflw (& self) -> :: std :: os :: raw :: c_uint {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u32)
}

}
 # [inline] pub fn set_ipt_oflw (& mut self , val : :: std :: os :: raw :: c_uint) {
unsafe {
let val : u32 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (ipt_flg : :: std :: os :: raw :: c_uint , ipt_oflw : :: std :: os :: raw :: c_uint) -> __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let ipt_flg : u32 = unsafe { :: std :: mem :: transmute ( ipt_flg ) } ; ipt_flg as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let ipt_oflw : u32 = unsafe { :: std :: mem :: transmute ( ipt_oflw ) } ; ipt_oflw as u64 }) ;
 __bindgen_bitfield_unit
}

}
 pub type tcp_seq = u32 ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct tcphdr {
pub __bindgen_anon_1 : tcphdr__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union tcphdr__bindgen_ty_1 {
pub __bindgen_anon_1 : tcphdr__bindgen_ty_1__bindgen_ty_1 , pub __bindgen_anon_2 : tcphdr__bindgen_ty_1__bindgen_ty_2 , _bindgen_union_align : [u32 ; 5usize] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct tcphdr__bindgen_ty_1__bindgen_ty_1 {
pub th_sport : u16 , pub th_dport : u16 , pub th_seq : tcp_seq , pub th_ack : tcp_seq , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > , pub th_flags : u8 , pub th_win : u16 , pub th_sum : u16 , pub th_urp : u16 ,
}
 # [test] fn bindgen_test_layout_tcphdr__bindgen_ty_1__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_sport as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_sport ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_dport as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_dport ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_seq as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_seq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_ack as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_ack ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_flags as * const _ as usize } , 13usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_win as * const _ as usize } , 14usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_win ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_sum as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_sum ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . th_urp as * const _ as usize } , 18usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( th_urp ) )) ;

}
 impl tcphdr__bindgen_ty_1__bindgen_ty_1 {
# [inline] pub fn th_x2 (& self) -> u8 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u8)
}

}
 # [inline] pub fn set_th_x2 (& mut self , val : u8) {
unsafe {
let val : u8 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn th_off (& self) -> u8 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u8)
}

}
 # [inline] pub fn set_th_off (& mut self , val : u8) {
unsafe {
let val : u8 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (th_x2 : u8 , th_off : u8) -> __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let th_x2 : u8 = unsafe { :: std :: mem :: transmute ( th_x2 ) } ; th_x2 as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let th_off : u8 = unsafe { :: std :: mem :: transmute ( th_off ) } ; th_off as u64 }) ;
 __bindgen_bitfield_unit
}

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct tcphdr__bindgen_ty_1__bindgen_ty_2 {
pub source : u16 , pub dest : u16 , pub seq : u32 , pub ack_seq : u32 , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 2usize] , u8 > , pub window : u16 , pub check : u16 , pub urg_ptr : u16 ,
}
 # [test] fn bindgen_test_layout_tcphdr__bindgen_ty_1__bindgen_ty_2 () {
assert_eq ! (:: std :: mem :: size_of :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . source as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( source ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . dest as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( dest ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . seq as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( seq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . ack_seq as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( ack_seq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . window as * const _ as usize } , 14usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( window ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . check as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( check ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcphdr__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . urg_ptr as * const _ as usize } , 18usize , concat ! ( "Offset of field: " , stringify ! ( tcphdr__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( urg_ptr ) )) ;

}
 impl tcphdr__bindgen_ty_1__bindgen_ty_2 {
# [inline] pub fn res1 (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u16)
}

}
 # [inline] pub fn set_res1 (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn doff (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u16)
}

}
 # [inline] pub fn set_doff (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn fin (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 8usize , 1u8 ) as u16)
}

}
 # [inline] pub fn set_fin (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (8usize , 1u8 , val as u64)
}

}
 # [inline] pub fn syn (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 9usize , 1u8 ) as u16)
}

}
 # [inline] pub fn set_syn (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (9usize , 1u8 , val as u64)
}

}
 # [inline] pub fn rst (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 10usize , 1u8 ) as u16)
}

}
 # [inline] pub fn set_rst (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (10usize , 1u8 , val as u64)
}

}
 # [inline] pub fn psh (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 11usize , 1u8 ) as u16)
}

}
 # [inline] pub fn set_psh (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (11usize , 1u8 , val as u64)
}

}
 # [inline] pub fn ack (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 12usize , 1u8 ) as u16)
}

}
 # [inline] pub fn set_ack (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (12usize , 1u8 , val as u64)
}

}
 # [inline] pub fn urg (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 13usize , 1u8 ) as u16)
}

}
 # [inline] pub fn set_urg (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (13usize , 1u8 , val as u64)
}

}
 # [inline] pub fn res2 (& self) -> u16 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 14usize , 2u8 ) as u16)
}

}
 # [inline] pub fn set_res2 (& mut self , val : u16) {
unsafe {
let val : u16 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (14usize , 2u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (res1 : u16 , doff : u16 , fin : u16 , syn : u16 , rst : u16 , psh : u16 , ack : u16 , urg : u16 , res2 : u16) -> __BindgenBitfieldUnit < [u8 ; 2usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 2usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let res1 : u16 = unsafe { :: std :: mem :: transmute ( res1 ) } ; res1 as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let doff : u16 = unsafe { :: std :: mem :: transmute ( doff ) } ; doff as u64 }) ;
 __bindgen_bitfield_unit . set (8usize , 1u8 , { let fin : u16 = unsafe { :: std :: mem :: transmute ( fin ) } ; fin as u64 }) ;
 __bindgen_bitfield_unit . set (9usize , 1u8 , { let syn : u16 = unsafe { :: std :: mem :: transmute ( syn ) } ; syn as u64 }) ;
 __bindgen_bitfield_unit . set (10usize , 1u8 , { let rst : u16 = unsafe { :: std :: mem :: transmute ( rst ) } ; rst as u64 }) ;
 __bindgen_bitfield_unit . set (11usize , 1u8 , { let psh : u16 = unsafe { :: std :: mem :: transmute ( psh ) } ; psh as u64 }) ;
 __bindgen_bitfield_unit . set (12usize , 1u8 , { let ack : u16 = unsafe { :: std :: mem :: transmute ( ack ) } ; ack as u64 }) ;
 __bindgen_bitfield_unit . set (13usize , 1u8 , { let urg : u16 = unsafe { :: std :: mem :: transmute ( urg ) } ; urg as u64 }) ;
 __bindgen_bitfield_unit . set (14usize , 2u8 , { let res2 : u16 = unsafe { :: std :: mem :: transmute ( res2 ) } ; res2 as u64 }) ;
 __bindgen_bitfield_unit
}

}
 # [test] fn bindgen_test_layout_tcphdr__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < tcphdr__bindgen_ty_1 > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( tcphdr__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcphdr__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcphdr__bindgen_ty_1 ) )) ;

}
 # [test] fn bindgen_test_layout_tcphdr () {
assert_eq ! (:: std :: mem :: size_of :: < tcphdr > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( tcphdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcphdr > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcphdr ) )) ;

}
 pub const TCP_ESTABLISHED : _bindgen_ty_7 = 1 ;
 pub const TCP_SYN_SENT : _bindgen_ty_7 = 2 ;
 pub const TCP_SYN_RECV : _bindgen_ty_7 = 3 ;
 pub const TCP_FIN_WAIT1 : _bindgen_ty_7 = 4 ;
 pub const TCP_FIN_WAIT2 : _bindgen_ty_7 = 5 ;
 pub const TCP_TIME_WAIT : _bindgen_ty_7 = 6 ;
 pub const TCP_CLOSE : _bindgen_ty_7 = 7 ;
 pub const TCP_CLOSE_WAIT : _bindgen_ty_7 = 8 ;
 pub const TCP_LAST_ACK : _bindgen_ty_7 = 9 ;
 pub const TCP_LISTEN : _bindgen_ty_7 = 10 ;
 pub const TCP_CLOSING : _bindgen_ty_7 = 11 ;
 pub type _bindgen_ty_7 = u32 ;
 pub const tcp_ca_state_TCP_CA_Open : tcp_ca_state = 0 ;
 pub const tcp_ca_state_TCP_CA_Disorder : tcp_ca_state = 1 ;
 pub const tcp_ca_state_TCP_CA_CWR : tcp_ca_state = 2 ;
 pub const tcp_ca_state_TCP_CA_Recovery : tcp_ca_state = 3 ;
 pub const tcp_ca_state_TCP_CA_Loss : tcp_ca_state = 4 ;
 pub type tcp_ca_state = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct tcp_info {
pub tcpi_state : u8 , pub tcpi_ca_state : u8 , pub tcpi_retransmits : u8 , pub tcpi_probes : u8 , pub tcpi_backoff : u8 , pub tcpi_options : u8 , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > , pub tcpi_rto : u32 , pub tcpi_ato : u32 , pub tcpi_snd_mss : u32 , pub tcpi_rcv_mss : u32 , pub tcpi_unacked : u32 , pub tcpi_sacked : u32 , pub tcpi_lost : u32 , pub tcpi_retrans : u32 , pub tcpi_fackets : u32 , pub tcpi_last_data_sent : u32 , pub tcpi_last_ack_sent : u32 , pub tcpi_last_data_recv : u32 , pub tcpi_last_ack_recv : u32 , pub tcpi_pmtu : u32 , pub tcpi_rcv_ssthresh : u32 , pub tcpi_rtt : u32 , pub tcpi_rttvar : u32 , pub tcpi_snd_ssthresh : u32 , pub tcpi_snd_cwnd : u32 , pub tcpi_advmss : u32 , pub tcpi_reordering : u32 , pub tcpi_rcv_rtt : u32 , pub tcpi_rcv_space : u32 , pub tcpi_total_retrans : u32 ,
}
 # [test] fn bindgen_test_layout_tcp_info () {
assert_eq ! (:: std :: mem :: size_of :: < tcp_info > ( ) , 104usize , concat ! ( "Size of: " , stringify ! ( tcp_info ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcp_info > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcp_info ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_state as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_state ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_ca_state as * const _ as usize } , 1usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_ca_state ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_retransmits as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_retransmits ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_probes as * const _ as usize } , 3usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_probes ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_backoff as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_backoff ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_options as * const _ as usize } , 5usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_options ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rto as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rto ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_ato as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_ato ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_snd_mss as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_snd_mss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rcv_mss as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rcv_mss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_unacked as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_unacked ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_sacked as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_sacked ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_lost as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_lost ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_retrans as * const _ as usize } , 36usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_retrans ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_fackets as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_fackets ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_last_data_sent as * const _ as usize } , 44usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_last_data_sent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_last_ack_sent as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_last_ack_sent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_last_data_recv as * const _ as usize } , 52usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_last_data_recv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_last_ack_recv as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_last_ack_recv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_pmtu as * const _ as usize } , 60usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_pmtu ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rcv_ssthresh as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rcv_ssthresh ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rtt as * const _ as usize } , 68usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rtt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rttvar as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rttvar ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_snd_ssthresh as * const _ as usize } , 76usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_snd_ssthresh ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_snd_cwnd as * const _ as usize } , 80usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_snd_cwnd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_advmss as * const _ as usize } , 84usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_advmss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_reordering as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_reordering ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rcv_rtt as * const _ as usize } , 92usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rcv_rtt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_rcv_space as * const _ as usize } , 96usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_rcv_space ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_info > ( ) ) ) . tcpi_total_retrans as * const _ as usize } , 100usize , concat ! ( "Offset of field: " , stringify ! ( tcp_info ) , "::" , stringify ! ( tcpi_total_retrans ) )) ;

}
 impl tcp_info {
# [inline] pub fn tcpi_snd_wscale (& self) -> u8 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 0usize , 4u8 ) as u8)
}

}
 # [inline] pub fn set_tcpi_snd_wscale (& mut self , val : u8) {
unsafe {
let val : u8 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (0usize , 4u8 , val as u64)
}

}
 # [inline] pub fn tcpi_rcv_wscale (& self) -> u8 {
unsafe {
:: std :: mem :: transmute (self . _bitfield_1 . get ( 4usize , 4u8 ) as u8)
}

}
 # [inline] pub fn set_tcpi_rcv_wscale (& mut self , val : u8) {
unsafe {
let val : u8 = :: std :: mem :: transmute (val) ;
 self . _bitfield_1 . set (4usize , 4u8 , val as u64)
}

}
 # [inline] pub fn new_bitfield_1 (tcpi_snd_wscale : u8 , tcpi_rcv_wscale : u8) -> __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 1usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit . set (0usize , 4u8 , { let tcpi_snd_wscale : u8 = unsafe { :: std :: mem :: transmute ( tcpi_snd_wscale ) } ; tcpi_snd_wscale as u64 }) ;
 __bindgen_bitfield_unit . set (4usize , 4u8 , { let tcpi_rcv_wscale : u8 = unsafe { :: std :: mem :: transmute ( tcpi_rcv_wscale ) } ; tcpi_rcv_wscale as u64 }) ;
 __bindgen_bitfield_unit
}

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct tcp_md5sig {
pub tcpm_addr : sockaddr_storage , pub tcpm_flags : u8 , pub tcpm_prefixlen : u8 , pub tcpm_keylen : u16 , pub __tcpm_pad : u32 , pub tcpm_key : [u8 ; 80usize] ,
}
 # [test] fn bindgen_test_layout_tcp_md5sig () {
assert_eq ! (:: std :: mem :: size_of :: < tcp_md5sig > ( ) , 216usize , concat ! ( "Size of: " , stringify ! ( tcp_md5sig ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcp_md5sig > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( tcp_md5sig ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_md5sig > ( ) ) ) . tcpm_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcp_md5sig ) , "::" , stringify ! ( tcpm_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_md5sig > ( ) ) ) . tcpm_flags as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( tcp_md5sig ) , "::" , stringify ! ( tcpm_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_md5sig > ( ) ) ) . tcpm_prefixlen as * const _ as usize } , 129usize , concat ! ( "Offset of field: " , stringify ! ( tcp_md5sig ) , "::" , stringify ! ( tcpm_prefixlen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_md5sig > ( ) ) ) . tcpm_keylen as * const _ as usize } , 130usize , concat ! ( "Offset of field: " , stringify ! ( tcp_md5sig ) , "::" , stringify ! ( tcpm_keylen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_md5sig > ( ) ) ) . __tcpm_pad as * const _ as usize } , 132usize , concat ! ( "Offset of field: " , stringify ! ( tcp_md5sig ) , "::" , stringify ! ( __tcpm_pad ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_md5sig > ( ) ) ) . tcpm_key as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( tcp_md5sig ) , "::" , stringify ! ( tcpm_key ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct tcp_repair_opt {
pub opt_code : u32 , pub opt_val : u32 ,
}
 # [test] fn bindgen_test_layout_tcp_repair_opt () {
assert_eq ! (:: std :: mem :: size_of :: < tcp_repair_opt > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( tcp_repair_opt ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcp_repair_opt > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcp_repair_opt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_opt > ( ) ) ) . opt_code as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_opt ) , "::" , stringify ! ( opt_code ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_opt > ( ) ) ) . opt_val as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_opt ) , "::" , stringify ! ( opt_val ) )) ;

}
 pub const TCP_NO_QUEUE : _bindgen_ty_8 = 0 ;
 pub const TCP_RECV_QUEUE : _bindgen_ty_8 = 1 ;
 pub const TCP_SEND_QUEUE : _bindgen_ty_8 = 2 ;
 pub const TCP_QUEUES_NR : _bindgen_ty_8 = 3 ;
 pub type _bindgen_ty_8 = u32 ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct tcp_cookie_transactions {
pub tcpct_flags : u16 , pub __tcpct_pad1 : u8 , pub tcpct_cookie_desired : u8 , pub tcpct_s_data_desired : u16 , pub tcpct_used : u16 , pub tcpct_value : [u8 ; 536usize] ,
}
 # [test] fn bindgen_test_layout_tcp_cookie_transactions () {
assert_eq ! (:: std :: mem :: size_of :: < tcp_cookie_transactions > ( ) , 544usize , concat ! ( "Size of: " , stringify ! ( tcp_cookie_transactions ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcp_cookie_transactions > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( tcp_cookie_transactions ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_cookie_transactions > ( ) ) ) . tcpct_flags as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcp_cookie_transactions ) , "::" , stringify ! ( tcpct_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_cookie_transactions > ( ) ) ) . __tcpct_pad1 as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( tcp_cookie_transactions ) , "::" , stringify ! ( __tcpct_pad1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_cookie_transactions > ( ) ) ) . tcpct_cookie_desired as * const _ as usize } , 3usize , concat ! ( "Offset of field: " , stringify ! ( tcp_cookie_transactions ) , "::" , stringify ! ( tcpct_cookie_desired ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_cookie_transactions > ( ) ) ) . tcpct_s_data_desired as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tcp_cookie_transactions ) , "::" , stringify ! ( tcpct_s_data_desired ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_cookie_transactions > ( ) ) ) . tcpct_used as * const _ as usize } , 6usize , concat ! ( "Offset of field: " , stringify ! ( tcp_cookie_transactions ) , "::" , stringify ! ( tcpct_used ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_cookie_transactions > ( ) ) ) . tcpct_value as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( tcp_cookie_transactions ) , "::" , stringify ! ( tcpct_value ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct tcp_repair_window {
pub snd_wl1 : u32 , pub snd_wnd : u32 , pub max_window : u32 , pub rcv_wnd : u32 , pub rcv_wup : u32 ,
}
 # [test] fn bindgen_test_layout_tcp_repair_window () {
assert_eq ! (:: std :: mem :: size_of :: < tcp_repair_window > ( ) , 20usize , concat ! ( "Size of: " , stringify ! ( tcp_repair_window ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tcp_repair_window > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( tcp_repair_window ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_window > ( ) ) ) . snd_wl1 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_window ) , "::" , stringify ! ( snd_wl1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_window > ( ) ) ) . snd_wnd as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_window ) , "::" , stringify ! ( snd_wnd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_window > ( ) ) ) . max_window as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_window ) , "::" , stringify ! ( max_window ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_window > ( ) ) ) . rcv_wnd as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_window ) , "::" , stringify ! ( rcv_wnd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tcp_repair_window > ( ) ) ) . rcv_wup as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( tcp_repair_window ) , "::" , stringify ! ( rcv_wup ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct hash_node {
pub p_key : * mut :: std :: os :: raw :: c_void , pub p_value : * mut :: std :: os :: raw :: c_void , pub p_prev : * mut hash_node , pub p_next : * mut hash_node ,
}
 # [test] fn bindgen_test_layout_hash_node () {
assert_eq ! (:: std :: mem :: size_of :: < hash_node > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( hash_node ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < hash_node > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( hash_node ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash_node > ( ) ) ) . p_key as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( hash_node ) , "::" , stringify ! ( p_key ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash_node > ( ) ) ) . p_value as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( hash_node ) , "::" , stringify ! ( p_value ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash_node > ( ) ) ) . p_prev as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( hash_node ) , "::" , stringify ! ( p_prev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash_node > ( ) ) ) . p_next as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( hash_node ) , "::" , stringify ! ( p_next ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct hash {
pub buckets : :: std :: os :: raw :: c_uint , pub key_size : :: std :: os :: raw :: c_uint , pub value_size : :: std :: os :: raw :: c_uint , pub hash_func : hashfunc_t , pub p_nodes : * mut * mut hash_node ,
}
 # [test] fn bindgen_test_layout_hash () {
assert_eq ! (:: std :: mem :: size_of :: < hash > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( hash ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < hash > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( hash ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash > ( ) ) ) . buckets as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( hash ) , "::" , stringify ! ( buckets ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash > ( ) ) ) . key_size as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( hash ) , "::" , stringify ! ( key_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash > ( ) ) ) . value_size as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( hash ) , "::" , stringify ! ( value_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash > ( ) ) ) . hash_func as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( hash ) , "::" , stringify ! ( hash_func ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hash > ( ) ) ) . p_nodes as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( hash ) , "::" , stringify ! ( p_nodes ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL18s_p_saved_filename"] 
pub static mut s_p_saved_filename : * const :: std :: os :: raw :: c_char ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct parseconf_bool_setting {
pub p_setting_name : * const :: std :: os :: raw :: c_char , pub p_variable : * mut :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_parseconf_bool_setting () {
assert_eq ! (:: std :: mem :: size_of :: < parseconf_bool_setting > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( parseconf_bool_setting ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < parseconf_bool_setting > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( parseconf_bool_setting ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < parseconf_bool_setting > ( ) ) ) . p_setting_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( parseconf_bool_setting ) , "::" , stringify ! ( p_setting_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < parseconf_bool_setting > ( ) ) ) . p_variable as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( parseconf_bool_setting ) , "::" , stringify ! ( p_variable ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL20parseconf_bool_array"] 
pub static mut parseconf_bool_array : [parseconf_bool_setting ; 81usize] ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct parseconf_uint_setting {
pub p_setting_name : * const :: std :: os :: raw :: c_char , pub p_variable : * mut :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_parseconf_uint_setting () {
assert_eq ! (:: std :: mem :: size_of :: < parseconf_uint_setting > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( parseconf_uint_setting ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < parseconf_uint_setting > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( parseconf_uint_setting ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < parseconf_uint_setting > ( ) ) ) . p_setting_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( parseconf_uint_setting ) , "::" , stringify ! ( p_setting_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < parseconf_uint_setting > ( ) ) ) . p_variable as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( parseconf_uint_setting ) , "::" , stringify ! ( p_variable ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL20parseconf_uint_array"] 
pub static mut parseconf_uint_array : [parseconf_uint_setting ; 21usize] ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct parseconf_str_setting {
pub p_setting_name : * const :: std :: os :: raw :: c_char , pub p_variable : * mut * const :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_parseconf_str_setting () {
assert_eq ! (:: std :: mem :: size_of :: < parseconf_str_setting > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( parseconf_str_setting ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < parseconf_str_setting > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( parseconf_str_setting ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < parseconf_str_setting > ( ) ) ) . p_setting_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( parseconf_str_setting ) , "::" , stringify ! ( p_setting_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < parseconf_str_setting > ( ) ) ) . p_variable as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( parseconf_str_setting ) , "::" , stringify ! ( p_variable ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL19parseconf_str_array"] 
pub static mut parseconf_str_array : [parseconf_str_setting ; 33usize] ;

}

 extern "C" {
pub static mut program_invocation_name : * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut program_invocation_short_name : * mut :: std :: os :: raw :: c_char ;

}
 pub type error_t = :: std :: os :: raw :: c_int ;
 # [repr ( C )] # [derive ( Default, Debug , Copy , Clone )] pub struct flock {
pub l_type : :: std :: os :: raw :: c_short , pub l_whence : :: std :: os :: raw :: c_short , pub l_start : __off_t , pub l_len : __off_t , pub l_pid : __pid_t ,
}
 # [test] fn bindgen_test_layout_flock () {
assert_eq ! (:: std :: mem :: size_of :: < flock > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( flock ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < flock > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( flock ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock > ( ) ) ) . l_type as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( flock ) , "::" , stringify ! ( l_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock > ( ) ) ) . l_whence as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( flock ) , "::" , stringify ! ( l_whence ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock > ( ) ) ) . l_start as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( flock ) , "::" , stringify ! ( l_start ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock > ( ) ) ) . l_len as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( flock ) , "::" , stringify ! ( l_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock > ( ) ) ) . l_pid as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( flock ) , "::" , stringify ! ( l_pid ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct flock64 {
pub l_type : :: std :: os :: raw :: c_short , pub l_whence : :: std :: os :: raw :: c_short , pub l_start : __off64_t , pub l_len : __off64_t , pub l_pid : __pid_t ,
}
 # [test] fn bindgen_test_layout_flock64 () {
assert_eq ! (:: std :: mem :: size_of :: < flock64 > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( flock64 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < flock64 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( flock64 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock64 > ( ) ) ) . l_type as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( flock64 ) , "::" , stringify ! ( l_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock64 > ( ) ) ) . l_whence as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( flock64 ) , "::" , stringify ! ( l_whence ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock64 > ( ) ) ) . l_start as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( flock64 ) , "::" , stringify ! ( l_start ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock64 > ( ) ) ) . l_len as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( flock64 ) , "::" , stringify ! ( l_len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < flock64 > ( ) ) ) . l_pid as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( flock64 ) , "::" , stringify ! ( l_pid ) )) ;

}
 pub const __pid_type_F_OWNER_TID : __pid_type = 0 ;
 pub const __pid_type_F_OWNER_PID : __pid_type = 1 ;
 pub const __pid_type_F_OWNER_PGRP : __pid_type = 2 ;
 pub const __pid_type_F_OWNER_GID : __pid_type = 2 ;
 pub type __pid_type = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct f_owner_ex {
pub type_ : __pid_type , pub pid : __pid_t ,
}
 # [test] fn bindgen_test_layout_f_owner_ex () {
assert_eq ! (:: std :: mem :: size_of :: < f_owner_ex > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( f_owner_ex ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < f_owner_ex > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( f_owner_ex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < f_owner_ex > ( ) ) ) . type_ as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( f_owner_ex ) , "::" , stringify ! ( type_ ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < f_owner_ex > ( ) ) ) . pid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( f_owner_ex ) , "::" , stringify ! ( pid ) )) ;

}
 # [repr ( C )] # [derive ( Debug )] pub struct file_handle {
pub handle_bytes : :: std :: os :: raw :: c_uint , pub handle_type : :: std :: os :: raw :: c_int , pub f_handle : __IncompleteArrayField < :: std :: os :: raw :: c_uchar > ,
}
 # [test] fn bindgen_test_layout_file_handle () {
assert_eq ! (:: std :: mem :: size_of :: < file_handle > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( file_handle ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < file_handle > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( file_handle ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < file_handle > ( ) ) ) . handle_bytes as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( file_handle ) , "::" , stringify ! ( handle_bytes ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < file_handle > ( ) ) ) . handle_type as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( file_handle ) , "::" , stringify ! ( handle_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < file_handle > ( ) ) ) . f_handle as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( file_handle ) , "::" , stringify ! ( f_handle ) )) ;

}
 extern "C" {
pub fn readahead (__fd : :: std :: os :: raw :: c_int , __offset : __off64_t , __count : usize) -> __ssize_t ;

}
 extern "C" {
pub fn sync_file_range (__fd : :: std :: os :: raw :: c_int , __offset : __off64_t , __count : __off64_t , __flags : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vmsplice (__fdout : :: std :: os :: raw :: c_int , __iov : * const iovec , __count : usize , __flags : :: std :: os :: raw :: c_uint) -> __ssize_t ;

}
 extern "C" {
pub fn splice (__fdin : :: std :: os :: raw :: c_int , __offin : * mut __off64_t , __fdout : :: std :: os :: raw :: c_int , __offout : * mut __off64_t , __len : usize , __flags : :: std :: os :: raw :: c_uint) -> __ssize_t ;

}
 extern "C" {
pub fn tee (__fdin : :: std :: os :: raw :: c_int , __fdout : :: std :: os :: raw :: c_int , __len : usize , __flags : :: std :: os :: raw :: c_uint) -> __ssize_t ;

}
 extern "C" {
pub fn fallocate (__fd : :: std :: os :: raw :: c_int , __mode : :: std :: os :: raw :: c_int , __offset : __off_t , __len : __off_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fallocate64 (__fd : :: std :: os :: raw :: c_int , __mode : :: std :: os :: raw :: c_int , __offset : __off64_t , __len : __off64_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn name_to_handle_at (__dfd : :: std :: os :: raw :: c_int , __name : * const :: std :: os :: raw :: c_char , __handle : * mut file_handle , __mnt_id : * mut :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn open_by_handle_at (__mountdirfd : :: std :: os :: raw :: c_int , __handle : * mut file_handle , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct stat {
pub st_dev : __dev_t , pub st_ino : __ino_t , pub st_nlink : __nlink_t , pub st_mode : __mode_t , pub st_uid : __uid_t , pub st_gid : __gid_t , pub __pad0 : :: std :: os :: raw :: c_int , pub st_rdev : __dev_t , pub st_size : __off_t , pub st_blksize : __blksize_t , pub st_blocks : __blkcnt_t , pub st_atim : timespec , pub st_mtim : timespec , pub st_ctim : timespec , pub __glibc_reserved : [__syscall_slong_t ; 3usize] ,
}
 # [test] fn bindgen_test_layout_stat () {
assert_eq ! (:: std :: mem :: size_of :: < stat > ( ) , 144usize , concat ! ( "Size of: " , stringify ! ( stat ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < stat > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( stat ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_dev as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_dev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_ino as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_ino ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_nlink as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_nlink ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_mode as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_mode ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_uid as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_gid as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_gid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . __pad0 as * const _ as usize } , 36usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( __pad0 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_rdev as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_rdev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_size as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_blksize as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_blksize ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_blocks as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_blocks ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_atim as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_atim ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_mtim as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_mtim ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . st_ctim as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( st_ctim ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat > ( ) ) ) . __glibc_reserved as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( stat ) , "::" , stringify ! ( __glibc_reserved ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct stat64 {
pub st_dev : __dev_t , pub st_ino : __ino64_t , pub st_nlink : __nlink_t , pub st_mode : __mode_t , pub st_uid : __uid_t , pub st_gid : __gid_t , pub __pad0 : :: std :: os :: raw :: c_int , pub st_rdev : __dev_t , pub st_size : __off_t , pub st_blksize : __blksize_t , pub st_blocks : __blkcnt64_t , pub st_atim : timespec , pub st_mtim : timespec , pub st_ctim : timespec , pub __glibc_reserved : [__syscall_slong_t ; 3usize] ,
}
 # [test] fn bindgen_test_layout_stat64 () {
assert_eq ! (:: std :: mem :: size_of :: < stat64 > ( ) , 144usize , concat ! ( "Size of: " , stringify ! ( stat64 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < stat64 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( stat64 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_dev as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_dev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_ino as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_ino ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_nlink as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_nlink ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_mode as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_mode ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_uid as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_gid as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_gid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . __pad0 as * const _ as usize } , 36usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( __pad0 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_rdev as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_rdev ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_size as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_blksize as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_blksize ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_blocks as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_blocks ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_atim as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_atim ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_mtim as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_mtim ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . st_ctim as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( st_ctim ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stat64 > ( ) ) ) . __glibc_reserved as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( stat64 ) , "::" , stringify ! ( __glibc_reserved ) )) ;

}
 extern "C" {
pub fn fcntl (__fd : :: std :: os :: raw :: c_int , __cmd : :: std :: os :: raw :: c_int , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn open (__file : * const :: std :: os :: raw :: c_char , __oflag : :: std :: os :: raw :: c_int , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn open64 (__file : * const :: std :: os :: raw :: c_char , __oflag : :: std :: os :: raw :: c_int , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn openat (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __oflag : :: std :: os :: raw :: c_int , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn openat64 (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __oflag : :: std :: os :: raw :: c_int , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn creat (__file : * const :: std :: os :: raw :: c_char , __mode : mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn creat64 (__file : * const :: std :: os :: raw :: c_char , __mode : mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lockf (__fd : :: std :: os :: raw :: c_int , __cmd : :: std :: os :: raw :: c_int , __len : off_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lockf64 (__fd : :: std :: os :: raw :: c_int , __cmd : :: std :: os :: raw :: c_int , __len : off64_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn posix_fadvise (__fd : :: std :: os :: raw :: c_int , __offset : off_t , __len : off_t , __advise : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn posix_fadvise64 (__fd : :: std :: os :: raw :: c_int , __offset : off64_t , __len : off64_t , __advise : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn posix_fallocate (__fd : :: std :: os :: raw :: c_int , __offset : off_t , __len : off_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn posix_fallocate64 (__fd : :: std :: os :: raw :: c_int , __offset : off64_t , __len : off64_t) -> :: std :: os :: raw :: c_int ;

}
 pub type __s8 = :: std :: os :: raw :: c_schar ;
 pub type __u8 = :: std :: os :: raw :: c_uchar ;
 pub type __s16 = :: std :: os :: raw :: c_short ;
 pub type __u16 = :: std :: os :: raw :: c_ushort ;
 pub type __s32 = :: std :: os :: raw :: c_int ;
 pub type __u32 = :: std :: os :: raw :: c_uint ;
 pub type __s64 = :: std :: os :: raw :: c_longlong ;
 pub type __u64 = :: std :: os :: raw :: c_ulonglong ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __kernel_fd_set {
pub fds_bits : [:: std :: os :: raw :: c_ulong ; 16usize] ,
}
 # [test] fn bindgen_test_layout___kernel_fd_set () {
assert_eq ! (:: std :: mem :: size_of :: < __kernel_fd_set > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( __kernel_fd_set ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __kernel_fd_set > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __kernel_fd_set ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __kernel_fd_set > ( ) ) ) . fds_bits as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __kernel_fd_set ) , "::" , stringify ! ( fds_bits ) )) ;

}
 pub type __kernel_sighandler_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int) > ;
 pub type __kernel_key_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_mqd_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_old_uid_t = :: std :: os :: raw :: c_ushort ;
 pub type __kernel_old_gid_t = :: std :: os :: raw :: c_ushort ;
 pub type __kernel_old_dev_t = :: std :: os :: raw :: c_ulong ;
 pub type __kernel_long_t = :: std :: os :: raw :: c_long ;
 pub type __kernel_ulong_t = :: std :: os :: raw :: c_ulong ;
 pub type __kernel_ino_t = __kernel_ulong_t ;
 pub type __kernel_mode_t = :: std :: os :: raw :: c_uint ;
 pub type __kernel_pid_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_ipc_pid_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_uid_t = :: std :: os :: raw :: c_uint ;
 pub type __kernel_gid_t = :: std :: os :: raw :: c_uint ;
 pub type __kernel_suseconds_t = __kernel_long_t ;
 pub type __kernel_daddr_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_uid32_t = :: std :: os :: raw :: c_uint ;
 pub type __kernel_gid32_t = :: std :: os :: raw :: c_uint ;
 pub type __kernel_size_t = __kernel_ulong_t ;
 pub type __kernel_ssize_t = __kernel_long_t ;
 pub type __kernel_ptrdiff_t = __kernel_long_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __kernel_fsid_t {
pub val : [:: std :: os :: raw :: c_int ; 2usize] ,
}
 # [test] fn bindgen_test_layout___kernel_fsid_t () {
assert_eq ! (:: std :: mem :: size_of :: < __kernel_fsid_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __kernel_fsid_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __kernel_fsid_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __kernel_fsid_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __kernel_fsid_t > ( ) ) ) . val as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __kernel_fsid_t ) , "::" , stringify ! ( val ) )) ;

}
 pub type __kernel_off_t = __kernel_long_t ;
 pub type __kernel_loff_t = :: std :: os :: raw :: c_longlong ;
 pub type __kernel_time_t = __kernel_long_t ;
 pub type __kernel_clock_t = __kernel_long_t ;
 pub type __kernel_timer_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_clockid_t = :: std :: os :: raw :: c_int ;
 pub type __kernel_caddr_t = * mut :: std :: os :: raw :: c_char ;
 pub type __kernel_uid16_t = :: std :: os :: raw :: c_ushort ;
 pub type __kernel_gid16_t = :: std :: os :: raw :: c_ushort ;
 pub type __le16 = __u16 ;
 pub type __be16 = __u16 ;
 pub type __le32 = __u32 ;
 pub type __be32 = __u32 ;
 pub type __le64 = __u64 ;
 pub type __be64 = __u64 ;
 pub type __sum16 = __u16 ;
 pub type __wsum = __u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct prctl_mm_map {
pub start_code : __u64 , pub end_code : __u64 , pub start_data : __u64 , pub end_data : __u64 , pub start_brk : __u64 , pub brk : __u64 , pub start_stack : __u64 , pub arg_start : __u64 , pub arg_end : __u64 , pub env_start : __u64 , pub env_end : __u64 , pub auxv : * mut __u64 , pub auxv_size : __u32 , pub exe_fd : __u32 ,
}
 # [test] fn bindgen_test_layout_prctl_mm_map () {
assert_eq ! (:: std :: mem :: size_of :: < prctl_mm_map > ( ) , 104usize , concat ! ( "Size of: " , stringify ! ( prctl_mm_map ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < prctl_mm_map > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( prctl_mm_map ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . start_code as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( start_code ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . end_code as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( end_code ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . start_data as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( start_data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . end_data as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( end_data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . start_brk as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( start_brk ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . brk as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( brk ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . start_stack as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( start_stack ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . arg_start as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( arg_start ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . arg_end as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( arg_end ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . env_start as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( env_start ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . env_end as * const _ as usize } , 80usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( env_end ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . auxv as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( auxv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . auxv_size as * const _ as usize } , 96usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( auxv_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < prctl_mm_map > ( ) ) ) . exe_fd as * const _ as usize } , 100usize , concat ! ( "Offset of field: " , stringify ! ( prctl_mm_map ) , "::" , stringify ! ( exe_fd ) )) ;

}
 extern "C" {
pub fn prctl (__option : :: std :: os :: raw :: c_int , ...) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sock_filter {
pub code : __u16 , pub jt : __u8 , pub jf : __u8 , pub k : __u32 ,
}
 # [test] fn bindgen_test_layout_sock_filter () {
assert_eq ! (:: std :: mem :: size_of :: < sock_filter > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( sock_filter ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sock_filter > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( sock_filter ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sock_filter > ( ) ) ) . code as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sock_filter ) , "::" , stringify ! ( code ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sock_filter > ( ) ) ) . jt as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( sock_filter ) , "::" , stringify ! ( jt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sock_filter > ( ) ) ) . jf as * const _ as usize } , 3usize , concat ! ( "Offset of field: " , stringify ! ( sock_filter ) , "::" , stringify ! ( jf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sock_filter > ( ) ) ) . k as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( sock_filter ) , "::" , stringify ! ( k ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sock_fprog {
pub len : :: std :: os :: raw :: c_ushort , pub filter : * mut sock_filter ,
}
 # [test] fn bindgen_test_layout_sock_fprog () {
assert_eq ! (:: std :: mem :: size_of :: < sock_fprog > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( sock_fprog ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sock_fprog > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sock_fprog ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sock_fprog > ( ) ) ) . len as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sock_fprog ) , "::" , stringify ! ( len ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sock_fprog > ( ) ) ) . filter as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sock_fprog ) , "::" , stringify ! ( filter ) )) ;

}
 pub const kOpenFlags : :: std :: os :: raw :: c_int = 593088 ;
 extern "C" {
//# [link_name = "\u{1}_ZL15s_syscall_index"] 
pub static mut s_syscall_index : usize ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL19s_1_arg_validations"] 
pub static mut s_1_arg_validations : usize ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL19s_2_arg_validations"] 
pub static mut s_2_arg_validations : usize ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL19s_3_arg_validations"] 
pub static mut s_3_arg_validations : usize ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL10s_syscalls"] 
pub static mut s_syscalls : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_errnos"] 
pub static mut s_errnos : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_args_1"] 
pub static mut s_args_1 : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_vals_1"] 
pub static mut s_vals_1 : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_args_2"] 
pub static mut s_args_2 : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_vals_2"] 
pub static mut s_vals_2 : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_args_3"] 
pub static mut s_args_3 : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL8s_vals_3"] 
pub static mut s_vals_3 : [:: std :: os :: raw :: c_int ; 100usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL10s_children"] 
pub static mut s_children : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL17s_p_ip_count_hash"] 
pub static mut s_p_ip_count_hash : * mut hash ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL15s_p_pid_ip_hash"] 
pub static mut s_p_pid_ip_hash : * mut hash ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL13s_ipaddr_size"] 
pub static mut s_ipaddr_size : :: std :: os :: raw :: c_uint ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mystr_list_node {
pub str : mystr , pub sort_key_str : mystr ,
}
 # [test] fn bindgen_test_layout_mystr_list_node () {
assert_eq ! (:: std :: mem :: size_of :: < mystr_list_node > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( mystr_list_node ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mystr_list_node > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mystr_list_node ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list_node > ( ) ) ) . str as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list_node ) , "::" , stringify ! ( str ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mystr_list_node > ( ) ) ) . sort_key_str as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( mystr_list_node ) , "::" , stringify ! ( sort_key_str ) )) ;

}
 pub const kMaxStrlist : :: std :: os :: raw :: c_uint = 10000000 ;
 extern "C" {
//# [link_name = "\u{1}_ZL10s_null_str"] 
pub static mut s_null_str : mystr ;

}
 pub type sig_atomic_t = __sig_atomic_t ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub union sigval {
pub sival_int : :: std :: os :: raw :: c_int , pub sival_ptr : * mut :: std :: os :: raw :: c_void , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_sigval () {
assert_eq ! (:: std :: mem :: size_of :: < sigval > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( sigval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigval > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigval > ( ) ) ) . sival_int as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigval ) , "::" , stringify ! ( sival_int ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigval > ( ) ) ) . sival_ptr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigval ) , "::" , stringify ! ( sival_ptr ) )) ;

}
 pub type __sigval_t = sigval ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct siginfo_t {
pub si_signo : :: std :: os :: raw :: c_int , pub si_errno : :: std :: os :: raw :: c_int , pub si_code : :: std :: os :: raw :: c_int , pub __pad0 : :: std :: os :: raw :: c_int , pub _sifields : siginfo_t__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union siginfo_t__bindgen_ty_1 {
pub _pad : [:: std :: os :: raw :: c_int ; 28usize] , pub _kill : siginfo_t__bindgen_ty_1__bindgen_ty_1 , pub _timer : siginfo_t__bindgen_ty_1__bindgen_ty_2 , pub _rt : siginfo_t__bindgen_ty_1__bindgen_ty_3 , pub _sigchld : siginfo_t__bindgen_ty_1__bindgen_ty_4 , pub _sigfault : siginfo_t__bindgen_ty_1__bindgen_ty_5 , pub _sigpoll : siginfo_t__bindgen_ty_1__bindgen_ty_6 , pub _sigsys : siginfo_t__bindgen_ty_1__bindgen_ty_7 , _bindgen_union_align : [u64 ; 14usize] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_1 {
pub si_pid : __pid_t , pub si_uid : __uid_t ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . si_pid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( si_pid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . si_uid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( si_uid ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_2 {
pub si_tid : :: std :: os :: raw :: c_int , pub si_overrun : :: std :: os :: raw :: c_int , pub si_sigval : __sigval_t ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_2 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_2 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_2 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . si_tid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( si_tid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . si_overrun as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( si_overrun ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_2 > ( ) ) ) . si_sigval as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_2 ) , "::" , stringify ! ( si_sigval ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_3 {
pub si_pid : __pid_t , pub si_uid : __uid_t , pub si_sigval : __sigval_t ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_3 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_3 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_3 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_3 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_3 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_3 > ( ) ) ) . si_pid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_3 ) , "::" , stringify ! ( si_pid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_3 > ( ) ) ) . si_uid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_3 ) , "::" , stringify ! ( si_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_3 > ( ) ) ) . si_sigval as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_3 ) , "::" , stringify ! ( si_sigval ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_4 {
pub si_pid : __pid_t , pub si_uid : __uid_t , pub si_status : :: std :: os :: raw :: c_int , pub si_utime : __clock_t , pub si_stime : __clock_t ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_4 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) ) ) . si_pid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) , "::" , stringify ! ( si_pid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) ) ) . si_uid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) , "::" , stringify ! ( si_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) ) ) . si_status as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) , "::" , stringify ! ( si_status ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) ) ) . si_utime as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) , "::" , stringify ! ( si_utime ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_4 > ( ) ) ) . si_stime as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_4 ) , "::" , stringify ! ( si_stime ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_5 {
pub si_addr : * mut :: std :: os :: raw :: c_void , pub si_addr_lsb : :: std :: os :: raw :: c_short , pub _bounds : siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
pub _addr_bnd : siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 , pub _pkey : __uint32_t , _bindgen_union_align : [u64 ; 2usize] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
pub _lower : * mut :: std :: os :: raw :: c_void , pub _upper : * mut :: std :: os :: raw :: c_void ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . _lower as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( _lower ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . _upper as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( _upper ) )) ;

}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 > ( ) ) ) . _addr_bnd as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 ) , "::" , stringify ! ( _addr_bnd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 > ( ) ) ) . _pkey as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 ) , "::" , stringify ! ( _pkey ) )) ;

}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_5 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_5 > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_5 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5 > ( ) ) ) . si_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5 ) , "::" , stringify ! ( si_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5 > ( ) ) ) . si_addr_lsb as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5 ) , "::" , stringify ! ( si_addr_lsb ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_5 > ( ) ) ) . _bounds as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_5 ) , "::" , stringify ! ( _bounds ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_6 {
pub si_band : :: std :: os :: raw :: c_long , pub si_fd : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_6 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_6 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_6 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_6 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_6 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_6 > ( ) ) ) . si_band as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_6 ) , "::" , stringify ! ( si_band ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_6 > ( ) ) ) . si_fd as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_6 ) , "::" , stringify ! ( si_fd ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct siginfo_t__bindgen_ty_1__bindgen_ty_7 {
pub _call_addr : * mut :: std :: os :: raw :: c_void , pub _syscall : :: std :: os :: raw :: c_int , pub _arch : :: std :: os :: raw :: c_uint ,
}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1__bindgen_ty_7 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_7 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_7 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1__bindgen_ty_7 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_7 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_7 > ( ) ) ) . _call_addr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_7 ) , "::" , stringify ! ( _call_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_7 > ( ) ) ) . _syscall as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_7 ) , "::" , stringify ! ( _syscall ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1__bindgen_ty_7 > ( ) ) ) . _arch as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1__bindgen_ty_7 ) , "::" , stringify ! ( _arch ) )) ;

}
 # [test] fn bindgen_test_layout_siginfo_t__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t__bindgen_ty_1 > ( ) , 112usize , concat ! ( "Size of: " , stringify ! ( siginfo_t__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _pad as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _pad ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _kill as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _kill ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _timer as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _timer ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _rt as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _rt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _sigchld as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _sigchld ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _sigfault as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _sigfault ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _sigpoll as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _sigpoll ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t__bindgen_ty_1 > ( ) ) ) . _sigsys as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t__bindgen_ty_1 ) , "::" , stringify ! ( _sigsys ) )) ;

}
 # [test] fn bindgen_test_layout_siginfo_t () {
assert_eq ! (:: std :: mem :: size_of :: < siginfo_t > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( siginfo_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < siginfo_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( siginfo_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t > ( ) ) ) . si_signo as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t ) , "::" , stringify ! ( si_signo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t > ( ) ) ) . si_errno as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t ) , "::" , stringify ! ( si_errno ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t > ( ) ) ) . si_code as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t ) , "::" , stringify ! ( si_code ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t > ( ) ) ) . __pad0 as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t ) , "::" , stringify ! ( __pad0 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < siginfo_t > ( ) ) ) . _sifields as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( siginfo_t ) , "::" , stringify ! ( _sifields ) )) ;

}
 pub const SI_ASYNCNL : _bindgen_ty_9 = - 60 ;
 pub const SI_TKILL : _bindgen_ty_9 = - 6 ;
 pub const SI_SIGIO : _bindgen_ty_9 = - 5 ;
 pub const SI_ASYNCIO : _bindgen_ty_9 = - 4 ;
 pub const SI_MESGQ : _bindgen_ty_9 = - 3 ;
 pub const SI_TIMER : _bindgen_ty_9 = - 2 ;
 pub const SI_QUEUE : _bindgen_ty_9 = - 1 ;
 pub const SI_USER : _bindgen_ty_9 = 0 ;
 pub const SI_KERNEL : _bindgen_ty_9 = 128 ;
 pub type _bindgen_ty_9 = i32 ;
 pub const ILL_ILLOPC : _bindgen_ty_10 = 1 ;
 pub const ILL_ILLOPN : _bindgen_ty_10 = 2 ;
 pub const ILL_ILLADR : _bindgen_ty_10 = 3 ;
 pub const ILL_ILLTRP : _bindgen_ty_10 = 4 ;
 pub const ILL_PRVOPC : _bindgen_ty_10 = 5 ;
 pub const ILL_PRVREG : _bindgen_ty_10 = 6 ;
 pub const ILL_COPROC : _bindgen_ty_10 = 7 ;
 pub const ILL_BADSTK : _bindgen_ty_10 = 8 ;
 pub type _bindgen_ty_10 = u32 ;
 pub const FPE_INTDIV : _bindgen_ty_11 = 1 ;
 pub const FPE_INTOVF : _bindgen_ty_11 = 2 ;
 pub const FPE_FLTDIV : _bindgen_ty_11 = 3 ;
 pub const FPE_FLTOVF : _bindgen_ty_11 = 4 ;
 pub const FPE_FLTUND : _bindgen_ty_11 = 5 ;
 pub const FPE_FLTRES : _bindgen_ty_11 = 6 ;
 pub const FPE_FLTINV : _bindgen_ty_11 = 7 ;
 pub const FPE_FLTSUB : _bindgen_ty_11 = 8 ;
 pub type _bindgen_ty_11 = u32 ;
 pub const SEGV_MAPERR : _bindgen_ty_12 = 1 ;
 pub const SEGV_ACCERR : _bindgen_ty_12 = 2 ;
 pub const SEGV_BNDERR : _bindgen_ty_12 = 3 ;
 pub const SEGV_PKUERR : _bindgen_ty_12 = 4 ;
 pub type _bindgen_ty_12 = u32 ;
 pub const BUS_ADRALN : _bindgen_ty_13 = 1 ;
 pub const BUS_ADRERR : _bindgen_ty_13 = 2 ;
 pub const BUS_OBJERR : _bindgen_ty_13 = 3 ;
 pub const BUS_MCEERR_AR : _bindgen_ty_13 = 4 ;
 pub const BUS_MCEERR_AO : _bindgen_ty_13 = 5 ;
 pub type _bindgen_ty_13 = u32 ;
 pub const TRAP_BRKPT : _bindgen_ty_14 = 1 ;
 pub const TRAP_TRACE : _bindgen_ty_14 = 2 ;
 pub type _bindgen_ty_14 = u32 ;
 pub const CLD_EXITED : _bindgen_ty_15 = 1 ;
 pub const CLD_KILLED : _bindgen_ty_15 = 2 ;
 pub const CLD_DUMPED : _bindgen_ty_15 = 3 ;
 pub const CLD_TRAPPED : _bindgen_ty_15 = 4 ;
 pub const CLD_STOPPED : _bindgen_ty_15 = 5 ;
 pub const CLD_CONTINUED : _bindgen_ty_15 = 6 ;
 pub type _bindgen_ty_15 = u32 ;
 pub const POLL_IN : _bindgen_ty_16 = 1 ;
 pub const POLL_OUT : _bindgen_ty_16 = 2 ;
 pub const POLL_MSG : _bindgen_ty_16 = 3 ;
 pub const POLL_ERR : _bindgen_ty_16 = 4 ;
 pub const POLL_PRI : _bindgen_ty_16 = 5 ;
 pub const POLL_HUP : _bindgen_ty_16 = 6 ;
 pub type _bindgen_ty_16 = u32 ;
 pub type sigval_t = __sigval_t ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct sigevent {
pub sigev_value : __sigval_t , pub sigev_signo : :: std :: os :: raw :: c_int , pub sigev_notify : :: std :: os :: raw :: c_int , pub _sigev_un : sigevent__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union sigevent__bindgen_ty_1 {
pub _pad : [:: std :: os :: raw :: c_int ; 12usize] , pub _tid : __pid_t , pub _sigev_thread : sigevent__bindgen_ty_1__bindgen_ty_1 , _bindgen_union_align : [u64 ; 6usize] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sigevent__bindgen_ty_1__bindgen_ty_1 {
pub _function : :: std :: option :: Option < unsafe extern "C" fn (arg1 : __sigval_t) > , pub _attribute : * mut pthread_attr_t ,
}
 # [test] fn bindgen_test_layout_sigevent__bindgen_ty_1__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < sigevent__bindgen_ty_1__bindgen_ty_1 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( sigevent__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigevent__bindgen_ty_1__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigevent__bindgen_ty_1__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . _function as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigevent__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( _function ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent__bindgen_ty_1__bindgen_ty_1 > ( ) ) ) . _attribute as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sigevent__bindgen_ty_1__bindgen_ty_1 ) , "::" , stringify ! ( _attribute ) )) ;

}
 # [test] fn bindgen_test_layout_sigevent__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < sigevent__bindgen_ty_1 > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( sigevent__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigevent__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigevent__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent__bindgen_ty_1 > ( ) ) ) . _pad as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigevent__bindgen_ty_1 ) , "::" , stringify ! ( _pad ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent__bindgen_ty_1 > ( ) ) ) . _tid as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigevent__bindgen_ty_1 ) , "::" , stringify ! ( _tid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent__bindgen_ty_1 > ( ) ) ) . _sigev_thread as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigevent__bindgen_ty_1 ) , "::" , stringify ! ( _sigev_thread ) )) ;

}
 # [test] fn bindgen_test_layout_sigevent () {
assert_eq ! (:: std :: mem :: size_of :: < sigevent > ( ) , 64usize , concat ! ( "Size of: " , stringify ! ( sigevent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigevent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigevent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent > ( ) ) ) . sigev_value as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigevent ) , "::" , stringify ! ( sigev_value ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent > ( ) ) ) . sigev_signo as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sigevent ) , "::" , stringify ! ( sigev_signo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent > ( ) ) ) . sigev_notify as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( sigevent ) , "::" , stringify ! ( sigev_notify ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigevent > ( ) ) ) . _sigev_un as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( sigevent ) , "::" , stringify ! ( _sigev_un ) )) ;

}
 pub type sigevent_t = sigevent ;
 pub const SIGEV_SIGNAL : _bindgen_ty_17 = 0 ;
 pub const SIGEV_NONE : _bindgen_ty_17 = 1 ;
 pub const SIGEV_THREAD : _bindgen_ty_17 = 2 ;
 pub const SIGEV_THREAD_ID : _bindgen_ty_17 = 4 ;
 pub type _bindgen_ty_17 = u32 ;
 use std::mem;

 pub type __sighandler_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int) > ;

extern "C" {
  pub static mut SIG_ERR:  __sighandler_t ;//=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(-1)) } ;	/* Error return.  */
  pub static mut SIG_DFL:  __sighandler_t ;//=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(0) ) } ;	/* Default action.  */
  pub static mut SIG_IGN:  __sighandler_t ;//=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(1) ) } ;	/* Ignore signal.  */
  pub static mut SIG_HOLD: __sighandler_t ;//=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(2) ) } ;  /* Add signal to hold mask.  */
}

//SIG_ERR = unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(1) ) };

/*
 pub const SIG_ERR:__sighandler_t=	 unsafe { Some( std::mem::const_transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(-1)  ) };	/* Error return.  */
 pub const SIG_DFL:__sighandler_t=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(0) ) };	/* Default action.  */
 pub const SIG_IGN:__sighandler_t=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(1) ) };	/* Ignore signal.  */
 pub const SIG_HOLD:__sighandler_t=	 unsafe { Some( std::mem::transmute::<isize,unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int)>(2) ) };  /* Add signal to hold mask.  */
*/

 extern "C" {
pub fn __sysv_signal (__sig : :: std :: os :: raw :: c_int , __handler : __sighandler_t) -> __sighandler_t ;

}
 extern "C" {
pub fn sysv_signal (__sig : :: std :: os :: raw :: c_int , __handler : __sighandler_t) -> __sighandler_t ;

}
 extern "C" {
pub fn signal (__sig : :: std :: os :: raw :: c_int , __handler : __sighandler_t) -> __sighandler_t ;

}
 extern "C" {
pub fn kill (__pid : __pid_t , __sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn killpg (__pgrp : __pid_t , __sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn raise (__sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ssignal (__sig : :: std :: os :: raw :: c_int , __handler : __sighandler_t) -> __sighandler_t ;

}
 extern "C" {
pub fn gsignal (__sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn psignal (__sig : :: std :: os :: raw :: c_int , __s : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
pub fn psiginfo (__pinfo : * const siginfo_t , __s : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
//# [link_name = "\u{1}__xpg_sigpause"] 
pub fn sigpause (__sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigblock (__mask : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigsetmask (__mask : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn siggetmask () -> :: std :: os :: raw :: c_int ;

}
 pub type sighandler_t = __sighandler_t ;
 pub type sig_t = __sighandler_t ;
 extern "C" {
pub fn sigemptyset (__set : * mut sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigfillset (__set : * mut sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigaddset (__set : * mut sigset_t , __signo : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigdelset (__set : * mut sigset_t , __signo : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigismember (__set : * const sigset_t , __signo : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigisemptyset (__set : * const sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigandset (__set : * mut sigset_t , __left : * const sigset_t , __right : * const sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigorset (__set : * mut sigset_t , __left : * const sigset_t , __right : * const sigset_t) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Default, Copy , Clone )] pub struct sigaction {
  pub sa_handler : __sighandler_t ,
   pub sa_sigaction : :: std :: option :: Option < unsafe extern "C" fn (arg1 : :: std :: os :: raw :: c_int , arg2 : * mut siginfo_t , arg3 : * mut :: std :: os :: raw :: c_void) > ,
   pub  _bindgen_union_align : u64 ,
  //pub __sigaction_handler : sigaction__bindgen_ty_1 ,
   pub sa_mask : __sigset_t , 
   pub sa_flags : :: std :: os :: raw :: c_int ,
    pub sa_restorer : :: std :: option :: Option < unsafe extern "C" fn () > ,
}
 //# [repr ( C )] # [derive ( Copy , Clone )] pub union sigaction__bindgen_ty_1 {

//}
 # [test] fn bindgen_test_layout_sigaction__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < sigaction__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( sigaction__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigaction__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigaction__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigaction__bindgen_ty_1 > ( ) ) ) . sa_handler as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigaction__bindgen_ty_1 ) , "::" , stringify ! ( sa_handler ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigaction__bindgen_ty_1 > ( ) ) ) . sa_sigaction as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigaction__bindgen_ty_1 ) , "::" , stringify ! ( sa_sigaction ) )) ;

}
 # [test] fn bindgen_test_layout_sigaction () {
assert_eq ! (:: std :: mem :: size_of :: < sigaction > ( ) , 152usize , concat ! ( "Size of: " , stringify ! ( sigaction ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigaction > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigaction ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigaction > ( ) ) ) . __sigaction_handler as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigaction ) , "::" , stringify ! ( __sigaction_handler ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigaction > ( ) ) ) . sa_mask as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sigaction ) , "::" , stringify ! ( sa_mask ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigaction > ( ) ) ) . sa_flags as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( sigaction ) , "::" , stringify ! ( sa_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigaction > ( ) ) ) . sa_restorer as * const _ as usize } , 144usize , concat ! ( "Offset of field: " , stringify ! ( sigaction ) , "::" , stringify ! ( sa_restorer ) )) ;

}
 extern "C" {
pub fn sigprocmask (__how : :: std :: os :: raw :: c_int , __set : * const sigset_t , __oset : * mut sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigsuspend (__set : * const sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigaction (__sig : :: std :: os :: raw :: c_int , __act : * const sigaction , __oact : * mut sigaction) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigpending (__set : * mut sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigwait (__set : * const sigset_t , __sig : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigwaitinfo (__set : * const sigset_t , __info : * mut siginfo_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigtimedwait (__set : * const sigset_t , __info : * mut siginfo_t , __timeout : * const timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigqueue (__pid : __pid_t , __sig : :: std :: os :: raw :: c_int , __val : sigval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut _sys_siglist : [* const :: std :: os :: raw :: c_char ; 65usize] ;

}
 extern "C" {
pub static mut sys_siglist : [* const :: std :: os :: raw :: c_char ; 65usize] ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _fpx_sw_bytes {
pub magic1 : __uint32_t , pub extended_size : __uint32_t , pub xstate_bv : __uint64_t , pub xstate_size : __uint32_t , pub __glibc_reserved1 : [__uint32_t ; 7usize] ,
}
 # [test] fn bindgen_test_layout__fpx_sw_bytes () {
assert_eq ! (:: std :: mem :: size_of :: < _fpx_sw_bytes > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( _fpx_sw_bytes ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _fpx_sw_bytes > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _fpx_sw_bytes ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpx_sw_bytes > ( ) ) ) . magic1 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _fpx_sw_bytes ) , "::" , stringify ! ( magic1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpx_sw_bytes > ( ) ) ) . extended_size as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( _fpx_sw_bytes ) , "::" , stringify ! ( extended_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpx_sw_bytes > ( ) ) ) . xstate_bv as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _fpx_sw_bytes ) , "::" , stringify ! ( xstate_bv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpx_sw_bytes > ( ) ) ) . xstate_size as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( _fpx_sw_bytes ) , "::" , stringify ! ( xstate_size ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpx_sw_bytes > ( ) ) ) . __glibc_reserved1 as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( _fpx_sw_bytes ) , "::" , stringify ! ( __glibc_reserved1 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _fpreg {
pub significand : [:: std :: os :: raw :: c_ushort ; 4usize] , pub exponent : :: std :: os :: raw :: c_ushort ,
}
 # [test] fn bindgen_test_layout__fpreg () {
assert_eq ! (:: std :: mem :: size_of :: < _fpreg > ( ) , 10usize , concat ! ( "Size of: " , stringify ! ( _fpreg ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _fpreg > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( _fpreg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpreg > ( ) ) ) . significand as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _fpreg ) , "::" , stringify ! ( significand ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpreg > ( ) ) ) . exponent as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _fpreg ) , "::" , stringify ! ( exponent ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _fpxreg {
pub significand : [:: std :: os :: raw :: c_ushort ; 4usize] , pub exponent : :: std :: os :: raw :: c_ushort , pub __glibc_reserved1 : [:: std :: os :: raw :: c_ushort ; 3usize] ,
}
 # [test] fn bindgen_test_layout__fpxreg () {
assert_eq ! (:: std :: mem :: size_of :: < _fpxreg > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( _fpxreg ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _fpxreg > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( _fpxreg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpxreg > ( ) ) ) . significand as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _fpxreg ) , "::" , stringify ! ( significand ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpxreg > ( ) ) ) . exponent as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _fpxreg ) , "::" , stringify ! ( exponent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpxreg > ( ) ) ) . __glibc_reserved1 as * const _ as usize } , 10usize , concat ! ( "Offset of field: " , stringify ! ( _fpxreg ) , "::" , stringify ! ( __glibc_reserved1 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _xmmreg {
pub element : [__uint32_t ; 4usize] ,
}
 # [test] fn bindgen_test_layout__xmmreg () {
assert_eq ! (:: std :: mem :: size_of :: < _xmmreg > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( _xmmreg ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _xmmreg > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( _xmmreg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xmmreg > ( ) ) ) . element as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _xmmreg ) , "::" , stringify ! ( element ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _fpstate {
pub cwd : __uint16_t , pub swd : __uint16_t , pub ftw : __uint16_t , pub fop : __uint16_t , pub rip : __uint64_t , pub rdp : __uint64_t , pub mxcsr : __uint32_t , pub mxcr_mask : __uint32_t , pub _st : [_fpxreg ; 8usize] , pub _xmm : [_xmmreg ; 16usize] , pub __glibc_reserved1 : [__uint32_t ; 24usize] ,
}
 # [test] fn bindgen_test_layout__fpstate () {
assert_eq ! (:: std :: mem :: size_of :: < _fpstate > ( ) , 512usize , concat ! ( "Size of: " , stringify ! ( _fpstate ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _fpstate > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _fpstate ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . cwd as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( cwd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . swd as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( swd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . ftw as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( ftw ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . fop as * const _ as usize } , 6usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( fop ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . rip as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( rip ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . rdp as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( rdp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . mxcsr as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( mxcsr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . mxcr_mask as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( mxcr_mask ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . _st as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( _st ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . _xmm as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( _xmm ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _fpstate > ( ) ) ) . __glibc_reserved1 as * const _ as usize } , 416usize , concat ! ( "Offset of field: " , stringify ! ( _fpstate ) , "::" , stringify ! ( __glibc_reserved1 ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct sigcontext {
pub r8 : __uint64_t , pub r9 : __uint64_t , pub r10 : __uint64_t , pub r11 : __uint64_t , pub r12 : __uint64_t , pub r13 : __uint64_t , pub r14 : __uint64_t , pub r15 : __uint64_t , pub rdi : __uint64_t , pub rsi : __uint64_t , pub rbp : __uint64_t , pub rbx : __uint64_t , pub rdx : __uint64_t , pub rax : __uint64_t , pub rcx : __uint64_t , pub rsp : __uint64_t , pub rip : __uint64_t , pub eflags : __uint64_t , pub cs : :: std :: os :: raw :: c_ushort , pub gs : :: std :: os :: raw :: c_ushort , pub fs : :: std :: os :: raw :: c_ushort , pub __pad0 : :: std :: os :: raw :: c_ushort , pub err : __uint64_t , pub trapno : __uint64_t , pub oldmask : __uint64_t , pub cr2 : __uint64_t , pub __bindgen_anon_1 : sigcontext__bindgen_ty_1 , pub __reserved1 : [__uint64_t ; 8usize] ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union sigcontext__bindgen_ty_1 {
pub fpstate : * mut _fpstate , pub __fpstate_word : __uint64_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_sigcontext__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < sigcontext__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( sigcontext__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigcontext__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigcontext__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext__bindgen_ty_1 > ( ) ) ) . fpstate as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext__bindgen_ty_1 ) , "::" , stringify ! ( fpstate ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext__bindgen_ty_1 > ( ) ) ) . __fpstate_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext__bindgen_ty_1 ) , "::" , stringify ! ( __fpstate_word ) )) ;

}
 # [test] fn bindgen_test_layout_sigcontext () {
assert_eq ! (:: std :: mem :: size_of :: < sigcontext > ( ) , 256usize , concat ! ( "Size of: " , stringify ! ( sigcontext ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigcontext > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigcontext ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r8 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r8 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r9 as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r9 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r10 as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r10 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r11 as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r11 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r12 as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r12 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r13 as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r13 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r14 as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r14 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . r15 as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( r15 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rdi as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rdi ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rsi as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rsi ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rbp as * const _ as usize } , 80usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rbp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rbx as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rbx ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rdx as * const _ as usize } , 96usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rdx ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rax as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rax ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rcx as * const _ as usize } , 112usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rcx ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rsp as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rsp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . rip as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( rip ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . eflags as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( eflags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . cs as * const _ as usize } , 144usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( cs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . gs as * const _ as usize } , 146usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( gs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . fs as * const _ as usize } , 148usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( fs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . __pad0 as * const _ as usize } , 150usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( __pad0 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . err as * const _ as usize } , 152usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( err ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . trapno as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( trapno ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . oldmask as * const _ as usize } , 168usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( oldmask ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . cr2 as * const _ as usize } , 176usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( cr2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigcontext > ( ) ) ) . __reserved1 as * const _ as usize } , 192usize , concat ! ( "Offset of field: " , stringify ! ( sigcontext ) , "::" , stringify ! ( __reserved1 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _xsave_hdr {
pub xstate_bv : __uint64_t , pub __glibc_reserved1 : [__uint64_t ; 2usize] , pub __glibc_reserved2 : [__uint64_t ; 5usize] ,
}
 # [test] fn bindgen_test_layout__xsave_hdr () {
assert_eq ! (:: std :: mem :: size_of :: < _xsave_hdr > ( ) , 64usize , concat ! ( "Size of: " , stringify ! ( _xsave_hdr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _xsave_hdr > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _xsave_hdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xsave_hdr > ( ) ) ) . xstate_bv as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _xsave_hdr ) , "::" , stringify ! ( xstate_bv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xsave_hdr > ( ) ) ) . __glibc_reserved1 as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _xsave_hdr ) , "::" , stringify ! ( __glibc_reserved1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xsave_hdr > ( ) ) ) . __glibc_reserved2 as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( _xsave_hdr ) , "::" , stringify ! ( __glibc_reserved2 ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct _ymmh_state {
pub ymmh_space : [__uint32_t ; 64usize] ,
}
 # [test] fn bindgen_test_layout__ymmh_state () {
assert_eq ! (:: std :: mem :: size_of :: < _ymmh_state > ( ) , 256usize , concat ! ( "Size of: " , stringify ! ( _ymmh_state ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _ymmh_state > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( _ymmh_state ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _ymmh_state > ( ) ) ) . ymmh_space as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _ymmh_state ) , "::" , stringify ! ( ymmh_space ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct _xstate {
pub fpstate : _fpstate , pub xstate_hdr : _xsave_hdr , pub ymmh : _ymmh_state ,
}
 # [test] fn bindgen_test_layout__xstate () {
assert_eq ! (:: std :: mem :: size_of :: < _xstate > ( ) , 832usize , concat ! ( "Size of: " , stringify ! ( _xstate ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _xstate > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _xstate ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xstate > ( ) ) ) . fpstate as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _xstate ) , "::" , stringify ! ( fpstate ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xstate > ( ) ) ) . xstate_hdr as * const _ as usize } , 512usize , concat ! ( "Offset of field: " , stringify ! ( _xstate ) , "::" , stringify ! ( xstate_hdr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _xstate > ( ) ) ) . ymmh as * const _ as usize } , 576usize , concat ! ( "Offset of field: " , stringify ! ( _xstate ) , "::" , stringify ! ( ymmh ) )) ;

}
 extern "C" {
pub fn sigreturn (__scp : * mut sigcontext) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct stack_t {
pub ss_sp : * mut :: std :: os :: raw :: c_void , pub ss_flags : :: std :: os :: raw :: c_int , pub ss_size : usize ,
}
 # [test] fn bindgen_test_layout_stack_t () {
assert_eq ! (:: std :: mem :: size_of :: < stack_t > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( stack_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < stack_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( stack_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stack_t > ( ) ) ) . ss_sp as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( stack_t ) , "::" , stringify ! ( ss_sp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stack_t > ( ) ) ) . ss_flags as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( stack_t ) , "::" , stringify ! ( ss_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < stack_t > ( ) ) ) . ss_size as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( stack_t ) , "::" , stringify ! ( ss_size ) )) ;

}
 pub type greg_t = :: std :: os :: raw :: c_longlong ;
 pub type gregset_t = [greg_t ; 23usize] ;
 pub const REG_R8 : _bindgen_ty_18 = 0 ;
 pub const REG_R9 : _bindgen_ty_18 = 1 ;
 pub const REG_R10 : _bindgen_ty_18 = 2 ;
 pub const REG_R11 : _bindgen_ty_18 = 3 ;
 pub const REG_R12 : _bindgen_ty_18 = 4 ;
 pub const REG_R13 : _bindgen_ty_18 = 5 ;
 pub const REG_R14 : _bindgen_ty_18 = 6 ;
 pub const REG_R15 : _bindgen_ty_18 = 7 ;
 pub const REG_RDI : _bindgen_ty_18 = 8 ;
 pub const REG_RSI : _bindgen_ty_18 = 9 ;
 pub const REG_RBP : _bindgen_ty_18 = 10 ;
 pub const REG_RBX : _bindgen_ty_18 = 11 ;
 pub const REG_RDX : _bindgen_ty_18 = 12 ;
 pub const REG_RAX : _bindgen_ty_18 = 13 ;
 pub const REG_RCX : _bindgen_ty_18 = 14 ;
 pub const REG_RSP : _bindgen_ty_18 = 15 ;
 pub const REG_RIP : _bindgen_ty_18 = 16 ;
 pub const REG_EFL : _bindgen_ty_18 = 17 ;
 pub const REG_CSGSFS : _bindgen_ty_18 = 18 ;
 pub const REG_ERR : _bindgen_ty_18 = 19 ;
 pub const REG_TRAPNO : _bindgen_ty_18 = 20 ;
 pub const REG_OLDMASK : _bindgen_ty_18 = 21 ;
 pub const REG_CR2 : _bindgen_ty_18 = 22 ;
 pub type _bindgen_ty_18 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _libc_fpxreg {
pub significand : [:: std :: os :: raw :: c_ushort ; 4usize] , pub exponent : :: std :: os :: raw :: c_ushort , pub __glibc_reserved1 : [:: std :: os :: raw :: c_ushort ; 3usize] ,
}
 # [test] fn bindgen_test_layout__libc_fpxreg () {
assert_eq ! (:: std :: mem :: size_of :: < _libc_fpxreg > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( _libc_fpxreg ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _libc_fpxreg > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( _libc_fpxreg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpxreg > ( ) ) ) . significand as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpxreg ) , "::" , stringify ! ( significand ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpxreg > ( ) ) ) . exponent as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpxreg ) , "::" , stringify ! ( exponent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpxreg > ( ) ) ) . __glibc_reserved1 as * const _ as usize } , 10usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpxreg ) , "::" , stringify ! ( __glibc_reserved1 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _libc_xmmreg {
pub element : [__uint32_t ; 4usize] ,
}
 # [test] fn bindgen_test_layout__libc_xmmreg () {
assert_eq ! (:: std :: mem :: size_of :: < _libc_xmmreg > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( _libc_xmmreg ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _libc_xmmreg > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( _libc_xmmreg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_xmmreg > ( ) ) ) . element as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _libc_xmmreg ) , "::" , stringify ! ( element ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _libc_fpstate {
pub cwd : __uint16_t , pub swd : __uint16_t , pub ftw : __uint16_t , pub fop : __uint16_t , pub rip : __uint64_t , pub rdp : __uint64_t , pub mxcsr : __uint32_t , pub mxcr_mask : __uint32_t , pub _st : [_libc_fpxreg ; 8usize] , pub _xmm : [_libc_xmmreg ; 16usize] , pub __glibc_reserved1 : [__uint32_t ; 24usize] ,
}
 # [test] fn bindgen_test_layout__libc_fpstate () {
assert_eq ! (:: std :: mem :: size_of :: < _libc_fpstate > ( ) , 512usize , concat ! ( "Size of: " , stringify ! ( _libc_fpstate ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _libc_fpstate > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _libc_fpstate ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . cwd as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( cwd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . swd as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( swd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . ftw as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( ftw ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . fop as * const _ as usize } , 6usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( fop ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . rip as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( rip ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . rdp as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( rdp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . mxcsr as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( mxcsr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . mxcr_mask as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( mxcr_mask ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . _st as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( _st ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . _xmm as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( _xmm ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _libc_fpstate > ( ) ) ) . __glibc_reserved1 as * const _ as usize } , 416usize , concat ! ( "Offset of field: " , stringify ! ( _libc_fpstate ) , "::" , stringify ! ( __glibc_reserved1 ) )) ;

}
 pub type fpregset_t = * mut _libc_fpstate ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct mcontext_t {
pub gregs : gregset_t , pub fpregs : fpregset_t , pub __reserved1 : [:: std :: os :: raw :: c_ulonglong ; 8usize] ,
}
 # [test] fn bindgen_test_layout_mcontext_t () {
assert_eq ! (:: std :: mem :: size_of :: < mcontext_t > ( ) , 256usize , concat ! ( "Size of: " , stringify ! ( mcontext_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < mcontext_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( mcontext_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mcontext_t > ( ) ) ) . gregs as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( mcontext_t ) , "::" , stringify ! ( gregs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mcontext_t > ( ) ) ) . fpregs as * const _ as usize } , 184usize , concat ! ( "Offset of field: " , stringify ! ( mcontext_t ) , "::" , stringify ! ( fpregs ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < mcontext_t > ( ) ) ) . __reserved1 as * const _ as usize } , 192usize , concat ! ( "Offset of field: " , stringify ! ( mcontext_t ) , "::" , stringify ! ( __reserved1 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ucontext_t {
pub uc_flags : :: std :: os :: raw :: c_ulong , pub uc_link : * mut ucontext_t , pub uc_stack : stack_t , pub uc_mcontext : mcontext_t , pub uc_sigmask : sigset_t , pub __fpregs_mem : _libc_fpstate ,
}
 # [test] fn bindgen_test_layout_ucontext_t () {
assert_eq ! (:: std :: mem :: size_of :: < ucontext_t > ( ) , 936usize , concat ! ( "Size of: " , stringify ! ( ucontext_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ucontext_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( ucontext_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucontext_t > ( ) ) ) . uc_flags as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ucontext_t ) , "::" , stringify ! ( uc_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucontext_t > ( ) ) ) . uc_link as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ucontext_t ) , "::" , stringify ! ( uc_link ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucontext_t > ( ) ) ) . uc_stack as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( ucontext_t ) , "::" , stringify ! ( uc_stack ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucontext_t > ( ) ) ) . uc_mcontext as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( ucontext_t ) , "::" , stringify ! ( uc_mcontext ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucontext_t > ( ) ) ) . uc_sigmask as * const _ as usize } , 296usize , concat ! ( "Offset of field: " , stringify ! ( ucontext_t ) , "::" , stringify ! ( uc_sigmask ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ucontext_t > ( ) ) ) . __fpregs_mem as * const _ as usize } , 424usize , concat ! ( "Offset of field: " , stringify ! ( ucontext_t ) , "::" , stringify ! ( __fpregs_mem ) )) ;

}
 extern "C" {
pub fn siginterrupt (__sig : :: std :: os :: raw :: c_int , __interrupt : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 pub const SS_ONSTACK : _bindgen_ty_19 = 1 ;
 pub const SS_DISABLE : _bindgen_ty_19 = 2 ;
 pub type _bindgen_ty_19 = u32 ;
 extern "C" {
pub fn sigaltstack (__ss : * const stack_t , __oss : * mut stack_t) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sigstack {
pub ss_sp : * mut :: std :: os :: raw :: c_void , pub ss_onstack : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_sigstack () {
assert_eq ! (:: std :: mem :: size_of :: < sigstack > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( sigstack ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sigstack > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( sigstack ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigstack > ( ) ) ) . ss_sp as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sigstack ) , "::" , stringify ! ( ss_sp ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sigstack > ( ) ) ) . ss_onstack as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( sigstack ) , "::" , stringify ! ( ss_onstack ) )) ;

}
 extern "C" {
pub fn sigstack (__ss : * mut sigstack , __oss : * mut sigstack) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sighold (__sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigrelse (__sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigignore (__sig : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sigset (__sig : :: std :: os :: raw :: c_int , __disp : __sighandler_t) -> __sighandler_t ;

}
 extern "C" {
pub fn pthread_sigmask (__how : :: std :: os :: raw :: c_int , __newmask : * const __sigset_t , __oldmask : * mut __sigset_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pthread_kill (__threadid : pthread_t , __signo : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pthread_sigqueue (__threadid : pthread_t , __signo : :: std :: os :: raw :: c_int , __value : sigval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __libc_current_sigrtmin () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __libc_current_sigrtmax () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn readv (__fd : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn writev (__fd : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn preadv (__fd : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off_t) -> isize ;

}
 extern "C" {
pub fn pwritev (__fd : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off_t) -> isize ;

}
 extern "C" {
pub fn preadv64 (__fd : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off64_t) -> isize ;

}
 extern "C" {
pub fn pwritev64 (__fd : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off64_t) -> isize ;

}
 extern "C" {
pub fn preadv2 (__fp : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off_t , ___flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn pwritev2 (__fd : :: std :: os :: raw :: c_int , __iodev : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off_t , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn preadv64v2 (__fp : :: std :: os :: raw :: c_int , __iovec : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off64_t , ___flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn pwritev64v2 (__fd : :: std :: os :: raw :: c_int , __iodev : * const iovec , __count : :: std :: os :: raw :: c_int , __offset : __off64_t , __flags : :: std :: os :: raw :: c_int) -> isize ;

}
 extern "C" {
pub fn process_vm_readv (__pid : pid_t , __lvec : * const iovec , __liovcnt : :: std :: os :: raw :: c_ulong , __rvec : * const iovec , __riovcnt : :: std :: os :: raw :: c_ulong , __flags : :: std :: os :: raw :: c_ulong) -> isize ;

}
 extern "C" {
pub fn process_vm_writev (__pid : pid_t , __lvec : * const iovec , __liovcnt : :: std :: os :: raw :: c_ulong , __rvec : * const iovec , __riovcnt : :: std :: os :: raw :: c_ulong , __flags : :: std :: os :: raw :: c_ulong) -> isize ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct timezone {
pub tz_minuteswest : :: std :: os :: raw :: c_int , pub tz_dsttime : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_timezone () {
assert_eq ! (:: std :: mem :: size_of :: < timezone > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( timezone ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timezone > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( timezone ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timezone > ( ) ) ) . tz_minuteswest as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timezone ) , "::" , stringify ! ( tz_minuteswest ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timezone > ( ) ) ) . tz_dsttime as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( timezone ) , "::" , stringify ! ( tz_dsttime ) )) ;

}
 pub type __timezone_ptr_t = * mut timezone ;
 extern "C" {
pub fn gettimeofday (__tv : * mut timeval , __tz : __timezone_ptr_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn settimeofday (__tv : * const timeval , __tz : * const timezone) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn adjtime (__delta : * const timeval , __olddelta : * mut timeval) -> :: std :: os :: raw :: c_int ;

}
 pub const __itimer_which_ITIMER_REAL : __itimer_which = 0 ;
 pub const __itimer_which_ITIMER_VIRTUAL : __itimer_which = 1 ;
 pub const __itimer_which_ITIMER_PROF : __itimer_which = 2 ;
 pub type __itimer_which = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct itimerval {
pub it_interval : timeval , pub it_value : timeval ,
}
 # [test] fn bindgen_test_layout_itimerval () {
assert_eq ! (:: std :: mem :: size_of :: < itimerval > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( itimerval ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < itimerval > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( itimerval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < itimerval > ( ) ) ) . it_interval as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( itimerval ) , "::" , stringify ! ( it_interval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < itimerval > ( ) ) ) . it_value as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( itimerval ) , "::" , stringify ! ( it_value ) )) ;

}
 pub type __itimer_which_t = :: std :: os :: raw :: c_int ;
 extern "C" {
pub fn getitimer (__which : __itimer_which_t , __value : * mut itimerval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setitimer (__which : __itimer_which_t , __new : * const itimerval , __old : * mut itimerval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn utimes (__file : * const :: std :: os :: raw :: c_char , __tvp : * const timeval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lutimes (__file : * const :: std :: os :: raw :: c_char , __tvp : * const timeval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn futimes (__fd : :: std :: os :: raw :: c_int , __tvp : * const timeval) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn futimesat (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __tvp : * const timeval) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __exit_status {
pub e_termination : :: std :: os :: raw :: c_short , pub e_exit : :: std :: os :: raw :: c_short ,
}
 # [test] fn bindgen_test_layout___exit_status () {
assert_eq ! (:: std :: mem :: size_of :: < __exit_status > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( __exit_status ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __exit_status > ( ) , 2usize , concat ! ( "Alignment of " , stringify ! ( __exit_status ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __exit_status > ( ) ) ) . e_termination as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __exit_status ) , "::" , stringify ! ( e_termination ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __exit_status > ( ) ) ) . e_exit as * const _ as usize } , 2usize , concat ! ( "Offset of field: " , stringify ! ( __exit_status ) , "::" , stringify ! ( e_exit ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct utmpx {
pub ut_type : :: std :: os :: raw :: c_short , pub ut_pid : __pid_t , pub ut_line : [:: std :: os :: raw :: c_char ; 32usize] , pub ut_id : [:: std :: os :: raw :: c_char ; 4usize] , pub ut_user : [:: std :: os :: raw :: c_char ; 32usize] , pub ut_host : [:: std :: os :: raw :: c_char ; 256usize] , pub ut_exit : __exit_status , pub ut_session : __int32_t , pub ut_tv : utmpx__bindgen_ty_1 , pub ut_addr_v6 : [__int32_t ; 4usize] , pub __glibc_reserved : [:: std :: os :: raw :: c_char ; 20usize] ,
}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct utmpx__bindgen_ty_1 {
pub tv_sec : __int32_t , pub tv_usec : __int32_t ,
}
 # [test] fn bindgen_test_layout_utmpx__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < utmpx__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( utmpx__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < utmpx__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( utmpx__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx__bindgen_ty_1 > ( ) ) ) . tv_sec as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( utmpx__bindgen_ty_1 ) , "::" , stringify ! ( tv_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx__bindgen_ty_1 > ( ) ) ) . tv_usec as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( utmpx__bindgen_ty_1 ) , "::" , stringify ! ( tv_usec ) )) ;

}
 # [test] fn bindgen_test_layout_utmpx () {
assert_eq ! (:: std :: mem :: size_of :: < utmpx > ( ) , 384usize , concat ! ( "Size of: " , stringify ! ( utmpx ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < utmpx > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( utmpx ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_type as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_pid as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_pid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_line as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_line ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_id as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_id ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_user as * const _ as usize } , 44usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_user ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_host as * const _ as usize } , 76usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_host ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_exit as * const _ as usize } , 332usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_exit ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_session as * const _ as usize } , 336usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_session ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_tv as * const _ as usize } , 340usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_tv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . ut_addr_v6 as * const _ as usize } , 348usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( ut_addr_v6 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utmpx > ( ) ) ) . __glibc_reserved as * const _ as usize } , 364usize , concat ! ( "Offset of field: " , stringify ! ( utmpx ) , "::" , stringify ! ( __glibc_reserved ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct utmp {
_unused : [u8 ; 0] ,
}
 extern "C" {
pub fn setutxent () ;

}
 extern "C" {
pub fn endutxent () ;

}
 extern "C" {
pub fn getutxent () -> * mut utmpx ;

}
 extern "C" {
pub fn getutxid (__id : * const utmpx) -> * mut utmpx ;

}
 extern "C" {
pub fn getutxline (__line : * const utmpx) -> * mut utmpx ;

}
 extern "C" {
pub fn pututxline (__utmpx : * const utmpx) -> * mut utmpx ;

}
 extern "C" {
pub fn utmpxname (__file : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn updwtmpx (__wtmpx_file : * const :: std :: os :: raw :: c_char , __utmpx : * const utmpx) ;

}
 extern "C" {
pub fn getutmp (__utmpx : * const utmpx , __utmp : * mut utmp) ;

}
 extern "C" {
pub fn getutmpx (__utmp : * const utmp , __utmpx : * mut utmpx) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct sched_param {
pub sched_priority : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_sched_param () {
assert_eq ! (:: std :: mem :: size_of :: < sched_param > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( sched_param ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < sched_param > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( sched_param ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < sched_param > ( ) ) ) . sched_priority as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( sched_param ) , "::" , stringify ! ( sched_priority ) )) ;

}
 extern "C" {
pub fn clone (__fn : :: std :: option :: Option < unsafe extern "C" fn ( __arg : * mut :: std :: os :: raw :: c_void ) -> :: std :: os :: raw :: c_int > , __child_stack : * mut :: std :: os :: raw :: c_void , __flags : :: std :: os :: raw :: c_int , __arg : * mut :: std :: os :: raw :: c_void , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn unshare (__flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_getcpu () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setns (__fd : :: std :: os :: raw :: c_int , __nstype : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 pub type __cpu_mask = :: std :: os :: raw :: c_ulong ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct cpu_set_t {
pub __bits : [__cpu_mask ; 16usize] ,
}
 # [test] fn bindgen_test_layout_cpu_set_t () {
assert_eq ! (:: std :: mem :: size_of :: < cpu_set_t > ( ) , 128usize , concat ! ( "Size of: " , stringify ! ( cpu_set_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < cpu_set_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( cpu_set_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < cpu_set_t > ( ) ) ) . __bits as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( cpu_set_t ) , "::" , stringify ! ( __bits ) )) ;

}
 extern "C" {
pub fn __sched_cpucount (__setsize : usize , __setp : * const cpu_set_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __sched_cpualloc (__count : usize) -> * mut cpu_set_t ;

}
 extern "C" {
pub fn __sched_cpufree (__set : * mut cpu_set_t) ;

}
 extern "C" {
pub fn sched_setparam (__pid : __pid_t , __param : * const sched_param) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_getparam (__pid : __pid_t , __param : * mut sched_param) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_setscheduler (__pid : __pid_t , __policy : :: std :: os :: raw :: c_int , __param : * const sched_param) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_getscheduler (__pid : __pid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_yield () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_get_priority_max (__algorithm : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_get_priority_min (__algorithm : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_rr_get_interval (__pid : __pid_t , __t : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_setaffinity (__pid : __pid_t , __cpusetsize : usize , __cpuset : * const cpu_set_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sched_getaffinity (__pid : __pid_t , __cpusetsize : usize , __cpuset : * mut cpu_set_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn access (__name : * const :: std :: os :: raw :: c_char , __type : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn euidaccess (__name : * const :: std :: os :: raw :: c_char , __type : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn eaccess (__name : * const :: std :: os :: raw :: c_char , __type : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn faccessat (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __type : :: std :: os :: raw :: c_int , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lseek (__fd : :: std :: os :: raw :: c_int , __offset : __off_t , __whence : :: std :: os :: raw :: c_int) -> __off_t ;

}
 extern "C" {
pub fn lseek64 (__fd : :: std :: os :: raw :: c_int , __offset : __off64_t , __whence : :: std :: os :: raw :: c_int) -> __off64_t ;

}
 extern "C" {
pub fn close (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn read (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __nbytes : usize) -> isize ;

}
 extern "C" {
pub fn write (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize) -> isize ;

}
 extern "C" {
pub fn pread (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __nbytes : usize , __offset : __off_t) -> isize ;

}
 extern "C" {
pub fn pwrite (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize , __offset : __off_t) -> isize ;

}
 extern "C" {
pub fn pread64 (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_void , __nbytes : usize , __offset : __off64_t) -> isize ;

}
 extern "C" {
pub fn pwrite64 (__fd : :: std :: os :: raw :: c_int , __buf : * const :: std :: os :: raw :: c_void , __n : usize , __offset : __off64_t) -> isize ;

}
 extern "C" {
pub fn pipe (__pipedes : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pipe2 (__pipedes : * mut :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn alarm (__seconds : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn sleep (__seconds : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn ualarm (__value : __useconds_t , __interval : __useconds_t) -> __useconds_t ;

}
 extern "C" {
pub fn usleep (__useconds : __useconds_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn pause () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn chown (__file : * const :: std :: os :: raw :: c_char , __owner : __uid_t , __group : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fchown (__fd : :: std :: os :: raw :: c_int , __owner : __uid_t , __group : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lchown (__file : * const :: std :: os :: raw :: c_char , __owner : __uid_t , __group : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fchownat (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __owner : __uid_t , __group : __gid_t , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn chdir (__path : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fchdir (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getcwd (__buf : * mut :: std :: os :: raw :: c_char , __size : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn get_current_dir_name () -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn getwd (__buf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn dup (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn dup2 (__fd : :: std :: os :: raw :: c_int , __fd2 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn dup3 (__fd : :: std :: os :: raw :: c_int , __fd2 : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut __environ : * mut * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut environ : * mut * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn execve (__path : * const :: std :: os :: raw :: c_char , __argv : * const * mut :: std :: os :: raw :: c_char , __envp : * const * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fexecve (__fd : :: std :: os :: raw :: c_int , __argv : * const * mut :: std :: os :: raw :: c_char , __envp : * const * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn execv (__path : * const :: std :: os :: raw :: c_char , __argv : * const * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn execle (__path : * const :: std :: os :: raw :: c_char , __arg : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn execl (__path : * const :: std :: os :: raw :: c_char , __arg : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn execvp (__file : * const :: std :: os :: raw :: c_char , __argv : * const * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn execlp (__file : * const :: std :: os :: raw :: c_char , __arg : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn execvpe (__file : * const :: std :: os :: raw :: c_char , __argv : * const * mut :: std :: os :: raw :: c_char , __envp : * const * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn nice (__inc : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _exit (__status : :: std :: os :: raw :: c_int) ;

}

extern "C"{
  pub fn __errno_location () -> *mut :: std::os ::raw::c_int;
}

 pub const _PC_LINK_MAX : _bindgen_ty_20 = 0 ;
 pub const _PC_MAX_CANON : _bindgen_ty_20 = 1 ;
 pub const _PC_MAX_INPUT : _bindgen_ty_20 = 2 ;
 pub const _PC_NAME_MAX : _bindgen_ty_20 = 3 ;
 pub const _PC_PATH_MAX : _bindgen_ty_20 = 4 ;
 pub const _PC_PIPE_BUF : _bindgen_ty_20 = 5 ;
 pub const _PC_CHOWN_RESTRICTED : _bindgen_ty_20 = 6 ;
 pub const _PC_NO_TRUNC : _bindgen_ty_20 = 7 ;
 pub const _PC_VDISABLE : _bindgen_ty_20 = 8 ;
 pub const _PC_SYNC_IO : _bindgen_ty_20 = 9 ;
 pub const _PC_ASYNC_IO : _bindgen_ty_20 = 10 ;
 pub const _PC_PRIO_IO : _bindgen_ty_20 = 11 ;
 pub const _PC_SOCK_MAXBUF : _bindgen_ty_20 = 12 ;
 pub const _PC_FILESIZEBITS : _bindgen_ty_20 = 13 ;
 pub const _PC_REC_INCR_XFER_SIZE : _bindgen_ty_20 = 14 ;
 pub const _PC_REC_MAX_XFER_SIZE : _bindgen_ty_20 = 15 ;
 pub const _PC_REC_MIN_XFER_SIZE : _bindgen_ty_20 = 16 ;
 pub const _PC_REC_XFER_ALIGN : _bindgen_ty_20 = 17 ;
 pub const _PC_ALLOC_SIZE_MIN : _bindgen_ty_20 = 18 ;
 pub const _PC_SYMLINK_MAX : _bindgen_ty_20 = 19 ;
 pub const _PC_2_SYMLINKS : _bindgen_ty_20 = 20 ;
 pub type _bindgen_ty_20 = u32 ;
 pub const _SC_ARG_MAX : _bindgen_ty_21 = 0 ;
 pub const _SC_CHILD_MAX : _bindgen_ty_21 = 1 ;
 pub const _SC_CLK_TCK : _bindgen_ty_21 = 2 ;
 pub const _SC_NGROUPS_MAX : _bindgen_ty_21 = 3 ;
 pub const _SC_OPEN_MAX : _bindgen_ty_21 = 4 ;
 pub const _SC_STREAM_MAX : _bindgen_ty_21 = 5 ;
 pub const _SC_TZNAME_MAX : _bindgen_ty_21 = 6 ;
 pub const _SC_JOB_CONTROL : _bindgen_ty_21 = 7 ;
 pub const _SC_SAVED_IDS : _bindgen_ty_21 = 8 ;
 pub const _SC_REALTIME_SIGNALS : _bindgen_ty_21 = 9 ;
 pub const _SC_PRIORITY_SCHEDULING : _bindgen_ty_21 = 10 ;
 pub const _SC_TIMERS : _bindgen_ty_21 = 11 ;
 pub const _SC_ASYNCHRONOUS_IO : _bindgen_ty_21 = 12 ;
 pub const _SC_PRIORITIZED_IO : _bindgen_ty_21 = 13 ;
 pub const _SC_SYNCHRONIZED_IO : _bindgen_ty_21 = 14 ;
 pub const _SC_FSYNC : _bindgen_ty_21 = 15 ;
 pub const _SC_MAPPED_FILES : _bindgen_ty_21 = 16 ;
 pub const _SC_MEMLOCK : _bindgen_ty_21 = 17 ;
 pub const _SC_MEMLOCK_RANGE : _bindgen_ty_21 = 18 ;
 pub const _SC_MEMORY_PROTECTION : _bindgen_ty_21 = 19 ;
 pub const _SC_MESSAGE_PASSING : _bindgen_ty_21 = 20 ;
 pub const _SC_SEMAPHORES : _bindgen_ty_21 = 21 ;
 pub const _SC_SHARED_MEMORY_OBJECTS : _bindgen_ty_21 = 22 ;
 pub const _SC_AIO_LISTIO_MAX : _bindgen_ty_21 = 23 ;
 pub const _SC_AIO_MAX : _bindgen_ty_21 = 24 ;
 pub const _SC_AIO_PRIO_DELTA_MAX : _bindgen_ty_21 = 25 ;
 pub const _SC_DELAYTIMER_MAX : _bindgen_ty_21 = 26 ;
 pub const _SC_MQ_OPEN_MAX : _bindgen_ty_21 = 27 ;
 pub const _SC_MQ_PRIO_MAX : _bindgen_ty_21 = 28 ;
 pub const _SC_VERSION : _bindgen_ty_21 = 29 ;
 pub const _SC_PAGESIZE : _bindgen_ty_21 = 30 ;
 pub const _SC_RTSIG_MAX : _bindgen_ty_21 = 31 ;
 pub const _SC_SEM_NSEMS_MAX : _bindgen_ty_21 = 32 ;
 pub const _SC_SEM_VALUE_MAX : _bindgen_ty_21 = 33 ;
 pub const _SC_SIGQUEUE_MAX : _bindgen_ty_21 = 34 ;
 pub const _SC_TIMER_MAX : _bindgen_ty_21 = 35 ;
 pub const _SC_BC_BASE_MAX : _bindgen_ty_21 = 36 ;
 pub const _SC_BC_DIM_MAX : _bindgen_ty_21 = 37 ;
 pub const _SC_BC_SCALE_MAX : _bindgen_ty_21 = 38 ;
 pub const _SC_BC_STRING_MAX : _bindgen_ty_21 = 39 ;
 pub const _SC_COLL_WEIGHTS_MAX : _bindgen_ty_21 = 40 ;
 pub const _SC_EQUIV_CLASS_MAX : _bindgen_ty_21 = 41 ;
 pub const _SC_EXPR_NEST_MAX : _bindgen_ty_21 = 42 ;
 pub const _SC_LINE_MAX : _bindgen_ty_21 = 43 ;
 pub const _SC_RE_DUP_MAX : _bindgen_ty_21 = 44 ;
 pub const _SC_CHARCLASS_NAME_MAX : _bindgen_ty_21 = 45 ;
 pub const _SC_2_VERSION : _bindgen_ty_21 = 46 ;
 pub const _SC_2_C_BIND : _bindgen_ty_21 = 47 ;
 pub const _SC_2_C_DEV : _bindgen_ty_21 = 48 ;
 pub const _SC_2_FORT_DEV : _bindgen_ty_21 = 49 ;
 pub const _SC_2_FORT_RUN : _bindgen_ty_21 = 50 ;
 pub const _SC_2_SW_DEV : _bindgen_ty_21 = 51 ;
 pub const _SC_2_LOCALEDEF : _bindgen_ty_21 = 52 ;
 pub const _SC_PII : _bindgen_ty_21 = 53 ;
 pub const _SC_PII_XTI : _bindgen_ty_21 = 54 ;
 pub const _SC_PII_SOCKET : _bindgen_ty_21 = 55 ;
 pub const _SC_PII_INTERNET : _bindgen_ty_21 = 56 ;
 pub const _SC_PII_OSI : _bindgen_ty_21 = 57 ;
 pub const _SC_POLL : _bindgen_ty_21 = 58 ;
 pub const _SC_SELECT : _bindgen_ty_21 = 59 ;
 pub const _SC_UIO_MAXIOV : _bindgen_ty_21 = 60 ;
 pub const _SC_IOV_MAX : _bindgen_ty_21 = 60 ;
 pub const _SC_PII_INTERNET_STREAM : _bindgen_ty_21 = 61 ;
 pub const _SC_PII_INTERNET_DGRAM : _bindgen_ty_21 = 62 ;
 pub const _SC_PII_OSI_COTS : _bindgen_ty_21 = 63 ;
 pub const _SC_PII_OSI_CLTS : _bindgen_ty_21 = 64 ;
 pub const _SC_PII_OSI_M : _bindgen_ty_21 = 65 ;
 pub const _SC_T_IOV_MAX : _bindgen_ty_21 = 66 ;
 pub const _SC_THREADS : _bindgen_ty_21 = 67 ;
 pub const _SC_THREAD_SAFE_FUNCTIONS : _bindgen_ty_21 = 68 ;
 pub const _SC_GETGR_R_SIZE_MAX : _bindgen_ty_21 = 69 ;
 pub const _SC_GETPW_R_SIZE_MAX : _bindgen_ty_21 = 70 ;
 pub const _SC_LOGIN_NAME_MAX : _bindgen_ty_21 = 71 ;
 pub const _SC_TTY_NAME_MAX : _bindgen_ty_21 = 72 ;
 pub const _SC_THREAD_DESTRUCTOR_ITERATIONS : _bindgen_ty_21 = 73 ;
 pub const _SC_THREAD_KEYS_MAX : _bindgen_ty_21 = 74 ;
 pub const _SC_THREAD_STACK_MIN : _bindgen_ty_21 = 75 ;
 pub const _SC_THREAD_THREADS_MAX : _bindgen_ty_21 = 76 ;
 pub const _SC_THREAD_ATTR_STACKADDR : _bindgen_ty_21 = 77 ;
 pub const _SC_THREAD_ATTR_STACKSIZE : _bindgen_ty_21 = 78 ;
 pub const _SC_THREAD_PRIORITY_SCHEDULING : _bindgen_ty_21 = 79 ;
 pub const _SC_THREAD_PRIO_INHERIT : _bindgen_ty_21 = 80 ;
 pub const _SC_THREAD_PRIO_PROTECT : _bindgen_ty_21 = 81 ;
 pub const _SC_THREAD_PROCESS_SHARED : _bindgen_ty_21 = 82 ;
 pub const _SC_NPROCESSORS_CONF : _bindgen_ty_21 = 83 ;
 pub const _SC_NPROCESSORS_ONLN : _bindgen_ty_21 = 84 ;
 pub const _SC_PHYS_PAGES : _bindgen_ty_21 = 85 ;
 pub const _SC_AVPHYS_PAGES : _bindgen_ty_21 = 86 ;
 pub const _SC_ATEXIT_MAX : _bindgen_ty_21 = 87 ;
 pub const _SC_PASS_MAX : _bindgen_ty_21 = 88 ;
 pub const _SC_XOPEN_VERSION : _bindgen_ty_21 = 89 ;
 pub const _SC_XOPEN_XCU_VERSION : _bindgen_ty_21 = 90 ;
 pub const _SC_XOPEN_UNIX : _bindgen_ty_21 = 91 ;
 pub const _SC_XOPEN_CRYPT : _bindgen_ty_21 = 92 ;
 pub const _SC_XOPEN_ENH_I18N : _bindgen_ty_21 = 93 ;
 pub const _SC_XOPEN_SHM : _bindgen_ty_21 = 94 ;
 pub const _SC_2_CHAR_TERM : _bindgen_ty_21 = 95 ;
 pub const _SC_2_C_VERSION : _bindgen_ty_21 = 96 ;
 pub const _SC_2_UPE : _bindgen_ty_21 = 97 ;
 pub const _SC_XOPEN_XPG2 : _bindgen_ty_21 = 98 ;
 pub const _SC_XOPEN_XPG3 : _bindgen_ty_21 = 99 ;
 pub const _SC_XOPEN_XPG4 : _bindgen_ty_21 = 100 ;
 pub const _SC_CHAR_BIT : _bindgen_ty_21 = 101 ;
 pub const _SC_CHAR_MAX : _bindgen_ty_21 = 102 ;
 pub const _SC_CHAR_MIN : _bindgen_ty_21 = 103 ;
 pub const _SC_INT_MAX : _bindgen_ty_21 = 104 ;
 pub const _SC_INT_MIN : _bindgen_ty_21 = 105 ;
 pub const _SC_LONG_BIT : _bindgen_ty_21 = 106 ;
 pub const _SC_WORD_BIT : _bindgen_ty_21 = 107 ;
 pub const _SC_MB_LEN_MAX : _bindgen_ty_21 = 108 ;
 pub const _SC_NZERO : _bindgen_ty_21 = 109 ;
 pub const _SC_SSIZE_MAX : _bindgen_ty_21 = 110 ;
 pub const _SC_SCHAR_MAX : _bindgen_ty_21 = 111 ;
 pub const _SC_SCHAR_MIN : _bindgen_ty_21 = 112 ;
 pub const _SC_SHRT_MAX : _bindgen_ty_21 = 113 ;
 pub const _SC_SHRT_MIN : _bindgen_ty_21 = 114 ;
 pub const _SC_UCHAR_MAX : _bindgen_ty_21 = 115 ;
 pub const _SC_UINT_MAX : _bindgen_ty_21 = 116 ;
 pub const _SC_ULONG_MAX : _bindgen_ty_21 = 117 ;
 pub const _SC_USHRT_MAX : _bindgen_ty_21 = 118 ;
 pub const _SC_NL_ARGMAX : _bindgen_ty_21 = 119 ;
 pub const _SC_NL_LANGMAX : _bindgen_ty_21 = 120 ;
 pub const _SC_NL_MSGMAX : _bindgen_ty_21 = 121 ;
 pub const _SC_NL_NMAX : _bindgen_ty_21 = 122 ;
 pub const _SC_NL_SETMAX : _bindgen_ty_21 = 123 ;
 pub const _SC_NL_TEXTMAX : _bindgen_ty_21 = 124 ;
 pub const _SC_XBS5_ILP32_OFF32 : _bindgen_ty_21 = 125 ;
 pub const _SC_XBS5_ILP32_OFFBIG : _bindgen_ty_21 = 126 ;
 pub const _SC_XBS5_LP64_OFF64 : _bindgen_ty_21 = 127 ;
 pub const _SC_XBS5_LPBIG_OFFBIG : _bindgen_ty_21 = 128 ;
 pub const _SC_XOPEN_LEGACY : _bindgen_ty_21 = 129 ;
 pub const _SC_XOPEN_REALTIME : _bindgen_ty_21 = 130 ;
 pub const _SC_XOPEN_REALTIME_THREADS : _bindgen_ty_21 = 131 ;
 pub const _SC_ADVISORY_INFO : _bindgen_ty_21 = 132 ;
 pub const _SC_BARRIERS : _bindgen_ty_21 = 133 ;
 pub const _SC_BASE : _bindgen_ty_21 = 134 ;
 pub const _SC_C_LANG_SUPPORT : _bindgen_ty_21 = 135 ;
 pub const _SC_C_LANG_SUPPORT_R : _bindgen_ty_21 = 136 ;
 pub const _SC_CLOCK_SELECTION : _bindgen_ty_21 = 137 ;
 pub const _SC_CPUTIME : _bindgen_ty_21 = 138 ;
 pub const _SC_THREAD_CPUTIME : _bindgen_ty_21 = 139 ;
 pub const _SC_DEVICE_IO : _bindgen_ty_21 = 140 ;
 pub const _SC_DEVICE_SPECIFIC : _bindgen_ty_21 = 141 ;
 pub const _SC_DEVICE_SPECIFIC_R : _bindgen_ty_21 = 142 ;
 pub const _SC_FD_MGMT : _bindgen_ty_21 = 143 ;
 pub const _SC_FIFO : _bindgen_ty_21 = 144 ;
 pub const _SC_PIPE : _bindgen_ty_21 = 145 ;
 pub const _SC_FILE_ATTRIBUTES : _bindgen_ty_21 = 146 ;
 pub const _SC_FILE_LOCKING : _bindgen_ty_21 = 147 ;
 pub const _SC_FILE_SYSTEM : _bindgen_ty_21 = 148 ;
 pub const _SC_MONOTONIC_CLOCK : _bindgen_ty_21 = 149 ;
 pub const _SC_MULTI_PROCESS : _bindgen_ty_21 = 150 ;
 pub const _SC_SINGLE_PROCESS : _bindgen_ty_21 = 151 ;
 pub const _SC_NETWORKING : _bindgen_ty_21 = 152 ;
 pub const _SC_READER_WRITER_LOCKS : _bindgen_ty_21 = 153 ;
 pub const _SC_SPIN_LOCKS : _bindgen_ty_21 = 154 ;
 pub const _SC_REGEXP : _bindgen_ty_21 = 155 ;
 pub const _SC_REGEX_VERSION : _bindgen_ty_21 = 156 ;
 pub const _SC_SHELL : _bindgen_ty_21 = 157 ;
 pub const _SC_SIGNALS : _bindgen_ty_21 = 158 ;
 pub const _SC_SPAWN : _bindgen_ty_21 = 159 ;
 pub const _SC_SPORADIC_SERVER : _bindgen_ty_21 = 160 ;
 pub const _SC_THREAD_SPORADIC_SERVER : _bindgen_ty_21 = 161 ;
 pub const _SC_SYSTEM_DATABASE : _bindgen_ty_21 = 162 ;
 pub const _SC_SYSTEM_DATABASE_R : _bindgen_ty_21 = 163 ;
 pub const _SC_TIMEOUTS : _bindgen_ty_21 = 164 ;
 pub const _SC_TYPED_MEMORY_OBJECTS : _bindgen_ty_21 = 165 ;
 pub const _SC_USER_GROUPS : _bindgen_ty_21 = 166 ;
 pub const _SC_USER_GROUPS_R : _bindgen_ty_21 = 167 ;
 pub const _SC_2_PBS : _bindgen_ty_21 = 168 ;
 pub const _SC_2_PBS_ACCOUNTING : _bindgen_ty_21 = 169 ;
 pub const _SC_2_PBS_LOCATE : _bindgen_ty_21 = 170 ;
 pub const _SC_2_PBS_MESSAGE : _bindgen_ty_21 = 171 ;
 pub const _SC_2_PBS_TRACK : _bindgen_ty_21 = 172 ;
 pub const _SC_SYMLOOP_MAX : _bindgen_ty_21 = 173 ;
 pub const _SC_STREAMS : _bindgen_ty_21 = 174 ;
 pub const _SC_2_PBS_CHECKPOINT : _bindgen_ty_21 = 175 ;
 pub const _SC_V6_ILP32_OFF32 : _bindgen_ty_21 = 176 ;
 pub const _SC_V6_ILP32_OFFBIG : _bindgen_ty_21 = 177 ;
 pub const _SC_V6_LP64_OFF64 : _bindgen_ty_21 = 178 ;
 pub const _SC_V6_LPBIG_OFFBIG : _bindgen_ty_21 = 179 ;
 pub const _SC_HOST_NAME_MAX : _bindgen_ty_21 = 180 ;
 pub const _SC_TRACE : _bindgen_ty_21 = 181 ;
 pub const _SC_TRACE_EVENT_FILTER : _bindgen_ty_21 = 182 ;
 pub const _SC_TRACE_INHERIT : _bindgen_ty_21 = 183 ;
 pub const _SC_TRACE_LOG : _bindgen_ty_21 = 184 ;
 pub const _SC_LEVEL1_ICACHE_SIZE : _bindgen_ty_21 = 185 ;
 pub const _SC_LEVEL1_ICACHE_ASSOC : _bindgen_ty_21 = 186 ;
 pub const _SC_LEVEL1_ICACHE_LINESIZE : _bindgen_ty_21 = 187 ;
 pub const _SC_LEVEL1_DCACHE_SIZE : _bindgen_ty_21 = 188 ;
 pub const _SC_LEVEL1_DCACHE_ASSOC : _bindgen_ty_21 = 189 ;
 pub const _SC_LEVEL1_DCACHE_LINESIZE : _bindgen_ty_21 = 190 ;
 pub const _SC_LEVEL2_CACHE_SIZE : _bindgen_ty_21 = 191 ;
 pub const _SC_LEVEL2_CACHE_ASSOC : _bindgen_ty_21 = 192 ;
 pub const _SC_LEVEL2_CACHE_LINESIZE : _bindgen_ty_21 = 193 ;
 pub const _SC_LEVEL3_CACHE_SIZE : _bindgen_ty_21 = 194 ;
 pub const _SC_LEVEL3_CACHE_ASSOC : _bindgen_ty_21 = 195 ;
 pub const _SC_LEVEL3_CACHE_LINESIZE : _bindgen_ty_21 = 196 ;
 pub const _SC_LEVEL4_CACHE_SIZE : _bindgen_ty_21 = 197 ;
 pub const _SC_LEVEL4_CACHE_ASSOC : _bindgen_ty_21 = 198 ;
 pub const _SC_LEVEL4_CACHE_LINESIZE : _bindgen_ty_21 = 199 ;
 pub const _SC_IPV6 : _bindgen_ty_21 = 235 ;
 pub const _SC_RAW_SOCKETS : _bindgen_ty_21 = 236 ;
 pub const _SC_V7_ILP32_OFF32 : _bindgen_ty_21 = 237 ;
 pub const _SC_V7_ILP32_OFFBIG : _bindgen_ty_21 = 238 ;
 pub const _SC_V7_LP64_OFF64 : _bindgen_ty_21 = 239 ;
 pub const _SC_V7_LPBIG_OFFBIG : _bindgen_ty_21 = 240 ;
 pub const _SC_SS_REPL_MAX : _bindgen_ty_21 = 241 ;
 pub const _SC_TRACE_EVENT_NAME_MAX : _bindgen_ty_21 = 242 ;
 pub const _SC_TRACE_NAME_MAX : _bindgen_ty_21 = 243 ;
 pub const _SC_TRACE_SYS_MAX : _bindgen_ty_21 = 244 ;
 pub const _SC_TRACE_USER_EVENT_MAX : _bindgen_ty_21 = 245 ;
 pub const _SC_XOPEN_STREAMS : _bindgen_ty_21 = 246 ;
 pub const _SC_THREAD_ROBUST_PRIO_INHERIT : _bindgen_ty_21 = 247 ;
 pub const _SC_THREAD_ROBUST_PRIO_PROTECT : _bindgen_ty_21 = 248 ;
 pub type _bindgen_ty_21 = u32 ;
 pub const _CS_PATH : _bindgen_ty_22 = 0 ;
 pub const _CS_V6_WIDTH_RESTRICTED_ENVS : _bindgen_ty_22 = 1 ;
 pub const _CS_GNU_LIBC_VERSION : _bindgen_ty_22 = 2 ;
 pub const _CS_GNU_LIBPTHREAD_VERSION : _bindgen_ty_22 = 3 ;
 pub const _CS_V5_WIDTH_RESTRICTED_ENVS : _bindgen_ty_22 = 4 ;
 pub const _CS_V7_WIDTH_RESTRICTED_ENVS : _bindgen_ty_22 = 5 ;
 pub const _CS_LFS_CFLAGS : _bindgen_ty_22 = 1000 ;
 pub const _CS_LFS_LDFLAGS : _bindgen_ty_22 = 1001 ;
 pub const _CS_LFS_LIBS : _bindgen_ty_22 = 1002 ;
 pub const _CS_LFS_LINTFLAGS : _bindgen_ty_22 = 1003 ;
 pub const _CS_LFS64_CFLAGS : _bindgen_ty_22 = 1004 ;
 pub const _CS_LFS64_LDFLAGS : _bindgen_ty_22 = 1005 ;
 pub const _CS_LFS64_LIBS : _bindgen_ty_22 = 1006 ;
 pub const _CS_LFS64_LINTFLAGS : _bindgen_ty_22 = 1007 ;
 pub const _CS_XBS5_ILP32_OFF32_CFLAGS : _bindgen_ty_22 = 1100 ;
 pub const _CS_XBS5_ILP32_OFF32_LDFLAGS : _bindgen_ty_22 = 1101 ;
 pub const _CS_XBS5_ILP32_OFF32_LIBS : _bindgen_ty_22 = 1102 ;
 pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS : _bindgen_ty_22 = 1103 ;
 pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS : _bindgen_ty_22 = 1104 ;
 pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS : _bindgen_ty_22 = 1105 ;
 pub const _CS_XBS5_ILP32_OFFBIG_LIBS : _bindgen_ty_22 = 1106 ;
 pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS : _bindgen_ty_22 = 1107 ;
 pub const _CS_XBS5_LP64_OFF64_CFLAGS : _bindgen_ty_22 = 1108 ;
 pub const _CS_XBS5_LP64_OFF64_LDFLAGS : _bindgen_ty_22 = 1109 ;
 pub const _CS_XBS5_LP64_OFF64_LIBS : _bindgen_ty_22 = 1110 ;
 pub const _CS_XBS5_LP64_OFF64_LINTFLAGS : _bindgen_ty_22 = 1111 ;
 pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS : _bindgen_ty_22 = 1112 ;
 pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS : _bindgen_ty_22 = 1113 ;
 pub const _CS_XBS5_LPBIG_OFFBIG_LIBS : _bindgen_ty_22 = 1114 ;
 pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS : _bindgen_ty_22 = 1115 ;
 pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS : _bindgen_ty_22 = 1116 ;
 pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS : _bindgen_ty_22 = 1117 ;
 pub const _CS_POSIX_V6_ILP32_OFF32_LIBS : _bindgen_ty_22 = 1118 ;
 pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS : _bindgen_ty_22 = 1119 ;
 pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS : _bindgen_ty_22 = 1120 ;
 pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS : _bindgen_ty_22 = 1121 ;
 pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS : _bindgen_ty_22 = 1122 ;
 pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS : _bindgen_ty_22 = 1123 ;
 pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS : _bindgen_ty_22 = 1124 ;
 pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS : _bindgen_ty_22 = 1125 ;
 pub const _CS_POSIX_V6_LP64_OFF64_LIBS : _bindgen_ty_22 = 1126 ;
 pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS : _bindgen_ty_22 = 1127 ;
 pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS : _bindgen_ty_22 = 1128 ;
 pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS : _bindgen_ty_22 = 1129 ;
 pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS : _bindgen_ty_22 = 1130 ;
 pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS : _bindgen_ty_22 = 1131 ;
 pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS : _bindgen_ty_22 = 1132 ;
 pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS : _bindgen_ty_22 = 1133 ;
 pub const _CS_POSIX_V7_ILP32_OFF32_LIBS : _bindgen_ty_22 = 1134 ;
 pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS : _bindgen_ty_22 = 1135 ;
 pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS : _bindgen_ty_22 = 1136 ;
 pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS : _bindgen_ty_22 = 1137 ;
 pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS : _bindgen_ty_22 = 1138 ;
 pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS : _bindgen_ty_22 = 1139 ;
 pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS : _bindgen_ty_22 = 1140 ;
 pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS : _bindgen_ty_22 = 1141 ;
 pub const _CS_POSIX_V7_LP64_OFF64_LIBS : _bindgen_ty_22 = 1142 ;
 pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS : _bindgen_ty_22 = 1143 ;
 pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS : _bindgen_ty_22 = 1144 ;
 pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS : _bindgen_ty_22 = 1145 ;
 pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS : _bindgen_ty_22 = 1146 ;
 pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS : _bindgen_ty_22 = 1147 ;
 pub const _CS_V6_ENV : _bindgen_ty_22 = 1148 ;
 pub const _CS_V7_ENV : _bindgen_ty_22 = 1149 ;
 pub type _bindgen_ty_22 = u32 ;
 extern "C" {
pub fn pathconf (__path : * const :: std :: os :: raw :: c_char , __name : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn fpathconf (__fd : :: std :: os :: raw :: c_int , __name : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn sysconf (__name : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn confstr (__name : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> usize ;

}
 extern "C" {
pub fn getpid () -> __pid_t ;

}
 extern "C" {
pub fn getppid () -> __pid_t ;

}
 extern "C" {
pub fn getpgrp () -> __pid_t ;

}
 extern "C" {
pub fn __getpgid (__pid : __pid_t) -> __pid_t ;

}
 extern "C" {
pub fn getpgid (__pid : __pid_t) -> __pid_t ;

}
 extern "C" {
pub fn setpgid (__pid : __pid_t , __pgid : __pid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setpgrp () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setsid () -> __pid_t ;

}
 extern "C" {
pub fn getsid (__pid : __pid_t) -> __pid_t ;

}
 extern "C" {
pub fn getuid () -> __uid_t ;

}
 extern "C" {
pub fn geteuid () -> __uid_t ;

}
 extern "C" {
pub fn getgid () -> __gid_t ;

}
 extern "C" {
pub fn getegid () -> __gid_t ;

}
 extern "C" {
pub fn getgroups (__size : :: std :: os :: raw :: c_int , __list : * mut __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn group_member (__gid : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setuid (__uid : __uid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setreuid (__ruid : __uid_t , __euid : __uid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn seteuid (__uid : __uid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setgid (__gid : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setregid (__rgid : __gid_t , __egid : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setegid (__gid : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getresuid (__ruid : * mut __uid_t , __euid : * mut __uid_t , __suid : * mut __uid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getresgid (__rgid : * mut __gid_t , __egid : * mut __gid_t , __sgid : * mut __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setresuid (__ruid : __uid_t , __euid : __uid_t , __suid : __uid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setresgid (__rgid : __gid_t , __egid : __gid_t , __sgid : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fork () -> __pid_t ;

}
 extern "C" {
pub fn vfork () -> __pid_t ;

}
 extern "C" {
pub fn ttyname (__fd : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn ttyname_r (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isatty (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ttyslot () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn link (__from : * const :: std :: os :: raw :: c_char , __to : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn linkat (__fromfd : :: std :: os :: raw :: c_int , __from : * const :: std :: os :: raw :: c_char , __tofd : :: std :: os :: raw :: c_int , __to : * const :: std :: os :: raw :: c_char , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn symlink (__from : * const :: std :: os :: raw :: c_char , __to : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn readlink (__path : * const :: std :: os :: raw :: c_char , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> isize ;

}
 extern "C" {
pub fn symlinkat (__from : * const :: std :: os :: raw :: c_char , __tofd : :: std :: os :: raw :: c_int , __to : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn readlinkat (__fd : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> isize ;

}
 extern "C" {
pub fn unlink (__name : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn unlinkat (__fd : :: std :: os :: raw :: c_int , __name : * const :: std :: os :: raw :: c_char , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rmdir (__path : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn tcgetpgrp (__fd : :: std :: os :: raw :: c_int) -> __pid_t ;

}
 extern "C" {
pub fn tcsetpgrp (__fd : :: std :: os :: raw :: c_int , __pgrp_id : __pid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getlogin () -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn getlogin_r (__name : * mut :: std :: os :: raw :: c_char , __name_len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setlogin (__name : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut optarg : * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut optind : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut opterr : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut optopt : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getopt (___argc : :: std :: os :: raw :: c_int , ___argv : * const * mut :: std :: os :: raw :: c_char , __shortopts : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gethostname (__name : * mut :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sethostname (__name : * const :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sethostid (__id : :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getdomainname (__name : * mut :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setdomainname (__name : * const :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vhangup () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn revoke (__file : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn profil (__sample_buffer : * mut :: std :: os :: raw :: c_ushort , __size : usize , __offset : usize , __scale : :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn acct (__name : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getusershell () -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn endusershell () ;

}
 extern "C" {
pub fn setusershell () ;

}
 extern "C" {
pub fn daemon (__nochdir : :: std :: os :: raw :: c_int , __noclose : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn chroot (__path : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpass (__prompt : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn fsync (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn syncfs (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gethostid () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn sync () ;

}
 extern "C" {
pub fn getpagesize () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getdtablesize () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn truncate (__file : * const :: std :: os :: raw :: c_char , __length : __off_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn truncate64 (__file : * const :: std :: os :: raw :: c_char , __length : __off64_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ftruncate (__fd : :: std :: os :: raw :: c_int , __length : __off_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ftruncate64 (__fd : :: std :: os :: raw :: c_int , __length : __off64_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn brk (__addr : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sbrk (__delta : isize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn syscall (__sysno : :: std :: os :: raw :: c_long , ...) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn copy_file_range (__infd : :: std :: os :: raw :: c_int , __pinoff : * mut __off64_t , __outfd : :: std :: os :: raw :: c_int , __poutoff : * mut __off64_t , __length : usize , __flags : :: std :: os :: raw :: c_uint) -> isize ;

}
 extern "C" {
pub fn fdatasync (__fildes : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn crypt (__key : * const :: std :: os :: raw :: c_char , __salt : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn encrypt (__glibc_block : * mut :: std :: os :: raw :: c_char , __edflag : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn swab (__from : * const :: std :: os :: raw :: c_void , __to : * mut :: std :: os :: raw :: c_void , __n : isize) ;

}
 extern "C" {
pub fn getentropy (__buffer : * mut :: std :: os :: raw :: c_void , __length : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sendfile (__out_fd : :: std :: os :: raw :: c_int , __in_fd : :: std :: os :: raw :: c_int , __offset : * mut off_t , __count : usize) -> isize ;

}
 extern "C" {
pub fn sendfile64 (__out_fd : :: std :: os :: raw :: c_int , __in_fd : :: std :: os :: raw :: c_int , __offset : * mut __off64_t , __count : usize) -> isize ;

}
 pub const s_proctitle_space : :: std :: os :: raw :: c_uint = 0 ;
 pub const s_proctitle_inited : :: std :: os :: raw :: c_int = 0 ;
 extern "C" {
//# [link_name = "\u{1}_ZL13s_p_proctitle"] 
pub static mut s_p_proctitle : * mut :: std :: os :: raw :: c_char ;

}
 pub type pam_item_t = * const :: std :: os :: raw :: c_void ;
 extern "C" {
pub fn memcpy (__dest : * mut :: std :: os :: raw :: c_void , __src : * const :: std :: os :: raw :: c_void , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn memmove (__dest : * mut :: std :: os :: raw :: c_void , __src : * const :: std :: os :: raw :: c_void , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn memccpy (__dest : * mut :: std :: os :: raw :: c_void , __src : * const :: std :: os :: raw :: c_void , __c : :: std :: os :: raw :: c_int , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn memset (__s : * mut :: std :: os :: raw :: c_void , __c : :: std :: os :: raw :: c_int , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn memcmp (__s1 : * const :: std :: os :: raw :: c_void , __s2 : * const :: std :: os :: raw :: c_void , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn memchr (__s : * const :: std :: os :: raw :: c_void , __c : :: std :: os :: raw :: c_int , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn rawmemchr (__s : * const :: std :: os :: raw :: c_void , __c : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn memrchr (__s : * const :: std :: os :: raw :: c_void , __c : :: std :: os :: raw :: c_int , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn strcpy (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strncpy (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char , __n : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strcat (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strncat (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char , __n : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strcmp (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strncmp (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strcoll (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strxfrm (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char , __n : usize) -> usize ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __locale_struct {
pub __locales : [* mut __locale_data ; 13usize] , pub __ctype_b : * const :: std :: os :: raw :: c_ushort , pub __ctype_tolower : * const :: std :: os :: raw :: c_int , pub __ctype_toupper : * const :: std :: os :: raw :: c_int , pub __names : [* const :: std :: os :: raw :: c_char ; 13usize] ,
}
 # [test] fn bindgen_test_layout___locale_struct () {
assert_eq ! (:: std :: mem :: size_of :: < __locale_struct > ( ) , 232usize , concat ! ( "Size of: " , stringify ! ( __locale_struct ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __locale_struct > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __locale_struct ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __locales as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __locales ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __ctype_b as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __ctype_b ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __ctype_tolower as * const _ as usize } , 112usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __ctype_tolower ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __ctype_toupper as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __ctype_toupper ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __locale_struct > ( ) ) ) . __names as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( __locale_struct ) , "::" , stringify ! ( __names ) )) ;

}
 pub type __locale_t = * mut __locale_struct ;
 pub type locale_t = __locale_t ;
 extern "C" {
pub fn strcoll_l (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char , __l : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strxfrm_l (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char , __n : usize , __l : locale_t) -> usize ;

}
 extern "C" {
pub fn strdup (__s : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strndup (__string : * const :: std :: os :: raw :: c_char , __n : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strchr (__s : * const :: std :: os :: raw :: c_char , __c : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strrchr (__s : * const :: std :: os :: raw :: c_char , __c : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strchrnul (__s : * const :: std :: os :: raw :: c_char , __c : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strcspn (__s : * const :: std :: os :: raw :: c_char , __reject : * const :: std :: os :: raw :: c_char) -> usize ;

}
 extern "C" {
pub fn strspn (__s : * const :: std :: os :: raw :: c_char , __accept : * const :: std :: os :: raw :: c_char) -> usize ;

}
 extern "C" {
pub fn strpbrk (__s : * const :: std :: os :: raw :: c_char , __accept : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strstr (__haystack : * const :: std :: os :: raw :: c_char , __needle : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strtok (__s : * mut :: std :: os :: raw :: c_char , __delim : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn __strtok_r (__s : * mut :: std :: os :: raw :: c_char , __delim : * const :: std :: os :: raw :: c_char , __save_ptr : * mut * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strtok_r (__s : * mut :: std :: os :: raw :: c_char , __delim : * const :: std :: os :: raw :: c_char , __save_ptr : * mut * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strcasestr (__haystack : * const :: std :: os :: raw :: c_char , __needle : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn memmem (__haystack : * const :: std :: os :: raw :: c_void , __haystacklen : usize , __needle : * const :: std :: os :: raw :: c_void , __needlelen : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn __mempcpy (__dest : * mut :: std :: os :: raw :: c_void , __src : * const :: std :: os :: raw :: c_void , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn mempcpy (__dest : * mut :: std :: os :: raw :: c_void , __src : * const :: std :: os :: raw :: c_void , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn strlen (__s : * const :: std :: os :: raw :: c_char) -> usize ;

}
 extern "C" {
pub fn strnlen (__string : * const :: std :: os :: raw :: c_char , __maxlen : usize) -> usize ;

}
 extern "C" {
pub fn strerror (__errnum : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strerror_r (__errnum : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strerror_l (__errnum : :: std :: os :: raw :: c_int , __l : locale_t) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn bcmp (__s1 : * const :: std :: os :: raw :: c_void , __s2 : * const :: std :: os :: raw :: c_void , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn bcopy (__src : * const :: std :: os :: raw :: c_void , __dest : * mut :: std :: os :: raw :: c_void , __n : usize) ;

}
 extern "C" {
pub fn bzero (__s : * mut :: std :: os :: raw :: c_void , __n : usize) ;

}
 extern "C" {
pub fn index (__s : * const :: std :: os :: raw :: c_char , __c : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn rindex (__s : * const :: std :: os :: raw :: c_char , __c : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn ffs (__i : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ffsl (__l : :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ffsll (__ll : :: std :: os :: raw :: c_longlong) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strcasecmp (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strncasecmp (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strcasecmp_l (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char , __loc : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strncasecmp_l (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char , __n : usize , __loc : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn explicit_bzero (__s : * mut :: std :: os :: raw :: c_void , __n : usize) ;

}
 extern "C" {
pub fn strsep (__stringp : * mut * mut :: std :: os :: raw :: c_char , __delim : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strsignal (__sig : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn __stpcpy (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn stpcpy (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn __stpncpy (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char , __n : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn stpncpy (__dest : * mut :: std :: os :: raw :: c_char , __src : * const :: std :: os :: raw :: c_char , __n : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strverscmp (__s1 : * const :: std :: os :: raw :: c_char , __s2 : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfry (__string : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn memfrob (__s : * mut :: std :: os :: raw :: c_void , __n : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn basename (__filename : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 pub type std_nullptr_t = * const :: std :: os :: raw :: c_void ;
 pub const idtype_t_P_ALL : idtype_t = 0 ;
 pub const idtype_t_P_PID : idtype_t = 1 ;
 pub const idtype_t_P_PGID : idtype_t = 2 ;
 pub type idtype_t = u32 ;
 pub type _Float32 = f32 ;
 pub type _Float64 = f64 ;
 pub type _Float32x = f64 ;
 pub type _Float64x = u128 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct div_t {
pub quot : :: std :: os :: raw :: c_int , pub rem : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_div_t () {
assert_eq ! (:: std :: mem :: size_of :: < div_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( div_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < div_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( div_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < div_t > ( ) ) ) . quot as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( div_t ) , "::" , stringify ! ( quot ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < div_t > ( ) ) ) . rem as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( div_t ) , "::" , stringify ! ( rem ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct ldiv_t {
pub quot : :: std :: os :: raw :: c_long , pub rem : :: std :: os :: raw :: c_long ,
}
 # [test] fn bindgen_test_layout_ldiv_t () {
assert_eq ! (:: std :: mem :: size_of :: < ldiv_t > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( ldiv_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < ldiv_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( ldiv_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ldiv_t > ( ) ) ) . quot as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( ldiv_t ) , "::" , stringify ! ( quot ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < ldiv_t > ( ) ) ) . rem as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( ldiv_t ) , "::" , stringify ! ( rem ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct lldiv_t {
pub quot : :: std :: os :: raw :: c_longlong , pub rem : :: std :: os :: raw :: c_longlong ,
}
 # [test] fn bindgen_test_layout_lldiv_t () {
assert_eq ! (:: std :: mem :: size_of :: < lldiv_t > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( lldiv_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < lldiv_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( lldiv_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < lldiv_t > ( ) ) ) . quot as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( lldiv_t ) , "::" , stringify ! ( quot ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < lldiv_t > ( ) ) ) . rem as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( lldiv_t ) , "::" , stringify ! ( rem ) )) ;

}
 extern "C" {
pub fn __ctype_get_mb_cur_max () -> usize ;

}
 extern "C" {
pub fn atof (__nptr : * const :: std :: os :: raw :: c_char) -> f64 ;

}
 extern "C" {
pub fn atoi (__nptr : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn atol (__nptr : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn atoll (__nptr : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_longlong ;

}
 extern "C" {
pub fn strtod (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> f64 ;

}
 extern "C" {
pub fn strtof (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> f32 ;

}
 extern "C" {
pub fn strtold (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> u128 ;

}
 extern "C" {
pub fn strtof32 (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> _Float32 ;

}
 extern "C" {
pub fn strtof64 (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> _Float64 ;

}
 extern "C" {
pub fn strtof32x (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> _Float32x ;

}
 extern "C" {
pub fn strtof64x (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char) -> _Float64x ;

}
 extern "C" {
pub fn strtol (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn strtoul (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_ulong ;

}
 extern "C" {
pub fn strtoq (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_longlong ;

}
 extern "C" {
pub fn strtouq (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_ulonglong ;

}
 extern "C" {
pub fn strtoll (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_longlong ;

}
 extern "C" {
pub fn strtoull (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_ulonglong ;

}
 extern "C" {
pub fn strfromd (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : f64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfromf (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : f32) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfroml (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : u128) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfromf32 (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : _Float32) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfromf64 (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : _Float64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfromf32x (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : _Float32x) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strfromf64x (__dest : * mut :: std :: os :: raw :: c_char , __size : usize , __format : * const :: std :: os :: raw :: c_char , __f : _Float64x) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn strtol_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int , __loc : locale_t) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn strtoul_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int , __loc : locale_t) -> :: std :: os :: raw :: c_ulong ;

}
 extern "C" {
pub fn strtoll_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int , __loc : locale_t) -> :: std :: os :: raw :: c_longlong ;

}
 extern "C" {
pub fn strtoull_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __base : :: std :: os :: raw :: c_int , __loc : locale_t) -> :: std :: os :: raw :: c_ulonglong ;

}
 extern "C" {
pub fn strtod_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> f64 ;

}
 extern "C" {
pub fn strtof_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> f32 ;

}
 extern "C" {
pub fn strtold_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> u128 ;

}
 extern "C" {
pub fn strtof32_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> _Float32 ;

}
 extern "C" {
pub fn strtof64_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> _Float64 ;

}
 extern "C" {
pub fn strtof32x_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> _Float32x ;

}
 extern "C" {
pub fn strtof64x_l (__nptr : * const :: std :: os :: raw :: c_char , __endptr : * mut * mut :: std :: os :: raw :: c_char , __loc : locale_t) -> _Float64x ;

}
 extern "C" {
pub fn l64a (__n : :: std :: os :: raw :: c_long) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn a64l (__s : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn random () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn srandom (__seed : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
pub fn initstate (__seed : :: std :: os :: raw :: c_uint , __statebuf : * mut :: std :: os :: raw :: c_char , __statelen : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn setstate (__statebuf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct random_data {
pub fptr : * mut i32 , pub rptr : * mut i32 , pub state : * mut i32 , pub rand_type : :: std :: os :: raw :: c_int , pub rand_deg : :: std :: os :: raw :: c_int , pub rand_sep : :: std :: os :: raw :: c_int , pub end_ptr : * mut i32 ,
}
 # [test] fn bindgen_test_layout_random_data () {
assert_eq ! (:: std :: mem :: size_of :: < random_data > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( random_data ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < random_data > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( random_data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . fptr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( fptr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . rptr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( rptr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . state as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( state ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . rand_type as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( rand_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . rand_deg as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( rand_deg ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . rand_sep as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( rand_sep ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < random_data > ( ) ) ) . end_ptr as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( random_data ) , "::" , stringify ! ( end_ptr ) )) ;

}
 extern "C" {
pub fn random_r (__buf : * mut random_data , __result : * mut i32) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn srandom_r (__seed : :: std :: os :: raw :: c_uint , __buf : * mut random_data) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn initstate_r (__seed : :: std :: os :: raw :: c_uint , __statebuf : * mut :: std :: os :: raw :: c_char , __statelen : usize , __buf : * mut random_data) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setstate_r (__statebuf : * mut :: std :: os :: raw :: c_char , __buf : * mut random_data) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rand () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn srand (__seed : :: std :: os :: raw :: c_uint) ;

}
 extern "C" {
pub fn rand_r (__seed : * mut :: std :: os :: raw :: c_uint) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn drand48 () -> f64 ;

}
 extern "C" {
pub fn erand48 (__xsubi : * mut :: std :: os :: raw :: c_ushort) -> f64 ;

}
 extern "C" {
pub fn lrand48 () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn nrand48 (__xsubi : * mut :: std :: os :: raw :: c_ushort) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn mrand48 () -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn jrand48 (__xsubi : * mut :: std :: os :: raw :: c_ushort) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn srand48 (__seedval : :: std :: os :: raw :: c_long) ;

}
 extern "C" {
pub fn seed48 (__seed16v : * mut :: std :: os :: raw :: c_ushort) -> * mut :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
pub fn lcong48 (__param : * mut :: std :: os :: raw :: c_ushort) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct drand48_data {
pub __x : [:: std :: os :: raw :: c_ushort ; 3usize] , pub __old_x : [:: std :: os :: raw :: c_ushort ; 3usize] , pub __c : :: std :: os :: raw :: c_ushort , pub __init : :: std :: os :: raw :: c_ushort , pub __a : :: std :: os :: raw :: c_ulonglong ,
}
 # [test] fn bindgen_test_layout_drand48_data () {
assert_eq ! (:: std :: mem :: size_of :: < drand48_data > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( drand48_data ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < drand48_data > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( drand48_data ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < drand48_data > ( ) ) ) . __x as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( drand48_data ) , "::" , stringify ! ( __x ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < drand48_data > ( ) ) ) . __old_x as * const _ as usize } , 6usize , concat ! ( "Offset of field: " , stringify ! ( drand48_data ) , "::" , stringify ! ( __old_x ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < drand48_data > ( ) ) ) . __c as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( drand48_data ) , "::" , stringify ! ( __c ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < drand48_data > ( ) ) ) . __init as * const _ as usize } , 14usize , concat ! ( "Offset of field: " , stringify ! ( drand48_data ) , "::" , stringify ! ( __init ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < drand48_data > ( ) ) ) . __a as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( drand48_data ) , "::" , stringify ! ( __a ) )) ;

}
 extern "C" {
pub fn drand48_r (__buffer : * mut drand48_data , __result : * mut f64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn erand48_r (__xsubi : * mut :: std :: os :: raw :: c_ushort , __buffer : * mut drand48_data , __result : * mut f64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lrand48_r (__buffer : * mut drand48_data , __result : * mut :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn nrand48_r (__xsubi : * mut :: std :: os :: raw :: c_ushort , __buffer : * mut drand48_data , __result : * mut :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mrand48_r (__buffer : * mut drand48_data , __result : * mut :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn jrand48_r (__xsubi : * mut :: std :: os :: raw :: c_ushort , __buffer : * mut drand48_data , __result : * mut :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn srand48_r (__seedval : :: std :: os :: raw :: c_long , __buffer : * mut drand48_data) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn seed48_r (__seed16v : * mut :: std :: os :: raw :: c_ushort , __buffer : * mut drand48_data) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lcong48_r (__param : * mut :: std :: os :: raw :: c_ushort , __buffer : * mut drand48_data) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn malloc (__size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn calloc (__nmemb : usize , __size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn realloc (__ptr : * mut :: std :: os :: raw :: c_void , __size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn reallocarray (__ptr : * mut :: std :: os :: raw :: c_void , __nmemb : usize , __size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn free (__ptr : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
pub fn alloca (__size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn valloc (__size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn posix_memalign (__memptr : * mut * mut :: std :: os :: raw :: c_void , __alignment : usize , __size : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn aligned_alloc (__alignment : usize , __size : usize) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn abort () ;

}
 extern "C" {
pub fn atexit (__func : :: std :: option :: Option < unsafe extern "C" fn ( ) >) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn at_quick_exit (__func : :: std :: option :: Option < unsafe extern "C" fn ( ) >) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn on_exit (__func : :: std :: option :: Option < unsafe extern "C" fn ( __status : :: std :: os :: raw :: c_int , __arg : * mut :: std :: os :: raw :: c_void ) > , __arg : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn exit (__status : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn quick_exit (__status : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn _Exit (__status : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn getenv (__name : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn secure_getenv (__name : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn putenv (__string : * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setenv (__name : * const :: std :: os :: raw :: c_char , __value : * const :: std :: os :: raw :: c_char , __replace : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn unsetenv (__name : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clearenv () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mktemp (__template : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn mkstemp (__template : * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkstemp64 (__template : * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkstemps (__template : * mut :: std :: os :: raw :: c_char , __suffixlen : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkstemps64 (__template : * mut :: std :: os :: raw :: c_char , __suffixlen : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkdtemp (__template : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn mkostemp (__template : * mut :: std :: os :: raw :: c_char , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkostemp64 (__template : * mut :: std :: os :: raw :: c_char , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkostemps (__template : * mut :: std :: os :: raw :: c_char , __suffixlen : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkostemps64 (__template : * mut :: std :: os :: raw :: c_char , __suffixlen : :: std :: os :: raw :: c_int , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn system (__command : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn canonicalize_file_name (__name : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn realpath (__name : * const :: std :: os :: raw :: c_char , __resolved : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 pub type __compar_fn_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_void , arg2 : * const :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int > ;
 pub type comparison_fn_t = __compar_fn_t ;
 pub type __compar_d_fn_t = :: std :: option :: Option < unsafe extern "C" fn (arg1 : * const :: std :: os :: raw :: c_void , arg2 : * const :: std :: os :: raw :: c_void , arg3 : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int > ;
 extern "C" {
pub fn bsearch (__key : * const :: std :: os :: raw :: c_void , __base : * const :: std :: os :: raw :: c_void , __nmemb : usize , __size : usize , __compar : __compar_fn_t) -> * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
pub fn qsort (__base : * mut :: std :: os :: raw :: c_void , __nmemb : usize , __size : usize , __compar : __compar_fn_t) ;

}
 extern "C" {
pub fn qsort_r (__base : * mut :: std :: os :: raw :: c_void , __nmemb : usize , __size : usize , __compar : __compar_d_fn_t , __arg : * mut :: std :: os :: raw :: c_void) ;

}
 extern "C" {
pub fn abs (__x : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn labs (__x : :: std :: os :: raw :: c_long) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn llabs (__x : :: std :: os :: raw :: c_longlong) -> :: std :: os :: raw :: c_longlong ;

}
 extern "C" {
pub fn div (__numer : :: std :: os :: raw :: c_int , __denom : :: std :: os :: raw :: c_int) -> div_t ;

}
 extern "C" {
pub fn ldiv (__numer : :: std :: os :: raw :: c_long , __denom : :: std :: os :: raw :: c_long) -> ldiv_t ;

}
 extern "C" {
pub fn lldiv (__numer : :: std :: os :: raw :: c_longlong , __denom : :: std :: os :: raw :: c_longlong) -> lldiv_t ;

}
 extern "C" {
pub fn ecvt (__value : f64 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn fcvt (__value : f64 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn gcvt (__value : f64 , __ndigit : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn qecvt (__value : u128 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn qfcvt (__value : u128 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn qgcvt (__value : u128 , __ndigit : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn ecvt_r (__value : f64 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fcvt_r (__value : f64 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn qecvt_r (__value : u128 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn qfcvt_r (__value : u128 , __ndigit : :: std :: os :: raw :: c_int , __decpt : * mut :: std :: os :: raw :: c_int , __sign : * mut :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mblen (__s : * const :: std :: os :: raw :: c_char , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mbtowc (__pwc : * mut u32 , __s : * const :: std :: os :: raw :: c_char , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn wctomb (__s : * mut :: std :: os :: raw :: c_char , __wchar : u32) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mbstowcs (__pwcs : * mut u32 , __s : * const :: std :: os :: raw :: c_char , __n : usize) -> usize ;

}
 extern "C" {
pub fn wcstombs (__s : * mut :: std :: os :: raw :: c_char , __pwcs : * const u32 , __n : usize) -> usize ;

}
 extern "C" {
pub fn rpmatch (__response : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getsubopt (__optionp : * mut * mut :: std :: os :: raw :: c_char , __tokens : * const * mut :: std :: os :: raw :: c_char , __valuep : * mut * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setkey (__key : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
pub fn posix_openpt (__oflag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn grantpt (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn unlockpt (__fd : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ptsname (__fd : :: std :: os :: raw :: c_int) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn ptsname_r (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpt () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getloadavg (__loadavg : * mut f64 , __nelem : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn stat (__file : * const :: std :: os :: raw :: c_char , __buf : * mut stat) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fstat (__fd : :: std :: os :: raw :: c_int , __buf : * mut stat) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn stat64 (__file : * const :: std :: os :: raw :: c_char , __buf : * mut stat64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fstat64 (__fd : :: std :: os :: raw :: c_int , __buf : * mut stat64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fstatat (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __buf : * mut stat , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fstatat64 (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __buf : * mut stat64 , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lstat (__file : * const :: std :: os :: raw :: c_char , __buf : * mut stat) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lstat64 (__file : * const :: std :: os :: raw :: c_char , __buf : * mut stat64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn chmod (__file : * const :: std :: os :: raw :: c_char , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn lchmod (__file : * const :: std :: os :: raw :: c_char , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fchmod (__fd : :: std :: os :: raw :: c_int , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fchmodat (__fd : :: std :: os :: raw :: c_int , __file : * const :: std :: os :: raw :: c_char , __mode : __mode_t , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn umask (__mask : __mode_t) -> __mode_t ;

}
 extern "C" {
pub fn getumask () -> __mode_t ;

}
 extern "C" {
pub fn mkdir (__path : * const :: std :: os :: raw :: c_char , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkdirat (__fd : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mknod (__path : * const :: std :: os :: raw :: c_char , __mode : __mode_t , __dev : __dev_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mknodat (__fd : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __mode : __mode_t , __dev : __dev_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkfifo (__path : * const :: std :: os :: raw :: c_char , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn mkfifoat (__fd : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __mode : __mode_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn utimensat (__fd : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __times : * const timespec , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn futimens (__fd : :: std :: os :: raw :: c_int , __times : * const timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __fxstat (__ver : :: std :: os :: raw :: c_int , __fildes : :: std :: os :: raw :: c_int , __stat_buf : * mut stat) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __xstat (__ver : :: std :: os :: raw :: c_int , __filename : * const :: std :: os :: raw :: c_char , __stat_buf : * mut stat) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __lxstat (__ver : :: std :: os :: raw :: c_int , __filename : * const :: std :: os :: raw :: c_char , __stat_buf : * mut stat) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __fxstatat (__ver : :: std :: os :: raw :: c_int , __fildes : :: std :: os :: raw :: c_int , __filename : * const :: std :: os :: raw :: c_char , __stat_buf : * mut stat , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __fxstat64 (__ver : :: std :: os :: raw :: c_int , __fildes : :: std :: os :: raw :: c_int , __stat_buf : * mut stat64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __xstat64 (__ver : :: std :: os :: raw :: c_int , __filename : * const :: std :: os :: raw :: c_char , __stat_buf : * mut stat64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __lxstat64 (__ver : :: std :: os :: raw :: c_int , __filename : * const :: std :: os :: raw :: c_char , __stat_buf : * mut stat64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __fxstatat64 (__ver : :: std :: os :: raw :: c_int , __fildes : :: std :: os :: raw :: c_int , __filename : * const :: std :: os :: raw :: c_char , __stat_buf : * mut stat64 , __flag : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __xmknod (__ver : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __mode : __mode_t , __dev : * mut __dev_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __xmknodat (__ver : :: std :: os :: raw :: c_int , __fd : :: std :: os :: raw :: c_int , __path : * const :: std :: os :: raw :: c_char , __mode : __mode_t , __dev : * mut __dev_t) -> :: std :: os :: raw :: c_int ;

}
 pub type __FILE = _IO_FILE ;
 pub type FILE = _IO_FILE ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct __mbstate_t {
pub __count : :: std :: os :: raw :: c_int , pub __value : __mbstate_t__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union __mbstate_t__bindgen_ty_1 {
pub __wch : :: std :: os :: raw :: c_uint , pub __wchb : [:: std :: os :: raw :: c_char ; 4usize] , _bindgen_union_align : u32 ,
}
 # [test] fn bindgen_test_layout___mbstate_t__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < __mbstate_t__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Size of: " , stringify ! ( __mbstate_t__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __mbstate_t__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __mbstate_t__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __mbstate_t__bindgen_ty_1 > ( ) ) ) . __wch as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __mbstate_t__bindgen_ty_1 ) , "::" , stringify ! ( __wch ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __mbstate_t__bindgen_ty_1 > ( ) ) ) . __wchb as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __mbstate_t__bindgen_ty_1 ) , "::" , stringify ! ( __wchb ) )) ;

}
 # [test] fn bindgen_test_layout___mbstate_t () {
assert_eq ! (:: std :: mem :: size_of :: < __mbstate_t > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( __mbstate_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __mbstate_t > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( __mbstate_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __mbstate_t > ( ) ) ) . __count as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __mbstate_t ) , "::" , stringify ! ( __count ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __mbstate_t > ( ) ) ) . __value as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __mbstate_t ) , "::" , stringify ! ( __value ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct _G_fpos_t {
pub __pos : __off_t , pub __state : __mbstate_t ,
}
 # [test] fn bindgen_test_layout__G_fpos_t () {
assert_eq ! (:: std :: mem :: size_of :: < _G_fpos_t > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( _G_fpos_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _G_fpos_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _G_fpos_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _G_fpos_t > ( ) ) ) . __pos as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _G_fpos_t ) , "::" , stringify ! ( __pos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _G_fpos_t > ( ) ) ) . __state as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _G_fpos_t ) , "::" , stringify ! ( __state ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct _G_fpos64_t {
pub __pos : __off64_t , pub __state : __mbstate_t ,
}
 # [test] fn bindgen_test_layout__G_fpos64_t () {
assert_eq ! (:: std :: mem :: size_of :: < _G_fpos64_t > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( _G_fpos64_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _G_fpos64_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _G_fpos64_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _G_fpos64_t > ( ) ) ) . __pos as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _G_fpos64_t ) , "::" , stringify ! ( __pos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _G_fpos64_t > ( ) ) ) . __state as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _G_fpos64_t ) , "::" , stringify ! ( __state ) )) ;

}
 pub type va_list = __builtin_va_list ;
 pub type __gnuc_va_list = __builtin_va_list ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _IO_jump_t {
_unused : [u8 ; 0] ,
}
 pub type _IO_lock_t = :: std :: os :: raw :: c_void ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _IO_marker {
pub _next : * mut _IO_marker , pub _sbuf : * mut _IO_FILE , pub _pos : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout__IO_marker () {
assert_eq ! (:: std :: mem :: size_of :: < _IO_marker > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( _IO_marker ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _IO_marker > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _IO_marker ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_marker > ( ) ) ) . _next as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _IO_marker ) , "::" , stringify ! ( _next ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_marker > ( ) ) ) . _sbuf as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _IO_marker ) , "::" , stringify ! ( _sbuf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_marker > ( ) ) ) . _pos as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( _IO_marker ) , "::" , stringify ! ( _pos ) )) ;

}
 pub const __codecvt_result___codecvt_ok : __codecvt_result = 0 ;
 pub const __codecvt_result___codecvt_partial : __codecvt_result = 1 ;
 pub const __codecvt_result___codecvt_error : __codecvt_result = 2 ;
 pub const __codecvt_result___codecvt_noconv : __codecvt_result = 3 ;
 pub type __codecvt_result = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _IO_FILE {
pub _flags : :: std :: os :: raw :: c_int , pub _IO_read_ptr : * mut :: std :: os :: raw :: c_char , pub _IO_read_end : * mut :: std :: os :: raw :: c_char , pub _IO_read_base : * mut :: std :: os :: raw :: c_char , pub _IO_write_base : * mut :: std :: os :: raw :: c_char , pub _IO_write_ptr : * mut :: std :: os :: raw :: c_char , pub _IO_write_end : * mut :: std :: os :: raw :: c_char , pub _IO_buf_base : * mut :: std :: os :: raw :: c_char , pub _IO_buf_end : * mut :: std :: os :: raw :: c_char , pub _IO_save_base : * mut :: std :: os :: raw :: c_char , pub _IO_backup_base : * mut :: std :: os :: raw :: c_char , pub _IO_save_end : * mut :: std :: os :: raw :: c_char , pub _markers : * mut _IO_marker , pub _chain : * mut _IO_FILE , pub _fileno : :: std :: os :: raw :: c_int , pub _flags2 : :: std :: os :: raw :: c_int , pub _old_offset : __off_t , pub _cur_column : :: std :: os :: raw :: c_ushort , pub _vtable_offset : :: std :: os :: raw :: c_schar , pub _shortbuf : [:: std :: os :: raw :: c_char ; 1usize] , pub _lock : * mut _IO_lock_t , pub _offset : __off64_t , pub __pad1 : * mut :: std :: os :: raw :: c_void , pub __pad2 : * mut :: std :: os :: raw :: c_void , pub __pad3 : * mut :: std :: os :: raw :: c_void , pub __pad4 : * mut :: std :: os :: raw :: c_void , pub __pad5 : usize , pub _mode : :: std :: os :: raw :: c_int , pub _unused2 : [:: std :: os :: raw :: c_char ; 20usize] ,
}
 # [test] fn bindgen_test_layout__IO_FILE () {
assert_eq ! (:: std :: mem :: size_of :: < _IO_FILE > ( ) , 216usize , concat ! ( "Size of: " , stringify ! ( _IO_FILE ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _IO_FILE > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _IO_FILE ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _flags as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_read_ptr as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_read_ptr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_read_end as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_read_end ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_read_base as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_read_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_write_base as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_write_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_write_ptr as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_write_ptr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_write_end as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_write_end ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_buf_base as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_buf_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_buf_end as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_buf_end ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_save_base as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_save_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_backup_base as * const _ as usize } , 80usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_backup_base ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _IO_save_end as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _IO_save_end ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _markers as * const _ as usize } , 96usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _markers ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _chain as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _chain ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _fileno as * const _ as usize } , 112usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _fileno ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _flags2 as * const _ as usize } , 116usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _flags2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _old_offset as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _old_offset ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _cur_column as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _cur_column ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _vtable_offset as * const _ as usize } , 130usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _vtable_offset ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _shortbuf as * const _ as usize } , 131usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _shortbuf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _lock as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _lock ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _offset as * const _ as usize } , 144usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _offset ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . __pad1 as * const _ as usize } , 152usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( __pad1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . __pad2 as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( __pad2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . __pad3 as * const _ as usize } , 168usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( __pad3 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . __pad4 as * const _ as usize } , 176usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( __pad4 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . __pad5 as * const _ as usize } , 184usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( __pad5 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _mode as * const _ as usize } , 192usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _mode ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_FILE > ( ) ) ) . _unused2 as * const _ as usize } , 196usize , concat ! ( "Offset of field: " , stringify ! ( _IO_FILE ) , "::" , stringify ! ( _unused2 ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _IO_FILE_plus {
_unused : [u8 ; 0] ,
}
 extern "C" {
pub static mut _IO_2_1_stdin_ : _IO_FILE_plus ;

}
 extern "C" {
pub static mut _IO_2_1_stdout_ : _IO_FILE_plus ;

}
 extern "C" {
pub static mut _IO_2_1_stderr_ : _IO_FILE_plus ;

}
 pub type __io_read_fn = :: std :: option :: Option < unsafe extern "C" fn (__cookie : * mut :: std :: os :: raw :: c_void , __buf : * mut :: std :: os :: raw :: c_char , __nbytes : usize) -> __ssize_t > ;
 pub type __io_write_fn = :: std :: option :: Option < unsafe extern "C" fn (__cookie : * mut :: std :: os :: raw :: c_void , __buf : * const :: std :: os :: raw :: c_char , __n : usize) -> __ssize_t > ;
 pub type __io_seek_fn = :: std :: option :: Option < unsafe extern "C" fn (__cookie : * mut :: std :: os :: raw :: c_void , __pos : * mut __off64_t , __w : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int > ;
 pub type __io_close_fn = :: std :: option :: Option < unsafe extern "C" fn (__cookie : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int > ;
 pub type cookie_read_function_t = __io_read_fn ;
 pub type cookie_write_function_t = __io_write_fn ;
 pub type cookie_seek_function_t = __io_seek_fn ;
 pub type cookie_close_function_t = __io_close_fn ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _IO_cookie_io_functions_t {
pub read : __io_read_fn , pub write : __io_write_fn , pub seek : __io_seek_fn , pub close : __io_close_fn ,
}
 # [test] fn bindgen_test_layout__IO_cookie_io_functions_t () {
assert_eq ! (:: std :: mem :: size_of :: < _IO_cookie_io_functions_t > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( _IO_cookie_io_functions_t ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < _IO_cookie_io_functions_t > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( _IO_cookie_io_functions_t ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_cookie_io_functions_t > ( ) ) ) . read as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( _IO_cookie_io_functions_t ) , "::" , stringify ! ( read ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_cookie_io_functions_t > ( ) ) ) . write as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( _IO_cookie_io_functions_t ) , "::" , stringify ! ( write ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_cookie_io_functions_t > ( ) ) ) . seek as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( _IO_cookie_io_functions_t ) , "::" , stringify ! ( seek ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < _IO_cookie_io_functions_t > ( ) ) ) . close as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( _IO_cookie_io_functions_t ) , "::" , stringify ! ( close ) )) ;

}
 pub type cookie_io_functions_t = _IO_cookie_io_functions_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct _IO_cookie_file {
_unused : [u8 ; 0] ,
}
 extern "C" {
pub fn _IO_cookie_init (__cfile : * mut _IO_cookie_file , __read_write : :: std :: os :: raw :: c_int , __cookie : * mut :: std :: os :: raw :: c_void , __fns : _IO_cookie_io_functions_t) ;

}
 extern "C" {
pub fn __underflow (arg1 : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __uflow (arg1 : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __overflow (arg1 : * mut _IO_FILE , arg2 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_getc (__fp : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_putc (__c : :: std :: os :: raw :: c_int , __fp : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_feof (__fp : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_ferror (__fp : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_peekc_locked (__fp : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_flockfile (arg1 : * mut _IO_FILE) ;

}
 extern "C" {
pub fn _IO_funlockfile (arg1 : * mut _IO_FILE) ;

}
 extern "C" {
pub fn _IO_ftrylockfile (arg1 : * mut _IO_FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_vfscanf (arg1 : * mut _IO_FILE , arg2 : * const :: std :: os :: raw :: c_char , arg3 : * mut __va_list_tag , arg4 : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_vfprintf (arg1 : * mut _IO_FILE , arg2 : * const :: std :: os :: raw :: c_char , arg3 : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _IO_padn (arg1 : * mut _IO_FILE , arg2 : :: std :: os :: raw :: c_int , arg3 : __ssize_t) -> __ssize_t ;

}
 extern "C" {
pub fn _IO_sgetn (arg1 : * mut _IO_FILE , arg2 : * mut :: std :: os :: raw :: c_void , arg3 : usize) -> usize ;

}
 extern "C" {
pub fn _IO_seekoff (arg1 : * mut _IO_FILE , arg2 : __off64_t , arg3 : :: std :: os :: raw :: c_int , arg4 : :: std :: os :: raw :: c_int) -> __off64_t ;

}
 extern "C" {
pub fn _IO_seekpos (arg1 : * mut _IO_FILE , arg2 : __off64_t , arg3 : :: std :: os :: raw :: c_int) -> __off64_t ;

}
 extern "C" {
pub fn _IO_free_backup_area (arg1 : * mut _IO_FILE) ;

}
 pub type fpos_t = _G_fpos_t ;
 pub type fpos64_t = _G_fpos64_t ;
 extern "C" {
pub static mut stdin : * mut _IO_FILE ;

}
 extern "C" {
pub static mut stdout : * mut _IO_FILE ;

}
 extern "C" {
pub static mut stderr : * mut _IO_FILE ;

}
 extern "C" {
pub fn remove (__filename : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rename (__old : * const :: std :: os :: raw :: c_char , __new : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn renameat (__oldfd : :: std :: os :: raw :: c_int , __old : * const :: std :: os :: raw :: c_char , __newfd : :: std :: os :: raw :: c_int , __new : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn tmpfile () -> * mut FILE ;

}
 extern "C" {
pub fn tmpfile64 () -> * mut FILE ;

}
 extern "C" {
pub fn tmpnam (__s : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn tmpnam_r (__s : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn tempnam (__dir : * const :: std :: os :: raw :: c_char , __pfx : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn fclose (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fflush (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fflush_unlocked (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fcloseall () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fopen (__filename : * const :: std :: os :: raw :: c_char , __modes : * const :: std :: os :: raw :: c_char) -> * mut FILE ;

}
 extern "C" {
pub fn freopen (__filename : * const :: std :: os :: raw :: c_char , __modes : * const :: std :: os :: raw :: c_char , __stream : * mut FILE) -> * mut FILE ;

}
 extern "C" {
pub fn fopen64 (__filename : * const :: std :: os :: raw :: c_char , __modes : * const :: std :: os :: raw :: c_char) -> * mut FILE ;

}
 extern "C" {
pub fn freopen64 (__filename : * const :: std :: os :: raw :: c_char , __modes : * const :: std :: os :: raw :: c_char , __stream : * mut FILE) -> * mut FILE ;

}
 extern "C" {
pub fn fdopen (__fd : :: std :: os :: raw :: c_int , __modes : * const :: std :: os :: raw :: c_char) -> * mut FILE ;

}
 extern "C" {
pub fn fopencookie (__magic_cookie : * mut :: std :: os :: raw :: c_void , __modes : * const :: std :: os :: raw :: c_char , __io_funcs : _IO_cookie_io_functions_t) -> * mut FILE ;

}
 extern "C" {
pub fn fmemopen (__s : * mut :: std :: os :: raw :: c_void , __len : usize , __modes : * const :: std :: os :: raw :: c_char) -> * mut FILE ;

}
 extern "C" {
pub fn open_memstream (__bufloc : * mut * mut :: std :: os :: raw :: c_char , __sizeloc : * mut usize) -> * mut FILE ;

}
 extern "C" {
pub fn setbuf (__stream : * mut FILE , __buf : * mut :: std :: os :: raw :: c_char) ;

}
 extern "C" {
pub fn setvbuf (__stream : * mut FILE , __buf : * mut :: std :: os :: raw :: c_char , __modes : :: std :: os :: raw :: c_int , __n : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setbuffer (__stream : * mut FILE , __buf : * mut :: std :: os :: raw :: c_char , __size : usize) ;

}
 extern "C" {
pub fn setlinebuf (__stream : * mut FILE) ;

}
 extern "C" {
pub fn fprintf (__stream : * mut FILE , __format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn printf (__format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sprintf (__s : * mut :: std :: os :: raw :: c_char , __format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vfprintf (__s : * mut FILE , __format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vprintf (__format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vsprintf (__s : * mut :: std :: os :: raw :: c_char , __format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn snprintf (__s : * mut :: std :: os :: raw :: c_char , __maxlen : usize , __format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vsnprintf (__s : * mut :: std :: os :: raw :: c_char , __maxlen : usize , __format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vasprintf (__ptr : * mut * mut :: std :: os :: raw :: c_char , __f : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __asprintf (__ptr : * mut * mut :: std :: os :: raw :: c_char , __fmt : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn asprintf (__ptr : * mut * mut :: std :: os :: raw :: c_char , __fmt : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vdprintf (__fd : :: std :: os :: raw :: c_int , __fmt : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn dprintf (__fd : :: std :: os :: raw :: c_int , __fmt : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fscanf (__stream : * mut FILE , __format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn scanf (__format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn sscanf (__s : * const :: std :: os :: raw :: c_char , __format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vfscanf (__s : * mut FILE , __format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vscanf (__format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn vsscanf (__s : * const :: std :: os :: raw :: c_char , __format : * const :: std :: os :: raw :: c_char , __arg : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fgetc (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getc (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getchar () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getc_unlocked (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getchar_unlocked () -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fgetc_unlocked (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fputc (__c : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn putc (__c : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn putchar (__c : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fputc_unlocked (__c : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn putc_unlocked (__c : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn putchar_unlocked (__c : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getw (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn putw (__w : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fgets (__s : * mut :: std :: os :: raw :: c_char , __n : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn gets (__s : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn fgets_unlocked (__s : * mut :: std :: os :: raw :: c_char , __n : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn __getdelim (__lineptr : * mut * mut :: std :: os :: raw :: c_char , __n : * mut usize , __delimiter : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> __ssize_t ;

}
 extern "C" {
pub fn getdelim (__lineptr : * mut * mut :: std :: os :: raw :: c_char , __n : * mut usize , __delimiter : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> __ssize_t ;

}
 extern "C" {
pub fn getline (__lineptr : * mut * mut :: std :: os :: raw :: c_char , __n : * mut usize , __stream : * mut FILE) -> __ssize_t ;

}
 extern "C" {
pub fn fputs (__s : * const :: std :: os :: raw :: c_char , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn puts (__s : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ungetc (__c : :: std :: os :: raw :: c_int , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fread (__ptr : * mut :: std :: os :: raw :: c_void , __size : usize , __n : usize , __stream : * mut FILE) -> usize ;

}
 extern "C" {
pub fn fwrite (__ptr : * const :: std :: os :: raw :: c_void , __size : usize , __n : usize , __s : * mut FILE) -> usize ;

}
 extern "C" {
pub fn fputs_unlocked (__s : * const :: std :: os :: raw :: c_char , __stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fread_unlocked (__ptr : * mut :: std :: os :: raw :: c_void , __size : usize , __n : usize , __stream : * mut FILE) -> usize ;

}
 extern "C" {
pub fn fwrite_unlocked (__ptr : * const :: std :: os :: raw :: c_void , __size : usize , __n : usize , __stream : * mut FILE) -> usize ;

}
 extern "C" {
pub fn fseek (__stream : * mut FILE , __off : :: std :: os :: raw :: c_long , __whence : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ftell (__stream : * mut FILE) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn rewind (__stream : * mut FILE) ;

}
 extern "C" {
pub fn fseeko (__stream : * mut FILE , __off : __off_t , __whence : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ftello (__stream : * mut FILE) -> __off_t ;

}
 extern "C" {
pub fn fgetpos (__stream : * mut FILE , __pos : * mut fpos_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fsetpos (__stream : * mut FILE , __pos : * const fpos_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fseeko64 (__stream : * mut FILE , __off : __off64_t , __whence : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ftello64 (__stream : * mut FILE) -> __off64_t ;

}
 extern "C" {
pub fn fgetpos64 (__stream : * mut FILE , __pos : * mut fpos64_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fsetpos64 (__stream : * mut FILE , __pos : * const fpos64_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clearerr (__stream : * mut FILE) ;

}
 extern "C" {
pub fn feof (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ferror (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clearerr_unlocked (__stream : * mut FILE) ;

}
 extern "C" {
pub fn feof_unlocked (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ferror_unlocked (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn perror (__s : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
pub static mut sys_nerr : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut sys_errlist : [* const :: std :: os :: raw :: c_char ; 0usize] ;

}
 extern "C" {
pub static mut _sys_nerr : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut _sys_errlist : [* const :: std :: os :: raw :: c_char ; 0usize] ;

}
 extern "C" {
pub fn fileno (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fileno_unlocked (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn popen (__command : * const :: std :: os :: raw :: c_char , __modes : * const :: std :: os :: raw :: c_char) -> * mut FILE ;

}
 extern "C" {
pub fn pclose (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ctermid (__s : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn cuserid (__s : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct obstack {
_unused : [u8 ; 0] ,
}
 extern "C" {
pub fn obstack_printf (__obstack : * mut obstack , __format : * const :: std :: os :: raw :: c_char , ...) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn obstack_vprintf (__obstack : * mut obstack , __format : * const :: std :: os :: raw :: c_char , __args : * mut __va_list_tag) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn flockfile (__stream : * mut FILE) ;

}
 extern "C" {
pub fn ftrylockfile (__stream : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn funlockfile (__stream : * mut FILE) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct dirent {
pub d_ino : __ino_t , pub d_off : __off_t , pub d_reclen : :: std :: os :: raw :: c_ushort , pub d_type : :: std :: os :: raw :: c_uchar , pub d_name : [:: std :: os :: raw :: c_char ; 256usize] ,
}
 # [test] fn bindgen_test_layout_dirent () {
assert_eq ! (:: std :: mem :: size_of :: < dirent > ( ) , 280usize , concat ! ( "Size of: " , stringify ! ( dirent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < dirent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( dirent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent > ( ) ) ) . d_ino as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( dirent ) , "::" , stringify ! ( d_ino ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent > ( ) ) ) . d_off as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( dirent ) , "::" , stringify ! ( d_off ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent > ( ) ) ) . d_reclen as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( dirent ) , "::" , stringify ! ( d_reclen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent > ( ) ) ) . d_type as * const _ as usize } , 18usize , concat ! ( "Offset of field: " , stringify ! ( dirent ) , "::" , stringify ! ( d_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent > ( ) ) ) . d_name as * const _ as usize } , 19usize , concat ! ( "Offset of field: " , stringify ! ( dirent ) , "::" , stringify ! ( d_name ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct dirent64 {
pub d_ino : __ino64_t , pub d_off : __off64_t , pub d_reclen : :: std :: os :: raw :: c_ushort , pub d_type : :: std :: os :: raw :: c_uchar , pub d_name : [:: std :: os :: raw :: c_char ; 256usize] ,
}
 # [test] fn bindgen_test_layout_dirent64 () {
assert_eq ! (:: std :: mem :: size_of :: < dirent64 > ( ) , 280usize , concat ! ( "Size of: " , stringify ! ( dirent64 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < dirent64 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( dirent64 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent64 > ( ) ) ) . d_ino as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( dirent64 ) , "::" , stringify ! ( d_ino ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent64 > ( ) ) ) . d_off as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( dirent64 ) , "::" , stringify ! ( d_off ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent64 > ( ) ) ) . d_reclen as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( dirent64 ) , "::" , stringify ! ( d_reclen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent64 > ( ) ) ) . d_type as * const _ as usize } , 18usize , concat ! ( "Offset of field: " , stringify ! ( dirent64 ) , "::" , stringify ! ( d_type ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < dirent64 > ( ) ) ) . d_name as * const _ as usize } , 19usize , concat ! ( "Offset of field: " , stringify ! ( dirent64 ) , "::" , stringify ! ( d_name ) )) ;

}
 pub const DT_UNKNOWN : _bindgen_ty_23 = 0 ;
 pub const DT_FIFO : _bindgen_ty_23 = 1 ;
 pub const DT_CHR : _bindgen_ty_23 = 2 ;
 pub const DT_DIR : _bindgen_ty_23 = 4 ;
 pub const DT_BLK : _bindgen_ty_23 = 6 ;
 pub const DT_REG : _bindgen_ty_23 = 8 ;
 pub const DT_LNK : _bindgen_ty_23 = 10 ;
 pub const DT_SOCK : _bindgen_ty_23 = 12 ;
 pub const DT_WHT : _bindgen_ty_23 = 14 ;
 pub type _bindgen_ty_23 = u32 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __dirstream {
_unused : [u8 ; 0] ,
}
 pub type DIR = __dirstream ;
 extern "C" {
pub fn opendir (__name : * const :: std :: os :: raw :: c_char) -> * mut DIR ;

}
 extern "C" {
pub fn fdopendir (__fd : :: std :: os :: raw :: c_int) -> * mut DIR ;

}
 extern "C" {
pub fn closedir (__dirp : * mut DIR) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn readdir (__dirp : * mut DIR) -> * mut dirent ;

}
 extern "C" {
pub fn readdir64 (__dirp : * mut DIR) -> * mut dirent64 ;

}
 extern "C" {
pub fn readdir_r (__dirp : * mut DIR , __entry : * mut dirent , __result : * mut * mut dirent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn readdir64_r (__dirp : * mut DIR , __entry : * mut dirent64 , __result : * mut * mut dirent64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rewinddir (__dirp : * mut DIR) ;

}
 extern "C" {
pub fn seekdir (__dirp : * mut DIR , __pos : :: std :: os :: raw :: c_long) ;

}
 extern "C" {
pub fn telldir (__dirp : * mut DIR) -> :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn scandir (__dir : * const :: std :: os :: raw :: c_char , __namelist : * mut * mut * mut dirent , __selector : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * const dirent ) -> :: std :: os :: raw :: c_int > , __cmp : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut * const dirent , arg2 : * mut * const dirent ) -> :: std :: os :: raw :: c_int >) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn scandir64 (__dir : * const :: std :: os :: raw :: c_char , __namelist : * mut * mut * mut dirent64 , __selector : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * const dirent64 ) -> :: std :: os :: raw :: c_int > , __cmp : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut * const dirent64 , arg2 : * mut * const dirent64 ) -> :: std :: os :: raw :: c_int >) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn scandirat (__dfd : :: std :: os :: raw :: c_int , __dir : * const :: std :: os :: raw :: c_char , __namelist : * mut * mut * mut dirent , __selector : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * const dirent ) -> :: std :: os :: raw :: c_int > , __cmp : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut * const dirent , arg2 : * mut * const dirent ) -> :: std :: os :: raw :: c_int >) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn scandirat64 (__dfd : :: std :: os :: raw :: c_int , __dir : * const :: std :: os :: raw :: c_char , __namelist : * mut * mut * mut dirent64 , __selector : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * const dirent64 ) -> :: std :: os :: raw :: c_int > , __cmp : :: std :: option :: Option < unsafe extern "C" fn ( arg1 : * mut * const dirent64 , arg2 : * mut * const dirent64 ) -> :: std :: os :: raw :: c_int >) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn alphasort (__e1 : * mut * const dirent , __e2 : * mut * const dirent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn alphasort64 (__e1 : * mut * const dirent64 , __e2 : * mut * const dirent64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getdirentries (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __nbytes : usize , __basep : * mut __off_t) -> __ssize_t ;

}
 extern "C" {
pub fn getdirentries64 (__fd : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __nbytes : usize , __basep : * mut __off64_t) -> __ssize_t ;

}
 extern "C" {
pub fn versionsort (__e1 : * mut * const dirent , __e2 : * mut * const dirent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn versionsort64 (__e1 : * mut * const dirent64 , __e2 : * mut * const dirent64) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive (  Copy , Clone )] pub struct timex {
pub modes : :: std :: os :: raw :: c_uint , pub offset : __syscall_slong_t , pub freq : __syscall_slong_t , pub maxerror : __syscall_slong_t , pub esterror : __syscall_slong_t , pub status : :: std :: os :: raw :: c_int , pub constant : __syscall_slong_t , pub precision : __syscall_slong_t , pub tolerance : __syscall_slong_t , pub time : timeval , pub tick : __syscall_slong_t , pub ppsfreq : __syscall_slong_t , pub jitter : __syscall_slong_t , pub shift : :: std :: os :: raw :: c_int , pub stabil : __syscall_slong_t , pub jitcnt : __syscall_slong_t , pub calcnt : __syscall_slong_t , pub errcnt : __syscall_slong_t , pub stbcnt : __syscall_slong_t , pub tai : :: std :: os :: raw :: c_int , pub _bitfield_1 : __BindgenBitfieldUnit < [u8 ; 44usize] , u8 > ,
}
 # [test] fn bindgen_test_layout_timex () {
assert_eq ! (:: std :: mem :: size_of :: < timex > ( ) , 208usize , concat ! ( "Size of: " , stringify ! ( timex ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < timex > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( timex ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . modes as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( modes ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . offset as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( offset ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . freq as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( freq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . maxerror as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( maxerror ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . esterror as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( esterror ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . status as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( status ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . constant as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( constant ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . precision as * const _ as usize } , 56usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( precision ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . tolerance as * const _ as usize } , 64usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( tolerance ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . time as * const _ as usize } , 72usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( time ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . tick as * const _ as usize } , 88usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( tick ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . ppsfreq as * const _ as usize } , 96usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( ppsfreq ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . jitter as * const _ as usize } , 104usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( jitter ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . shift as * const _ as usize } , 112usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( shift ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . stabil as * const _ as usize } , 120usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( stabil ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . jitcnt as * const _ as usize } , 128usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( jitcnt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . calcnt as * const _ as usize } , 136usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( calcnt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . errcnt as * const _ as usize } , 144usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( errcnt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . stbcnt as * const _ as usize } , 152usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( stbcnt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < timex > ( ) ) ) . tai as * const _ as usize } , 160usize , concat ! ( "Offset of field: " , stringify ! ( timex ) , "::" , stringify ! ( tai ) )) ;

}

/*
impl timex {
# [inline]
pub fn new_bitfield_1 () -> __BindgenBitfieldUnit < [u8 ; 44usize] , u8 > {
let mut __bindgen_bitfield_unit : __BindgenBitfieldUnit < [u8 ; 44usize] , u8 > = Default :: default () ;
 __bindgen_bitfield_unit
}

}
*/

 extern "C" {
pub fn clock_adjtime (__clock_id : __clockid_t , __utx : * mut timex) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct tm {
pub tm_sec : :: std :: os :: raw :: c_int , pub tm_min : :: std :: os :: raw :: c_int , pub tm_hour : :: std :: os :: raw :: c_int , pub tm_mday : :: std :: os :: raw :: c_int , pub tm_mon : :: std :: os :: raw :: c_int , pub tm_year : :: std :: os :: raw :: c_int , pub tm_wday : :: std :: os :: raw :: c_int , pub tm_yday : :: std :: os :: raw :: c_int , pub tm_isdst : :: std :: os :: raw :: c_int , pub tm_gmtoff : :: std :: os :: raw :: c_long , pub tm_zone : * const :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_tm () {
assert_eq ! (:: std :: mem :: size_of :: < tm > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( tm ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < tm > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( tm ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_sec as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_sec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_min as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_min ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_hour as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_hour ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_mday as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_mday ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_mon as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_mon ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_year as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_year ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_wday as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_wday ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_yday as * const _ as usize } , 28usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_yday ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_isdst as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_isdst ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_gmtoff as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_gmtoff ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < tm > ( ) ) ) . tm_zone as * const _ as usize } , 48usize , concat ! ( "Offset of field: " , stringify ! ( tm ) , "::" , stringify ! ( tm_zone ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct itimerspec {
pub it_interval : timespec , pub it_value : timespec ,
}
 # [test] fn bindgen_test_layout_itimerspec () {
assert_eq ! (:: std :: mem :: size_of :: < itimerspec > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( itimerspec ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < itimerspec > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( itimerspec ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < itimerspec > ( ) ) ) . it_interval as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( itimerspec ) , "::" , stringify ! ( it_interval ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < itimerspec > ( ) ) ) . it_value as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( itimerspec ) , "::" , stringify ! ( it_value ) )) ;

}
 extern "C" {
pub fn clock () -> clock_t ;

}
 extern "C" {
pub fn time (__timer : * mut time_t) -> time_t ;

}
 extern "C" {
pub fn difftime (__time1 : time_t , __time0 : time_t) -> f64 ;

}
 extern "C" {
pub fn mktime (__tp : * mut tm) -> time_t ;

}
 extern "C" {
pub fn strftime (__s : * mut :: std :: os :: raw :: c_char , __maxsize : usize , __format : * const :: std :: os :: raw :: c_char , __tp : * const tm) -> usize ;

}
 extern "C" {
pub fn strptime (__s : * const :: std :: os :: raw :: c_char , __fmt : * const :: std :: os :: raw :: c_char , __tp : * mut tm) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn strftime_l (__s : * mut :: std :: os :: raw :: c_char , __maxsize : usize , __format : * const :: std :: os :: raw :: c_char , __tp : * const tm , __loc : locale_t) -> usize ;

}
 extern "C" {
pub fn strptime_l (__s : * const :: std :: os :: raw :: c_char , __fmt : * const :: std :: os :: raw :: c_char , __tp : * mut tm , __loc : locale_t) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn gmtime (__timer : * const time_t) -> * mut tm ;

}
 extern "C" {
pub fn localtime (__timer : * const time_t) -> * mut tm ;

}
 extern "C" {
pub fn gmtime_r (__timer : * const time_t , __tp : * mut tm) -> * mut tm ;

}
 extern "C" {
pub fn localtime_r (__timer : * const time_t , __tp : * mut tm) -> * mut tm ;

}
 extern "C" {
pub fn asctime (__tp : * const tm) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn ctime (__timer : * const time_t) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn asctime_r (__tp : * const tm , __buf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn ctime_r (__timer : * const time_t , __buf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub static mut __tzname : [* mut :: std :: os :: raw :: c_char ; 2usize] ;

}
 extern "C" {
pub static mut __daylight : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut __timezone : :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub static mut tzname : [* mut :: std :: os :: raw :: c_char ; 2usize] ;

}
 extern "C" {
pub fn tzset () ;

}
 extern "C" {
pub static mut daylight : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut timezone : :: std :: os :: raw :: c_long ;

}
 extern "C" {
pub fn stime (__when : * const time_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timegm (__tp : * mut tm) -> time_t ;

}
 extern "C" {
pub fn timelocal (__tp : * mut tm) -> time_t ;

}
 extern "C" {
pub fn dysize (__year : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn nanosleep (__requested_time : * const timespec , __remaining : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clock_getres (__clock_id : clockid_t , __res : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clock_gettime (__clock_id : clockid_t , __tp : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clock_settime (__clock_id : clockid_t , __tp : * const timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clock_nanosleep (__clock_id : clockid_t , __flags : :: std :: os :: raw :: c_int , __req : * const timespec , __rem : * mut timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn clock_getcpuclockid (__pid : pid_t , __clock_id : * mut clockid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timer_create (__clock_id : clockid_t , __evp : * mut sigevent , __timerid : * mut timer_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timer_delete (__timerid : timer_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timer_settime (__timerid : timer_t , __flags : :: std :: os :: raw :: c_int , __value : * const itimerspec , __ovalue : * mut itimerspec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timer_gettime (__timerid : timer_t , __value : * mut itimerspec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timer_getoverrun (__timerid : timer_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn timespec_get (__ts : * mut timespec , __base : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub static mut getdate_err : :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getdate (__string : * const :: std :: os :: raw :: c_char) -> * mut tm ;

}
 extern "C" {
pub fn getdate_r (__string : * const :: std :: os :: raw :: c_char , __resbufp : * mut tm) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet_addr (__cp : * const :: std :: os :: raw :: c_char) -> in_addr_t ;

}
 extern "C" {
pub fn inet_lnaof (__in : in_addr) -> in_addr_t ;

}
 extern "C" {
pub fn inet_makeaddr (__net : in_addr_t , __host : in_addr_t) -> in_addr ;

}
 extern "C" {
pub fn inet_netof (__in : in_addr) -> in_addr_t ;

}
 extern "C" {
pub fn inet_network (__cp : * const :: std :: os :: raw :: c_char) -> in_addr_t ;

}
 extern "C" {
pub fn inet_ntoa (__in : in_addr) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn inet_pton (__af : :: std :: os :: raw :: c_int , __cp : * const :: std :: os :: raw :: c_char , __buf : * mut :: std :: os :: raw :: c_void) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet_ntop (__af : :: std :: os :: raw :: c_int , __cp : * const :: std :: os :: raw :: c_void , __buf : * mut :: std :: os :: raw :: c_char , __len : socklen_t) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn inet_aton (__cp : * const :: std :: os :: raw :: c_char , __inp : * mut in_addr) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet_neta (__net : in_addr_t , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn inet_net_ntop (__af : :: std :: os :: raw :: c_int , __cp : * const :: std :: os :: raw :: c_void , __bits : :: std :: os :: raw :: c_int , __buf : * mut :: std :: os :: raw :: c_char , __len : usize) -> * mut :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn inet_net_pton (__af : :: std :: os :: raw :: c_int , __cp : * const :: std :: os :: raw :: c_char , __buf : * mut :: std :: os :: raw :: c_void , __len : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn inet_nsap_addr (__cp : * const :: std :: os :: raw :: c_char , __buf : * mut :: std :: os :: raw :: c_uchar , __len : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_uint ;

}
 extern "C" {
pub fn inet_nsap_ntoa (__len : :: std :: os :: raw :: c_int , __cp : * const :: std :: os :: raw :: c_uchar , __buf : * mut :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct passwd {
pub pw_name : * mut :: std :: os :: raw :: c_char , pub pw_passwd : * mut :: std :: os :: raw :: c_char , pub pw_uid : __uid_t , pub pw_gid : __gid_t , pub pw_gecos : * mut :: std :: os :: raw :: c_char , pub pw_dir : * mut :: std :: os :: raw :: c_char , pub pw_shell : * mut :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_passwd () {
assert_eq ! (:: std :: mem :: size_of :: < passwd > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( passwd ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < passwd > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( passwd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_passwd as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_passwd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_uid as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_uid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_gid as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_gid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_gecos as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_gecos ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_dir as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_dir ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < passwd > ( ) ) ) . pw_shell as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( passwd ) , "::" , stringify ! ( pw_shell ) )) ;

}
 extern "C" {
pub fn setpwent () ;

}
 extern "C" {
pub fn endpwent () ;

}
 extern "C" {
pub fn getpwent () -> * mut passwd ;

}
 extern "C" {
pub fn fgetpwent (__stream : * mut FILE) -> * mut passwd ;

}
 extern "C" {
pub fn putpwent (__p : * const passwd , __f : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpwuid (__uid : __uid_t) -> * mut passwd ;

}
 extern "C" {
pub fn getpwnam (__name : * const :: std :: os :: raw :: c_char) -> * mut passwd ;

}
 extern "C" {
pub fn getpwent_r (__resultbuf : * mut passwd , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut passwd) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpwuid_r (__uid : __uid_t , __resultbuf : * mut passwd , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut passwd) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpwnam_r (__name : * const :: std :: os :: raw :: c_char , __resultbuf : * mut passwd , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut passwd) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fgetpwent_r (__stream : * mut FILE , __resultbuf : * mut passwd , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut passwd) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpw (__uid : __uid_t , __buffer : * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct group {
pub gr_name : * mut :: std :: os :: raw :: c_char , pub gr_passwd : * mut :: std :: os :: raw :: c_char , pub gr_gid : __gid_t , pub gr_mem : * mut * mut :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_group () {
assert_eq ! (:: std :: mem :: size_of :: < group > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( group ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < group > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( group ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group > ( ) ) ) . gr_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( group ) , "::" , stringify ! ( gr_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group > ( ) ) ) . gr_passwd as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( group ) , "::" , stringify ! ( gr_passwd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group > ( ) ) ) . gr_gid as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( group ) , "::" , stringify ! ( gr_gid ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < group > ( ) ) ) . gr_mem as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( group ) , "::" , stringify ! ( gr_mem ) )) ;

}
 extern "C" {
pub fn setgrent () ;

}
 extern "C" {
pub fn endgrent () ;

}
 extern "C" {
pub fn getgrent () -> * mut group ;

}
 extern "C" {
pub fn fgetgrent (__stream : * mut FILE) -> * mut group ;

}
 extern "C" {
pub fn putgrent (__p : * const group , __f : * mut FILE) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getgrgid (__gid : __gid_t) -> * mut group ;

}
 extern "C" {
pub fn getgrnam (__name : * const :: std :: os :: raw :: c_char) -> * mut group ;

}
 extern "C" {
pub fn getgrent_r (__resultbuf : * mut group , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut group) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getgrgid_r (__gid : __gid_t , __resultbuf : * mut group , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut group) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getgrnam_r (__name : * const :: std :: os :: raw :: c_char , __resultbuf : * mut group , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut group) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn fgetgrent_r (__stream : * mut FILE , __resultbuf : * mut group , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut group) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setgroups (__n : usize , __groups : * const __gid_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getgrouplist (__user : * const :: std :: os :: raw :: c_char , __group : __gid_t , __groups : * mut __gid_t , __ngroups : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn initgroups (__user : * const :: std :: os :: raw :: c_char , __group : __gid_t) -> :: std :: os :: raw :: c_int ;

}
 pub const _ISupper : _bindgen_ty_24 = 256 ;
 pub const _ISlower : _bindgen_ty_24 = 512 ;
 pub const _ISalpha : _bindgen_ty_24 = 1024 ;
 pub const _ISdigit : _bindgen_ty_24 = 2048 ;
 pub const _ISxdigit : _bindgen_ty_24 = 4096 ;
 pub const _ISspace : _bindgen_ty_24 = 8192 ;
 pub const _ISprint : _bindgen_ty_24 = 16384 ;
 pub const _ISgraph : _bindgen_ty_24 = 32768 ;
 pub const _ISblank : _bindgen_ty_24 = 1 ;
 pub const _IScntrl : _bindgen_ty_24 = 2 ;
 pub const _ISpunct : _bindgen_ty_24 = 4 ;
 pub const _ISalnum : _bindgen_ty_24 = 8 ;
 pub type _bindgen_ty_24 = u32 ;
 extern "C" {
pub fn __ctype_b_loc () -> * mut * const :: std :: os :: raw :: c_ushort ;

}
 extern "C" {
pub fn __ctype_tolower_loc () -> * mut * const __int32_t ;

}
 extern "C" {
pub fn __ctype_toupper_loc () -> * mut * const __int32_t ;

}
 extern "C" {
pub fn isalnum (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isalpha (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn iscntrl (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isdigit (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn islower (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isgraph (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isprint (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ispunct (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isspace (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isupper (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isxdigit (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn tolower (__c : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn toupper (__c : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isblank (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isctype (__c : :: std :: os :: raw :: c_int , __mask : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isascii (__c : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn toascii (__c : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _toupper (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn _tolower (arg1 : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isalnum_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isalpha_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn iscntrl_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isdigit_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn islower_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isgraph_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isprint_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ispunct_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isspace_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isupper_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isxdigit_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn isblank_l (arg1 : :: std :: os :: raw :: c_int , arg2 : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __tolower_l (__c : :: std :: os :: raw :: c_int , __l : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn tolower_l (__c : :: std :: os :: raw :: c_int , __l : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn __toupper_l (__c : :: std :: os :: raw :: c_int , __l : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn toupper_l (__c : :: std :: os :: raw :: c_int , __l : locale_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn wait (__stat_loc : * mut :: std :: os :: raw :: c_int) -> __pid_t ;

}
 extern "C" {
pub fn waitpid (__pid : __pid_t , __stat_loc : * mut :: std :: os :: raw :: c_int , __options : :: std :: os :: raw :: c_int) -> __pid_t ;

}
 extern "C" {
pub fn waitid (__idtype : idtype_t , __id : __id_t , __infop : * mut siginfo_t , __options : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn wait3 (__stat_loc : * mut :: std :: os :: raw :: c_int , __options : :: std :: os :: raw :: c_int , __usage : * mut rusage) -> __pid_t ;

}
 extern "C" {
pub fn wait4 (__pid : __pid_t , __stat_loc : * mut :: std :: os :: raw :: c_int , __options : :: std :: os :: raw :: c_int , __usage : * mut rusage) -> __pid_t ;

}
 extern "C" {
pub fn closelog () ;

}
 extern "C" {
pub fn openlog (__ident : * const :: std :: os :: raw :: c_char , __option : :: std :: os :: raw :: c_int , __facility : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn setlogmask (__mask : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn syslog (__pri : :: std :: os :: raw :: c_int , __fmt : * const :: std :: os :: raw :: c_char , ...) ;

}
 extern "C" {
pub fn vsyslog (__pri : :: std :: os :: raw :: c_int , __fmt : * const :: std :: os :: raw :: c_char , __ap : * mut __va_list_tag) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct utimbuf {
pub actime : __time_t , pub modtime : __time_t ,
}
 # [test] fn bindgen_test_layout_utimbuf () {
assert_eq ! (:: std :: mem :: size_of :: < utimbuf > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( utimbuf ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < utimbuf > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( utimbuf ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utimbuf > ( ) ) ) . actime as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( utimbuf ) , "::" , stringify ! ( actime ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < utimbuf > ( ) ) ) . modtime as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( utimbuf ) , "::" , stringify ! ( modtime ) )) ;

}
 extern "C" {
pub fn utime (__file : * const :: std :: os :: raw :: c_char , __file_times : * const utimbuf) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct rpcent {
pub r_name : * mut :: std :: os :: raw :: c_char , pub r_aliases : * mut * mut :: std :: os :: raw :: c_char , pub r_number : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_rpcent () {
assert_eq ! (:: std :: mem :: size_of :: < rpcent > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( rpcent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rpcent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rpcent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rpcent > ( ) ) ) . r_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rpcent ) , "::" , stringify ! ( r_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rpcent > ( ) ) ) . r_aliases as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( rpcent ) , "::" , stringify ! ( r_aliases ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rpcent > ( ) ) ) . r_number as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( rpcent ) , "::" , stringify ! ( r_number ) )) ;

}
 extern "C" {
pub fn setrpcent (__stayopen : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn endrpcent () ;

}
 extern "C" {
pub fn getrpcbyname (__name : * const :: std :: os :: raw :: c_char) -> * mut rpcent ;

}
 extern "C" {
pub fn getrpcbynumber (__number : :: std :: os :: raw :: c_int) -> * mut rpcent ;

}
 extern "C" {
pub fn getrpcent () -> * mut rpcent ;

}
 extern "C" {
pub fn getrpcbyname_r (__name : * const :: std :: os :: raw :: c_char , __result_buf : * mut rpcent , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut rpcent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getrpcbynumber_r (__number : :: std :: os :: raw :: c_int , __result_buf : * mut rpcent , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut rpcent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getrpcent_r (__result_buf : * mut rpcent , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut rpcent) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct netent {
pub n_name : * mut :: std :: os :: raw :: c_char , pub n_aliases : * mut * mut :: std :: os :: raw :: c_char , pub n_addrtype : :: std :: os :: raw :: c_int , pub n_net : u32 ,
}
 # [test] fn bindgen_test_layout_netent () {
assert_eq ! (:: std :: mem :: size_of :: < netent > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( netent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < netent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( netent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < netent > ( ) ) ) . n_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( netent ) , "::" , stringify ! ( n_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < netent > ( ) ) ) . n_aliases as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( netent ) , "::" , stringify ! ( n_aliases ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < netent > ( ) ) ) . n_addrtype as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( netent ) , "::" , stringify ! ( n_addrtype ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < netent > ( ) ) ) . n_net as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( netent ) , "::" , stringify ! ( n_net ) )) ;

}
 extern "C" {
pub fn __h_errno_location () -> * mut :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn herror (__str : * const :: std :: os :: raw :: c_char) ;

}
 extern "C" {
pub fn hstrerror (__err_num : :: std :: os :: raw :: c_int) -> * const :: std :: os :: raw :: c_char ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct hostent {
pub h_name : * mut :: std :: os :: raw :: c_char , pub h_aliases : * mut * mut :: std :: os :: raw :: c_char , pub h_addrtype : :: std :: os :: raw :: c_int , pub h_length : :: std :: os :: raw :: c_int , pub h_addr_list : * mut * mut :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_hostent () {
assert_eq ! (:: std :: mem :: size_of :: < hostent > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( hostent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < hostent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( hostent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hostent > ( ) ) ) . h_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( hostent ) , "::" , stringify ! ( h_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hostent > ( ) ) ) . h_aliases as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( hostent ) , "::" , stringify ! ( h_aliases ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hostent > ( ) ) ) . h_addrtype as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( hostent ) , "::" , stringify ! ( h_addrtype ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hostent > ( ) ) ) . h_length as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( hostent ) , "::" , stringify ! ( h_length ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < hostent > ( ) ) ) . h_addr_list as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( hostent ) , "::" , stringify ! ( h_addr_list ) )) ;

}
 extern "C" {
pub fn sethostent (__stay_open : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn endhostent () ;

}
 extern "C" {
pub fn gethostent () -> * mut hostent ;

}
 extern "C" {
pub fn gethostbyaddr (__addr : * const :: std :: os :: raw :: c_void , __len : __socklen_t , __type : :: std :: os :: raw :: c_int) -> * mut hostent ;

}
 extern "C" {
pub fn gethostbyname (__name : * const :: std :: os :: raw :: c_char) -> * mut hostent ;

}
 extern "C" {
pub fn gethostbyname2 (__name : * const :: std :: os :: raw :: c_char , __af : :: std :: os :: raw :: c_int) -> * mut hostent ;

}
 extern "C" {
pub fn gethostent_r (__result_buf : * mut hostent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut hostent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gethostbyaddr_r (__addr : * const :: std :: os :: raw :: c_void , __len : __socklen_t , __type : :: std :: os :: raw :: c_int , __result_buf : * mut hostent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut hostent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gethostbyname_r (__name : * const :: std :: os :: raw :: c_char , __result_buf : * mut hostent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut hostent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gethostbyname2_r (__name : * const :: std :: os :: raw :: c_char , __af : :: std :: os :: raw :: c_int , __result_buf : * mut hostent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut hostent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setnetent (__stay_open : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn endnetent () ;

}
 extern "C" {
pub fn getnetent () -> * mut netent ;

}
 extern "C" {
pub fn getnetbyaddr (__net : u32 , __type : :: std :: os :: raw :: c_int) -> * mut netent ;

}
 extern "C" {
pub fn getnetbyname (__name : * const :: std :: os :: raw :: c_char) -> * mut netent ;

}
 extern "C" {
pub fn getnetent_r (__result_buf : * mut netent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut netent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getnetbyaddr_r (__net : u32 , __type : :: std :: os :: raw :: c_int , __result_buf : * mut netent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut netent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getnetbyname_r (__name : * const :: std :: os :: raw :: c_char , __result_buf : * mut netent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut netent , __h_errnop : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct servent {
pub s_name : * mut :: std :: os :: raw :: c_char , pub s_aliases : * mut * mut :: std :: os :: raw :: c_char , pub s_port : :: std :: os :: raw :: c_int , pub s_proto : * mut :: std :: os :: raw :: c_char ,
}
 # [test] fn bindgen_test_layout_servent () {
assert_eq ! (:: std :: mem :: size_of :: < servent > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( servent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < servent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( servent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < servent > ( ) ) ) . s_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( servent ) , "::" , stringify ! ( s_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < servent > ( ) ) ) . s_aliases as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( servent ) , "::" , stringify ! ( s_aliases ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < servent > ( ) ) ) . s_port as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( servent ) , "::" , stringify ! ( s_port ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < servent > ( ) ) ) . s_proto as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( servent ) , "::" , stringify ! ( s_proto ) )) ;

}
 extern "C" {
pub fn setservent (__stay_open : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn endservent () ;

}
 extern "C" {
pub fn getservent () -> * mut servent ;

}
 extern "C" {
pub fn getservbyname (__name : * const :: std :: os :: raw :: c_char , __proto : * const :: std :: os :: raw :: c_char) -> * mut servent ;

}
 extern "C" {
pub fn getservbyport (__port : :: std :: os :: raw :: c_int , __proto : * const :: std :: os :: raw :: c_char) -> * mut servent ;

}
 extern "C" {
pub fn getservent_r (__result_buf : * mut servent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut servent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getservbyname_r (__name : * const :: std :: os :: raw :: c_char , __proto : * const :: std :: os :: raw :: c_char , __result_buf : * mut servent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut servent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getservbyport_r (__port : :: std :: os :: raw :: c_int , __proto : * const :: std :: os :: raw :: c_char , __result_buf : * mut servent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut servent) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct protoent {
pub p_name : * mut :: std :: os :: raw :: c_char , pub p_aliases : * mut * mut :: std :: os :: raw :: c_char , pub p_proto : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_protoent () {
assert_eq ! (:: std :: mem :: size_of :: < protoent > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( protoent ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < protoent > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( protoent ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < protoent > ( ) ) ) . p_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( protoent ) , "::" , stringify ! ( p_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < protoent > ( ) ) ) . p_aliases as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( protoent ) , "::" , stringify ! ( p_aliases ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < protoent > ( ) ) ) . p_proto as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( protoent ) , "::" , stringify ! ( p_proto ) )) ;

}
 extern "C" {
pub fn setprotoent (__stay_open : :: std :: os :: raw :: c_int) ;

}
 extern "C" {
pub fn endprotoent () ;

}
 extern "C" {
pub fn getprotoent () -> * mut protoent ;

}
 extern "C" {
pub fn getprotobyname (__name : * const :: std :: os :: raw :: c_char) -> * mut protoent ;

}
 extern "C" {
pub fn getprotobynumber (__proto : :: std :: os :: raw :: c_int) -> * mut protoent ;

}
 extern "C" {
pub fn getprotoent_r (__result_buf : * mut protoent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut protoent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getprotobyname_r (__name : * const :: std :: os :: raw :: c_char , __result_buf : * mut protoent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut protoent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getprotobynumber_r (__proto : :: std :: os :: raw :: c_int , __result_buf : * mut protoent , __buf : * mut :: std :: os :: raw :: c_char , __buflen : usize , __result : * mut * mut protoent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setnetgrent (__netgroup : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn endnetgrent () ;

}
 extern "C" {
pub fn getnetgrent (__hostp : * mut * mut :: std :: os :: raw :: c_char , __userp : * mut * mut :: std :: os :: raw :: c_char , __domainp : * mut * mut :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn innetgr (__netgroup : * const :: std :: os :: raw :: c_char , __host : * const :: std :: os :: raw :: c_char , __user : * const :: std :: os :: raw :: c_char , __domain : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getnetgrent_r (__hostp : * mut * mut :: std :: os :: raw :: c_char , __userp : * mut * mut :: std :: os :: raw :: c_char , __domainp : * mut * mut :: std :: os :: raw :: c_char , __buffer : * mut :: std :: os :: raw :: c_char , __buflen : usize) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rcmd (__ahost : * mut * mut :: std :: os :: raw :: c_char , __rport : :: std :: os :: raw :: c_ushort , __locuser : * const :: std :: os :: raw :: c_char , __remuser : * const :: std :: os :: raw :: c_char , __cmd : * const :: std :: os :: raw :: c_char , __fd2p : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rcmd_af (__ahost : * mut * mut :: std :: os :: raw :: c_char , __rport : :: std :: os :: raw :: c_ushort , __locuser : * const :: std :: os :: raw :: c_char , __remuser : * const :: std :: os :: raw :: c_char , __cmd : * const :: std :: os :: raw :: c_char , __fd2p : * mut :: std :: os :: raw :: c_int , __af : sa_family_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rexec (__ahost : * mut * mut :: std :: os :: raw :: c_char , __rport : :: std :: os :: raw :: c_int , __name : * const :: std :: os :: raw :: c_char , __pass : * const :: std :: os :: raw :: c_char , __cmd : * const :: std :: os :: raw :: c_char , __fd2p : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rexec_af (__ahost : * mut * mut :: std :: os :: raw :: c_char , __rport : :: std :: os :: raw :: c_int , __name : * const :: std :: os :: raw :: c_char , __pass : * const :: std :: os :: raw :: c_char , __cmd : * const :: std :: os :: raw :: c_char , __fd2p : * mut :: std :: os :: raw :: c_int , __af : sa_family_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ruserok (__rhost : * const :: std :: os :: raw :: c_char , __suser : :: std :: os :: raw :: c_int , __remuser : * const :: std :: os :: raw :: c_char , __locuser : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn ruserok_af (__rhost : * const :: std :: os :: raw :: c_char , __suser : :: std :: os :: raw :: c_int , __remuser : * const :: std :: os :: raw :: c_char , __locuser : * const :: std :: os :: raw :: c_char , __af : sa_family_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn iruserok (__raddr : u32 , __suser : :: std :: os :: raw :: c_int , __remuser : * const :: std :: os :: raw :: c_char , __locuser : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn iruserok_af (__raddr : * const :: std :: os :: raw :: c_void , __suser : :: std :: os :: raw :: c_int , __remuser : * const :: std :: os :: raw :: c_char , __locuser : * const :: std :: os :: raw :: c_char , __af : sa_family_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rresvport (__alport : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn rresvport_af (__alport : * mut :: std :: os :: raw :: c_int , __af : sa_family_t) -> :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct addrinfo {
pub ai_flags : :: std :: os :: raw :: c_int , pub ai_family : :: std :: os :: raw :: c_int , pub ai_socktype : :: std :: os :: raw :: c_int , pub ai_protocol : :: std :: os :: raw :: c_int , pub ai_addrlen : socklen_t , pub ai_addr : * mut sockaddr , pub ai_canonname : * mut :: std :: os :: raw :: c_char , pub ai_next : * mut addrinfo ,
}
 # [test] fn bindgen_test_layout_addrinfo () {
assert_eq ! (:: std :: mem :: size_of :: < addrinfo > ( ) , 48usize , concat ! ( "Size of: " , stringify ! ( addrinfo ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < addrinfo > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( addrinfo ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_flags as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_flags ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_family as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_family ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_socktype as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_socktype ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_protocol as * const _ as usize } , 12usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_protocol ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_addrlen as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_addrlen ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_addr as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_addr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_canonname as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_canonname ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < addrinfo > ( ) ) ) . ai_next as * const _ as usize } , 40usize , concat ! ( "Offset of field: " , stringify ! ( addrinfo ) , "::" , stringify ! ( ai_next ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct gaicb {
pub ar_name : * const :: std :: os :: raw :: c_char , pub ar_service : * const :: std :: os :: raw :: c_char , pub ar_request : * const addrinfo , pub ar_result : * mut addrinfo , pub __return : :: std :: os :: raw :: c_int , pub __glibc_reserved : [:: std :: os :: raw :: c_int ; 5usize] ,
}
 # [test] fn bindgen_test_layout_gaicb () {
assert_eq ! (:: std :: mem :: size_of :: < gaicb > ( ) , 56usize , concat ! ( "Size of: " , stringify ! ( gaicb ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < gaicb > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( gaicb ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < gaicb > ( ) ) ) . ar_name as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( gaicb ) , "::" , stringify ! ( ar_name ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < gaicb > ( ) ) ) . ar_service as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( gaicb ) , "::" , stringify ! ( ar_service ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < gaicb > ( ) ) ) . ar_request as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( gaicb ) , "::" , stringify ! ( ar_request ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < gaicb > ( ) ) ) . ar_result as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( gaicb ) , "::" , stringify ! ( ar_result ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < gaicb > ( ) ) ) . __return as * const _ as usize } , 32usize , concat ! ( "Offset of field: " , stringify ! ( gaicb ) , "::" , stringify ! ( __return ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < gaicb > ( ) ) ) . __glibc_reserved as * const _ as usize } , 36usize , concat ! ( "Offset of field: " , stringify ! ( gaicb ) , "::" , stringify ! ( __glibc_reserved ) )) ;

}
 extern "C" {
pub fn getaddrinfo (__name : * const :: std :: os :: raw :: c_char , __service : * const :: std :: os :: raw :: c_char , __req : * const addrinfo , __pai : * mut * mut addrinfo) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn freeaddrinfo (__ai : * mut addrinfo) ;

}
 extern "C" {
pub fn gai_strerror (__ecode : :: std :: os :: raw :: c_int) -> * const :: std :: os :: raw :: c_char ;

}
 extern "C" {
pub fn getnameinfo (__sa : * const sockaddr , __salen : socklen_t , __host : * mut :: std :: os :: raw :: c_char , __hostlen : socklen_t , __serv : * mut :: std :: os :: raw :: c_char , __servlen : socklen_t , __flags : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getaddrinfo_a (__mode : :: std :: os :: raw :: c_int , __list : * mut * mut gaicb , __ent : :: std :: os :: raw :: c_int , __sig : * mut sigevent) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gai_suspend (__list : * const * const gaicb , __ent : :: std :: os :: raw :: c_int , __timeout : * const timespec) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gai_error (__req : * mut gaicb) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn gai_cancel (__gaicbp : * mut gaicb) -> :: std :: os :: raw :: c_int ;

}
 pub const __rlimit_resource_RLIMIT_CPU : __rlimit_resource = 0 ;
 pub const __rlimit_resource_RLIMIT_FSIZE : __rlimit_resource = 1 ;
 pub const __rlimit_resource_RLIMIT_DATA : __rlimit_resource = 2 ;
 pub const __rlimit_resource_RLIMIT_STACK : __rlimit_resource = 3 ;
 pub const __rlimit_resource_RLIMIT_CORE : __rlimit_resource = 4 ;
 pub const __rlimit_resource___RLIMIT_RSS : __rlimit_resource = 5 ;
 pub const __rlimit_resource_RLIMIT_NOFILE : __rlimit_resource = 7 ;
 pub const __rlimit_resource___RLIMIT_OFILE : __rlimit_resource = 7 ;
 pub const __rlimit_resource_RLIMIT_AS : __rlimit_resource = 9 ;
 pub const __rlimit_resource___RLIMIT_NPROC : __rlimit_resource = 6 ;
 pub const __rlimit_resource___RLIMIT_MEMLOCK : __rlimit_resource = 8 ;
 pub const __rlimit_resource___RLIMIT_LOCKS : __rlimit_resource = 10 ;
 pub const __rlimit_resource___RLIMIT_SIGPENDING : __rlimit_resource = 11 ;
 pub const __rlimit_resource___RLIMIT_MSGQUEUE : __rlimit_resource = 12 ;
 pub const __rlimit_resource___RLIMIT_NICE : __rlimit_resource = 13 ;
 pub const __rlimit_resource___RLIMIT_RTPRIO : __rlimit_resource = 14 ;
 pub const __rlimit_resource___RLIMIT_RTTIME : __rlimit_resource = 15 ;
 pub const __rlimit_resource___RLIMIT_NLIMITS : __rlimit_resource = 16 ;
 pub const __rlimit_resource___RLIM_NLIMITS : __rlimit_resource = 16 ;
 pub type __rlimit_resource = u32 ;
 pub type rlim_t = __rlim_t ;
 pub type rlim64_t = __rlim64_t ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct rlimit {
pub rlim_cur : rlim_t , pub rlim_max : rlim_t ,
}
 # [test] fn bindgen_test_layout_rlimit () {
assert_eq ! (:: std :: mem :: size_of :: < rlimit > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( rlimit ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rlimit > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rlimit ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rlimit > ( ) ) ) . rlim_cur as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rlimit ) , "::" , stringify ! ( rlim_cur ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rlimit > ( ) ) ) . rlim_max as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( rlimit ) , "::" , stringify ! ( rlim_max ) )) ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct rlimit64 {
pub rlim_cur : rlim64_t , pub rlim_max : rlim64_t ,
}
 # [test] fn bindgen_test_layout_rlimit64 () {
assert_eq ! (:: std :: mem :: size_of :: < rlimit64 > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( rlimit64 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rlimit64 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rlimit64 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rlimit64 > ( ) ) ) . rlim_cur as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rlimit64 ) , "::" , stringify ! ( rlim_cur ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rlimit64 > ( ) ) ) . rlim_max as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( rlimit64 ) , "::" , stringify ! ( rlim_max ) )) ;

}
 pub const __rusage_who_RUSAGE_SELF : __rusage_who = 0 ;
 pub const __rusage_who_RUSAGE_CHILDREN : __rusage_who = - 1 ;
 pub const __rusage_who_RUSAGE_THREAD : __rusage_who = 1 ;
 pub type __rusage_who = i32 ;
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct rusage {
pub ru_utime : timeval , pub ru_stime : timeval , pub __bindgen_anon_1 : rusage__bindgen_ty_1 , pub __bindgen_anon_2 : rusage__bindgen_ty_2 , pub __bindgen_anon_3 : rusage__bindgen_ty_3 , pub __bindgen_anon_4 : rusage__bindgen_ty_4 , pub __bindgen_anon_5 : rusage__bindgen_ty_5 , pub __bindgen_anon_6 : rusage__bindgen_ty_6 , pub __bindgen_anon_7 : rusage__bindgen_ty_7 , pub __bindgen_anon_8 : rusage__bindgen_ty_8 , pub __bindgen_anon_9 : rusage__bindgen_ty_9 , pub __bindgen_anon_10 : rusage__bindgen_ty_10 , pub __bindgen_anon_11 : rusage__bindgen_ty_11 , pub __bindgen_anon_12 : rusage__bindgen_ty_12 , pub __bindgen_anon_13 : rusage__bindgen_ty_13 , pub __bindgen_anon_14 : rusage__bindgen_ty_14 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_1 {
pub ru_maxrss : :: std :: os :: raw :: c_long , pub __ru_maxrss_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_1 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_1 > ( ) ) ) . ru_maxrss as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_1 ) , "::" , stringify ! ( ru_maxrss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_1 > ( ) ) ) . __ru_maxrss_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_1 ) , "::" , stringify ! ( __ru_maxrss_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_2 {
pub ru_ixrss : :: std :: os :: raw :: c_long , pub __ru_ixrss_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_2 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_2 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_2 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_2 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_2 > ( ) ) ) . ru_ixrss as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_2 ) , "::" , stringify ! ( ru_ixrss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_2 > ( ) ) ) . __ru_ixrss_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_2 ) , "::" , stringify ! ( __ru_ixrss_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_3 {
pub ru_idrss : :: std :: os :: raw :: c_long , pub __ru_idrss_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_3 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_3 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_3 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_3 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_3 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_3 > ( ) ) ) . ru_idrss as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_3 ) , "::" , stringify ! ( ru_idrss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_3 > ( ) ) ) . __ru_idrss_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_3 ) , "::" , stringify ! ( __ru_idrss_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_4 {
pub ru_isrss : :: std :: os :: raw :: c_long , pub __ru_isrss_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_4 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_4 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_4 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_4 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_4 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_4 > ( ) ) ) . ru_isrss as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_4 ) , "::" , stringify ! ( ru_isrss ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_4 > ( ) ) ) . __ru_isrss_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_4 ) , "::" , stringify ! ( __ru_isrss_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_5 {
pub ru_minflt : :: std :: os :: raw :: c_long , pub __ru_minflt_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_5 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_5 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_5 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_5 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_5 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_5 > ( ) ) ) . ru_minflt as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_5 ) , "::" , stringify ! ( ru_minflt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_5 > ( ) ) ) . __ru_minflt_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_5 ) , "::" , stringify ! ( __ru_minflt_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_6 {
pub ru_majflt : :: std :: os :: raw :: c_long , pub __ru_majflt_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_6 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_6 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_6 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_6 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_6 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_6 > ( ) ) ) . ru_majflt as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_6 ) , "::" , stringify ! ( ru_majflt ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_6 > ( ) ) ) . __ru_majflt_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_6 ) , "::" , stringify ! ( __ru_majflt_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_7 {
pub ru_nswap : :: std :: os :: raw :: c_long , pub __ru_nswap_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_7 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_7 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_7 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_7 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_7 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_7 > ( ) ) ) . ru_nswap as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_7 ) , "::" , stringify ! ( ru_nswap ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_7 > ( ) ) ) . __ru_nswap_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_7 ) , "::" , stringify ! ( __ru_nswap_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_8 {
pub ru_inblock : :: std :: os :: raw :: c_long , pub __ru_inblock_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_8 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_8 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_8 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_8 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_8 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_8 > ( ) ) ) . ru_inblock as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_8 ) , "::" , stringify ! ( ru_inblock ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_8 > ( ) ) ) . __ru_inblock_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_8 ) , "::" , stringify ! ( __ru_inblock_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_9 {
pub ru_oublock : :: std :: os :: raw :: c_long , pub __ru_oublock_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_9 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_9 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_9 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_9 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_9 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_9 > ( ) ) ) . ru_oublock as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_9 ) , "::" , stringify ! ( ru_oublock ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_9 > ( ) ) ) . __ru_oublock_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_9 ) , "::" , stringify ! ( __ru_oublock_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_10 {
pub ru_msgsnd : :: std :: os :: raw :: c_long , pub __ru_msgsnd_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_10 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_10 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_10 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_10 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_10 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_10 > ( ) ) ) . ru_msgsnd as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_10 ) , "::" , stringify ! ( ru_msgsnd ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_10 > ( ) ) ) . __ru_msgsnd_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_10 ) , "::" , stringify ! ( __ru_msgsnd_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_11 {
pub ru_msgrcv : :: std :: os :: raw :: c_long , pub __ru_msgrcv_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_11 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_11 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_11 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_11 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_11 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_11 > ( ) ) ) . ru_msgrcv as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_11 ) , "::" , stringify ! ( ru_msgrcv ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_11 > ( ) ) ) . __ru_msgrcv_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_11 ) , "::" , stringify ! ( __ru_msgrcv_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_12 {
pub ru_nsignals : :: std :: os :: raw :: c_long , pub __ru_nsignals_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_12 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_12 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_12 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_12 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_12 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_12 > ( ) ) ) . ru_nsignals as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_12 ) , "::" , stringify ! ( ru_nsignals ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_12 > ( ) ) ) . __ru_nsignals_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_12 ) , "::" , stringify ! ( __ru_nsignals_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_13 {
pub ru_nvcsw : :: std :: os :: raw :: c_long , pub __ru_nvcsw_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_13 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_13 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_13 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_13 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_13 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_13 > ( ) ) ) . ru_nvcsw as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_13 ) , "::" , stringify ! ( ru_nvcsw ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_13 > ( ) ) ) . __ru_nvcsw_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_13 ) , "::" , stringify ! ( __ru_nvcsw_word ) )) ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union rusage__bindgen_ty_14 {
pub ru_nivcsw : :: std :: os :: raw :: c_long , pub __ru_nivcsw_word : __syscall_slong_t , _bindgen_union_align : u64 ,
}
 # [test] fn bindgen_test_layout_rusage__bindgen_ty_14 () {
assert_eq ! (:: std :: mem :: size_of :: < rusage__bindgen_ty_14 > ( ) , 8usize , concat ! ( "Size of: " , stringify ! ( rusage__bindgen_ty_14 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage__bindgen_ty_14 > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage__bindgen_ty_14 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_14 > ( ) ) ) . ru_nivcsw as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_14 ) , "::" , stringify ! ( ru_nivcsw ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage__bindgen_ty_14 > ( ) ) ) . __ru_nivcsw_word as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage__bindgen_ty_14 ) , "::" , stringify ! ( __ru_nivcsw_word ) )) ;

}
 # [test] fn bindgen_test_layout_rusage () {
assert_eq ! (:: std :: mem :: size_of :: < rusage > ( ) , 144usize , concat ! ( "Size of: " , stringify ! ( rusage ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < rusage > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( rusage ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage > ( ) ) ) . ru_utime as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( rusage ) , "::" , stringify ! ( ru_utime ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < rusage > ( ) ) ) . ru_stime as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( rusage ) , "::" , stringify ! ( ru_stime ) )) ;

}
 pub const __priority_which_PRIO_PROCESS : __priority_which = 0 ;
 pub const __priority_which_PRIO_PGRP : __priority_which = 1 ;
 pub const __priority_which_PRIO_USER : __priority_which = 2 ;
 pub type __priority_which = u32 ;
 extern "C" {
pub fn prlimit (__pid : __pid_t , __resource : __rlimit_resource , __new_limit : * const rlimit , __old_limit : * mut rlimit) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn prlimit64 (__pid : __pid_t , __resource : __rlimit_resource , __new_limit : * const rlimit64 , __old_limit : * mut rlimit64) -> :: std :: os :: raw :: c_int ;

}
 pub type __rlimit_resource_t = :: std :: os :: raw :: c_int ;
 pub type __rusage_who_t = :: std :: os :: raw :: c_int ;
 pub type __priority_which_t = :: std :: os :: raw :: c_int ;
 extern "C" {
pub fn getrlimit (__resource : __rlimit_resource_t , __rlimits : * mut rlimit) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getrlimit64 (__resource : __rlimit_resource_t , __rlimits : * mut rlimit64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setrlimit (__resource : __rlimit_resource_t , __rlimits : * const rlimit) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setrlimit64 (__resource : __rlimit_resource_t , __rlimits : * const rlimit64) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getrusage (__who : __rusage_who_t , __usage : * mut rusage) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn getpriority (__which : __priority_which_t , __who : id_t) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
pub fn setpriority (__which : __priority_which_t , __who : id_t , __prio : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL15s_current_umask"] 
pub static mut s_current_umask : :: std :: os :: raw :: c_uint ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL14s_current_time"] 
pub static mut s_current_time : timeval ;

}
 pub static mut s_current_pid : :: std :: os :: raw :: c_int = - 1 ;
 extern "C" {
//# [link_name = "\u{1}_ZL11s_exit_func"] 
pub static mut s_exit_func : exitfunc_t ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL10s_timezone"] 
pub static mut s_timezone : :: std :: os :: raw :: c_long ;

}
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct vsf_sysutil_sig_details {
pub sync_sig_handler : vsf_sighandle_t ,
pub p_private : * mut :: std :: os :: raw :: c_void ,
pub pending : sig_atomic_t ,
pub running : :: std :: os :: raw :: c_int ,
pub use_alarm : :: std :: os :: raw :: c_int ,
}
 # [test] fn bindgen_test_layout_vsf_sysutil_sig_details () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_sig_details > ( ) , 32usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_sig_details ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_sig_details > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_sig_details ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sig_details > ( ) ) ) . sync_sig_handler as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sig_details ) , "::" , stringify ! ( sync_sig_handler ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sig_details > ( ) ) ) . p_private as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sig_details ) , "::" , stringify ! ( p_private ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sig_details > ( ) ) ) . pending as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sig_details ) , "::" , stringify ! ( pending ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sig_details > ( ) ) ) . running as * const _ as usize } , 20usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sig_details ) , "::" , stringify ! ( running ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sig_details > ( ) ) ) . use_alarm as * const _ as usize } , 24usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sig_details ) , "::" , stringify ! ( use_alarm ) )) ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL13s_sig_details"] 
pub static mut s_sig_details : [vsf_sysutil_sig_details ; 65usize] ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL12s_io_handler"] 
pub static mut s_io_handler : vsf_context_io_t ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL22s_p_io_handler_private"] 
pub static mut s_p_io_handler_private : * mut :: std :: os :: raw :: c_void ;

}
 extern "C" {
//# [link_name = "\u{1}_ZL20s_io_handler_running"] 
pub static mut s_io_handler_running : :: std :: os :: raw :: c_int ;

}
 # [repr ( C )] # [derive ( Copy , Clone )] pub struct vsf_sysutil_sockaddr {
pub u : vsf_sysutil_sockaddr__bindgen_ty_1 ,
}
 # [repr ( C )] # [derive ( Copy , Clone )] pub union vsf_sysutil_sockaddr__bindgen_ty_1 {
pub u_sockaddr : sockaddr , pub u_sockaddr_in : sockaddr_in , pub u_sockaddr_in6 : sockaddr_in6 , _bindgen_union_align : [u32 ; 7usize] ,
}
 # [test] fn bindgen_test_layout_vsf_sysutil_sockaddr__bindgen_ty_1 () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_sockaddr__bindgen_ty_1 > ( ) , 28usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_sockaddr__bindgen_ty_1 ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_sockaddr__bindgen_ty_1 > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_sockaddr__bindgen_ty_1 ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sockaddr__bindgen_ty_1 > ( ) ) ) . u_sockaddr as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sockaddr__bindgen_ty_1 ) , "::" , stringify ! ( u_sockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sockaddr__bindgen_ty_1 > ( ) ) ) . u_sockaddr_in as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sockaddr__bindgen_ty_1 ) , "::" , stringify ! ( u_sockaddr_in ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sockaddr__bindgen_ty_1 > ( ) ) ) . u_sockaddr_in6 as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sockaddr__bindgen_ty_1 ) , "::" , stringify ! ( u_sockaddr_in6 ) )) ;

}
 # [test] fn bindgen_test_layout_vsf_sysutil_sockaddr () {
assert_eq ! (:: std :: mem :: size_of :: < vsf_sysutil_sockaddr > ( ) , 28usize , concat ! ( "Size of: " , stringify ! ( vsf_sysutil_sockaddr ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < vsf_sysutil_sockaddr > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( vsf_sysutil_sockaddr ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < vsf_sysutil_sockaddr > ( ) ) ) . u as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( vsf_sysutil_sockaddr ) , "::" , stringify ! ( u ) )) ;

}
 pub const deny_severity : :: std :: os :: raw :: c_int = 4 ;
 pub const allow_severity : :: std :: os :: raw :: c_int = 6 ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __locale_data {
pub _address : u8 ,
}
 pub type __builtin_va_list = [__va_list_tag ; 1usize] ;
 # [repr ( C )] # [derive ( Debug , Copy , Clone )] pub struct __va_list_tag {
pub gp_offset : :: std :: os :: raw :: c_uint , pub fp_offset : :: std :: os :: raw :: c_uint , pub overflow_arg_area : * mut :: std :: os :: raw :: c_void , pub reg_save_area : * mut :: std :: os :: raw :: c_void ,
}
 # [test] fn bindgen_test_layout___va_list_tag () {
assert_eq ! (:: std :: mem :: size_of :: < __va_list_tag > ( ) , 24usize , concat ! ( "Size of: " , stringify ! ( __va_list_tag ) )) ;
 assert_eq ! (:: std :: mem :: align_of :: < __va_list_tag > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( __va_list_tag ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __va_list_tag > ( ) ) ) . gp_offset as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( __va_list_tag ) , "::" , stringify ! ( gp_offset ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __va_list_tag > ( ) ) ) . fp_offset as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( __va_list_tag ) , "::" , stringify ! ( fp_offset ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __va_list_tag > ( ) ) ) . overflow_arg_area as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( __va_list_tag ) , "::" , stringify ! ( overflow_arg_area ) )) ;
 assert_eq ! (unsafe { & ( * ( :: std :: ptr :: null :: < __va_list_tag > ( ) ) ) . reg_save_area as * const _ as usize } , 16usize , concat ! ( "Offset of field: " , stringify ! ( __va_list_tag ) , "::" , stringify ! ( reg_save_area ) )) ;

}
