use xmlwriter::{Options, XmlWriter};

pub struct RelationShip {
    next_seq_number: u32,
}

static RSS_XMLNS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
static RS_OFFICE_DOCUMENT: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
static RS_OFFICE_DOCUMENT_THEME: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument/theme";
static RS_OFFICE_DOCUMENT_STYLE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument/styles";
static RS_OFFICE_DOCUMENT_WS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument/worksheet";
static RS_OFFICE_DOCUMENT_SS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";

impl RelationShip {
    pub fn new() -> Self {
        RelationShip { next_seq_number: 1 }
    }

    #[inline]
    pub fn to_root_xml(mut self) -> String {
        // x1/workbook.xml
        let mut writer = XmlWriter::new(Options::default());

        writer.write_declaration();

        writer.start_element("Relationships");
        writer.write_attribute("xmlns", RSS_XMLNS);
        // workbook
        writer.start_element("Relationship");
        writer.write_attribute("Id", self.next_id().as_str());
        writer.write_attribute("Type", RS_OFFICE_DOCUMENT);
        writer.write_attribute("Target", "x1/workbook.xml");
        // TODO: add more.
        writer.end_document()
    }

    #[inline]
    pub fn to_work_book_rel_xml(mut self, no_of_themes: u32, no_of_sheets: usize) -> String {
        let mut writer = XmlWriter::new(Options::default());

        writer.write_declaration();
        writer.start_element("Relationships");
        writer.write_attribute("xmlns", RSS_XMLNS);

        // styles
        // writer.start_element("Relationship");
        // writer.write_attribute("Id", self.next_id().as_str());
        // writer.write_attribute("Type", RS_OFFICE_DOCUMENT_STYLE);
        // writer.write_attribute("Target", "styles.xml");
        // writer.end_element();

        // themes
        for i in 1..=no_of_themes {
            writer.start_element("Relationship");
            writer.write_attribute("Id", self.next_id().as_str());
            writer.write_attribute("Type", RS_OFFICE_DOCUMENT_THEME);
            writer.write_attribute("Target", format!("theme{}.xml", i).as_str());
            writer.end_element();
        }

        // worksheets
        for i in 1..=no_of_sheets {
            writer.start_element("Relationship");
            writer.write_attribute("Id", self.next_id().as_str());
            writer.write_attribute("Type", RS_OFFICE_DOCUMENT_WS);
            writer.write_attribute("Target", format!("worksheets/sheet{}.xml", i).as_str());
            writer.end_element();
        }
        // shared string
        writer.start_element("Relationship");
        writer.write_attribute("Id", self.next_id().as_str());
        writer.write_attribute("Type", RS_OFFICE_DOCUMENT_SS);
        writer.write_attribute("Target", "sharedStrings.xml");
        writer.end_element();

        writer.end_document()
    }

    fn next_id(&mut self) -> String {
        let id = format!("rId{}", self.next_seq_number);
        self.next_seq_number += 1;
        id
    }
}
