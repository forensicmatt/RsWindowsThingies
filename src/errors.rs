use std::ptr;
use mft::err::Error as MftError;
use std::io::Error as IoError;
use minidom::Error as MinidomError;
use std::string::FromUtf8Error;
use std::string::FromUtf16Error;
use serde_json::Error as SerdeJsonError;
use winapi::shared::ntdef::WCHAR;
use winapi::um::winbase::{
    FormatMessageW, 
    FORMAT_MESSAGE_FROM_SYSTEM, 
    FORMAT_MESSAGE_IGNORE_INSERTS,
};
use winapi::um::errhandlingapi::GetLastError;


#[derive(Debug)]
pub enum ErrorType {
    CliError,
    WinApiError,
    Utf16Error,
    Utf8Error,
    XmlError,
    UnhandledVariant,
    OsError,
    UnhandledLogic,
    WindowsError,
    SerdeJsonError,
    IoError,
    MftError,
    InvalidUsnJournalData
}

#[derive(Debug)]
pub struct WinThingError {
    pub message: String,
    pub kind: ErrorType
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

    pub fn from_windows_error_code(err_code: u32) -> Self {
        let err_str = format_win_error(
            Some(err_code)
        );

        Self {
            message: err_str,
            kind: ErrorType::WindowsError
        }
    }

    pub fn from_windows_last_error() -> Self{
        let err_str = format_win_error(None);
        Self {
            message: err_str,
            kind: ErrorType::WindowsError
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

    pub fn invalid_usn_journal_data(size: usize) -> Self {
        let err_str = format!("Unknown size for UsnJournalData structure: {}", size);

        Self {
            message: err_str,
            kind: ErrorType::InvalidUsnJournalData
        }
    }
}

impl From<IoError> for WinThingError {
    fn from(err: IoError) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::IoError,
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

impl From<SerdeJsonError> for WinThingError {
    fn from(err: SerdeJsonError) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::SerdeJsonError,
        }
    }
}

impl From<MftError> for WinThingError {
    fn from(err: MftError) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::MftError,
        }
    }
}


pub fn format_win_error(error_code: Option<u32>) -> String {
    let mut message_buffer = [0 as WCHAR; 2048];
    let error_num: u32 = match error_code {
        Some(code) => code,
        None => unsafe { GetLastError() }
    };

    let message_size = unsafe { 
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            error_num,
            0,
            message_buffer.as_mut_ptr(),
            message_buffer.len() as u32,
            ptr::null_mut(),
        )
    };

    if message_size == 0 {
        return format_win_error(None);
    } else {
        let err_msg = String::from_utf16(
            &message_buffer[..message_size as usize]
        ).unwrap();
        return err_msg;
    }
}