use core::cell::{RefCell, RefMut};

#[derive(Clone)]
pub struct UPSafeCell<T> {
    inner: RefCell<T>
}

impl<T> UPSafeCell<T> {
    pub fn new(inner: T) -> Self {
        UPSafeCell { inner: RefCell::new(inner) }
    }

    pub fn as_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}