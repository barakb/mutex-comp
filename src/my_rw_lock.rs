use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;

pub struct MyRWLock {
    pub rwlock: RwLock<()>,
    read_guards: Mutex<Vec<ManuallyDrop<RwLockReadGuard<'static, ()>>>>,
    write_guard: UnsafeCell<Option<ManuallyDrop<RwLockWriteGuard<'static, ()>>>>,
}

unsafe impl Send for MyRWLock {}
unsafe impl Sync for MyRWLock {}

impl MyRWLock {
    pub fn new() -> Self {
        MyRWLock {
            rwlock: RwLock::new(()),
            read_guards: Mutex::new(Vec::new()),
            write_guard: UnsafeCell::new(None),
        }
    }

    #[inline]
    pub fn acquire_read(&self) {
        let guard = self.rwlock.read();
        let static_guard = unsafe { std::mem::transmute(guard) };
        self.read_guards.lock().push(ManuallyDrop::new(static_guard));
    }

    #[inline]
    pub fn acquire_write(&self) {
        let guard = self.rwlock.write();
        unsafe {
            *self.write_guard.get() = Some(ManuallyDrop::new(std::mem::transmute(guard)));
        }
    }

    #[inline]
    pub fn release(&self) {
        unsafe {
            if let Some(mut write_guard) = (*self.write_guard.get()).take() {
                ManuallyDrop::drop(&mut write_guard);
            } else if let Some(mut read_guard) = self.read_guards.lock().pop() {
                ManuallyDrop::drop(&mut read_guard);
            }
        }
    }


    #[inline]
    pub fn is_locked(&self) -> bool {
        unsafe { (*self.write_guard.get()).is_some() || !self.read_guards.lock().is_empty() }
    }
}

impl Drop for MyRWLock {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(mut write_guard) = self.write_guard.get_mut().take() {
                ManuallyDrop::drop(&mut write_guard);
            }
            while let Some(mut read_guard) = self.read_guards.get_mut().pop() {
                ManuallyDrop::drop(&mut read_guard);
            }
        }
    }
}