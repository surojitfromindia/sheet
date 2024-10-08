use xmlwriter::{Options, XmlWriter};

use crate::{row::Row, traits::XMLString};

pub struct WorkSheet {
    pub name: String,
    pub rows: Vec<Row>,
    next_row: usize,
}

impl WorkSheet {
    // create a new
    pub fn blank(name: &str) -> Self {
        WorkSheet {
            name: name.to_string(),
            rows: vec![],
            next_row: 1,
        }
    }

    // return the newly created blank row mut.
    pub fn add_blank_row(&mut self) -> &mut Row {
        let row = Row::new(self.next_row);
        self.next_row += 1;
        self.rows.push(row);
        self.rows.last_mut().unwrap()
    }

    pub fn to_xml(self) -> String {
        let mut writer = XmlWriter::new(Options::default());
        writer.start_element("worksheet");
        writer.write_attribute(
            "xmlns",
            "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        );
        writer.write_attribute(
            "xmlns:r",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
        );

        writer.start_element("sheetData");
        for row in self.rows {
            row.to_xml(&mut writer);
        }
        writer.end_element();

        writer.end_element();
        writer.end_document()
    }
}
