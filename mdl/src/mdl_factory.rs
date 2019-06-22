use crate::{
    base::Interface, neuray::ApiComponent, ExpressionFactory,
    MdlExecutionContext, Transaction, TypeFactory, ValueFactory,
};
use mdl_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct MdlFactory {
    pub(crate) ptr: sys::IMdlFactory,
}

impl Drop for MdlFactory {
    fn drop(&mut self) {
        unsafe { sys::IMdl_factory_release(self.ptr) };
    }
}

impl Clone for MdlFactory {
    fn clone(&self) -> MdlFactory {
        unsafe {
            sys::IMdl_factory_retain(self.ptr);
        }
        MdlFactory { ptr: self.ptr }
    }
}

impl MdlFactory {
    pub fn create_execution_context(&self) -> MdlExecutionContext {
        let ptr =
            unsafe { sys::IMdl_factory_create_execution_context(self.ptr) };
        if ptr.is_null() {
            panic!("MdlFactory::create_execution_context returned null ptr");
        }
        MdlExecutionContext { ptr }
    }

    pub fn create_type_factory(
        &self,
        transaction: &Transaction,
    ) -> TypeFactory {
        let ptr = unsafe {
            sys::IMdl_factory_create_type_factory(self.ptr, transaction.ptr)
        };
        if ptr.is_null() {
            panic!("MdlFactory::create_type_factory() returned NULL");
        }
        TypeFactory { ptr }
    }

    pub fn create_value_factory(
        &self,
        transaction: &Transaction,
    ) -> ValueFactory {
        let ptr = unsafe {
            sys::IMdl_factory_create_value_factory(self.ptr, transaction.ptr)
        };
        if ptr.is_null() {
            panic!("MdlFactory::create_value_factory() returned NULL");
        }
        ValueFactory { ptr }
    }

    pub fn create_expression_factory(
        &self,
        transaction: &Transaction,
    ) -> ExpressionFactory {
        let ptr = unsafe {
            sys::IMdl_factory_create_expression_factory(
                self.ptr,
                transaction.ptr,
            )
        };
        if ptr.is_null() {
            panic!("MdlFactory::create_expression_factory() returned NULL");
        }
        ExpressionFactory { ptr }
    }
}

impl Interface for MdlFactory {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlFactory {
        MdlFactory {
            ptr: ptr as sys::IMdlFactory,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_factory_type_get_iid() }
    }
}

impl ApiComponent for MdlFactory {}

pub enum Error {}
