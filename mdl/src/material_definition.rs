use crate::{
    base::Interface, definition::Definition, expression::ExpressionList,
    itype::TypeList, material_instance::MaterialInstance,
};
use mdl_sys as sys;

use std::ffi::{CStr, CString};

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct MaterialDefinition {
    pub(crate) ptr: sys::IMaterialDefinition,
}

impl Drop for MaterialDefinition {
    fn drop(&mut self) {
        unsafe { sys::IMaterial_definition_release(self.ptr) };
    }
}

impl Clone for MaterialDefinition {
    fn clone(&self) -> MaterialDefinition {
        unsafe {
            sys::IMaterial_definition_retain(self.ptr);
        }
        MaterialDefinition { ptr: self.ptr }
    }
}

impl MaterialDefinition {
    pub fn create_material_instance(
        &self,
        arguments: Option<&ExpressionList>,
    ) -> Result<MaterialInstance, Error> {
        use sys::results::CreateMaterialInstanceResult as SysResult;

        let mut errors: SysResult = SysResult::Success;

        let args_ptr = if let Some(el) = arguments {
            el.ptr
        } else {
            std::ptr::null_mut()
        };

        let ptr = unsafe {
            sys::IMaterial_definition_create_material_instance(
                self.ptr,
                args_ptr,
                &mut errors as *mut sys::results::CreateMaterialInstanceResult,
            )
        };

        if ptr.is_null() {
            Err(match errors {
                SysResult::Success => unreachable!(),
                SysResult::NonExistingParameter => Error::NonExistingParameter,
                SysResult::ParameterTypeMismatch => {
                    Error::ParameterTypeMismatch
                }
                SysResult::NonDefaultParameterMissing => {
                    Error::NonDefaultParameterMissing
                }
                SysResult::DefinitionNotExported => {
                    Error::DefinitionNotExported
                }
                SysResult::UniformParameterGivenVaryingArgument => {
                    Error::UniformParameterGivenVaryingArgument
                }
                SysResult::ArgumentExpressionNotConstantNorCall => {
                    Error::ArgumentExpressionNotConstantNorCall
                }
                SysResult::UniformParameterGivenCallExpression => {
                    Error::UniformParameterGivenCallExpression
                }
            })
        } else {
            Ok(MaterialInstance { ptr })
        }
    }
}

impl Definition for MaterialDefinition {
    fn get_parameter_count(&self) -> usize {
        unsafe { sys::IMaterial_definition_get_parameter_count(self.ptr) }
    }

    fn get_parameter_index(&self, name: &str) -> Option<usize> {
        let name = CString::new(name).unwrap();
        let index = unsafe {
            sys::IMaterial_definition_get_parameter_index(
                self.ptr,
                name.as_ptr(),
            )
        };
        if index == std::usize::MAX {
            None
        } else {
            Some(index)
        }
    }

    fn get_parameter_name(&self, index: usize) -> Option<String> {
        let ptr = unsafe {
            sys::IMaterial_definition_get_parameter_name(self.ptr, index)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    fn get_parameter_types(&self) -> TypeList {
        let ptr =
            unsafe { sys::IMaterial_definition_get_parameter_types(self.ptr) };
        if ptr.is_null() {
            panic!("IMaterial_definition_get_parameter_types returned NULL");
        }
        TypeList { ptr }
    }

    fn get_defaults(&self) -> ExpressionList {
        let ptr = unsafe { sys::IMaterial_definition_get_defaults(self.ptr) };
        if ptr.is_null() {
            panic!("IMaterial_definition_get_defaults returned NULL");
        }
        ExpressionList { ptr }
    }
}

impl Interface for MaterialDefinition {
    fn from_interface(i: sys::IInterface) -> MaterialDefinition {
        let ptr = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if ptr.is_null() {
            panic!("Tried to convert from null interface");
        }

        // We rlease the original pointer
        unsafe { sys::IInterface_release(i) };

        MaterialDefinition {
            ptr: ptr as *mut sys::IMaterialDefinition_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMaterial_definition_type_get_iid() }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        display = "An argument for a non-existing parameter was provided in arguments"
    )]
    NonExistingParameter,
    #[error(
        display = "The type of an argument in arguments does not have the correct type, see get_parameter_types()"
    )]
    ParameterTypeMismatch,
    #[error(
        display = "A parameter that has no default was not provided with an argument value"
    )]
    NonDefaultParameterMissing,
    #[error(
        display = "The definition can not be instantiated because it is not exported"
    )]
    DefinitionNotExported,
    #[error(
        display = "A parameter type is uniform, but the corresponding argument has a varying return type"
    )]
    UniformParameterGivenVaryingArgument,
    #[error(display = "An argument expression is not a constant nor a call")]
    ArgumentExpressionNotConstantNorCall,
    #[error(
        display = "One of the parameter types is uniform, but the corresponding argument or default is a call expression and the return type of the called function definition is effectively varying since the function definition itself is varying"
    )]
    UniformParameterGivenCallExpression,
}
