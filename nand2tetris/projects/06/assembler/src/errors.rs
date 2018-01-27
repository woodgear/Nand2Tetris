use std::io;
use std::convert;
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "parser cmd fail : {}", _0)] ParseCmdFailError(String),
    #[fail(display = "I/O error")] Io(#[cause] ::std::io::Error),
}

impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
