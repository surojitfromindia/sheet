use std::{collections::HashSet, error::Error, num::ParseFloatError};

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
    col_row_ref: (String, String),
}

pub struct Row {
    cells: Vec<Cell>,
    row_number: usize,
    col_reference: char,
    cell_reference_set: HashSet<String>,
}

fn split_cell_ref(cell_ref: &str) -> Result<(String, String), &'static str> {
    let mut col_ref = String::new();
    let mut row_ref = String::new();
    for c in cell_ref.chars() {
        if c.is_alphabetic() {
            col_ref.push(c);
        } else {
            row_ref.push(c);
        }
    }
    if col_ref.is_empty() || row_ref.is_empty() {
        return Err("Invalid cell reference");
    }
    Ok((col_ref, row_ref))
}

impl Row {
    pub fn new(row_number: usize) -> Row {
        Row {
            row_number,
            col_reference: 'A',
            cells: Vec::new(),
            cell_reference_set: HashSet::new(),
        }
    }

    pub fn add_string(&mut self, value: String) -> &mut Cell {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_string(value, cell_reference);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn add_number(&mut self, value: String) -> Result<&mut Cell, ParseFloatError> {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_number(value, cell_reference)?;
        self.cells.push(cell);
        Ok(self.cells.last_mut().unwrap())
    }

    /// add a cell to an existing row
    /// can fail if the given reference is already present or not valid.
    pub fn add_cell(&mut self, cell: Cell) -> Result<&mut Cell, &'static str> {
        let cell_reference = cell.attributes.reference.as_ref().unwrap();
        if self.cell_reference_set.contains(cell_reference) {
            return Err("Cell reference already exists");
        }
        let (_, row_ref) = split_cell_ref(cell_reference).unwrap();
        if row_ref != self.row_number.to_string() {
            return Err("Invalid row reference");
        }

        // update the set
        self.cell_reference_set.insert(cell_reference.clone());
        self.cells.push(cell);
        Ok(self.cells.last_mut().unwrap())
    }

    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    fn get_next_cell_ref(&mut self) -> String {
        let col_reference = self.col_reference;
        self.col_reference = (self.col_reference as u8 + 1) as char;
        let s = format!("{}{}", col_reference, self.row_number);
        self.cell_reference_set.insert(s.clone());
        s
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

impl Cell {
    pub fn from_string(value: String, reference: String) -> Cell {
        Cell {
            value: CellValue::CString(value),
            formula: None,
            col_row_ref: split_cell_ref(&reference).unwrap(),
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
            col_row_ref: split_cell_ref(&reference).unwrap(),
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
            CellValue::Empty => {}
        }
        writer.end_element();
    }
}
