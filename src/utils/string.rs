use std::slice;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::shared::{
    guiddef::GUID,
    ntdef::PWSTR
};
use widestring::WideCString;


pub fn guid_to_string(guid: GUID) -> String {
    format!(
        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        guid.Data1,
        guid.Data2,
        guid.Data3,
        guid.Data4[0],
        guid.Data4[1],
        guid.Data4[2],
        guid.Data4[3],
        guid.Data4[4],
        guid.Data4[5],
        guid.Data4[6],
        guid.Data4[7]
    )
}


pub unsafe fn read_wstring_array(
    p: *const u8, 
    offset: isize
) -> Vec<WideCString> {
    assert!(!p.is_null());
    let mut offset: isize = offset;
    let mut values: Vec<WideCString> = Vec::new();

    // while *p.wrapping_offset(i) as u16 != 0 {
    //     println!("*p.wrapping_offset({}): {}", i, *p.wrapping_offset(i) as u16);
    //     i = i + 2;
    // }

    loop {
        if *p.wrapping_offset(offset) as u16 == 0 {
            break;
        }

        let wide_string = WideCString::from_ptr_str(
            p.wrapping_offset(offset) as *const u16
        );
        offset += (wide_string.len() * 2) as isize + 1;

        values.push(wide_string);
    }

    values
}


pub unsafe fn read_wstring_from_pointer(
    p: *const u8, 
    offset: isize
) -> WideCString {
    assert!(!p.is_null());
    WideCString::from_ptr_str(
        p.wrapping_offset(offset) as *const u16
    )
}