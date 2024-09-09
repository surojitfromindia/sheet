use crate::traits::XMLString;

struct Font {
    size: u8,
    name: String,
    bold: bool,
    italic: bool,
    strike: bool,
    undeline: UnderLine,
}

impl Font {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&mut self, size: u8) -> &mut Self {
        self.size = size;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
    pub fn bold(&mut self, bold: bool) -> &mut Self {
        self.bold = bold;
        self
    }

    pub fn italic(&mut self, italic: bool) -> &mut Self {
        self.italic = italic;
        self
    }

    pub fn strike(&mut self, strike: bool) -> &mut Self {
        self.strike = strike;
        self
    }

    pub fn underline(&mut self, underline: UnderLine) -> &mut Self {
        self.undeline = underline;
        self
    }

    pub fn build(self) -> Self {
        Self {
            size: self.size,
            name: self.name,
            bold: self.bold,
            italic: self.italic,
            strike: self.strike,
            undeline: self.undeline,
        }
    }
}

impl XMLString for Font {
    fn to_xml(self, writer: &mut xmlwriter::XmlWriter) {
        writer.start_element("font");
        writer.write_attribute("sz", &self.size.to_string());
        writer.write_attribute("name", &self.name);
        writer.write_attribute("b", &self.bold.to_string());
        writer.write_attribute("i", &self.italic.to_string());
        writer.write_attribute("strike", &self.strike.to_string());
        match self.undeline {
            UnderLine::Single => writer.write_attribute("u", "single"),
            UnderLine::Double => writer.write_attribute("u", "double"),
            UnderLine::SingleAcccounting => writer.write_attribute("u", "singleAccounting"),
            UnderLine::DoubleAccouting => writer.write_attribute("u", "doubleAccounting"),
            _ => (),
        }
        writer.end_element();
    }
}

enum UnderLine {
    Single,
    Double,
    SingleAcccounting,
    DoubleAccouting,
    None,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            size: 10,
            name: String::from("Arial"),
            bold: false,
            italic: false,
            strike: false,
            undeline: UnderLine::None,
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::*;
    use xmlwriter::{Options, XmlWriter};

    #[test]
    fn test_font() {
        let mut writer = XmlWriter::new(Options::default());
        let mut font = Font::new();
        font.bold(true)
            .italic(true)
            .strike(true)
            .size(12)
            .name("Times New Roman")
            .underline(UnderLine::Single);

        font.build().to_xml(&mut writer);
        let xml = writer.end_document();
        assert_eq!(
            xml,
            r#"<font sz="12" name="Times New Roman" b="true" i="true" strike="true" u="single"/>
"#
        );
    }
}
