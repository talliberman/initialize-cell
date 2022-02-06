use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops;

#[derive(Debug)]
pub struct InitializeCell<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T> Sync for InitializeCell<T> {}

impl<T> InitializeCell<T> {
    pub const unsafe fn new_uninitialized() -> InitializeCell<T> {
        InitializeCell {
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
    pub const fn new(init: T) -> InitializeCell<T> {
        InitializeCell {
            inner: UnsafeCell::new(MaybeUninit::new(init)),
        }
    }
    pub unsafe fn init(&self, init: T) {
        (*self.inner.get()) = MaybeUninit::new(init);
    }
}

impl<T> ops::Deref for InitializeCell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*(*self.inner.get()).as_ptr() }
    }
}
