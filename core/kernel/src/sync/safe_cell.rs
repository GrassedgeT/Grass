//! temp implementation of a safe cell
//! ! this is not a safe implementation
// TODO! wait for concurrent data structures to be implemented
use core::cell::{RefCell, RefMut};

pub struct SafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for SafeCell<T> {}

impl<T> SafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}