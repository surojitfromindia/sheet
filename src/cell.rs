use std::num::ParseFloatError;

use xmlwriter::XmlWriter;

use crate::traits::XMLString;

pub struct CellAttributes {
    pub reference: Option<String>,
}

pub enum CellValue {
    CString(String),
    CNumber(String),
    CFomula(String),
    CBool(bool),
    CInString(String),
    CDate(String),
    Empty,
}

pub struct Cell {
    pub value: CellValue,
    formula: Option<String>,
    pub attributes: CellAttributes,
}

impl Cell {
    pub fn from_string(value: String, reference: String, inline: bool) -> Cell {
        let cell_value = if inline {
            CellValue::CInString(value)
        } else {
            CellValue::CString(value)
        };
        Cell {
            value: cell_value,
            formula: None,
            attributes: CellAttributes {
                reference: Some(reference),
            },
        }
    }

    pub fn from_number(value: String, reference: String) -> Result<Cell, ParseFloatError> {
        let _ = value.parse::<f64>()?;
        Ok(Cell {
            value: CellValue::CNumber(value),
            formula: None,
            attributes: CellAttributes {
                reference: Some(reference),
            },
        })
    }
}

impl XMLString for Cell {
    fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("c");
        writer.write_attribute("r", &self.attributes.reference.unwrap());
        if let Some(formula) = self.formula {
            writer.start_element("formula");
            writer.write_text(&formula);
            writer.end_element();
        }
        match self.value {
            CellValue::CString(v) => {
                writer.write_attribute("t", "s");
                writer.start_element("v");
                writer.write_text(&v);
                writer.end_element();
            }
            CellValue::CNumber(v) => {
                writer.write_attribute("t", "n");
                writer.start_element("v");
                writer.write_text(&v);
                writer.end_element();
            }
            CellValue::CFomula(v) => {
                writer.write_attribute("t", "str");
                writer.start_element("v");
                writer.write_text(&v);
                writer.end_element();
            }
            CellValue::CBool(v) => {
                writer.write_attribute("t", "b");
                writer.start_element("v");
                writer.write_text(&v.to_string());
                writer.end_element();
            }

            CellValue::CInString(v) => {
                writer.write_attribute("t", "inlineStr");
                writer.start_element("is");
                writer.start_element("t");
                writer.write_text(&v);
                writer.end_element();
                writer.end_element();
            }
            CellValue::CDate(v) => {
                writer.write_attribute("t", "d");
                writer.start_element("v");
                writer.write_text(&v);
                writer.end_element();
            }

            CellValue::Empty => {}
        }
        writer.end_element();
    }
}
