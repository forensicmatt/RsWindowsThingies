use log::LevelFilter;
use crate::errors::WinThingError;


/// Set the debug level based off of a string. Must be valid
/// level or WinThingError.
pub fn set_debug_level(level: &str) -> Result<(), WinThingError> {
    let level_filter = match level {
        "Off" => LevelFilter::Off,
        "Error" => LevelFilter::Error,
        "Warn" => LevelFilter::Warn,
        "Info" => LevelFilter::Info,
        "Debug" => LevelFilter::Debug,
        "Trace" => LevelFilter::Trace,
        unknown => {
            return Err(
                WinThingError::cli_error(
                    format!("Unknown debug level [{}]", unknown)
                )
            );
        }
    };

    // Create logging with debug level that prints to stderr
    let result = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level_filter)
        .chain(std::io::stderr())
        .apply();
    
    // Ensure that logger was dispatched
    match result {
        Ok(_) => {
            trace!("Logging as been initialized!");
        },
        Err(error) => {
            return Err(
                WinThingError::cli_error(
                    format!(
                        "Error initializing fern logging: {}", 
                        error
                    )
                )
            );
        }
    }
    
    Ok(())
}