use crate::utils::xmltojson::xml_string_to_json;
use serde_json::Value;
use std::sync::mpsc::{channel, Receiver, Sender};

pub enum OutputFormat {
    XmlFormat,
    JsonFormat,
}

pub struct CallbackContext {
    format: OutputFormat,
    tx: Sender<Value>,
}

impl CallbackContext {
    pub fn new(tx: Sender<Value>) -> Self {
        Self {
            format: OutputFormat::JsonFormat,
            tx: tx,
        }
    }

    pub fn with_reciever() -> (Receiver<Value>, Self) {
        let (tx, rx): (Sender<Value>, Receiver<Value>) = channel();
        (rx, CallbackContext::new(tx))
    }

    pub fn with_format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    pub fn handle_record(&self, xml_string: String) {
        let value = match self.format {
            OutputFormat::JsonFormat => match xml_string_to_json(xml_string) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error converting XML string to Value: {:?}", e);
                    return;
                }
            },
            OutputFormat::XmlFormat => Value::String(xml_string),
        };

        println!("{}", value.to_string());

        // Doing anything with self.tx causes app crashes...
        match self.tx.send(value) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("error sending value: {:?}", error);
            }
        }
    }
}
