// in this file create the content type xml

use xmlwriter::{Options, XmlWriter};

pub struct ContentType {
    overrides: Vec<Override>,
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
static STYLE_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml";

impl ContentType {
    pub fn new() -> Self {
        ContentType {
            overrides: vec![
                // work book
                Override {
                    content_type: WORK_BOOK_CONTENT_TYPE.to_string(),
                    part_name: String::from("/x1/workbook.xml"),
                },
                // sharedStrings
                Override {
                    content_type: SS_CONTENT_TYPE.to_string(),
                    part_name: String::from("/x1/sharedStrings.xml"),
                },
                // styles
                Override {
                    content_type: STYLE_CONTENT_TYPE.to_string(),
                    part_name: String::from("/x1/styles.xml"),
                },
            ],
        }
    }

    // add a new sheet information with the work book
    pub fn add_sheet(&mut self, sheet_name: &str) {
        self.overrides.push(Override {
            content_type: format!("/x1/worksheets/{}", WORK_SHEET_CONTENT_TYPE.to_string()),
            part_name: sheet_name.to_string(),
        });
        
    }

    // retunr the complete content type.
    pub fn to_xml(self) -> String {
        let mut writer = XmlWriter::new(Options {
            ..Default::default()
        });
        writer.write_declaration();

        // around types the whole xml goes
        writer.start_element("Types");

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
