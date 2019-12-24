use crate::utils::xmltojson::xml_string_to_json;


pub enum OutputFormat {
    XmlFormat,
    JsonlFormat
}

pub struct CallbackContext {
    format: OutputFormat
}

impl CallbackContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    pub fn handle_record(&self, xml_string: String) {
        match self.format {
            OutputFormat::JsonlFormat => {
                let value = match xml_string_to_json(xml_string) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Error converting XML string to Value: {:?}", e);
                        return;
                    }
                };

                println!("{}", &value.to_string());
            },
            OutputFormat::XmlFormat => {
                println!("{}", xml_string);
            }
        }
    }
}

impl Default for CallbackContext {
    fn default() -> Self {
        Self {
            format: OutputFormat::JsonlFormat
        }
    }
}