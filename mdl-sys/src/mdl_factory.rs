use crate::{
    base::Uuid, components::IMdlFactory, expression::IExpressionFactory,
    itype::ITypeFactory, mdl_execution_context::IMdlExecutionContext,
    transaction::ITransaction, value::IValueFactory,

};
use std::os::raw::c_char;

extern "C" {
    pub fn IMdl_factory_release(f: IMdlFactory);
    pub fn IMdl_factory_retain(f: IMdlFactory);
    pub fn IMdl_factory_type_get_iid() -> Uuid;
    pub fn IMdl_factory_create_execution_context(
        f: IMdlFactory,
    ) -> IMdlExecutionContext;
    pub fn IMdl_factory_create_type_factory(
        f: IMdlFactory,
        t: ITransaction,
    ) -> ITypeFactory;
    pub fn IMdl_factory_create_value_factory(
        f: IMdlFactory,
        t: ITransaction,
    ) -> IValueFactory;
    pub fn IMdl_factory_create_expression_factory(
        f: IMdlFactory,
        t: ITransaction,
    ) -> IExpressionFactory;
}
