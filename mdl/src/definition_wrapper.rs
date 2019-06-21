use mdl_sys as sys;

use std::os::raw::c_char;

use std::ffi::{CStr, CString};

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

use crate::{
    expression::{AnnotationBlock, AnnotationList, ExpressionList},
    itype::{Type, TypeList},
    mdl_factory::MdlFactory,
    scene_element::SceneElement,
    transaction::Transaction,
};

pub struct DefinitionWrapper {
    ptr: sys::DefinitionWrapper,
}

impl DefinitionWrapper {
    pub fn new(
        transaction: &Transaction,
        name: &str,
        factory: &MdlFactory,
    ) -> Result<DefinitionWrapper> {
        let cname = CString::new(name).unwrap();
        let ptr = unsafe {
            sys::Definition_wrapper_new(
                transaction.ptr,
                cname.as_ptr(),
                factory.ptr,
            )
        };
        if ptr.is_null() {
            panic!("Failed to allocate definition wrapper");
        }

        let dw = DefinitionWrapper { ptr };
        if dw.is_valid() {
            Ok(dw)
        } else {
            Err(Error::InvalidDefinitionWrapper {
                name: name.to_string(),
            })
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sys::Definition_wrapper_is_valid(self.ptr) }
    }

    pub fn get_type(&self) -> sys::ElementType {
        unsafe { sys::Definition_wrapper_get_type(self.ptr) }
    }

    pub fn get_mdl_definition(&self) -> String {
        let ptr =
            unsafe { sys::Definition_wrapper_get_mdl_definition(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_mdl_definition returned NULL");
        } else {
            unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            }
        }
    }

    pub fn get_module(&self) -> String {
        let ptr = unsafe { sys::Definition_wrapper_get_module(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_module returned NULL");
        } else {
            unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            }
        }
    }

    pub fn is_exported(&self) -> bool {
        unsafe { sys::Definition_wrapper_is_exported(self.ptr) }
    }

    pub fn get_parameter_count(&self) -> usize {
        unsafe { sys::Definition_wrapper_get_parameter_count(self.ptr) }
    }

    pub fn get_parameter_name(&self, index: usize) -> Option<String> {
        let ptr = unsafe {
            sys::Definition_wrapper_get_parameter_name(self.ptr, index)
        };
        if ptr.is_null() {
            None
        } else {
            unsafe {
                Some(
                    CStr::from_ptr(ptr)
                        .to_string_lossy()
                        .to_owned()
                        .to_string(),
                )
            }
        }
    }

    pub fn get_parameter_index(&self, name: &str) -> Option<usize> {
        let cname = CString::new(name).unwrap();
        let index = unsafe {
            sys::Definition_wrapper_get_parameter_index(
                self.ptr,
                cname.as_ptr(),
            )
        };
        if index < 0 {
            None
        } else {
            Some(index as usize)
        }
    }

    pub fn get_parameter_types(&self) -> TypeList {
        let ptr =
            unsafe { sys::Definition_wrapper_get_parameter_types(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_parameter_types returned null");
        }

        TypeList { ptr }
    }

    pub fn get_return_type(&self) -> Option<Type> {
        let ptr = unsafe { sys::Definition_wrapper_get_return_type(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(Type { ptr })
        }
    }

    pub fn get_thumbnail(&self) -> Option<String> {
        let ptr = unsafe { sys::Definition_wrapper_get_thumbnail(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            unsafe {
                Some(
                    CStr::from_ptr(ptr)
                        .to_string_lossy()
                        .to_owned()
                        .to_string(),
                )
            }
        }
    }

    pub fn get_defaults(&self) -> ExpressionList {
        let ptr = unsafe { sys::Definition_wrapper_get_defaults(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_defaults returned NULL");
        } else {
            ExpressionList { ptr }
        }
    }

    pub fn get_annotations(&self) -> AnnotationBlock {
        let ptr = unsafe { sys::Definition_wrapper_get_annotations(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_annotations returned NULL");
        } else {
            AnnotationBlock { ptr }
        }
    }

    pub fn get_return_annotations(&self) -> AnnotationBlock {
        let ptr =
            unsafe { sys::Definition_wrapper_get_return_annotations(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_return_annotations returned NULL");
        } else {
            AnnotationBlock { ptr }
        }
    }

    pub fn get_parameter_annotations(&self) -> AnnotationList {
        let ptr = unsafe {
            sys::Definition_wrapper_get_parameter_annotations(self.ptr)
        };
        if ptr.is_null() {
            panic!(
                "Definition_wrapper_get_parameter_annotations returned NULL"
            );
        } else {
            AnnotationList { ptr }
        }
    }

    pub fn get_enable_if_conditions(&self) -> ExpressionList {
        let ptr = unsafe {
            sys::Definition_wrapper_get_enable_if_conditions(self.ptr)
        };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_enable_if_conditions returned NULL");
        } else {
            ExpressionList { ptr }
        }
    }

    pub fn get_enable_if_users(&self, index: usize) -> Result<usize> {
        let ret = unsafe {
            sys::Definition_wrapper_get_enable_if_users(self.ptr, index)
        };
        if ret == std::usize::MAX {
            Err(Error::ArgumentOutOfRange { index })
        } else {
            Ok(ret)
        }
    }

    pub fn get_enable_if_user(
        &self,
        index: usize,
        u_index: usize,
    ) -> Result<usize> {
        let ret = unsafe {
            sys::Definition_wrapper_get_enable_if_user(self.ptr, index, u_index)
        };
        if ret == std::usize::MAX {
            Err(Error::ArgumentsOutOfRange { index, u_index })
        } else {
            Ok(ret)
        }
    }

    pub fn create_instance(
        &self,
        args: Option<&ExpressionList>,
    ) -> Result<SceneElement> {
        let mut err: i32 = 0;
        let expr = if let Some(args) = args {
            args.ptr
        } else {
            std::ptr::null_mut()
        };
        let ptr = unsafe {
            sys::Definition_wrapper_create_instance(
                self.ptr,
                expr,
                &mut err as *mut i32,
            )
        };
        if ptr.is_null() {
            Err(match err {
                -1 => Error::NonExistingParameter,
                -2 => Error::TypeMismatch,
                -3 => Error::MissingDefault,
                _ => Error::UnknownError,
            })
        } else {
            Ok(SceneElement { ptr })
        }
    }

    pub fn get_mdl_factory(&self) -> MdlFactory {
        let ptr = unsafe { sys::Definition_wrapper_get_mdl_factory(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_mdl_factory returned NULL");
        }

        MdlFactory { ptr }
    }

    pub fn get_scene_element(&self) -> SceneElement {
        let ptr =
            unsafe { sys::Definition_wrapper_get_scene_element(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_scene_element returned NULL");
        }

        SceneElement { ptr }
    }

    pub fn get_element_type(&self) -> sys::ElementType {
        unsafe { sys::Definition_wrapper_get_element_type(self.ptr) }
    }

    pub fn get_name(&self) -> String {
        let ptr = unsafe { sys::Definition_wrapper_get_name(self.ptr) };
        if ptr.is_null() {
            panic!("Definition_wrapper_get_name returned NULL");
        }

        unsafe { CStr::from_ptr(ptr).to_str().unwrap().to_string() }
    }
}


impl Drop for DefinitionWrapper {
    fn drop(&mut self) {
        unsafe { sys::Definition_wrapper_delete(self.ptr) };
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Invalid definition wrapper: '{}'", name)]
    InvalidDefinitionWrapper { name: String },
    #[error(display = "Argument out of range: {}", index)]
    ArgumentOutOfRange { index: usize },
    #[error(display = "Arguments out of range: {} {}", index, u_index)]
    ArgumentsOutOfRange { index: usize, u_index: usize },
    #[error(
        display = "An argument for a non-existing parameter was provided in the arguments list"
    )]
    NonExistingParameter,
    #[error(
        display = "An argument in the arguments list does not have the correct type"
    )]
    TypeMismatch,
    #[error(
        display = "An argument that has no default was not provided with an argument value"
    )]
    MissingDefault,
    #[error(display = "An unknown error occurred (this should not happen)")]
    UnknownError,
}
