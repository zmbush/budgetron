use handlebars;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Handlebars(handlebars::RenderError),
    SerdeJson(serde_json::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(e: handlebars::RenderError) -> Error {
        Error::Handlebars(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::SerdeJson(e)
    }
}
