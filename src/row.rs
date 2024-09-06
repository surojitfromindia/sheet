use std::{collections::HashSet, num::ParseFloatError};

use crate::cell::Cell;
use crate::traits;
use traits::XMLString;
use xmlwriter::*;

pub struct Row {
    cells: Vec<Cell>,
    row_number: usize,
    col_reference: String,
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
            col_reference: "A".to_string(),
            cells: Vec::new(),
            cell_reference_set: HashSet::new(),
        }
    }

    pub fn add_string(&mut self, value: String) -> &mut Cell {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_string(value, cell_reference, false);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn add_inline_string(&mut self, value: String) -> &mut Cell {
        let cell_reference = self.get_next_cell_ref();
        let cell = Cell::from_string(value, cell_reference, true);
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
        let (col_ref, row_ref) = split_cell_ref(cell_reference).unwrap();
        if row_ref != self.row_number.to_string() {
            return Err("Invalid row reference");
        }

        // update the set
        self.cell_reference_set.insert(cell_reference.clone());
        self.cells.push(cell);
        self.col_reference = col_ref;
        Ok(self.cells.last_mut().unwrap())
    }

    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    fn get_next_cell_ref(&mut self) -> String {
        let current_col_ref = &mut self.col_reference;

        // Convert the current column reference ('A' = 1, 'B' = 2, ..., 'Z' = 26, 'AA' = 27, etc.)
        let mut col_chars: Vec<char> = current_col_ref.chars().collect();

        let mut i = col_chars.len() - 1;
        while i >= 0 {
            if col_chars[i] == 'Z' {
                col_chars[i] = 'A';
                if i == 0 {
                    // Prepend 'A' to the string if it overflows at the first character (e.g., "Z" -> "AA")
                    col_chars.insert(0, 'A');
                    break;
                }
                i -= 1;
            } else {
                col_chars[i] = ((col_chars[i] as u8) + 1) as char;
                break;
            }
        }

        // Combine the incremented column reference with the row number
        let cell_ref = format!("{}{}", current_col_ref, self.row_number);

        let next_col_ref: String = col_chars.into_iter().collect();
        *current_col_ref = next_col_ref;

        // Insert the generated cell reference into the set
        self.cell_reference_set.insert(cell_ref.clone());
        cell_ref
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

pub struct ColMovement<'a> {
    row: &'a mut Row,
}

impl<'a> ColMovement<'a> {
    pub fn new(row: &'a mut Row) -> ColMovement<'a> {
        ColMovement { row }
    }

    /// move to next column
    pub fn next(&mut self) -> String {
        self.row.get_next_cell_ref()
    }

    /// skip n columns
    pub fn skip(&mut self, n: usize) -> String {
        for _ in 0..n {
            self.row.get_next_cell_ref();
        }
        self.row.get_next_cell_ref()
    }
}
