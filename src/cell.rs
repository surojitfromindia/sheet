use xmlwriter::*;
use crate::traits;
use traits::XMLString;
pub struct Cell {
    value: String,
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
            value,
            formula: None,
            attributes: CellAttributes { reference },
        }
    }

    pub fn add_formula(&mut self, formula: String) {
        self.formula = Some(formula);
    }

    
}


impl  XMLString for Cell {
    fn to_xml(self, writer: &mut XmlWriter)  {
        writer.start_element("cell");
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