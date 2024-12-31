#![no_std]

mod ximpl;
mod errno;

#[no_mangle]
pub extern "C" fn write(fildes: i32, buf: *const u8, nbyte: usize) {
    ximpl::write_family::__write(fildes, buf, nbyte);
}
