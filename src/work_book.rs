use crate::cell::*;
use crate::{
    work_sheet::WorkSheet,
    xml_templates::{
        content_type::ContentType, relation_ship::RelationShip, shared_string::SharedStrings,
        style::Style,
    },
};
use std::{collections::HashSet, fs::File, io::Write};
use xmlwriter::{Options, XmlWriter};
use zip::write::SimpleFileOptions;

pub struct WorkBook {
    pub work_sheets: Vec<WorkSheet>,
    pub work_sheet_names: HashSet<String>,
    shared_string: SharedStrings,
    style: Style,
    content_type: ContentType,
    root_relation_ship: RelationShip,
    work_book_relation_ship: RelationShip,
}

impl WorkBook {
    pub fn new() -> Self {
        WorkBook {
            work_sheets: vec![],
            work_sheet_names: HashSet::new(),

            // other xmls
            shared_string: SharedStrings::new(),
            style: Style::new(),
            content_type: ContentType::new(),
            root_relation_ship: RelationShip::new(),
            work_book_relation_ship: RelationShip::new(),
        }
    }

    pub fn add_sheet(&mut self, mut work_sheet: WorkSheet) {
        if self.work_sheet_names.contains(&work_sheet.name) {
            work_sheet.name = format!("Sheet{}", self.work_sheet_names.len() + 1)
        }
        // register this sheet to content type.
        self.content_type.add_sheet();
        // update the share string.
        let row_itr = work_sheet.rows.iter_mut();
        for row in row_itr {
            let cell_itr = row.get_cells_mut().iter_mut();
            for cell in cell_itr {
                match &mut cell.value {
                    CellValue::CString(v) => {
                        self.shared_string.add_string(v);
                    }
                    _ => {}
                }

                // cells have style
                if let Some(i) = cell.get_style().as_ref() {
                    let sid = self.style.add_cell_xf(Some(&i.font_style));
                    cell.set_style_index(sid);
                }
            }
        }

        // append this work sheet
        self.work_sheets.push(work_sheet);
        let last = self.work_sheets.last().unwrap();

        self.work_sheet_names.insert(last.name.clone());
    }

    fn create_sheets(&self, writer: &mut XmlWriter) {
        writer.start_element("sheets");
        let mut r_id: u8 = 1;
        // for each worksheet print xml
        for (inx, work_sheet) in self.work_sheets.iter().enumerate() {
            writer.start_element("sheet");
            writer.write_attribute("name", work_sheet.name.as_str());
            writer.write_attribute("state", "visiable");
            let current_rid = format!("rId{}", r_id.to_string());
            // todo: add sheet to content type and get back the id, which will be set in here.
            writer.write_attribute("r:id", current_rid.as_str());
            writer.write_attribute("sheetId", (inx + 1).to_string().as_str());
            writer.end_element();
            r_id += 1;
        }
        writer.end_element();
    }

    // todo: we need to cover every thing here, be the following code cosume everthing.
    fn to_xml(&mut self) -> String {
        let mut writer = XmlWriter::new(Options::default());

        writer.start_element("workbook");
        writer.write_attribute(
            "xmlns",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        );
        writer.write_attribute(
            "xmlns:r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        );

        // self.create_file_version(&mut writer);
        // self.create_book_views(&mut writer);
        self.create_sheets(&mut writer);

        writer.end_element();
        writer.end_document()
    }

    pub fn save(mut self) {
        let work_book_xml = self.to_xml();

        let ss_xml = self.shared_string.to_xml();

        let style_xml = self.style.to_xml();

        println!("{}", style_xml);

        let content_type_xml = self.content_type.to_xml();

        let root_rs_xml = self.root_relation_ship.to_root_xml();

        let work_book_rs_xml = self
            .work_book_relation_ship
            .to_work_book_rel_xml(0, self.work_sheets.len());

        let cursor = std::io::Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(cursor);

        // content type root
        zip.start_file("[Content_Types].xml", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(content_type_xml.as_bytes()).unwrap();

        //_rels root
        zip.add_directory("_rels/", SimpleFileOptions::default())
            .unwrap();
        zip.start_file("_rels/.rels", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(root_rs_xml.as_bytes()).unwrap();

        // folder for x1
        zip.add_directory("xl/", SimpleFileOptions::default())
            .unwrap();
        zip.add_directory("xl/_rels/", SimpleFileOptions::default())
            .unwrap();
        zip.add_directory("xl/worksheets/", SimpleFileOptions::default())
            .unwrap();

        // add relation ship for workbook
        zip.start_file("xl/_rels/workbook.xml.rels", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(work_book_rs_xml.as_bytes()).unwrap();

        // add sheets
        for (i, work_sheet) in self.work_sheets.into_iter().enumerate() {
            let sheet_xml = work_sheet.to_xml();
            // println!("{}", sheet_xml);
            let sheet_name = format!("xl/worksheets/sheet{}.xml", i + 1);
            zip.start_file(sheet_name, SimpleFileOptions::default())
                .unwrap();
            zip.write_all(sheet_xml.as_bytes()).unwrap();
        }

        // add shared strings
        zip.start_file("xl/sharedStrings.xml", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(ss_xml.as_bytes()).unwrap();

        // add styles
        zip.start_file("xl/styles.xml", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(style_xml.as_bytes()).unwrap();

        // add workbook
        zip.start_file("xl/workbook.xml", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(work_book_xml.as_bytes()).unwrap();

        let cursor = zip.finish().unwrap();
        let data = cursor.into_inner();
        let mut file = File::create("test.xlsx").unwrap();
        file.write_all(&data).unwrap();

        let mut zip_file = File::create("test.zip").unwrap();
        zip_file.write_all(&data).unwrap();
    }
}
