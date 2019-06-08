use crate::{base::Interface, expression::ExpressionList, itype::TypeList, value::ValueList};

pub trait Definition {
    fn get_parameter_count(&self) -> usize;
    fn get_parameter_index(&self, name: &str) -> Option<usize>;
    fn get_parameter_name(&self, index: usize) -> Option<String>;
    fn get_parameter_types(&self) -> TypeList;
    fn get_defaults(&self) -> ExpressionList;
}