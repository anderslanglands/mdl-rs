use crate::components::IMdlFactory;
use crate::itype::ITypeFactory;
use crate::mdl_execution_context::IMdlExecutionContext;
use crate::transaction::ITransaction;
use crate::value::IValueFactory;
use std::os::raw::c_char;

extern "C" {
    pub fn IMdl_factory_release(f: IMdlFactory);
    pub fn IMdl_factory_retain(f: IMdlFactory);
    pub fn IMdl_factory_create_execution_context(f: IMdlFactory) -> IMdlExecutionContext;
    pub fn IMdl_factory_create_type_factory(f: IMdlFactory, t: ITransaction) -> ITypeFactory;
    pub fn IMdl_factory_create_value_factory(f: IMdlFactory, t: ITransaction) -> IValueFactory;
}
