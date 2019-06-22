use mdl_sys as sys;

pub use sys::ExpressionKind;

use crate::base::Interface;

use crate::itype::Type;

use crate::value::Value;

use std::ffi::{CStr, CString};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Expression {
    pub(crate) ptr: sys::IExpression,
}

impl Drop for Expression {
    fn drop(&mut self) {
        unsafe { sys::IExpression_release(self.ptr) };
    }
}

impl Clone for Expression {
    fn clone(&self) -> Expression {
        unsafe { sys::IExpression_retain(self.ptr) };
        Expression { ptr: self.ptr }
    }
}

impl Expression {
    pub fn get_kind(&self) -> ExpressionKind {
        unsafe { sys::IExpression_get_kind(self.ptr) }
    }

    pub fn get_type(&self) -> Type {
        let ptr = unsafe { sys::IExpression_get_type(self.ptr) };
        if ptr.is_null() {
            panic!("IExpression_get_type returned NULL");
        }
        Type { ptr }
    }
}

impl Interface for Expression {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> Expression {
        Expression {
            ptr: ptr as sys::IExpression,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IExpression_type_get_iid() }
    }
}

pub struct ExpressionDirectCall {
    pub(crate) ptr: sys::IExpressionDirectCall,
}

impl Drop for ExpressionDirectCall {
    fn drop(&mut self) {
        unsafe { sys::IExpression_direct_call_release(self.ptr) };
    }
}

impl Clone for ExpressionDirectCall {
    fn clone(&self) -> ExpressionDirectCall {
        unsafe { sys::IExpression_direct_call_retain(self.ptr) };
        ExpressionDirectCall { ptr: self.ptr }
    }
}

impl ExpressionDirectCall {
    pub fn get_definition(&self) -> String {
        let ptr =
            unsafe { sys::IExpression_direct_call_get_definition(self.ptr) };
        if ptr.is_null() {
            panic!("IExpression_direct_call_get_definition returned NULL");
        }

        unsafe { CStr::from_ptr(ptr).to_str().unwrap().to_string() }
    }

    pub fn get_arguments(&self) -> ExpressionList {
        let ptr =
            unsafe { sys::IExpression_direct_call_get_arguments(self.ptr) };
        if ptr.is_null() {
            panic!("IExpression_direct_call_get_arguments returned NULL");
        }

        unsafe { ExpressionList { ptr } }
    }
}

impl Interface for ExpressionDirectCall {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> ExpressionDirectCall {
        ExpressionDirectCall {
            ptr: ptr as sys::IExpressionDirectCall,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IExpression_direct_call_type_get_iid() }
    }
}

pub struct ExpressionCall {
    pub(crate) ptr: sys::IExpressionCall,
}

impl Drop for ExpressionCall {
    fn drop(&mut self) {
        unsafe { sys::IExpression_call_release(self.ptr) };
    }
}

impl Clone for ExpressionCall {
    fn clone(&self) -> ExpressionCall {
        unsafe { sys::IExpression_call_retain(self.ptr) };
        ExpressionCall { ptr: self.ptr }
    }
}

impl ExpressionCall {}

impl Interface for ExpressionCall {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> ExpressionCall {
        ExpressionCall {
            ptr: ptr as sys::IExpressionCall,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IExpression_call_type_get_iid() }
    }
}

pub struct ExpressionParameter {
    pub(crate) ptr: sys::IExpressionParameter,
}

impl Drop for ExpressionParameter {
    fn drop(&mut self) {
        unsafe { sys::IExpression_parameter_release(self.ptr) };
    }
}

impl Clone for ExpressionParameter {
    fn clone(&self) -> ExpressionParameter {
        unsafe { sys::IExpression_parameter_retain(self.ptr) };
        ExpressionParameter { ptr: self.ptr }
    }
}

impl ExpressionParameter {
    pub fn get_index(&self) -> usize {
        unsafe { sys::IExpression_parameter_get_index(self.ptr) }
    }

    pub fn set_index(&self, index: usize) {
        unsafe { sys::IExpression_parameter_set_index(self.ptr, index) };
    }
}

impl Interface for ExpressionParameter {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> ExpressionParameter {
        ExpressionParameter {
            ptr: ptr as sys::IExpressionParameter,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IExpression_parameter_type_get_iid() }
    }
}

pub struct ExpressionConstant {
    pub(crate) ptr: sys::IExpressionConstant,
}

impl Drop for ExpressionConstant {
    fn drop(&mut self) {
        unsafe { sys::IExpression_constant_release(self.ptr) };
    }
}

impl Clone for ExpressionConstant {
    fn clone(&self) -> ExpressionConstant {
        unsafe { sys::IExpression_constant_retain(self.ptr) };
        ExpressionConstant { ptr: self.ptr }
    }
}

impl ExpressionConstant {
    pub fn get_value(&self) -> Value {
        let ptr = unsafe { sys::IExpression_constant_get_value(self.ptr) };
        if ptr.is_null() {
            panic!("IExpression_constant_get_value returned NULL");
        }

        Value { ptr }
    }
}

impl Interface for ExpressionConstant {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> ExpressionConstant {
        ExpressionConstant {
            ptr: ptr as sys::IExpressionConstant,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IExpression_constant_type_get_iid() }
    }
}

pub struct ExpressionTemporary {
    pub(crate) ptr: sys::IExpressionTemporary,
}

impl Drop for ExpressionTemporary {
    fn drop(&mut self) {
        unsafe { sys::IExpression_temporary_release(self.ptr) };
    }
}

impl Clone for ExpressionTemporary {
    fn clone(&self) -> ExpressionTemporary {
        unsafe { sys::IExpression_temporary_retain(self.ptr) };
        ExpressionTemporary { ptr: self.ptr }
    }
}

impl ExpressionTemporary {
    pub fn get_index(&self) -> usize {
        unsafe { sys::IExpression_temporary_get_index(self.ptr) }
    }

    pub fn set_index(&self, index: usize) {
        unsafe { sys::IExpression_temporary_set_index(self.ptr, index) };
    }

}

impl Interface for ExpressionTemporary {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> ExpressionTemporary {
        ExpressionTemporary {
            ptr: ptr as sys::IExpressionTemporary,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IExpression_temporary_type_get_iid() }
    }
}

pub struct ExpressionList {
    pub(crate) ptr: sys::IExpressionList,
}

impl Drop for ExpressionList {
    fn drop(&mut self) {
        unsafe { sys::IExpression_list_release(self.ptr) };
    }
}

impl Clone for ExpressionList {
    fn clone(&self) -> ExpressionList {
        unsafe { sys::IExpression_list_retain(self.ptr) };
        ExpressionList { ptr: self.ptr }
    }
}

impl ExpressionList {
    pub fn get_size(&self) -> usize {
        unsafe { sys::IExpression_list_get_size(self.ptr) }
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        let cname = CString::new(name).unwrap();
        let index = unsafe {
            sys::IExpression_list_get_index(self.ptr, cname.as_ptr())
        };
        if index == std::usize::MAX {
            None
        } else {
            Some(index)
        }
    }

    pub fn get_name(&self, index: usize) -> Option<String> {
        let cname = unsafe { sys::IExpression_list_get_name(self.ptr, index) };
        if cname.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(cname)
                    .to_string_lossy()
                    .to_owned()
                    .to_string()
            })
        }
    }

    pub fn get_expression(&self, index: usize) -> Option<Expression> {
        let ptr =
            unsafe { sys::IExpression_list_get_expression(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Expression { ptr })
        }
    }

    pub fn get_expression_by_name(&self, name: &str) -> Option<Expression> {
        let name = CString::new(name).unwrap();
        let ptr = unsafe {
            sys::IExpression_list_get_expression_by_name(
                self.ptr,
                name.as_ptr(),
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(Expression { ptr })
        }
    }

    pub fn iter(&self) -> ExpressionListIterator {
        ExpressionListIterator {
            expression_list: self,
            current: 0,
            count: self.get_size(),
        }
    }
}

pub struct ExpressionListIterator<'a> {
    expression_list: &'a ExpressionList,
    current: usize,
    count: usize,
}

impl<'a> Iterator for ExpressionListIterator<'a> {
    type Item = Expression;
    fn next(&mut self) -> Option<Expression> {
        if self.current < self.count {
            self.current += 1;
            self.expression_list.get_expression(self.current - 1)
        } else {
            None
        }
    }
}

pub struct ExpressionFactory {
    pub(crate) ptr: sys::IExpressionFactory,
}

impl ExpressionFactory {
    pub fn dump(
        &self,
        t: &Expression,
        name: Option<&str>,
        depth: usize,
    ) -> Option<String> {
        let ptr = if let Some(name) = name {
            let name = CString::new(name).unwrap();
            unsafe {
                sys::IExpression_factory_dump(
                    self.ptr,
                    t.ptr,
                    name.as_ptr(),
                    depth,
                )
            }
        } else {
            unsafe {
                sys::IExpression_factory_dump(
                    self.ptr,
                    t.ptr,
                    std::ptr::null(),
                    depth,
                )
            }
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
}

pub struct AnnotationBlock {
    pub(crate) ptr: sys::IAnnotationBlock,
}

pub struct AnnotationList {
    pub(crate) ptr: sys::IAnnotationList,
}

pub enum Error {}
