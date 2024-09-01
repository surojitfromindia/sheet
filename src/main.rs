pub mod cell;
pub mod row;
pub mod traits;
pub mod xml_templates;
use std::collections::HashSet;
use xmlwriter::*;

use cell::Cell;
use row::Row;
use traits::XMLString;
use xml_templates::content_type::*;

fn main() {
    let mut work_book = WorkBook::new();
    let mut work_sheet_1 = WorkSheet::blank("Sheet1");

    let row = work_sheet_1.add_blank_row();
    let cell1 = Cell::new("Gi".to_string(), "A1".to_string());
    row.add_cell(cell1);

    let mut xml_writer = XmlWriter::new(Options::default());



























































    















    // generate content xml
    // let mut content_type = ContentType::new();
    // content_type.add_sheet("sheet1.xml");
    // content_type.add_sheet("sheet2.xml");
    // println!("{}", content_type.to_xml());
}

struct WorkBook<'wb> {
    work_sheets: Vec<WorkSheet>,
    work_sheet_names: HashSet<&'wb String>,
}

impl<'wb> WorkBook<'wb> {
    pub fn new() -> Self {
        WorkBook {
            work_sheets: vec![],
            work_sheet_names: HashSet::new(),
        }
    }

    pub fn add_sheet(&'wb mut self, mut work_sheet: WorkSheet) {
        if self.work_sheet_names.contains(&work_sheet.name) {
            work_sheet.name = format!("sheet{}", self.work_sheet_names.len())
        }

        // append this work sheet
        self.work_sheets.push(work_sheet);
        let last = self.work_sheets.last().unwrap();

        self.work_sheet_names.insert(&last.name);
    }

    fn to_xml(self, mut writer: XmlWriter) -> String {
        writer.write_declaration();
        writer.start_element("WorkBook");
        for ws in self.work_sheets {
            ws.to_xml(&mut writer);
        }
        writer.end_element();
        writer.end_document()
    }
}

struct WorkSheet {
    name: String,
    rows: Vec<Row>,
}

impl WorkSheet {
    // create a new
    pub fn blank(name: &str) -> Self {
        // check if the name already exists in the work_sheet_names of the work book

        WorkSheet {
            name: name.to_string(),
            rows: vec![],
        }
    }

    /// return the newly created blank row mut.
    pub fn add_blank_row(&mut self) -> &mut Row {
        let row = Row::new();
        self.rows.push(row);
        self.rows.last_mut().unwrap()
    }
}

impl XMLString for WorkSheet {
    fn to_xml(self, writer: &mut xmlwriter::XmlWriter) {
        writer.start_element("workSheet");
        for row in self.rows {
            row.to_xml(writer);
        }
        writer.end_element();
    }
}
