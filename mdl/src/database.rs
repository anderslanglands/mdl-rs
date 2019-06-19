use crate::{
    scope::Scope,
    base::Interface,
    neuray::ApiComponent,
};
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
    fn from_interface(i: sys::IInterface) -> Database {
        let i = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if i.is_null() {
            panic!("Tried to convert from null interface");
        }

        Database {
            ptr: i as *mut sys::IDatabase_api,
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
