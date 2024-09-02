use std::collections::HashSet;

use xmlwriter::{Options, XmlWriter};

use crate::{
    cell::CellType,
    work_sheet::WorkSheet,
    xml_templates::{content_type::ContentType, shared_string::SharedStrings},
};

pub struct WorkBook {
    pub work_sheets: Vec<WorkSheet>,
    pub work_sheet_names: HashSet<String>,
    shared_string: SharedStrings,
    content_type: ContentType,
}

impl WorkBook {
    pub fn new() -> Self {
        WorkBook {
            work_sheets: vec![],
            work_sheet_names: HashSet::new(),

            // other xmls
            shared_string: SharedStrings::new(),
            content_type: ContentType::new(),
        }
    }

    pub fn add_sheet(&mut self, mut work_sheet: WorkSheet) {
        if self.work_sheet_names.contains(&work_sheet.name) {
            work_sheet.name = format!("Sheet{}", self.work_sheet_names.len() + 1)
        }
        // register this sheet to content type.
        self.content_type.add_sheet(&work_sheet.name.as_str());
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

    fn create_file_version(&self, writer: &mut XmlWriter) {
        writer.start_element("fileVersion");
        writer.write_attribute("appName", "RustSheet");
        writer.end_element();
    }

    fn create_book_views(&self, writer: &mut XmlWriter) {
        writer.start_element("bookViews");
        writer.start_element("workbookView");
        // todo: add later.
        writer.end_element();
        writer.end_element();
    }

    fn create_sheets(&self, writer: &mut XmlWriter) {
        writer.start_element("sheets");
        let mut r_id: u8 = 2;
        // for each worksheet print xml
        for work_sheet in self.work_sheets.iter() {
            writer.start_element("sheet");
            writer.write_attribute("name", work_sheet.name.as_str());
            writer.write_attribute("state", "visiable");
            let current_rid = format!("rId{}", r_id.to_string());
            // todo: add sheet to content type and get back the id, which will be set in here.
            writer.write_attribute("r:id", current_rid.as_str());
            r_id += 1;
        }
        writer.end_element();
    }

    // todo: we need to cover every thing here, be the following code cosume everthing.
    fn to_xml(&mut self) -> String {
        let mut writer = XmlWriter::new(Options::default());
        writer.write_declaration();
        writer.start_element("workbook");

        self.create_file_version(&mut writer);
        self.create_book_views(&mut writer);
        self.create_sheets(&mut writer);

        writer.end_element();
        writer.end_document()
    }

    pub fn save(mut self) {
        let work_book_xml = self.to_xml();

        let ss_xml = self.shared_string.to_xml();

        let content_type_xml = self.content_type.to_xml();

        // todo: buit other xml here
    }
}
