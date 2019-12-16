[![Build Status](https://dev.azure.com/matthewseyer/dfir/_apis/build/status/forensicmatt.RsWindowsThingies?branchName=master)](https://dev.azure.com/matthewseyer/dfir/_build/latest?definitionId=3&branchName=master)
# RsWindowsThingies
Windows Thingies... but in Rust

# Tools
## event_listen
The event listen tool allows you to see Windows Event Logs in real time.

```
listen_events 0.0.1
Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>

Event listener written in Rust. Output is JSONL.

This tool queries the available list of channels then creates a XPath
query and uses the Windows API to monitor for events on the applicable
channels. Currently, all classic eventlog channels are selected for
monitoring. Use the print_channels tool to list available channels and
their configurations.

USAGE:
    listen_events.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --debug <DEBUG>    Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
```

## print_channels
The print channels tool allows to you dump the channels and their configs. This helps to identify what is avaiable 
on your system and the configuration settings. It is mainly an interface for some of the library componets that
are used in helping establish what channels to monior for in the event monitoring tool.
```
print_channels 0.0.1
Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>
Print Channel Propperties.

USAGE:
    print_channels.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --debug <DEBUG>      Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
    -f, --format <FORMAT>    Output format. (defaults to text) [possible values: text, jsonl]
```