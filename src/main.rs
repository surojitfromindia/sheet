pub mod cell;
pub mod row;
pub mod traits;
pub mod xml_templates;
use std::collections::{HashMap, HashSet};
use xmlwriter::*;

use cell::Cell;
use row::Row;
use std::mem;
use traits::XMLString;
use xml_templates::content_type::*;

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

    work_book.add_sheet(work_sheet_1);

    let mut writer = XmlWriter::new(Options::default());
    let p = work_book.to_xml(writer);
    println!("{}", p);
}

struct WorkBook {
    work_sheets: Vec<WorkSheet>,
    work_sheet_names: HashSet<String>,
}

impl WorkBook {
    pub fn new() -> Self {
        WorkBook {
            work_sheets: vec![],
            work_sheet_names: HashSet::new(),
        }
    }

    pub fn add_sheet(&mut self, mut work_sheet: WorkSheet) {
        if self.work_sheet_names.contains(&work_sheet.name) {
            work_sheet.name = format!("sheet{}", self.work_sheet_names.len())
        }

        // append this work sheet
        self.work_sheets.push(work_sheet);
        let last = self.work_sheets.last().unwrap();

        self.work_sheet_names.insert(last.name.clone());
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

struct SharedStrings {
    s_map: HashMap<String, u32>,
    next_index: u32,
}

impl SharedStrings {
    pub fn new() -> Self {
        SharedStrings {
            next_index: 1,
            s_map: HashMap::new(),
        }
    }
    pub fn add_string(&mut self, st: &mut String) -> u32 {
        // replace the underlaying st by mem::replace and take ownership of the string
        // we need to move it.
        // let s = mem::replace(st, "0".to_string());

        if let Some(ind) = self.s_map.get(st) {
            // a item is found then replace
            let _ = mem::replace(st, ind.to_string());
            *ind
        } else {
            // insert and increament the next_counter by 1
            let key = mem::replace(st, self.next_index.to_string());
            self.s_map.insert(key, self.next_index);
            {
                let temp = self.next_index;
                self.next_index += 1;
                temp
            }
        }
    }
}
