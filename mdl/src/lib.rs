pub mod neuray;
pub use neuray::Neuray;
pub mod version;
pub use version::Version;

pub use mdl_sys::Uuid;

use lazy_static::lazy_static;
use std::sync::Mutex;

use snafu::{ensure, Snafu};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}", "source"))]
    NeurayError { source: neuray::Error },
}

impl From<neuray::Error> for Error {
    fn from(e: neuray::Error) -> Error {
        Error::NeurayError { source: e }
    }
}

lazy_static! {
    pub static ref NEURAY: Mutex<Neuray> =
        { Mutex::new(Neuray::new().expect("Could not load ineuray interface from DSO")) };
}
