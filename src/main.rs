pub mod cell;
pub mod row;
pub mod traits;
pub mod xml_templates;
use std::collections::HashSet;
use xml_templates::content_type::ContentType;
use xmlwriter::*;

use cell::{Cell, CellType};
use row::Row;
use traits::XMLString;
use xml_templates::shared_string::SharedStrings;

fn main() {
    let mut work_book = WorkBook::new();

    let mut work_sheet_1 = WorkSheet::blank("Sheet1");

    work_sheet_1.add_blank_row().add_cells(vec![
        Cell::new("Octobar".to_string(), "A1".to_string()),
        Cell::new("Nov".to_string(), "A2".to_string()),
        Cell::new("Dec".to_string(), "A3".to_string()),
        Cell::new("Nov".to_string(), "A4".to_string()),
        Cell::new("Jan".to_string(), "A5".to_string()),
    ]);

    work_sheet_1.add_blank_row().add_cells(vec![
        Cell::new("Pen".to_string(), "A1".to_string()),
        Cell::new("Dan".to_string(), "A2".to_string()),
        Cell::new("Dec".to_string(), "A3".to_string()),
        Cell::new("Copy cat".to_string(), "A4".to_string()),
        Cell::new("Jan".to_string(), "A5".to_string()),
    ]);

    work_book.add_sheet(work_sheet_1);

    // print the shared string.
    println!("worksheet names {:?}", work_book.work_sheet_names);
    println!("ss {:?}", work_book.shared_string);

    // create the content type.
    let mut content_type = ContentType::new();
    for sh in work_book.work_sheet_names.iter() {
        content_type.add_sheet(sh.as_str());
    }

    // print the shared string xml
    let shared_string_xml = work_book.shared_string.to_xml();
    println!("shared string xml {}", shared_string_xml);

    let writer = XmlWriter::new(Options::default());
    let p = work_book.to_xml(writer);
    println!("workbook {}", p.0);
    p.1.iter().for_each(|f| {
        println!("{}", f);
    })
}

struct WorkBook {
    work_sheets: Vec<WorkSheet>,
    work_sheet_names: HashSet<String>,
    shared_string: SharedStrings,
}

impl WorkBook {
    pub fn new() -> Self {
        WorkBook {
            work_sheets: vec![],
            work_sheet_names: HashSet::new(),
            shared_string: SharedStrings::new(),
        }
    }

    pub fn add_sheet(&mut self, mut work_sheet: WorkSheet) {
        if self.work_sheet_names.contains(&work_sheet.name) {
            work_sheet.name = format!("Sheet{}", self.work_sheet_names.len() + 1)
        }
        // update the share string.
        let row_itr = work_sheet.rows.iter_mut();
        for row in row_itr {
            let cell_itr = row.cells.iter_mut();
            for cell in cell_itr {
                match cell.cell_type {
                    CellType::CString => {
                        self.shared_string.add_string(&mut cell.value);
                    }
                    _ => {}
                }
            }
        }

        // append this work sheet
        self.work_sheets.push(work_sheet);
        let last = self.work_sheets.last().unwrap();

        self.work_sheet_names.insert(last.name.clone());
    }

    // todo: we need to cover every thing here, be the following code cosume everthing.
    fn to_xml(self, mut writer: XmlWriter) -> (String, Vec<String>) {
        let mut shs = vec![];
        writer.write_declaration();
        writer.start_element("workbook");
        // for each worksheet print xml
        for work_sheet in self.work_sheets {
            shs.push(work_sheet.to_xml());
        }
        writer.end_element();
        (writer.end_document(), shs)
    }
}

struct WorkSheet {
    name: String,
    rows: Vec<Row>,
}

impl WorkSheet {
    // create a new
    pub fn blank(name: &str) -> Self {
        WorkSheet {
            name: name.to_string(),
            rows: vec![],
        }
    }

    // return the newly created blank row mut.
    pub fn add_blank_row(&mut self) -> &mut Row {
        let row = Row::new();
        self.rows.push(row);
        self.rows.last_mut().unwrap()
    }

    pub fn to_xml(self) -> String {
        let mut writer = XmlWriter::new(Options::default());
        writer.write_declaration();
        writer.start_element("worksheet");
        writer.write_attribute(
            "xmlns",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        );
        writer.write_attribute(
            "xmlns:r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        );
        writer.write_attribute(
            "xmlns:mc",
            "http://schemas.openxmlformats.org/markup-compatibility/2006",
        );
        writer.write_attribute("mc:Ignorable", "x14ac");
        writer.write_attribute(
            "xmlns:x14ac",
            "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac",
        );

        writer.start_element("sheets");
        for row in self.rows {
            writer.start_element("sheet");
            row.to_xml(&mut writer);
            writer.end_element();
        }
        writer.end_element();

        writer.end_element();
        writer.end_document()
    }
}
