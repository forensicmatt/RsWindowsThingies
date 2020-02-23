use crate::errors::WinThingError;
use crate::winevt::EvtHandle;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::um::winevt::*;

pub struct RemoteSession(EvtHandle);

impl RemoteSession {
    pub fn into_handle(self) -> EvtHandle {
        self.0
    }

    pub fn from_prompt_password(
        hostname: &str,
        username: Option<&str>,
        domain: Option<&str>,
        flags: Option<EVT_RPC_LOGIN_FLAGS>,
    ) -> Result<Self, WinThingError> {
        // NULL terminated buffers are needed
        let mut hostname_u16: Vec<u16> = hostname.encode_utf16().collect();
        hostname_u16.resize(hostname_u16.len() + 1, 0);

        let mut username_u16: Vec<u16>;
        let mut domain_u16: Vec<u16>;
        let mut password_u16: Vec<u16>;
        let password: String;

        let flags = match flags {
            Some(f) => f,
            None => EvtRpcLoginAuthNegotiate,
        };

        let evt_rpc_login = if username.is_some() {
            username_u16 = username
                .expect("Expected Username")
                .encode_utf16()
                .collect();
            username_u16.resize(username_u16.len() + 1, 0);

            let domain_ptr = match domain {
                Some(s) => {
                    domain_u16 = s.encode_utf16().collect();
                    domain_u16.resize(domain_u16.len() + 1, 0);
                    domain_u16.as_mut_ptr()
                }
                None => null_mut(),
            };

            password = rpassword::read_password_from_tty(Some("Password: "))?;
            // Does this need to be RPC_UNICODE_STRING like seen here?
            // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/50e9ef83-d6fd-4e22-a34a-2c6b4e3c24f3
            // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/94a16bb6-c610-4cb9-8db6-26f15f560061
            password_u16 = password.encode_utf16().collect();
            password_u16.resize(password_u16.len() + 1, 0);
            // https://github.com/MicrosoftDocs/win32/blob/03b5f241e441e4f60f47b7f1d57b7e8ac3dd72e0/desktop-src/WES/accessing-remote-computers.md

            EVT_RPC_LOGIN {
                Server: hostname_u16.as_mut_ptr(),
                User: username_u16.as_mut_ptr(),
                Domain: domain_ptr,
                Password: password_u16.as_mut_ptr(),
                Flags: flags,
            }
        } else {
            EVT_RPC_LOGIN {
                Server: hostname_u16.as_mut_ptr(),
                User: null_mut(),
                Domain: null_mut(),
                Password: null_mut(),
                Flags: flags,
            }
        };

        let session = evt_open_session(evt_rpc_login)?;

        Ok(Self(session))
    }
}

/// EVT_HANDLE EvtOpenSession(
///   EVT_LOGIN_CLASS LoginClass,
///   PVOID           Login,
///   DWORD           Timeout,
///   DWORD           Flags
/// );
pub fn evt_open_session(mut login: EVT_RPC_LOGIN) -> Result<EvtHandle, WinThingError> {
    let handle = unsafe { EvtOpenSession(EvtRpcLogin, &mut login as *mut _ as *mut c_void, 0, 0) };

    if handle.is_null() {
        return Err(WinThingError::from_windows_last_error());
    }

    Ok(EvtHandle(handle))
}
