unsafe extern "C" {
    /// errno
    pub static mut errno: i32;
}

// Standard Error Codes
pub const EDOM: i32 = 1;
pub const ERANGE: i32 = 2;
pub const EACCES: i32 = 111;
pub const EAGAIN: i32 = 112;
pub const EBADF: i32 = 113;
pub const EBUSY: i32 = 114;
pub const ECHILD: i32 = 115;
pub const EDEADLK: i32 = 116;
pub const EEXIST: i32 = 117;
pub const EFAULT: i32 = 118;
pub const EFBIG: i32 = 119;
pub const EINTR: i32 = 120;
pub const EINVAL: i32 = 121;
pub const EIO: i32 = 122;
pub const EISDIR: i32 = 123;
pub const EMFILE: i32 = 124;
pub const EMLINK: i32 = 125;
pub const ENAMETOOLONG: i32 = 126;
pub const ENFILE: i32 = 127;
pub const ENODEV: i32 = 128;
pub const ENOENT: i32 = 129;
pub const ENOEXEC: i32 = 130;
pub const ENOLCK: i32 = 131;
pub const ENOMEM: i32 = 132;
pub const ENOSPC: i32 = 133;
pub const ENOSYS: i32 = 134;
pub const ENOTDIR: i32 = 135;
pub const ENOTEMPTY: i32 = 136;
pub const ENOTTY: i32 = 137;
pub const ENXIO: i32 = 138;
pub const EPERM: i32 = 139;
pub const EPIPE: i32 = 140;
pub const EROFS: i32 = 141;
pub const ESPIPE: i32 = 142;
pub const ESRCH: i32 = 143;
pub const EXDEV: i32 = 144;
pub const E2BIG: i32 = 145;
pub const ELOOP: i32 = 146;
pub const EILSEQ: i32 = 147;
pub const EOVERFLOW: i32 = 149;
pub const EMVSNOTUP: i32 = 150;
pub const EMVSDYNALC: i32 = 151;
pub const EMVSCVAF: i32 = 152;
pub const EMVSCATLG: i32 = 153;
pub const EMVSINITIAL: i32 = 156;
pub const EMVSERR: i32 = 157;
pub const EMVSPARM: i32 = 158;
pub const EMVSPFSFILE: i32 = 159;
pub const EMVSPFSPERM: i32 = 162;
pub const EMVSSAFEXTRERR: i32 = 163;
pub const EMVSSAF2ERR: i32 = 164;
pub const EMVSNORTL: i32 = 167;
pub const EMVSEXPIRE: i32 = 168;
pub const EMVSPASSWORD: i32 = 169;
pub const EMVSWLMERROR: i32 = 170;
pub const EMVSCPLERROR: i32 = 171;
pub const EMVSARMERROR: i32 = 172;

// Network Error Codes
pub const ENOTSUP: i32 = 247;
pub const EIBMSOCKOUTOFRANGE: i32 = 1002;
pub const EIBMSOCKINUSE: i32 = 1003;
pub const EOFFLOADboxERROR: i32 = 1005;
pub const EOFFLOADboxRESTART: i32 = 1006;
pub const EOFFLOADboxDOWN: i32 = 1007;
pub const EIBMCONFLICT: i32 = 1008;
pub const EIBMCANCELLED: i32 = 1009;
pub const ENOTBLK: i32 = 1100;
pub const ETXTBSY: i32 = 1101;
pub const EWOULDBLOCK: i32 = 1102;
pub const EINPROGRESS: i32 = 1103;
pub const EALREADY: i32 = 1104;
pub const ENOTSOCK: i32 = 1105;
pub const EDESTADDRREQ: i32 = 1106;
pub const EMSGSIZE: i32 = 1107;
pub const EPROTOTYPE: i32 = 1108;
pub const ENOPROTOOPT: i32 = 1109;
pub const EPROTONOSUPPORT: i32 = 1110;
pub const ESOCKTNOSUPPORT: i32 = 1111;
pub const EOPNOTSUPP: i32 = 1112;
pub const ECONNREFUSED: i32 = 1128;
pub const ETIMEDOUT: i32 = 1127;
pub const ECONNRESET: i32 = 1121;
pub const ENOBUFS: i32 = 1122;

// Miscellaneous Errors
pub const ECANCELED: i32 = 1152;
pub const EUNATCH: i32 = 3448;
