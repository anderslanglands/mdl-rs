use crate::results::SetOptionResult;
use std::os::raw::c_char;

use crate::base::{MessageSeverity, Uuid};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlExecutionContext_api {
    _unused: [u8; 0],
}
pub type IMdlExecutionContext = *mut IMdlExecutionContext_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMessage_api {
    _unused: [u8; 0],
}
pub type IMessage = *mut IMessage_api;

#[repr(u32)]
pub enum MessageKind {
    CompilerCore,
    CompilerBacked,
    CompilerDag,
    CompilerArchiveTool,
    ImpExp,
    Integration,
    Uncategorized,
}

extern "C" {
    pub fn IMdl_execution_context_release(c: IMdlExecutionContext);
    pub fn IMdl_execution_context_retain(c: IMdlExecutionContext);
    pub fn IMdl_execution_context_compare_iid(id: Uuid);
    pub fn IMdl_execution_context_type_get_iid() -> Uuid;

    pub fn IMdl_execution_context_set_option_string(
        c: IMdlExecutionContext,
        name: *const c_char,
        value: *const c_char,
    ) -> SetOptionResult;

    pub fn IMdl_execution_context_set_option_float(
        c: IMdlExecutionContext,
        name: *const c_char,
        value: f32,
    ) -> SetOptionResult;

    pub fn IMdl_execution_context_set_option_bool(
        c: IMdlExecutionContext,
        name: *const c_char,
        value: bool,
    ) -> SetOptionResult;

    pub fn IMdl_execution_context_get_messages_count(
        c: IMdlExecutionContext,
    ) -> usize;
    pub fn IMdl_execution_context_get_error_messages_count(
        c: IMdlExecutionContext,
    ) -> usize;

    pub fn IMdl_execution_context_get_message(
        c: IMdlExecutionContext,
        index: usize,
    ) -> IMessage;
    pub fn IMdl_execution_context_get_error_message(
        c: IMdlExecutionContext,
        index: usize,
    ) -> IMessage;

}

extern "C" {
    pub fn IMessage_release(c: IMessage);
    pub fn IMessage_retain(c: IMessage);
    pub fn IMessage_compare_iid(id: Uuid);
    pub fn IMessage_type_get_iid() -> Uuid;

    pub fn IMessage_get_kind(m: IMessage) -> MessageKind;
    pub fn IMessage_get_severity(m: IMessage) -> MessageSeverity;
    pub fn IMessage_get_string(m: IMessage) -> *const c_char;
    pub fn IMessage_get_notes_count(m: IMessage) -> usize;
    pub fn IMessage_get_note(m: IMessage, index: usize) -> IMessage;
}
