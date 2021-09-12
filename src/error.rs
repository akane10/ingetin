use notify_rust::error::Error as NotifyError;
use serde_json::Error as SerdeJsonError;
use std::fmt;
use std::io::Error as IOError;
use std::num::ParseIntError;
use uuid::Error as UuidError;

#[derive(Debug)]
pub enum Error {
    NotifyErr(NotifyError),
    IOErr(IOError),
    SerdejsonErr(SerdeJsonError),
    UuidErr(UuidError),
    ParseIntErr(ParseIntError),
    // Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::NotifyErr(ref x) => write!(f, "{}", x),
            Error::IOErr(ref x) => write!(f, "{}", x),
            Error::SerdejsonErr(ref x) => write!(f, "{}", x),
            Error::UuidErr(ref x) => write!(f, "{}", x),
            Error::ParseIntErr(ref x) => write!(f, "{}", x),
            // Error::Other(ref x) => write!(f, "{}", x),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error_wrap {
    ($f:ty, $e:expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

error_wrap!(NotifyError, Error::NotifyErr);
error_wrap!(IOError, Error::IOErr);
error_wrap!(SerdeJsonError, Error::SerdejsonErr);
error_wrap!(UuidError, Error::UuidErr);
error_wrap!(ParseIntError, Error::ParseIntErr);
