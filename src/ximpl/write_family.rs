#[cfg(target_os = "linux")]
use nix::unistd::write;

use windows_sys::Win32::Foundation::{BOOL, HANDLE};
#[cfg(target_os = "windows")]
use windows_sys::Win32::Storage::FileSystem::WriteFile;

use crate::errno;

#[inline]
pub fn __write(fildes: i32, buf: *const u8, nbyte: usize) -> isize {
    #[cfg(target_os = "linux")]
    return unistd::write(fildes, buf, nbyte)
        .try_into()
        .expect("Cannot convert unistd::write() return value into isize (ssize_t)");
    #[cfg(target_os = "windows")]
    let handle: HANDLE = fildes as HANDLE;
    let mut bytes_written: u32 = 0;
    if handle == core::ptr::null_mut() {
        panic!("Invalid handle in write()");
    }
    unsafe {
        WriteFile(
            handle,
            buf,
            nbyte
                .try_into()
                .expect("nbyte exceeded nnumberofbytestowrite in WriteFile"),
            &mut bytes_written,
            core::ptr::null_mut(),
        );
    }

    if bytes_written > nbyte.try_into().expect("Cannot convert nbyte to u32") {
        return -1;
    }

    return bytes_written
        .try_into()
        .expect("Cannot convert bytes_written into isize (ssize_t)");
}
