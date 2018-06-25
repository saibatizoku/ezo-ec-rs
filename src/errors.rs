//! EC Sensor Errors.
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct EcError {
    inner: Context<ErrorKind>,
}

#[derive(Copy, Clone, Eq, Debug, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "response was not obtainable")]
    I2CRead,
    #[fail(display = "response is not a valid nul-terminated UTF-8 string")]
    MalformedResponse,
    #[fail(display = "could not parse command")]
    CommandParse,
    #[fail(display = "could not parse response")]
    ResponseParse,
    #[fail(display = "response was not yet available")]
    PendingResponse,
    #[fail(display = "the device responded with an error")]
    DeviceErrorResponse,
    #[fail(display = "the device has no data to respond")]
    NoDataExpectedResponse,
}

impl Fail for EcError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for EcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl EcError {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for EcError {
    fn from(kind: ErrorKind) -> EcError {
        EcError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for EcError {
    fn from(inner: Context<ErrorKind>) -> EcError {
        EcError { inner: inner }
    }
}
