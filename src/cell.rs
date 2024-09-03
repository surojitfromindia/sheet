use std::num::ParseFloatError;

use crate::traits;
use traits::XMLString;
use xmlwriter::*;

pub enum CellValue {
    CString(String),
    CNumber(String),
    Empty,
}

pub struct Cell {
    pub value: CellValue,
    formula: Option<String>,
    pub attributes: CellAttributes,
}

// excel cell attributes
pub struct CellAttributes {
    pub reference: Option<String>,
}
impl Default for CellAttributes {
    fn default() -> Self {
        CellAttributes { reference: None }
    }
}

impl Cell {
    pub fn of_string(value: String) -> Cell {
        Cell {
            value: CellValue::CString(value),
            formula: None,
            attributes: CellAttributes::default(),
        }
    }

    pub fn of_number(value: String) -> Result<Cell, ParseFloatError> {
        let _ = value.parse::<f64>()?;
        Ok(Cell {
            value: CellValue::CNumber(value),
            formula: None,
            attributes: CellAttributes::default(),
        })
    }

    pub fn add_formula(&mut self, formula: String) {
        self.formula = Some(formula);
    }

    pub fn set_reference(&mut self, reference: String) {
        self.attributes.reference = Some(reference);
    }
}

impl XMLString for Cell {
    fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("c");
        writer.write_attribute("reference", &self.attributes.reference.unwrap());
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
            CellValue::Empty => {}
        }
        writer.end_element();
    }
}
