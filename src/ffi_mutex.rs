use parking_lot::{Mutex, MutexGuard};
use std::mem::ManuallyDrop;

pub struct FFIMutex(Mutex<()>);

unsafe impl Send for FFIMutex {}
unsafe impl Sync for FFIMutex {}

#[no_mangle]
pub extern "C" fn mutex_create() -> *mut FFIMutex {
    Box::into_raw(Box::new(FFIMutex(Mutex::new(()))))
}

#[no_mangle]
pub extern "C" fn mutex_lock(mutex: *mut FFIMutex) -> *mut () {
    unsafe {
        let guard = (*mutex).0.lock();
        let raw_guard = Box::into_raw(Box::new(ManuallyDrop::new(guard)));
        raw_guard as *mut ()
    }
}

#[no_mangle]
pub extern "C" fn mutex_unlock(guard: *mut ()) {
    unsafe {
        let mut guard = Box::from_raw(guard as *mut ManuallyDrop<MutexGuard<'_, ()>>);
        ManuallyDrop::drop(&mut *guard);
    }
}

#[no_mangle]
pub extern "C" fn mutex_destroy(mutex: *mut FFIMutex) {
    unsafe {
        drop(Box::from_raw(mutex));
    }
}