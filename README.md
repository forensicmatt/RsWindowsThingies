[![Build Status](https://dev.azure.com/matthewseyer/dfir/_apis/build/status/forensicmatt.RsWindowsThingies?branchName=master)](https://dev.azure.com/matthewseyer/dfir/_build/latest?definitionId=3&branchName=master)
# RsWindowsThingies
Windows Thingies... but in Rust

# Tools
## listen_events
The event listen tool allows you to see Windows Event Logs in real time.

Note: It takes a minute for the event logs to catch up. I need to implement more of the Windows API to fix this.
When the "Waiting for new events..." message appears, you know it is actively listening.

```
listen_events 0.3.0
Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>

Event listener written in Rust. Output is JSONL.

This tool queries the available list of channels then creates a XPath
query and uses the Windows API to monitor for events on the applicable
channels. Use the print_channels tool to list available channels and
their configurations.

USAGE:
    listen_events.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -p, --historical    List historical records along with listening to new changes.
    -V, --version       Prints version information

OPTIONS:
    -c, --channel <CHANNEL>...    Specific Channel to listen to.
    -d, --debug <DEBUG>           Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
        --domain <DOMAIN>         The domain to which the user account belongs. Optional.
    -f, --format <FORMAT>         Output format to use. [defaults to jsonl] [possible values: xml, jsonl]
        --server <SERVER>         The name of the remote computer to connect to.
        --sflag <SFLAG>           The authentication method to use to authenticate the user when connecting to the
                                  remote computer. [possible values: Default, Negotiate, Kerberos, NTLM]
        --user <USER>             The user name to use to connect to the remote computer.

```

## print_channels
The print channels tool allows to you dump the channels and their configs. This helps to identify what is avaiable 
on your system and the configuration settings. It is mainly an interface for some of the library componets that
are used in helping establish what channels to monior for in the event monitoring tool.

### Usage
```
print_channels 0.2.0
Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>
Print Channel Propperties.

USAGE:
    print_channels.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --debug <DEBUG>      Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
        --domain <DOMAIN>    The domain to which the user account belongs. Optional.
    -f, --format <FORMAT>    Output format. (defaults to text) [possible values: text, jsonl]
        --server <SERVER>    The name of the remote computer to connect to.
        --sflag <SFLAG>      The authentication method to use to authenticate the user when connecting to the remote
                             computer. [possible values: Default, Negotiate, Kerberos, NTLM]
        --user <USER>        The user name to use to connect to the remote computer.

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

## print_publishers
The print publishers tool allows to you dump the publishers and their configs. This helps to identify what is avaiable 
on your system and the configuration settings. It is mainly an interface for some of the library componets that
are used in helping establish what providers exist for monitoring purposes.

### Usage
```
print_publishers 0.1.0
Matthew Seyer <https://github.com/forensicmatt/RsWindowsThingies>
Print Publisher Propperties.

USAGE:
    print_publishers.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --debug <DEBUG>             Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
        --domain <DOMAIN>           The domain to which the user account belongs. Optional.
    -f, --format <FORMAT>           Output format. (defaults to text) [possible values: text, jsonl]
    -p, --provider <PROVIDER>...    Specific Provider.
        --server <SERVER>           The name of the remote computer to connect to.
        --sflag <SFLAG>             The authentication method to use to authenticate the user when connecting to the
                                    remote computer. [possible values: Default, Negotiate, Kerberos, NTLM]
        --user <USER>               The user name to use to connect to the remote computer.
```

### Example
This is an example of what text output looks like. (You can also print out to jsonl)
```
----------------------------------------------
Publisher: Microsoft-Windows-Kernel-Process
GUID: 22FB2CD6-0E7B-422B-A0C7-2FAD1FD0E716
----------------------------------------------
Resource File Path: C:\WINDOWS\system32\Microsoft-Windows-System-Events.dll
Parameter File Path: Null
Message File Path: C:\WINDOWS\system32\Microsoft-Windows-System-Events.dll
Help Link: https://go.microsoft.com/fwlink/events.asp?CoName=Microsoft%20Corporation&ProdName=Microsoft%c2%ae%20Windows%c2%ae%20Operating%20System&ProdVer=10.0.18362.1&FileName=Microsoft-Windows-System-Events.dll&FileVer=10.0.18362.1
Publisher Message: Microsoft-Windows-Kernel-Process
--- Channels ---
0000000000000000: Microsoft-Windows-Kernel-Process/Analytic 
--- Keywords ---
0000000000000010: WINEVENT_KEYWORD_PROCESS 
0000000000000020: WINEVENT_KEYWORD_THREAD 
0000000000000040: WINEVENT_KEYWORD_IMAGE 
0000000000000080: WINEVENT_KEYWORD_CPU_PRIORITY 
0000000000000100: WINEVENT_KEYWORD_OTHER_PRIORITY 
0000000000000200: WINEVENT_KEYWORD_PROCESS_FREEZE 
0000000000000400: WINEVENT_KEYWORD_JOB 
0000000000000800: WINEVENT_KEYWORD_ENABLE_PROCESS_TRACING_CALLBACKS 
0000000000001000: WINEVENT_KEYWORD_JOB_IO 
0000000000002000: WINEVENT_KEYWORD_WORK_ON_BEHALF 
0000000000004000: WINEVENT_KEYWORD_JOB_SILO 
--- Operations ---
0000000000000000: win:Info [Info]
0000000000010000: win:Start [Start]
0000000000020000: win:Stop [Stop]
--- Levels ---
0000000000000004: win:Informational [Information]
--- Tasks ---
0000000000000001: ProcessStart 
0000000000000002: ProcessStop 
0000000000000003: ThreadStart 
0000000000000004: ThreadStop 
0000000000000005: ImageLoad 
0000000000000006: ImageUnload 
0000000000000007: CpuBasePriorityChange 
0000000000000008: CpuPriorityChange 
0000000000000009: PagePriorityChange 
000000000000000A: IoPriorityChange 
000000000000000B: ProcessFreeze 
000000000000000D: JobStart 
000000000000000E: JobTerminate 
000000000000000F: ProcessRundown 
0000000000000010: PsDiskIoAttribution 
0000000000000011: PsIoRateControl 
0000000000000012: ThreadWorkOnBehalfUpdate 
0000000000000013: JobServerSiloStart 
```
