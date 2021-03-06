pub mod neuray;
pub use neuray::Neuray;
pub mod version;
pub use version::Version;
pub mod mdl_compiler;
pub use mdl_compiler::{MdlBackend, MdlBackendKind, MdlCompiler};
pub mod mdl_factory;
pub use mdl_factory::MdlFactory;
pub mod database;
pub use database::Database;
pub mod scope;
pub use scope::Scope;
pub mod transaction;
pub use transaction::Transaction;
pub mod mdl_execution_context;
pub use mdl_execution_context::MdlExecutionContext;
pub mod module;
pub use module::Module;
pub mod compiled_material;
pub use compiled_material::CompiledMaterial;
pub mod base;
pub mod itype;
pub use itype::{Type, TypeBase, TypeFactory, TypeList, TypeResource};
pub mod value;
pub use value::{Value, ValueCompound, ValueFactory, ValueKind, ValueList};
pub mod expression;
pub use expression::{
    Expression, ExpressionCall, ExpressionConstant, ExpressionDirectCall,
    ExpressionFactory, ExpressionKind, ExpressionList, ExpressionParameter,
    ExpressionTemporary,
};
pub mod function_definition;
pub use function_definition::FunctionDefinition;
pub mod material_definition;
pub use material_definition::MaterialDefinition;
pub mod material_instance;
pub use material_instance::{CompilationOptions, MaterialInstance};
pub mod definition;
pub use definition::Definition;
pub mod discovery;
pub use discovery::*;
pub mod image_api;
pub use image_api::*;
pub mod definition_wrapper;
pub use definition_wrapper::*;
pub mod scene_element;
pub use scene_element::*;

pub use mdl_sys::Uuid;

use lazy_static::lazy_static;
use std::sync::Mutex;

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "{:?}", source)]
    NeurayError { source: neuray::Error },
    #[error(display = "{:?}", source)]
    MdlCompilerError { source: mdl_compiler::Error },
    #[error(display = "{:?}", source)]
    DatabaseError { source: database::Error },
    #[error(display = "{:?}", source)]
    TransactionError { source: transaction::Error },
}

impl From<neuray::Error> for Error {
    fn from(e: neuray::Error) -> Error {
        Error::NeurayError { source: e }
    }
}

impl From<mdl_compiler::Error> for Error {
    fn from(e: mdl_compiler::Error) -> Error {
        Error::MdlCompilerError { source: e }
    }
}

impl From<database::Error> for Error {
    fn from(e: database::Error) -> Error {
        Error::DatabaseError { source: e }
    }
}

impl From<transaction::Error> for Error {
    fn from(e: transaction::Error) -> Error {
        Error::TransactionError { source: e }
    }
}

lazy_static! {
    pub static ref NEURAY: Mutex<Neuray> = {
        Mutex::new(
            Neuray::new().expect("Could not load ineuray interface from DSO"),
        )
    };
}
