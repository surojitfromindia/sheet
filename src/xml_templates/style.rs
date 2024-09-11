use std::collections::HashMap;

use crate::traits::XMLString;

#[derive(Debug)]
pub struct FontStyle {
    size: u8,
    name: String,
    bold: bool,
    italic: bool,
    strike: bool,
    undeline: UnderLine,
}

impl FontStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: u8) -> Self {
        self.size = size;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn strike(mut self, strike: bool) -> Self {
        self.strike = strike;
        self
    }

    pub fn underline(mut self, underline: UnderLine) -> Self {
        self.undeline = underline;
        self
    }

    fn unqiue_id(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.size, self.name, self.bold, self.italic, self.strike
        )
    }
}

impl XMLString for FontStyle {
    fn to_xml(self, writer: &mut xmlwriter::XmlWriter) {
        writer.start_element("font");

        // size
        writer.start_element("sz");
        writer.write_attribute("val", &self.size.to_string());
        writer.end_element();

        // name
        writer.start_element("name");
        writer.write_attribute("val", &self.name);
        writer.end_element();

        // bold
        writer.start_element("b");
        writer.write_attribute("val", if self.bold { "true" } else { "false" });
        writer.end_element();

        // italic
        writer.start_element("i");
        writer.write_attribute("val", if self.italic { "true" } else { "false" });
        writer.end_element();

        // strike
        writer.start_element("strike");
        writer.write_attribute("val", if self.strike { "true" } else { "false" });
        writer.end_element();

        // underline
        // match self.undeline {
        //     UnderLine::Single => writer.write_attribute("u", "single"),
        //     UnderLine::Double => writer.write_attribute("u", "double"),
        //     UnderLine::SingleAcccounting => writer.write_attribute("u", "singleAccounting"),
        //     UnderLine::DoubleAccouting => writer.write_attribute("u", "doubleAccounting"),
        //     _ => (),
        // }
        writer.end_element();
    }
}
#[derive(Debug, Clone)]
pub enum UnderLine {
    Single,
    Double,
    SingleAcccounting,
    DoubleAccouting,
    None,
}

impl Default for FontStyle {
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

#[derive(Debug)]
struct CellXf {
    font_id: usize,
    fill_id: usize,
    border_id: usize,
}
impl Default for CellXf {
    fn default() -> Self {
        Self {
            font_id: 0,
            fill_id: 0,
            border_id: 0,
        }
    }
}

impl CellXf {
    pub fn new(font_id: usize) -> Self {
        Self {
            font_id,
            ..Default::default()
        }
    }

    fn unique_id(&self) -> String {
        format!("{}{}{}", self.font_id, self.fill_id, self.border_id)
    }
}

impl XMLString for CellXf {
    fn to_xml(self, writer: &mut xmlwriter::XmlWriter) {
        writer.start_element("xf");
        writer.write_attribute("fontId", &self.font_id.to_string());
        // writer.write_attribute("fillId", &self.fill_id.to_string());
        // writer.write_attribute("borderId", &self.border_id.to_string());
        writer.end_element();
    }
}

#[derive(Debug)]
pub struct Style {
    nex_uique_font_count: usize,
    fonts_map: HashMap<String, (usize, FontStyle)>,
    nex_uique_xf_count: usize,
    cell_xfs_map: HashMap<String, (usize, CellXf)>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            nex_uique_font_count: 0,
            nex_uique_xf_count: 0,
            fonts_map: HashMap::new(),
            cell_xfs_map: HashMap::new(),
        }
    }
}

static SS_XMLNS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
    fn add_font(&mut self, font: &FontStyle) -> usize {
        // first check check if the font is already in the fonts_map
        // if no add the font to the fonts_map and fonts and return the index.

        if let Some(&index) = self.fonts_map.get(&font.unqiue_id()).as_ref() {
            return index.0;
        } else {
            let _fonst = FontStyle::new()
                .size(font.size)
                .name(&font.name)
                .bold(font.bold)
                .italic(font.italic)
                .strike(font.strike)
                .underline(font.undeline.clone());

            let index = self.nex_uique_font_count;
            self.fonts_map
                .insert(font.unqiue_id(), (self.nex_uique_font_count, _fonst));
            self.nex_uique_font_count += 1;
            index
        }
    }
    // after call to add_font now we have font  and other ids
    pub fn add_cell_xf(&mut self, font: Option<&FontStyle>) -> usize {
        let font_id = match font {
            Some(font) => self.add_font(font),
            None => 0,
        };

        let cell_xf = CellXf::new(font_id);
        if let Some(&index) = self.cell_xfs_map.get(&cell_xf.unique_id()).as_ref() {
            index.0
        } else {
            let index = self.nex_uique_xf_count;

            let cell_xf = CellXf::new(font_id);

            self.cell_xfs_map
                .insert(cell_xf.unique_id(), (self.nex_uique_xf_count, cell_xf));
            self.nex_uique_xf_count += 1;
            index
        }
    }

    pub fn to_xml(self) -> String {
        let mut writer = xmlwriter::XmlWriter::new(xmlwriter::Options::default());
        writer.start_element("styleSheet");
        writer.write_attribute("xmlns", SS_XMLNS);

        // write fonts
        writer.start_element("fonts");
        writer.write_attribute("count", &self.fonts_map.len().to_string());
        let mut fonts_vec: Vec<Option<FontStyle>> = Vec::with_capacity(self.fonts_map.len());
        // fill the fonts_vec with None
        for _ in 0..self.fonts_map.len() {
            fonts_vec.push(None);
        }
        // iterate over the fonts_map and write the fonts
        for font_style in self.fonts_map.into_values() {
            fonts_vec[font_style.0] = Some(font_style.1);
        }
        // now that fonts are ordered
        for font in fonts_vec.into_iter() {
            if let Some(font) = font {
                font.to_xml(&mut writer);
            }
        }
        writer.end_element();

        // write cellXfs
        writer.start_element("cellXfs");
        writer.write_attribute("count", &self.cell_xfs_map.len().to_string());
        let mut cell_xfs_vec: Vec<Option<CellXf>> = Vec::with_capacity(self.cell_xfs_map.len());
        for _ in 0..self.cell_xfs_map.len() {
            cell_xfs_vec.push(None);
        }
        for cell_xf in self.cell_xfs_map.into_values() {
            cell_xfs_vec[cell_xf.0] = Some(cell_xf.1);
        }
        for cell_xf in cell_xfs_vec {
            if let Some(cell_xf) = cell_xf {
                cell_xf.to_xml(&mut writer);
            }
        }
        writer.end_element();

        writer.end_document()
    }
}
