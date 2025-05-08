#![no_std]

extern crate boxed;
extern crate macros;
extern crate ptr;
extern crate result;

use boxed::prelude::*;
use core::mem::forget;
use core::ops::{Deref, DerefMut};
use macros::prelude::*;
use ptr::Ptr;
use result::Result;

struct RcInner<T: ?Sized> {
    count: u64,
    value: T,
}

pub struct Rc<T: ?Sized> {
    inner: Box<RcInner<T>>,
}

impl<T: ?Sized> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let ptr = self.inner.as_ptr();
        let mut inner: Box<RcInner<T>> = unsafe { Box::from_raw(Ptr::new(ptr)) };
        unsafe {
            inner.leak();
        }
        aadd!(&mut inner.count, 1);
        Rc { inner }
    }
}

impl<T: ?Sized> Drop for Rc<T> {
    fn drop(&mut self) {
        let rci: *mut RcInner<T> = self.inner.as_ptr() as *mut RcInner<T>;
        if asub!(&mut (*rci).count, 1) == 1 {
            unsafe {
                self.inner.unleak();
            }
        }
    }
}

impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner.value
    }
}

impl<T: ?Sized> DerefMut for Rc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.value
    }
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Result<Self> {
        match Box::new(RcInner { value, count: 1 }) {
            Ok(mut inner) => {
                unsafe {
                    inner.leak();
                }
                Ok(Self { inner })
            }
            Err(e) => Err(e),
        }
    }

    pub unsafe fn from_raw(ptr: Ptr<T>) -> Self {
        let mut inner = Box::from_raw(Ptr::new(ptr.as_ptr() as *const RcInner<T>));
        inner.leak();
        Self { inner }
    }

    pub unsafe fn into_raw(self) -> Ptr<T> {
        let ret = Ptr::new(self.inner.as_ptr() as *const T);
        forget(self);
        ret
    }

    pub unsafe fn set_to_drop(&mut self) {
        let rci = self.inner.as_mut();
        astore!(&mut rci.count, 1);
    }
}
