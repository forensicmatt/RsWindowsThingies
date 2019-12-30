use std::ptr::null_mut;
use winapi::um::winevt::*;
use winapi::ctypes::c_void;
use crate::winevt::EvtHandle;
use crate::errors::WinThingError;


pub struct RemoteSession(
    pub EvtHandle
);
impl RemoteSession {
    pub fn from_prompt_password(
        hostname: &str, 
        username: Option<&str>, 
        domain: Option<&str>,
        flags: Option<EVT_RPC_LOGIN_FLAGS>
    ) -> Result<Self, WinThingError> {
        let mut hostname_u16: Vec<u16> = hostname.encode_utf16().collect();
        let mut username_u16: Vec<u16>;
        let mut domain_u16: Vec<u16>;
        let mut password_u16: Vec<u16>;
        let mut password: String;

        let flags = match flags {
            Some(f) => f,
            None => EvtRpcLoginAuthNegotiate
        };

        let evt_rpc_login = if username.is_some(){
            username_u16 = username.expect("Expected Username").encode_utf16().collect();

            let domain_ptr = match domain {
                Some(s) => {
                    domain_u16 = s.encode_utf16().collect();
                    domain_u16.as_mut_ptr()
                },
                None => null_mut()
            };

            password = rpassword::read_password_from_tty(
                Some("Password: ")
            )?;
            password_u16 = password.encode_utf16().collect();
            //password_u16.resize(password_u16.len() + 1, 0);

            EVT_RPC_LOGIN {
                Server: hostname_u16.as_mut_ptr(),
                User: username_u16.as_mut_ptr(),
                Domain: domain_ptr,
                Password: password_u16.as_mut_ptr(),
                Flags: flags
            }
        } 
        else {
            EVT_RPC_LOGIN {
                Server: hostname_u16.as_mut_ptr(),
                User: null_mut(),
                Domain: null_mut(),
                Password: null_mut(),
                Flags: flags
            }
        };

        let session = evt_open_session(
            evt_rpc_login
        )?;

        Ok(Self(session))
    }
}


/// EVT_HANDLE EvtOpenSession(
///   EVT_LOGIN_CLASS LoginClass,
///   PVOID           Login,
///   DWORD           Timeout,
///   DWORD           Flags
/// );
pub fn evt_open_session(
    mut login: EVT_RPC_LOGIN
) -> Result<EvtHandle, WinThingError> {
    let handle = unsafe {
        EvtOpenSession(
            EvtRpcLogin, 
            &mut login as *mut _ as *mut c_void, 
            0, 
            0
        )
    };

    if handle.is_null() {
        return Err(
            WinThingError::from_windows_last_error()
        );
    }

    Ok(EvtHandle(handle))
}