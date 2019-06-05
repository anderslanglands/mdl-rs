use crate::transaction::Transaction;
use mdl_sys as sys;

use std::ffi::CString;
use std::path::Path;

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Scope {
    pub(crate) ptr: sys::IScope,
}

impl Drop for Scope {
    fn drop(&mut self) {
        unsafe { sys::IScope_release(self.ptr) };
    }
}

impl Clone for Scope {
    fn clone(&self) -> Scope {
        unsafe { sys::IScope_retain(self.ptr) };
        Scope { ptr: self.ptr }
    }
}

impl Scope {
    // FIXME: Can only have one transaction at a time - what's the best
    // way of encoding this?
    pub fn create_transaction(&mut self) -> Transaction {
        let ptr = unsafe { sys::IScope_create_transaction(self.ptr) };
        if ptr.is_null() {
            panic!("Scope::create_transaction() returned NULL");
        }
        Transaction { ptr }
    }
}

pub enum Error {}
