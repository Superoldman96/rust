//! The functions in this module are needed by libunwind. These symbols are named
//! in pre-link args for the target specification, so keep that in sync.

#![cfg(not(test))]

use crate::sys::sync::RwLock;
use crate::{slice, str};

// Verify that the byte pattern libunwind uses to initialize an RwLock is
// equivalent to the value of RwLock::new(). If the value changes,
// `src/UnwindRustSgx.h` in libunwind needs to be changed too.
const _: () = unsafe {
    let bits_rust: usize = crate::mem::transmute(RwLock::new());
    assert!(bits_rust == 0);
};

const EINVAL: i32 = 22;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __rust_rwlock_rdlock(p: *mut RwLock) -> i32 {
    if p.is_null() {
        return EINVAL;
    }

    // We cannot differentiate between reads an writes in unlock and therefore
    // always use a write-lock. Unwinding isn't really in the hot path anyway.
    unsafe { (*p).write() };
    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __rust_rwlock_wrlock(p: *mut RwLock) -> i32 {
    if p.is_null() {
        return EINVAL;
    }
    unsafe { (*p).write() };
    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __rust_rwlock_unlock(p: *mut RwLock) -> i32 {
    if p.is_null() {
        return EINVAL;
    }
    unsafe { (*p).write_unlock() };
    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __rust_print_err(m: *mut u8, s: i32) {
    if s < 0 {
        return;
    }
    let buf = unsafe { slice::from_raw_parts(m as *const u8, s as _) };
    if let Ok(s) = str::from_utf8(&buf[..buf.iter().position(|&b| b == 0).unwrap_or(buf.len())]) {
        eprint!("{s}");
    }
}
