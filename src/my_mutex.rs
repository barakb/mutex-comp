use parking_lot::{Mutex, MutexGuard};
use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;

pub struct MyMutex {
    pub mutex: Mutex<()>,
    guard: UnsafeCell<Option<ManuallyDrop<MutexGuard<'static, ()>>>>,
}

unsafe impl Send for MyMutex {}
unsafe impl Sync for MyMutex {}

impl MyMutex {
    pub fn new() -> Self {
        MyMutex {
            mutex: Mutex::new(()),
            guard: UnsafeCell::new(None),
        }
    }

    #[inline]
    pub fn lock(&self) {
        let guard = self.mutex.lock();
        unsafe {
            *self.guard.get() = Some(ManuallyDrop::new(std::mem::transmute(guard)));
        }
    }

    #[inline]
    pub fn unlock(&self) {
        unsafe {
            if let Some(mut guard) = (*self.guard.get()).take() {
                ManuallyDrop::drop(&mut guard);
            }
        }
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        unsafe { (*self.guard.get()).is_some() }
    }
}

impl Drop for MyMutex {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(mut guard) = (*self.guard.get_mut()).take() {
                ManuallyDrop::drop(&mut guard);
            }
        }
    }
}
