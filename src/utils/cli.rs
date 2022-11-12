use crate::errors::WinThingError;
use crate::winevt::session::RemoteSession;
use clap::{App, Arg, ArgMatches};
use std::process::exit;
use winapi::um::winevt::EVT_RPC_LOGIN_FLAGS;
use winapi::um::winevt::{
    EvtRpcLoginAuthDefault, EvtRpcLoginAuthKerberos, EvtRpcLoginAuthNTLM, EvtRpcLoginAuthNegotiate,
};

fn flag_from_str(flag_str: &str) -> EVT_RPC_LOGIN_FLAGS {
    match flag_str {
        "Default" => EvtRpcLoginAuthDefault,
        "Negotiate" => EvtRpcLoginAuthNegotiate,
        "Kerberos" => EvtRpcLoginAuthKerberos,
        "NTLM" => EvtRpcLoginAuthNTLM,
        other => {
            eprintln!("{} Is not a handled flag for EVT_RPC_LOGIN_FLAGS!", other);
            exit(-1);
        }
    }
}

pub fn add_session_options_to_app<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let server = Arg::with_name("server")
        .long("server")
        .value_name("SERVER")
        .takes_value(true)
        .help("The name of the remote computer to connect to.");

    let user = Arg::with_name("user")
        .long("user")
        .value_name("USER")
        .takes_value(true)
        .help("The user name to use to connect to the remote computer.");

    let domain = Arg::with_name("domain")
        .long("domain")
        .value_name("DOMAIN")
        .takes_value(true)
        .help("The domain to which the user account belongs. Optional.");

    let sflag = Arg::with_name("sflag")
        .long("sflag")
        .value_name("SFLAG")
        .takes_value(true)
        .possible_values(&["Default", "Negotiate", "Kerberos", "NTLM"])
        .help("The authentication method to use to authenticate the user when connecting to the remote computer.");

    app.arg(server).arg(user).arg(domain).arg(sflag)
}

pub fn get_session_from_matches<'n>(
    options: &ArgMatches<'n>,
) -> Result<Option<RemoteSession>, WinThingError> {
    let server = match options.value_of("server") {
        Some(s) => s,
        None => return Ok(None),
    };

    let user = match options.value_of("user") {
        Some(s) => Some(s),
        None => None,
    };

    let domain = match options.value_of("domain") {
        Some(s) => Some(s),
        None => None,
    };

    let flags = match options.value_of("flags") {
        Some(s) => Some(flag_from_str(s)),
        None => None,
    };

    let remote_session = RemoteSession::from_prompt_password(server, user, domain, flags)?;

    Ok(Some(remote_session))
}
