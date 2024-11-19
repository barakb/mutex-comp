use libc::{pthread_rwlock_t, PTHREAD_RWLOCK_INITIALIZER};
use std::cell::UnsafeCell;
use std::ptr::null_mut;

/// Wrap C rwlock as we can't use Rust RWLock.
/// Used to lock the graph.
pub struct CRWLock {
    rwlock: UnsafeCell<pthread_rwlock_t>,
}

unsafe impl Send for CRWLock {}
unsafe impl Sync for CRWLock {}
impl CRWLock {
    pub fn new() -> Self {
        let rwlock = UnsafeCell::new(PTHREAD_RWLOCK_INITIALIZER);
        unsafe {
            libc::pthread_rwlock_init(rwlock.get(), null_mut());
        }
        CRWLock { rwlock }
    }

    pub fn acquire_read(&self) {
        unsafe {
            let res = libc::pthread_rwlock_rdlock(self.rwlock.get());
            debug_assert!(res == 0, "pthread_rwlock_rdlock failed");
        }
    }

    pub fn acquire_write(&self) {
        unsafe {
            let res = libc::pthread_rwlock_wrlock(self.rwlock.get());
            debug_assert!(res == 0, "pthread_rwlock_wrlock failed");
        }
    }

    pub fn release(&self) {
        unsafe {
            let res = libc::pthread_rwlock_unlock(self.rwlock.get());
            debug_assert!(res == 0, "pthread_rwlock_unlock failed");
        }
    }
}

impl Drop for CRWLock {
    fn drop(&mut self) {
        unsafe { libc::pthread_rwlock_destroy(self.rwlock.get()) };
    }
}
