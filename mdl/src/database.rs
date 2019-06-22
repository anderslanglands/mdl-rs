use crate::{base::Interface, neuray::ApiComponent, scope::Scope};
use mdl_sys as sys;

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Database {
    pub(crate) ptr: sys::IDatabase,
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe { sys::IDatabase_release(self.ptr) }
    }
}

impl Database {
    pub fn get_global_scope(&self) -> Result<Scope> {
        let ptr = unsafe { sys::IDatabase_get_global_scope(self.ptr) };
        // This *should* never fail, because the global scope is always
        // guaranteed to exist after startup, but you can't trust a C pointer
        if ptr.is_null() {
            Err(Error::ScopeNotFound {
                name: "GLOBAL".to_string(),
            })
        } else {
            Ok(Scope { ptr })
        }
    }
}

impl Interface for Database {
    fn from_interface_ptr(ptr: sys::IInterface) -> Database {
        Database {
            ptr: ptr as sys::IDatabase,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IDatabase_type_get_iid() }
    }
}

impl ApiComponent for Database {}

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Scope '{}' not found", name)]
    ScopeNotFound { name: String },
}
