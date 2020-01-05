use treediff;
use serde_json::Value;
use serde_json::Value as ValueType;

use treediff::tools::ChangeType::*;
use treediff::tools::Recorder;


pub fn get_difference_value(
    cmp1: &Value, 
    cmp2: &Value
) -> Value {
    let mut recorder = Recorder::default();
    let mut return_val = json!({});

    treediff::diff(
        cmp1, 
        cmp2, 
        &mut recorder
    );

    for call in recorder.calls {
        match call {
            Modified(keys, v1, v2, ) => {
                let mut path_list: Vec<String> = Vec::new();
                for key in keys {
                    path_list.push(key.to_string())
                }
                let path_str = path_list.join(".");
                
                return_val[path_str] = json!({
                    "before": v1.to_owned(),
                    "after": v2.to_owned()
                });
            },
            Added(keys, value) => {
                let mut path_list: Vec<String> = Vec::new();
                for key in keys {
                    path_list.push(key.to_string())
                }
                let path_str = path_list.join(".");

                return_val[path_str] = json!({
                    "created": value.to_owned()
                });
            },
            Removed(keys, value) => {
                let mut path_list: Vec<String> = Vec::new();
                for key in keys {
                    path_list.push(key.to_string())
                }
                let path_str = path_list.join(".");

                return_val[path_str] = json!("removed");
            }
            _ => {}
        }
    }

    return_val
}