use crate::base::Interface;
use mdl_sys as sys;

use std::ffi::CString;

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Transaction {
    pub(crate) ptr: sys::ITransaction,
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe { sys::ITransaction_release(self.ptr) };
    }
}

impl Clone for Transaction {
    fn clone(&self) -> Transaction {
        unsafe {
            sys::ITransaction_retain(self.ptr);
        }
        Transaction { ptr: self.ptr }
    }
}

impl Transaction {
    pub fn access<T: Interface>(&self, name: &str) -> Result<T> {
        let cname = CString::new(name).unwrap();
        let ptr = unsafe { sys::ITransaction_access(self.ptr, cname.as_ptr()) };
        if ptr.is_null() {
            Err(Error::AccessFailed {
                name: name.to_string(),
            })
        } else {
            Ok(T::from_interface(ptr))
        }
    }

    pub fn commit(&self) -> Result<()> {
        match unsafe { sys::ITransaction_commit(self.ptr) } {
            sys::TransactionCommitResult::Success => Ok(()),
            sys::TransactionCommitResult::UnspecifiedFailure => Err(Error::UnspecifiedFailure),
            sys::TransactionCommitResult::TransactionAlreadyClosed => {
                Err(Error::TransactionAlreadyClosed)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Transaction access to item '{}' failed", name)]
    AccessFailed { name: String },
    #[error(display = "Unspecified failure in ITransaction::commit()")]
    UnspecifiedFailure,
    #[error(display = "Transaction already closed when committing")]
    TransactionAlreadyClosed,
}
