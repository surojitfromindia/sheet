use xmlwriter::*;

fn main() {
    let mut writer = XmlWriter::new(Options::default());
    writer.start_element("worksheet");
    let mut cell1 = Cell::new("1".to_string(), "A1".to_string());
    cell1.add_formula("=B1+C1".to_string());

    let mut cell2 = Cell::new("2".to_string(), "B1".to_string());
    cell2.add_formula("=A1+C1".to_string());

    let mut cell3 = Cell::new("3".to_string(), "C1".to_string());
    cell3.add_formula("=A1+B1".to_string());
    let cell4 = Cell::new("4".to_string(), "D1".to_string());

    let mut row1 = Row::new();
    row1.add_cell(cell1);
    row1.add_cell(cell2);
    row1.add_cells(vec![cell3, cell4]);
    row1.to_xml(&mut writer);

    writer.end_element();
    println!("{}", writer.end_document())
}

struct Cell {
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

    pub fn to_xml(self, writer: &mut XmlWriter) {
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

struct Row {
    cells: Vec<Cell>,
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

    pub fn to_xml(self, writer: &mut XmlWriter) {
        writer.start_element("row");
        for cell in self.cells {
            cell.to_xml(writer);
        }
        writer.end_element();
    }
}
