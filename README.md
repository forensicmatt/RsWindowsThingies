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

### Usage
```
print_channels 0.1.0
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

### Example
This is an example of what text output looks like. (You can also print out to jsonl)
```
========================================================
Channel: Windows PowerShell
========================================================
EvtChannelConfigAccess: "O:BAG:SYD:(A;;0x2;;;S-1-15-2-1)(A;;0x2;;;S-1-15-3-1024-3153509613-960666767-3724611135-2725662640-12138253-543910227-1950414635-4190290187)(A;;0xf0007;;;SY)(A;;0x7;;;BA)(A;;0x7;;;SO)(A;;0x3;;;IU)(A;;0x3;;;SU)(A;;0x3;;;S-1-5-3)(A;;0x3;;;S-1-5-33)(A;;0x1;;;S-1-5-32-573)"
EvtChannelConfigClassicEventlog: true
EvtChannelConfigEnabled: true
EvtChannelConfigIsolation: 0
EvtChannelConfigOwningPublisher: ""
EvtChannelConfigType: 0
EvtChannelLoggingConfigAutoBackup: false
EvtChannelLoggingConfigLogFilePath: "%SystemRoot%\\System32\\Winevt\\Logs\\Windows PowerShell.evtx"
EvtChannelLoggingConfigMaxSize: 15728640
EvtChannelLoggingConfigRetention: false
EvtChannelPublishingConfigBufferSize: 64
EvtChannelPublishingConfigClockType: 0
EvtChannelPublishingConfigControlGuid: null
EvtChannelPublishingConfigFileMax: 1
EvtChannelPublishingConfigKeywords: null
EvtChannelPublishingConfigLatency: 1000
EvtChannelPublishingConfigLevel: null
EvtChannelPublishingConfigMaxBuffers: 64
EvtChannelPublishingConfigMinBuffers: 0
EvtChannelPublishingConfigSidType: 1
```
