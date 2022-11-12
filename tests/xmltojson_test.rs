#[macro_use]
extern crate serde_json;
use rswinthings::utils::xmltojson::xml_string_to_json;

static EVENT_STR1: &str = r###"
<Event xmlns='http://schemas.microsoft.com/win/2004/08/events/event'>
	<System>
		<Provider Name='Microsoft-Windows-AppXDeployment' Guid='{11111111-1111-1111-1111-111111111111}'/>
		<EventID>12</EventID>
		<Version>0</Version>
		<Level>4</Level>
		<Task>0</Task>
		<Opcode>0</Opcode>
		<Keywords>0x4000000000010000</Keywords>
		<TimeCreated SystemTime='2019-12-17T04:54:30.934121600Z'/>
		<EventRecordID>44444</EventRecordID>
		<Correlation ActivityID='{11111111-1111-1111-1111-111111111111}'/>
		<Execution ProcessID='3333' ThreadID='2222'/>
		<Channel>Microsoft-Windows-AppXDeployment/Operational</Channel>
		<Computer>MSI</Computer>
		<Security UserID='S-1-5-21-666666666-7777777777-777777777-9999'/>
	</System>
	<EventData>
		<Data Name='PackageFamilyName'>Microsoft.Windows.Cortana_xw5n1h2txyewy</Data>
	</EventData>
</Event>
"###;

static EVENT_STR2: &str = r###"
<Event xmlns='http://schemas.microsoft.com/win/2004/08/events/event'>
	<System>
		<Provider Name='PowerShell'/>
		<EventID Qualifiers='0'>600</EventID>
		<Level>4</Level>
		<Task>6</Task>
		<Keywords>0x80000000000000</Keywords>
		<TimeCreated SystemTime='2019-12-22T19:21:41.888384600Z'/>
		<EventRecordID>1124</EventRecordID>
		<Channel>Windows PowerShell</Channel>
		<Computer>MSI</Computer>
		<Security/>
	</System>
	<EventData>
		<Data>Variable</Data>
		<Data>Started</Data>
		<Data>      ProviderName=Variable
        NewProviderState=Started

        SequenceNumber=11

        HostName=ConsoleHost
        HostVersion=5.1.18362.145
        HostId=a60f1524-4af5-45d1-9fac-30c9df6dade7
        HostApplication=C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe
        EngineVersion=
        RunspaceId=
        PipelineId=
        CommandName=
        CommandType=
        ScriptName=
        CommandPath=
        CommandLine=</Data>
	</EventData>
</Event>
"###;

#[test]
fn xml_to_json_test() {
    let json_value1 = xml_string_to_json(EVENT_STR1.to_string()).expect("Error...");

    println!("{}", json_value1.to_string());

    let json_value2 = xml_string_to_json(EVENT_STR2.to_string()).expect("Error...");

    println!("{}", json_value2.to_string());
}

#[test]
fn xml_to_json_test2() {
    let xml_str = r###"<EventID Qualifiers='0'>600</EventID>"###;
    let json_value = json!({
        "EventID": 600,
        "EventID_attributes": {
            "Qualifiers": 0
        }
    });

    let parsed_value = xml_string_to_json(xml_str.to_string()).expect("Error parsing XML");

    println!("{}", parsed_value.to_string());
    assert_eq!(json_value, parsed_value)
}
