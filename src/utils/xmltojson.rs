use crate::errors::WinThingError;
use minidom::element::Attrs;
use minidom::Element;
use serde_json::{Map, Number, Value};
use std::str::FromStr;

/// Transform a XML String into a json Value
///
pub fn xml_string_to_json(xml: String) -> Result<Value, WinThingError> {
    let root = Element::from_str(xml.as_str())?;

    xml_to_value(&root)
}

/// Trasnform a minidom Element into a json Value
///
fn xml_to_value(element: &Element) -> Result<Value, WinThingError> {
    visit_element(&element)
}

/// Get value from a text value
///
fn parse_text_value(text_string: &str) -> Value {
    // ints
    if let Ok(v) = text_string.parse::<u64>() {
        return Value::Number(Number::from(v));
    }

    // floats
    if let Ok(v) = text_string.parse::<f64>() {
        if let Some(val) = Number::from_f64(v) {
            return Value::Number(val);
        }
    }

    // booleans
    if let Ok(v) = text_string.parse::<bool>() {
        return Value::Bool(v);
    }

    Value::String(text_string.into())
}

fn get_data_value(element: &Element) -> Result<Value, WinThingError> {
    if element.children().next().is_some() {
        return Err(WinThingError::xml_error(format!(
            "Unhandled logic - Data node has children. {:#?}",
            element
        )));
    }

    match element.attr("Name") {
        Some(name_value) => Ok(json!({
            name_value: parse_text_value(
                &element.text()[..]
            )
        })),
        None => Ok(json!({
            element.name(): parse_text_value(
                &element.text()[..]
            )
        })),
    }
}

fn get_value_from_attributes(attrs: Attrs) -> Value {
    let mut attributes = json!({});

    for (key, value) in attrs {
        attributes[key] = parse_text_value(value);
    }

    attributes
}

/// Get values from element.
/// An elevent can have a key and key_attributes.
///
fn visit_element(element: &Element) -> Result<Value, WinThingError> {
    let mut element_value = json!({});
    let e_name = element.name();

    // We handle Data elements differently
    if e_name == "Data" {
        let d_value = get_data_value(element)?;
        return Ok(d_value);
    }

    // Add element attributes to the Value
    if element.attrs().count() > 0 {
        // Create the key name
        let a_key = format!("{}_attributes", e_name);
        // Get the attribute Value
        let a_value = get_value_from_attributes(element.attrs());
        element_value[a_key] = a_value;
    }

    if element.text().trim() != "" {
        let e_value = parse_text_value(&element.text()[..]);
        element_value[e_name] = e_value;
    } else {
        let mut children_map = Map::new();
        for child in element.children() {
            let child_value = visit_element(child)?;

            match child_value {
                Value::Object(child_map) => {
                    for (key, value) in child_map {
                        if children_map.contains_key(&key) {
                            if !children_map[&key].is_array() {
                                let orig_value = children_map[&key].to_owned();
                                children_map[&key] = Value::Array(vec![orig_value, value]);
                            } else {
                                let array = children_map[&key]
                                    .as_array_mut()
                                    .expect("Value should be array!");
                                array.push(value);
                            }
                        } else {
                            children_map.insert(key.to_owned(), value.to_owned());
                        }
                    }
                }
                other => {
                    return Err(WinThingError::xml_error(format!(
                        "child_value was expected to be an object! {:?}",
                        other
                    )));
                }
            }
        }

        if children_map.len() > 0 {
            element_value[e_name] = Value::Object(children_map);
        }
    }

    Ok(element_value)
}
