# Makefile for systems with GNU tools
CC 	=	gcc
INSTALL	=	install
IFLAGS  = -idirafter dummyinc
#CFLAGS = -g
CFLAGS	=	-O2 -fPIE -fstack-protector --param=ssp-buffer-size=4 \
	-Wall -W -Wshadow -Werror -Wformat-security \
	-D_FORTIFY_SOURCE=2 \
#	-pedantic -Wconversion

LIBS	=	`./vsf_findlibs.sh`
LINK	=	-Wl,-s
LDFLAGS	=	-fPIE -pie -Wl,-z,relro -Wl,-z,now

OBJS	=	main.o utility.o prelogin.o ftpcmdio.o postlogin.o privsock.o \
		tunables.o ftpdataio.o secbuf.o ls.o \
		postprivparent.o logging.o str.o netstr.o sysstr.o strlist.o \
    banner.o filestr.o parseconf.o secutil.o \
    ascii.o oneprocess.o twoprocess.o privops.o standalone.o hash.o \
    tcpwrap.o ipaddrparse.o access.o features.o readwrite.o \
    ssl.o sslslave.o ptracesandbox.o ftppolicy.o sysutil.o sysdeputil.o \
    seccompsandbox.o

RUSTC = rustc
RUST_FLAGS = --crate-type=staticlib --emit=obj -C panic=abort

RUST_OBJS = opts.o
RUST_STDLIB_PATH = /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-c78f872a5c746c09.so

vsftpd: $(OBJS) $(RUST_OBJS)
#	echo "LIBS=",$(LIBS)
#	$(CC) -o vsftpd $(OBJS) $(LINK) $(LDFLAGS) $(LIBS)
#	ld -r -o CObject.o $(OBJS)
#	ld -r -o RustObject.o $(RUST_OBJS)
#	clang -o vsftpd CObject.o RustObject.o $(RUST_STDLIB_PATH) $(LINK) $(LDFLAGS) $(LIBS)
	clang -o vsftpd $(OBJS) $(RUST_OBJS) $(RUST_STDLIB_PATH) $(LINK) $(LDFLAGS) $(LIBS)

opts.o:
	$(RUSTC) $(RUST_FLAGS) rust/opts.rs -A warnings

.c.o:
	$(CC) -c $*.c $(CFLAGS) $(IFLAGS)

install:
	if [ -x /usr/local/sbin ]; then \
		$(INSTALL) -m 755 vsftpd /usr/local/sbin/vsftpd; \
	else \
		$(INSTALL) -m 755 vsftpd /usr/sbin/vsftpd; fi
	if [ -x /usr/local/man ]; then \
		$(INSTALL) -m 644 vsftpd.8 /usr/local/man/man8/vsftpd.8; \
		$(INSTALL) -m 644 vsftpd.conf.5 /usr/local/man/man5/vsftpd.conf.5; \
	elif [ -x /usr/share/man ]; then \
		$(INSTALL) -m 644 vsftpd.8 /usr/share/man/man8/vsftpd.8; \
		$(INSTALL) -m 644 vsftpd.conf.5 /usr/share/man/man5/vsftpd.conf.5; \
	else \
		$(INSTALL) -m 644 vsftpd.8 /usr/man/man8/vsftpd.8; \
		$(INSTALL) -m 644 vsftpd.conf.5 /usr/man/man5/vsftpd.conf.5; fi
	if [ -x /etc/xinetd.d ]; then \
		$(INSTALL) -m 644 xinetd.d/vsftpd /etc/xinetd.d/vsftpd; fi

clean:
	rm -f *.o *.swp vsftpd

