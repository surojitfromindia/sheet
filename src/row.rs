use crate::cell::Cell;
use xmlwriter::*;
use crate::traits;
use traits::XMLString;


pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Row {
        Row { cells: Vec::new() }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }
    pub fn add_cells(&mut self, cells: Vec<Cell>) {
        self.cells.extend(cells);
    }
    
}


impl  XMLString for Row {
     fn to_xml(self, writer: &mut XmlWriter)  {
        writer.start_element("r");
        for cell in self.cells {
            cell.to_xml(writer);
        }
        writer.end_element();
    }
}