use xmlwriter::{Options, XmlWriter};

use crate::{row::Row, traits::XMLString};


pub struct WorkSheet {
    pub name: String,
    pub rows: Vec<Row>,
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
