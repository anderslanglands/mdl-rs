use mdl_sys as sys;

use crate::base::Interface;

use std::ffi::CStr;

type Result<T, E = Error> = std::result::Result<T, E>;

use err_derive::Error;

pub struct MdlExecutionContext {
    pub(crate) ptr: sys::IMdlExecutionContext,
}

impl Drop for MdlExecutionContext {
    fn drop(&mut self) {
        unsafe { sys::IMdl_execution_context_release(self.ptr) };
    }
}

impl Clone for MdlExecutionContext {
    fn clone(&self) -> MdlExecutionContext {
        unsafe {
            sys::IMdl_execution_context_retain(self.ptr);
        }
        MdlExecutionContext { ptr: self.ptr }
    }
}

pub trait Opt {
    fn set_option(&self, name: &str, ctx: &MdlExecutionContext) -> Result<()>;
}

impl Opt for String {
    fn set_option(&self, name: &str, ctx: &MdlExecutionContext) -> Result<()> {
        let cname = std::ffi::CString::new(name).unwrap();
        let cvalue = std::ffi::CString::new(self.as_str()).unwrap();
        let result = unsafe {
            sys::IMdl_execution_context_set_option_string(
                ctx.ptr,
                cname.as_ptr(),
                cvalue.as_ptr(),
            )
        };

        match result {
            sys::SetOptionResult::Success => Ok(()),
            sys::SetOptionResult::InvalidOptionName => {
                Err(Error::InvalidOptionName(name.to_string()))
            }
            sys::SetOptionResult::WrongType => {
                Err(Error::WrongType("String".to_string(), name.to_string()))
            }
        }
    }
}

impl Opt for &str {
    fn set_option(&self, name: &str, ctx: &MdlExecutionContext) -> Result<()> {
        let cname = std::ffi::CString::new(name).unwrap();
        let cvalue = std::ffi::CString::new(*self).unwrap();
        let result = unsafe {
            sys::IMdl_execution_context_set_option_string(
                ctx.ptr,
                cname.as_ptr(),
                cvalue.as_ptr(),
            )
        };

        match result {
            sys::SetOptionResult::Success => Ok(()),
            sys::SetOptionResult::InvalidOptionName => {
                Err(Error::InvalidOptionName(name.to_string()))
            }
            sys::SetOptionResult::WrongType => {
                Err(Error::WrongType("&str".to_string(), name.to_string()))
            }
        }
    }
}

impl Opt for f32 {
    fn set_option(&self, name: &str, ctx: &MdlExecutionContext) -> Result<()> {
        let cname = std::ffi::CString::new(name).unwrap();
        let result = unsafe {
            sys::IMdl_execution_context_set_option_float(
                ctx.ptr,
                cname.as_ptr(),
                *self,
            )
        };

        match result {
            sys::SetOptionResult::Success => Ok(()),
            sys::SetOptionResult::InvalidOptionName => {
                Err(Error::InvalidOptionName(name.to_string()))
            }
            sys::SetOptionResult::WrongType => {
                Err(Error::WrongType("f32".to_string(), name.to_string()))
            }
        }
    }
}

impl Opt for bool {
    fn set_option(&self, name: &str, ctx: &MdlExecutionContext) -> Result<()> {
        let cname = std::ffi::CString::new(name).unwrap();
        let result = unsafe {
            sys::IMdl_execution_context_set_option_bool(
                ctx.ptr,
                cname.as_ptr(),
                *self,
            )
        };

        match result {
            sys::SetOptionResult::Success => Ok(()),
            sys::SetOptionResult::InvalidOptionName => {
                Err(Error::InvalidOptionName(name.to_string()))
            }
            sys::SetOptionResult::WrongType => {
                Err(Error::WrongType("bool".to_string(), name.to_string()))
            }
        }
    }
}

impl MdlExecutionContext {
    pub fn set_option<O: Opt>(&self, name: &str, value: O) -> Result<()> {
        value.set_option(name, self)
    }

    pub fn get_messages_count(&self) -> usize {
        unsafe { sys::IMdl_execution_context_get_messages_count(self.ptr) }
    }

    pub fn get_error_messages_count(&self) -> usize {
        unsafe {
            sys::IMdl_execution_context_get_error_messages_count(self.ptr)
        }
    }

    pub fn get_message(&self, index: usize) -> Option<Message> {
        let ptr =
            unsafe { sys::IMdl_execution_context_get_message(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Message { ptr })
        }
    }

    pub fn get_error_message(&self, index: usize) -> Option<Message> {
        let ptr = unsafe {
            sys::IMdl_execution_context_get_error_message(self.ptr, index)
        };
        if ptr.is_null() {
            None
        } else {
            Some(Message { ptr })
        }
    }

    pub fn messages(&self) -> MessageIterator {
        MessageIterator {
            ctx: self,
            current: 0,
            count: self.get_messages_count(),
        }
    }

    pub fn error_messages(&self) -> ErrorMessageIterator {
        ErrorMessageIterator {
            ctx: self,
            current: 0,
            count: self.get_error_messages_count(),
        }
    }
}

pub struct MessageIterator<'a> {
    ctx: &'a MdlExecutionContext,
    current: usize,
    count: usize,
}

impl<'a> Iterator for MessageIterator<'a> {
    type Item = Message;
    fn next(&mut self) -> Option<Message> {
        if self.current < self.count {
            self.current += 1;
            self.ctx.get_message(self.current - 1)
        } else {
            None
        }
    }
}

pub struct ErrorMessageIterator<'a> {
    ctx: &'a MdlExecutionContext,
    current: usize,
    count: usize,
}

impl<'a> Iterator for ErrorMessageIterator<'a> {
    type Item = Message;
    fn next(&mut self) -> Option<Message> {
        if self.current < self.count {
            self.current += 1;
            self.ctx.get_error_message(self.current - 1)
        } else {
            None
        }
    }
}

impl Interface for MdlExecutionContext {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlExecutionContext {
        MdlExecutionContext {
            ptr: ptr as sys::IMdlExecutionContext,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_execution_context_type_get_iid() }
    }
}

pub struct Message {
    pub(crate) ptr: sys::IMessage,
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { sys::IMessage_release(self.ptr) };
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
        unsafe {
            sys::IMessage_retain(self.ptr);
        }
        Message { ptr: self.ptr }
    }
}

pub use sys::MessageKind;
pub use sys::MessageSeverity;

impl Message {
    pub fn get_kind(&self) -> MessageKind {
        unsafe { sys::IMessage_get_kind(self.ptr) }
    }

    pub fn get_severity(&self) -> MessageSeverity {
        unsafe { sys::IMessage_get_severity(self.ptr) }
    }

    pub fn get_string(&self) -> Option<String> {
        let ptr = unsafe { sys::IMessage_get_string(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn get_notes_count(&self) -> usize {
        unsafe { sys::IMessage_get_notes_count(self.ptr) }
    }

    pub fn get_note(&self, index: usize) -> Option<Message> {
        let ptr = unsafe { sys::IMessage_get_note(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Message { ptr })
        }
    }

    pub fn notes(&self) -> NoteIterator {
        NoteIterator {
            msg: self,
            current: 0,
            count: self.get_notes_count(),
        }
    }
}

pub struct NoteIterator<'a> {
    msg: &'a Message,
    current: usize,
    count: usize,
}

impl<'a> Iterator for NoteIterator<'a> {
    type Item = Message;
    fn next(&mut self) -> Option<Message> {
        if self.current < self.count {
            self.current += 1;
            self.msg.get_note(self.current - 1)
        } else {
            None
        }
    }
}


impl Interface for Message {
    fn from_interface_ptr(ptr: sys::IInterface) -> Message {
        Message {
            ptr: ptr as sys::IMessage,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMessage_type_get_iid() }
    }
}


#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Invalid option name: {}", _0)]
    InvalidOptionName(String),
    #[error(display = "Wrong type '{}' for option '{}'", _0, _1)]
    WrongType(String, String),
}
