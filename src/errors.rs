use std::string::FromUtf8Error;
use std::string::FromUtf16Error;


#[derive(Debug)]
pub enum ErrorType {
    WinApiError,
    Utf16Error,
    Utf8Error,
    UnhandledVariant
}

#[derive(Debug)]
pub struct WinThingError {
    message: String,
    kind: ErrorType
}

impl WinThingError {
    pub fn unhandled_variant(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorType::UnhandledVariant
        }
    }

    pub fn utf16_error(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorType::Utf16Error
        }
    }

    pub fn winapi_error(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorType::WinApiError
        }
    }
}

impl From<FromUtf8Error> for WinThingError {
    fn from(err: FromUtf8Error) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::Utf8Error,
        }
    }
}

impl From<FromUtf16Error> for WinThingError {
    fn from(err: FromUtf16Error) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::Utf16Error,
        }
    }
}