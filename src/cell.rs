use crate::traits;
use traits::XMLString;
use xmlwriter::*;

pub enum CellType {
    CString,
    CNumber,
}

pub struct Cell {
    pub cell_type: CellType,
    pub value: String,
    formula: Option<String>,
    attributes: CellAttributes,
}

// excel cell attributes
struct CellAttributes {
    reference: String,
}

impl Cell {
    pub fn new(value: String, reference: String) -> Cell {
        Cell {
            cell_type: CellType::CString,
            value,
            formula: None,
            attributes: CellAttributes { reference },
        }
    }

    pub fn add_formula(&mut self, formula: String) {
        self.formula = Some(formula);
    }
}

impl XMLString for Cell {
    fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("c");
        writer.write_attribute("reference", &self.attributes.reference);
        if let Some(formula) = self.formula {
            writer.start_element("formula");
            writer.write_text(&formula);
            writer.end_element();
        }
        writer.write_text(&self.value);
        writer.end_element();
    }
}
