use libc::pthread_mutex_t;
use std::cell::UnsafeCell;
use std::{mem::MaybeUninit, ptr::null_mut};

/// Wrap C mutex as we can't use Rust Mutex.
/// Used to lock the matrix only when we apply pending changes.
pub struct CMutex {
    mutex: UnsafeCell<pthread_mutex_t>,
}
unsafe impl Send for CMutex {}
unsafe impl Sync for CMutex {}

impl CMutex {
    pub fn new() -> Self {
        unsafe {
            let mut mutex = MaybeUninit::uninit();
            libc::pthread_mutex_init(mutex.as_mut_ptr(), null_mut());
            Self {
                mutex: UnsafeCell::new(mutex.assume_init()),
            }
        }
    }

    pub fn lock(&self) {
        unsafe {
            libc::pthread_mutex_lock(self.mutex.get());
        }
    }

    pub fn unlock(&self) {
        unsafe {
            libc::pthread_mutex_unlock(self.mutex.get());
        }
    }
}

impl Drop for CMutex {
    fn drop(&mut self) {
        unsafe { libc::pthread_mutex_destroy(self.mutex.get()) };
    }
}