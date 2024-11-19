use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::mem::ManuallyDrop;

pub struct FFIRwLock(RwLock<()>);

unsafe impl Send for FFIRwLock {}
unsafe impl Sync for FFIRwLock {}

#[no_mangle]
pub extern "C" fn rwlock_create() -> *mut FFIRwLock {
    Box::into_raw(Box::new(FFIRwLock(RwLock::new(()))))
}

#[no_mangle]
pub extern "C" fn rwlock_read_lock(rwlock: *mut FFIRwLock) -> *mut () {
    unsafe {
        let guard = (*rwlock).0.read();
        let raw_guard = Box::into_raw(Box::new(ManuallyDrop::new(guard)));
        raw_guard as *mut ()
    }
}

#[no_mangle]
pub extern "C" fn rwlock_write_lock(rwlock: *mut FFIRwLock) -> *mut () {
    unsafe {
        let guard = (*rwlock).0.write();
        let raw_guard = Box::into_raw(Box::new(ManuallyDrop::new(guard)));
        raw_guard as *mut ()
    }
}

#[no_mangle]
pub extern "C" fn rwlock_read_unlock(guard: *mut ()) {
    unsafe {
        let mut guard = Box::from_raw(guard as *mut ManuallyDrop<RwLockReadGuard<'_, ()>>);
        ManuallyDrop::drop(&mut *guard);
    }
}

#[no_mangle]
pub extern "C" fn rwlock_write_unlock(guard: *mut ()) {
    unsafe {
        let mut guard = Box::from_raw(guard as *mut ManuallyDrop<RwLockWriteGuard<'_, ()>>);
        ManuallyDrop::drop(&mut *guard);
    }
}

#[no_mangle]
pub extern "C" fn rwlock_destroy(rwlock: *mut FFIRwLock) {
    unsafe {
        drop(Box::from_raw(rwlock));
    }
}