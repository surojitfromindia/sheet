use crate::cell::Cell;
use crate::traits;
use traits::XMLString;
use xmlwriter::*;

pub struct Row {
    pub cells: Vec<Cell>,
    row_reference: usize,
    col_reference: char,
}

impl Row {
    pub fn new(row_reference: usize) -> Row {
        Row {
            row_reference,
            col_reference: 'A',
            cells: Vec::new(),
        }
    }

    pub fn add_cells(&mut self, mut cells: Vec<Cell>) -> Result<(), String> {
        // first we need to check if any of the cells have a reference
        // if so, check the reference is valid for the row,
        // if not, return an error
        for cell in cells.iter_mut() {
            if let Some(reference) = cell.attributes.reference.as_ref() {
                if !reference.ends_with(&self.row_reference.to_string()) {
                    return Err(format!("Invalid reference for row: {}", reference));
                }
            } else {
                let cell_reference = self.get_next_cell_ref();
                cell.set_reference(cell_reference);
            }
        }
        self.cells.extend(cells);
        Ok(())
    }

    pub fn add_cell_of_string(&mut self, value: String) -> &mut Cell {
        let mut cell = Cell::of_string(value);
        let cell_reference = self.get_next_cell_ref();
        cell.set_reference(cell_reference);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn add_cell_of_number(&mut self, value: String) -> &mut Cell {
        let mut cell = Cell::of_number(value).unwrap();
        let cell_reference = self.get_next_cell_ref();
        cell.set_reference(cell_reference);
        self.cells.push(cell);
        self.cells.last_mut().unwrap()
    }

    pub fn get_next_cell_ref(&mut self) -> String {
        let col_reference = self.col_reference;
        self.col_reference = (self.col_reference as u8 + 1) as char;
        format!("{}{}", col_reference, self.row_reference)
    }
}

impl XMLString for Row {
    fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("r");
        for cell in self.cells {
            cell.to_xml(writer);
        }
        writer.end_element();
    }
}
