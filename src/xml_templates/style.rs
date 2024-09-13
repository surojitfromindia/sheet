use core::fmt;
use std::{collections::HashMap, fmt::Debug};

use crate::traits::XMLString;

#[derive(Debug)]
pub struct FontStyle {
    name: String,
    size: u8,
    bold: bool,
    italic: bool,
    strike: bool,
    undeline: Option<UnderLine>,
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

    pub fn underline(mut self, underline: Option<UnderLine>) -> Self {
        self.undeline = underline;
        self
    }

    fn unqiue_id(&self) -> String {
        let und: String = self
            .undeline
            .as_ref()
            .map_or_else(|| "u_none".to_string(), |v| format!("{:?}", v));
        format!(
            "{}-{}-{}-{}-{}-{}",
            self.size, self.name, self.bold, self.italic, self.strike, und
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
        if self.bold {
            writer.start_element("b");
            writer.write_attribute("val", "true");
            writer.end_element();
        }

        // italic
        if self.italic {
            writer.start_element("i");
            writer.write_attribute("val", "true");
            writer.end_element();
        }

        // strike
        if self.strike {
            writer.start_element("strike");
            writer.end_element();
        }

        // underline

        if let Some(v) = self.undeline {
            writer.start_element("u");
            match v {
                UnderLine::Single => writer.write_attribute("val", "single"),
                UnderLine::Double => writer.write_attribute("val", "double"),
                UnderLine::SingleAcccounting => writer.write_attribute("val", "singleAccounting"),
                UnderLine::DoubleAccouting => writer.write_attribute("val", "doubleAccounting"),
            }
            writer.end_element();
        }

        writer.end_element();
    }
}
#[derive(Clone)]
pub enum UnderLine {
    Single,
    Double,
    SingleAcccounting,
    DoubleAccouting,
}
impl Debug for UnderLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            UnderLine::Single => "u_single",
            UnderLine::Double => "u_double",
            UnderLine::SingleAcccounting => "u_single_acc",
            UnderLine::DoubleAccouting => "u_double_acc",
        };
        write!(f, "{}", description)
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            size: 10,
            name: String::from("Arial"),
            bold: false,
            italic: false,
            strike: false,
            undeline: None,
        }
    }
}

#[derive(Debug)]
struct CellXf {
    font_id: usize,
    fill_id: usize,
    border_id: usize,
    num_fmt_id: usize,
}
impl Default for CellXf {
    fn default() -> Self {
        Self {
            font_id: 0,
            fill_id: 0,
            num_fmt_id: 164,
            border_id: 0,
        }
    }
}

impl CellXf {
    pub fn new(font_id: usize, num_fmt_id: usize) -> Self {
        Self {
            font_id,
            num_fmt_id,
            ..Default::default()
        }
    }

    fn unique_id(&self) -> String {
        format!("{}{}{}", self.font_id, self.fill_id, self.border_id)
    }
}

impl XMLString for CellXf {
    fn to_xml(self, writer: &mut xmlwriter::XmlWriter) {
        println!("{:?}", self);
        writer.start_element("xf");
        writer.write_attribute("fontId", &self.font_id.to_string());
        writer.write_attribute("numFmtId", &self.num_fmt_id.to_string());

        writer.write_attribute("applyFont", "true");
        writer.write_attribute("applyBorder", "false");

        // writer.write_attribute("fillId", &self.fill_id.to_string());
        // writer.write_attribute("borderId", &self.border_id.to_string());
        writer.end_element();
    }
}

#[derive(Debug)]
pub struct Style {
    next_unique_font_count: usize,
    fonts_map: HashMap<String, (usize, FontStyle)>,
    next_unique_xf_count: usize,
    cell_xfs_map: HashMap<String, (usize, CellXf)>,
    num_fmts: Vec<NumFmt>,
}

impl Default for Style {
    fn default() -> Self {
        // we need to add some default font and cell xf styles to work with
        // the style properly.
        let default_font = FontStyle::new();
        let only_bold_font = FontStyle::new().bold(true);
        let only_strike_font = FontStyle::new().strike(true);
        let mut fonts_map = HashMap::new();
        fonts_map.insert(default_font.unqiue_id(), (0, default_font));
        fonts_map.insert(only_bold_font.unqiue_id(), (1, only_bold_font));
        fonts_map.insert(only_strike_font.unqiue_id(), (2, only_strike_font));

        let default_cell_xf = CellXf::new(0, 164);
        let mut cell_xfs_map = HashMap::new();
        cell_xfs_map.insert(default_cell_xf.unique_id(), (0, default_cell_xf));

        Self {
            next_unique_font_count: fonts_map.len(),
            next_unique_xf_count: cell_xfs_map.len(),
            fonts_map,
            cell_xfs_map,
            num_fmts: vec![NumFmt::new(164, "General")],
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

            let index = self.next_unique_font_count;
            self.fonts_map
                .insert(font.unqiue_id(), (self.next_unique_font_count, _fonst));
            self.next_unique_font_count += 1;
            index
        }
    }
    // after call to add_font now we have font  and other ids
    pub fn add_cell_xf(&mut self, font: Option<&FontStyle>) -> usize {
        let font_id = match font {
            Some(font) => self.add_font(font),
            None => 0,
        };

        let cell_xf = CellXf::new(font_id, 164);
        if let Some(&index) = self.cell_xfs_map.get(&cell_xf.unique_id()).as_ref() {
            index.0
        } else {
            let index = self.next_unique_xf_count;

            let cell_xf = CellXf::new(font_id, 164);

            self.cell_xfs_map
                .insert(cell_xf.unique_id(), (self.next_unique_xf_count, cell_xf));
            self.next_unique_xf_count += 1;
            index
        }
    }

    pub fn to_xml(self) -> String {
        println!("{:?}", self.fonts_map);

        let mut writer = xmlwriter::XmlWriter::new(xmlwriter::Options::default());
        writer.start_element("styleSheet");
        writer.write_attribute("xmlns", SS_XMLNS);

        // write numFmts
        writer.start_element("numFmts");
        writer.write_attribute("count", &self.num_fmts.len().to_string());
        for num_fmt in self.num_fmts {
            num_fmt.to_xml(&mut writer);
        }
        writer.end_element();

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

        // fills
        writer.start_element("fills");
        writer.write_attribute("count", "1");
        writer.start_element("fill");
        writer.start_element("patternFill");
        writer.write_attribute("patternType", "none");
        writer.end_element();
        writer.end_element();
        writer.end_element();

        // borders
        writer.start_element("borders");
        writer.write_attribute("count", "1");
        writer.start_element("border");
        writer.start_element("left");
        writer.end_element();
        writer.start_element("right");
        writer.end_element();
        writer.start_element("top");
        writer.end_element();
        writer.start_element("bottom");
        writer.end_element();
        writer.start_element("diagonal");
        writer.end_element();
        writer.end_element();
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

#[derive(Debug)]
struct NumFmt {
    format_id: usize,
    format_code: String,
}

impl NumFmt {
    pub fn new(format_id: usize, format_code: &str) -> Self {
        Self {
            format_id,
            format_code: format_code.to_string(),
        }
    }
}

impl XMLString for NumFmt {
    fn to_xml(self, writer: &mut xmlwriter::XmlWriter) {
        writer.start_element("numFmt");
        writer.write_attribute("numFmtId", &self.format_id.to_string());
        writer.write_attribute("formatCode", &self.format_code);
        writer.end_element();
    }
}
