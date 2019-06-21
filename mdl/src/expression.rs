use mdl_sys as sys;

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

impl Expression {}

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

pub enum Error {}
