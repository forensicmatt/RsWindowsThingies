#[macro_use] extern crate serde_json;
use rswinthings::utils::json::get_difference_value;


#[test]
fn json_test() {
    let json_value1 = json!({
        "attributes": [{
                "data": {
                    "accessed": "2020-01-05T19:12:13.854919Z",
                    "class_id": 0,
                    "created": "2019-12-24T05:20:45.777951Z",
                    "file_flags": "FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_COMPRESSED",
                    "max_version": 0,
                    "mft_modified": "2020-01-05T19:14:33.933359Z",
                    "modified": "2020-01-05T19:10:58.349669Z",
                    "owner_id": 0,
                    "quota": 0,
                    "security_id": 4076,
                    "usn": 57216,
                    "version": 0
                },
                "header": {
                    "data_flags": "(empty)",
                    "form_code": 0,
                    "instance": 0,
                    "name": "",
                    "name_offset": null,
                    "name_size": 0,
                    "record_length": 96,
                    "residential_header": {
                        "index_flag": 0
                    },
                    "type_code": "StandardInformation"
                }
            }, {
                "data": {
                    "accessed": "2019-12-24T05:20:45.777951Z",
                    "created": "2019-12-24T05:20:45.777951Z",
                    "flags": "FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_COMPRESSED",
                    "logical_size": 0,
                    "mft_modified": "2019-12-24T05:20:45.777951Z",
                    "modified": "2019-12-24T05:20:45.777951Z",
                    "name": "channels.txt",
                    "name_length": 12,
                    "namespace": "Win32AndDos",
                    "parent": {
                        "entry": 131252,
                        "sequence": 203
                    },
                    "physical_size": 0,
                    "reparse_value": 0
                },
                "header": {
                    "data_flags": "(empty)",
                    "form_code": 0,
                    "instance": 2,
                    "name": "",
                    "name_offset": null,
                    "name_size": 0,
                    "record_length": 120,
                    "residential_header": {
                        "index_flag": 1
                    },
                    "type_code": "FileName"
                }
            }, {
                "data": null,
                "header": {
                    "data_flags": "IS_COMPRESSED",
                    "form_code": 1,
                    "instance": 3,
                    "name": "",
                    "name_offset": null,
                    "name_size": 0,
                    "record_length": 200,
                    "residential_header": {
                        "allocated_length": 1572864,
                        "file_size": 1525662,
                        "total_allocated": 475136,
                        "unit_compression_size": 4,
                        "valid_data_length": 1525662,
                        "vnc_first": 0,
                        "vnc_last": 383
                    },
                    "type_code": "DATA"
                }
            }
        ],
        "header": {
            "base_reference": {
                "entry": 0,
                "sequence": 0
            },
            "first_attribute_id": 4,
            "first_attribute_record_offset": 56,
            "flags": "ALLOCATED",
            "hard_link_count": 1,
            "metadata_transaction_journal": 35690,
            "record_number": 6090,
            "sequence": 71,
            "signature": [70, 73, 76, 69],
            "total_entry_size": 1024,
            "usa_offset": 48,
            "usa_size": 3,
            "used_entry_size": 480
        }
    });

    let json_value2 = json!({
        "attributes": [{
                "data": {
                    "accessed": "2020-01-05T19:14:34.350555Z",
                    "class_id": 0,
                    "created": "2019-12-24T05:20:45.777951Z",
                    "file_flags": "FILE_ATTRIBUTE_READONLY | FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_COMPRESSED",
                    "max_version": 0,
                    "mft_modified": "2020-01-05T19:38:34.618693Z",
                    "modified": "2020-01-05T19:10:58.349669Z",
                    "owner_id": 0,
                    "quota": 0,
                    "security_id": 4076,
                    "usn": 84272,
                    "version": 0
                },
                "header": {
                    "data_flags": "(empty)",
                    "form_code": 0,
                    "instance": 0,
                    "name": "",
                    "name_offset": null,
                    "name_size": 0,
                    "record_length": 96,
                    "residential_header": {
                        "index_flag": 0
                    },
                    "type_code": "StandardInformation"
                }
            }, {
                "data": {
                    "accessed": "2019-12-24T05:20:45.777951Z",
                    "created": "2019-12-24T05:20:45.777951Z",
                    "flags": "FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_COMPRESSED",
                    "logical_size": 0,
                    "mft_modified": "2019-12-24T05:20:45.777951Z",
                    "modified": "2019-12-24T05:20:45.777951Z",
                    "name": "channels.txt",
                    "name_length": 12,
                    "namespace": "Win32AndDos",
                    "parent": {
                        "entry": 131252,
                        "sequence": 203
                    },
                    "physical_size": 0,
                    "reparse_value": 0
                },
                "header": {
                    "data_flags": "(empty)",
                    "form_code": 0,
                    "instance": 2,
                    "name": "",
                    "name_offset": null,
                    "name_size": 0,
                    "record_length": 120,
                    "residential_header": {
                        "index_flag": 1
                    },
                    "type_code": "FileName"
                }
            }, {
                "data": null,
                "header": {
                    "data_flags": "IS_COMPRESSED",
                    "form_code": 1,
                    "instance": 3,
                    "name": "",
                    "name_offset": null,
                    "name_size": 0,
                    "record_length": 200,
                    "residential_header": {
                        "allocated_length": 1572864,
                        "file_size": 1525662,
                        "total_allocated": 475136,
                        "unit_compression_size": 4,
                        "valid_data_length": 1525662,
                        "vnc_first": 0,
                        "vnc_last": 383
                    },
                    "type_code": "DATA"
                }
            }
        ],
        "header": {
            "base_reference": {
                "entry": 0,
                "sequence": 0
            },
            "first_attribute_id": 4,
            "first_attribute_record_offset": 56,
            "flags": "ALLOCATED",
            "hard_link_count": 1,
            "metadata_transaction_journal": 111907,
            "record_number": 6090,
            "sequence": 71,
            "signature": [70, 73, 76, 69],
            "total_entry_size": 1024,
            "usa_offset": 48,
            "usa_size": 3,
            "used_entry_size": 480
        }
    });

    let new_value = get_difference_value(
        &json_value1, 
        &json_value2
    );

    println!("{}", new_value.to_string());
}

