use std::num::ParseFloatError;

use xmlwriter::XmlWriter;

use crate::{traits::XMLString, xml_templates::style::FontStyle};

#[derive(Debug)]
pub struct CellStyle {
    pub font_style: FontStyle,
}

impl Default for CellStyle {
    fn default() -> Self {
        CellStyle {
            font_style: FontStyle::default(),
        }
    }
}

#[derive(Debug)]
pub struct CellAttributes {
    pub reference: Option<String>,
    style_index: Option<String>,
}
#[derive(Debug)]
pub enum CellValue {
    CString(String),
    CNumber(String),
    CFomula(String),
    CBool(bool),
    CInString(String),
    CDate(String),
    Empty,
}

#[derive(Debug)]
pub struct Cell {
    pub value: CellValue,
    formula: Option<String>,
    attributes: CellAttributes,
    cell_style: Option<CellStyle>,
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
                style_index: None,
            },
            cell_style: None,
        }
    }

    pub fn from_number(value: String, reference: String) -> Result<Cell, ParseFloatError> {
        let _ = value.parse::<f64>()?;
        Ok(Cell {
            value: CellValue::CNumber(value),
            formula: None,
            attributes: CellAttributes {
                reference: Some(reference),
                style_index: None,
            },
            cell_style: None,
        })
    }

    pub fn get_attributes(&self) -> &CellAttributes {
        &self.attributes
    }

    pub fn get_style(&mut self) -> &Option<CellStyle> {
        &self.cell_style
    }

    pub fn set_font_style(&mut self, style: FontStyle) {
        self.cell_style = Some(CellStyle { font_style: style });
    }
    pub fn set_style_index(&mut self, index: usize) {
        self.attributes.style_index = Some(index.to_string());
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
        if let Some(style) = self.attributes.style_index {
            writer.write_attribute("s", &style);
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
