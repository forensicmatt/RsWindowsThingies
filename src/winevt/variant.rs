use hex;
use std::fmt;
use serde::Serialize;
use serde_json::Value;
use std::ffi::OsString;
use serde_json::Number;
use std::os::windows::prelude::*;
use winapi::shared::guiddef::GUID;
use winapi::um::winevt::*;
use crate::errors::WinThingError;


#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum VariantValue {
    Null,
    String(String),
    UInt(u64),
    Int(i64),
    Single(f32),
    Double(f64),
    Boolean(bool),
    Binary(Vec<u8>),
}
impl VariantValue {
    pub fn to_string(&self) -> String {
        match self {
            VariantValue::Null => "Null".to_string(),
            VariantValue::String(s) => s.clone(),
            VariantValue::UInt(v) => v.to_string(),
            VariantValue::Int(v) => v.to_string(),
            VariantValue::Single(v) => v.to_string(),
            VariantValue::Double(v) => v.to_string(),
            VariantValue::Boolean(v) => v.to_string(),
            VariantValue::Binary(v) => hex::encode(&v)
        }
    }

    pub fn to_json_value(&self) -> Value {
        match self {
            VariantValue::Null => Value::Null,
            VariantValue::String(s) => Value::String(s.clone()),
            VariantValue::UInt(v) => Value::Number(
                Number::from(*v)
            ),
            VariantValue::Int(v) => Value::Number(
                Number::from(*v)
            ),
            VariantValue::Single(v) => {
                match Number::from_f64(*v as f64) {
                    Some(n) => Value::Number(n),
                    None => {
                        eprintln!("Could not convert VariantValue::Single {} to Number from_f64.", v);
                        Value::Null
                    } 
                }
            },
            VariantValue::Double(v) => {
                match Number::from_f64(*v) {
                    Some(n) => Value::Number(n),
                    None => {
                        eprintln!("Could not convert VariantValue::Double {} to Number from_f64.", v);
                        Value::Null
                    } 
                }
            },
            VariantValue::Boolean(v) => Value::Bool(v.clone()),
            VariantValue::Binary(v) => Value::String(
                hex::encode(&v)
            )
        }
    }

    pub fn from_variant(variant: &EVT_VARIANT) -> Result<Self, WinThingError> {
        #[allow(non_upper_case_globals)]
        let value = match variant.Type {
            EvtVarTypeNull => VariantValue::Null,
            EvtVarTypeString => {
                let slice: &[u16];
                unsafe {
                    let ptr = variant.u.StringVal();
                    let len = (0..).take_while(
                        |&i| *ptr.offset(i) != 0
                    ).count();
                    slice = std::slice::from_raw_parts(
                        *ptr, 
                        len
                    );
                }

                let string = OsString::from_wide(
                    &slice[..]
                ).to_string_lossy().to_string();

                VariantValue::String(string)
            },
            EvtVarTypeAnsiString => {
                let slice : &[u8];
                unsafe {
                    let ptr = variant.u.AnsiStringVal();
                    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
                    slice = std::slice::from_raw_parts(*ptr as *const u8, len);
                }
                VariantValue::String(
                    String::from_utf8(
                        slice.to_vec()
                    )?
                )
            },
            EvtVarTypeSByte => {
                let val: &i8 = unsafe {
                    variant.u.SByteVal()
                };
                VariantValue::Int(
                    *val as i64
                )
            },
            EvtVarTypeByte => {
                let val: &u8 = unsafe {
                    variant.u.ByteVal()
                };
                VariantValue::UInt(
                    *val as u64
                )
            },
            EvtVarTypeInt16 => {
                let val: &i16 = unsafe {
                    variant.u.Int16Val()
                };
                VariantValue::Int(
                    *val as i64
                )
            },
            EvtVarTypeUInt16 => {
                let val: &u16 = unsafe {
                    variant.u.UInt16Val()
                };
                VariantValue::UInt(
                    *val as u64
                )
            },
            EvtVarTypeInt32 => {
                let val: &i32 = unsafe {
                    variant.u.Int32Val()
                };
                VariantValue::Int(
                    *val as i64
                )
            },
            EvtVarTypeUInt32 => {
                let val: &u32 = unsafe {
                    variant.u.UInt32Val()
                };
                VariantValue::UInt(
                    *val as u64
                )
            },
            EvtVarTypeInt64 => {
                let val: &i64 = unsafe {
                    variant.u.Int64Val()
                };
                VariantValue::Int(
                    *val as i64
                )
            },
            EvtVarTypeUInt64 => {
                let val: &u64 = unsafe {
                    variant.u.UInt64Val()
                };
                VariantValue::UInt(
                    *val as u64
                )
            },
            EvtVarTypeSingle => {
                let val: &f32 = unsafe {
                    variant.u.SingleVal()
                };
                VariantValue::Single(
                    *val
                )
            },
            EvtVarTypeDouble => {
                let val: &f64 = unsafe {
                    variant.u.DoubleVal()
                };
                VariantValue::Double(
                    *val
                )
            },
            EvtVarTypeBoolean => {
                let val: &i32 = unsafe {
                    variant.u.BooleanVal()
                };
                VariantValue::Boolean(
                    *val != 0
                )
            },
            EvtVarTypeBinary => {
                let val: u8 = unsafe {
                    **variant.u.BinaryVal()
                };
                VariantValue::String(
                    format!("{:02X}", val)
                )
            },
            EvtVarTypeGuid => {
                let val: GUID = unsafe {
                    std::ptr::read(
                        *variant.u.GuidVal()
                    )
                };
                VariantValue::String(
                    format!(
                        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                        val.Data1, val.Data2, val.Data3, 
                        val.Data4[0], val.Data4[1], val.Data4[2],
                        val.Data4[3], val.Data4[4], val.Data4[5], 
                        val.Data4[6], val.Data4[7]
                    )
                )
            },
            EvtVarTypeHexInt64 => {
                let val: u64 = unsafe {
                    std::ptr::read(
                        &variant.u as *const _ as *const u64
                    )
                };
                VariantValue::UInt(val)
            },
            EvtVarTypeHexInt32 => {
                let val: u32 = unsafe {
                    std::ptr::read(
                        &variant.u as *const _ as *const u32
                    )
                };
                VariantValue::UInt(
                    val as u64
                )
            }
            unknown => {
                let message = format!(
                    "Unhandled EVT_VARIANT type {} (count {}) (contents {})",
                    unknown, 
                    variant.Count, 
                    unsafe {
                        std::ptr::read(
                            &variant.u as *const _ as *const u64
                        )
                    }
                );

                return Err(
                    WinThingError::unhandled_variant(
                        message
                    )
                )
            },
        };

        Ok(value)
    }
}
impl fmt::Display for VariantValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


pub struct EvtVariant(
    pub EVT_VARIANT
);
impl EvtVariant {
    pub fn get_variant_value(&self) -> Result<VariantValue, WinThingError> {
        VariantValue::from_variant(
            &self.0
        )
    }

    pub fn get_json_value(&self) -> Result<Value, WinThingError> {
        Ok(
            self.get_variant_value()?.to_json_value()
        )
    }
}

impl fmt::Display for EvtVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.get_variant_value() {
            Ok(v) => write!(f, "{}", v),
            Err(e) => {
                eprintln!("Error formating EvtVariant {:?}", e);
                write!(f, "{:?}", e)
            }
        }
    }
}