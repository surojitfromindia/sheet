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
    attributes: CellAttributes,
}

pub struct Row {
    cells: Vec<Cell>,
    row_number: usize,
    col_reference: char,
}

impl Row {
    pub fn new(row_number: usize) -> Row {
        Row {
            row_number,
            col_reference: 'A',
            cells: Vec::new(),
        }
    }

    pub fn add_cell_of_string(&mut self, value: String) -> &mut Cell {
        let mut cell = Cell::of_string(value);
        let cell_reference = self.get_next_cell_ref();
        cell.set_reference(cell_reference);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn add_cell_of_number(&mut self, value: String) -> Result<&mut Cell, ParseFloatError> {
        let mut cell = Cell::of_number(value)?;
        let cell_reference = self.get_next_cell_ref();
        cell.set_reference(cell_reference);
        self.cells.push(cell);
        Ok(self.cells.last_mut().unwrap())
    }

    pub fn add_cell(&mut self, mut cell: Cell) -> &mut Cell {
        // check if the reference is present
        // if not, set the reference
        if cell.attributes.reference.is_none() {
            let cell_reference = self.get_next_cell_ref();
            cell.set_reference(cell_reference);
        }

        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    fn get_next_cell_ref(&mut self) -> String {
        let col_reference = self.col_reference;
        self.col_reference = (self.col_reference as u8 + 1) as char;
        format!("{}{}", col_reference, self.row_number)
    }
}

impl XMLString for Row {
    fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("row");
        writer.write_attribute("r", &self.row_number.to_string());
        for cell in self.cells {
            cell.to_xml(writer);
        }
        writer.end_element();
    }
}

// excel cell attributes
struct CellAttributes {
    reference: Option<String>,
}
impl Default for CellAttributes {
    fn default() -> Self {
        CellAttributes { reference: None }
    }
}

impl Cell {
    // pub fn of_string_with_reference(value: String, reference: String) -> Cell {
    //     let mut cell = Cell::of_string(value);
    //     cell.set_reference(reference);
    //     cell
    // }

    // pub fn of_number_with_reference(
    //     value: String,
    //     reference: String,
    // ) -> Result<Cell, ParseFloatError> {
    //     let mut cell = Cell::of_number(value)?;
    //     cell.set_reference(reference);
    //     Ok(cell)
    // }

    fn of_string(value: String) -> Cell {
        Cell {
            value: CellValue::CString(value),
            formula: None,
            attributes: CellAttributes::default(),
        }
    }

    fn of_number(value: String) -> Result<Cell, ParseFloatError> {
        let _ = value.parse::<f64>()?;
        Ok(Cell {
            value: CellValue::CNumber(value),
            formula: None,
            attributes: CellAttributes::default(),
        })
    }

    pub fn set_reference(&mut self, reference: String) {
        self.attributes.reference = Some(reference);
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
            CellValue::Empty => {}
        }
        writer.end_element();
    }
}
