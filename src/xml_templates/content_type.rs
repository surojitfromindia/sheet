// in this file create the content type xml

use xmlwriter::{Options, XmlWriter};

pub struct ContentType {
    overrides: Vec<Override>,
    next_sheet_number: i32,
}

struct Override {
    content_type: String,
    part_name: String,
}

static WORK_BOOK_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
static WORK_SHEET_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
static SS_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
static RS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.relationships+xml";

impl ContentType {
    pub fn new() -> Self {
        ContentType {
            next_sheet_number: 1,
            overrides: vec![
                // work book
                Override {
                    content_type: WORK_BOOK_CONTENT_TYPE.to_string(),
                    part_name: String::from("/xl/workbook.xml"),
                },
                Override {
                    content_type: RS_CONTENT_TYPE.to_string(),
                    part_name: String::from("/_rels/.rels"),
                },
                Override {
                    content_type: RS_CONTENT_TYPE.to_string(),
                    part_name: String::from("/xl/_rels/workbook.xml.rels"),
                },
                // sharedStrings
                Override {
                    content_type: SS_CONTENT_TYPE.to_string(),
                    part_name: String::from("/xl/sharedStrings.xml"),
                },
            ],
        }
    }

    // add a new sheet information with the work book
    pub fn add_sheet(&mut self) {
        self.overrides.push(Override {
            content_type: WORK_SHEET_CONTENT_TYPE.to_string(),
            // sheet1, ... sheet12
            part_name: format!("/xl/worksheets/sheet{}.xml", self.next_sheet_number),
        });
        // increase the sheet counter by 1
        self.next_sheet_number += 1;
    }

    // retunr the complete content type.
    pub fn to_xml(self) -> String {
        let mut writer = XmlWriter::new(Options {
            ..Default::default()
        });
        writer.write_declaration();

        // around types the whole xml goes
        writer.start_element("Types");
        writer.write_attribute(
            "xmlns",
            "http://schemas.openxmlformats.org/package/2006/content-types",
        );

        // write 2 defaults
        writer.start_element("Default");
        writer.write_attribute("Extension", "rels");
        writer.write_attribute(
            "ContentType",
            "application/vnd.openxmlformats-package.relationships+xml",
        );
        writer.end_element();

        writer.start_element("Default");
        writer.write_attribute("Extension", "xml");
        writer.write_attribute("ContentType", "application/xml");
        writer.end_element();

        // loop over each ovrrides
        for ov in self.overrides {
            writer.start_element("Override");
            writer.write_attribute("PartName", &ov.part_name);
            writer.write_attribute("ContentType", &ov.content_type);
            writer.end_element();
        }

        writer.end_document()
    }
}
