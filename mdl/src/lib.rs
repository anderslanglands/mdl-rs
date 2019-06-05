pub mod neuray;
pub use neuray::Neuray;
pub mod version;
pub use version::Version;
pub mod mdl_compiler;
pub use mdl_compiler::MdlCompiler;
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
pub mod base;
pub mod itype;
pub use itype::{Type, TypeFactory, TypeList};
pub mod value;
pub use value::{Value, ValueFactory, ValueList};

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

lazy_static! {
    pub static ref NEURAY: Mutex<Neuray> =
        { Mutex::new(Neuray::new().expect("Could not load ineuray interface from DSO")) };
}
