use std::io::Error as IoError;
use minidom::Error as MinidomError;
use std::string::FromUtf8Error;
use std::string::FromUtf16Error;


#[derive(Debug)]
pub enum ErrorType {
    CliError,
    WinApiError,
    Utf16Error,
    Utf8Error,
    XmlError,
    UnhandledVariant,
    OsError,
    UnhandledLogic
}

#[derive(Debug)]
pub struct WinThingError {
    message: String,
    kind: ErrorType
}

impl WinThingError {
    pub fn cli_error(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorType::CliError
        }
    }

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

    pub fn xml_error(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorType::XmlError
        }
    }

    pub fn unhandled(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorType::UnhandledLogic
        }
    }

    pub fn os_error(error_code: i32) -> Self {
        let error = IoError::from_raw_os_error(
            error_code
        );

        Self {
            message: error.to_string(),
            kind: ErrorType::OsError
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

impl From<MinidomError> for WinThingError {
    fn from(err: MinidomError) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::XmlError,
        }
    }
}
