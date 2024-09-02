use std::{collections::HashMap, mem};

use xmlwriter::Options;

#[derive(Debug)]
pub struct SharedStrings {
    // first one is the index, second one is the total counter
    pub s_map: HashMap<String, u32>,
    pub next_index: u32,
    pub total_counter: u32,
}

static SST_XMLNS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";

impl SharedStrings {
    pub fn new() -> Self {
        SharedStrings {
            next_index: 1,
            s_map: HashMap::new(),
            total_counter: 0,
        }
    }
    pub fn add_string(&mut self, st: &mut String) -> u32 {
        // increase the counter by 1 in any case.
        self.total_counter += 1;
        if let Some(ind) = self.s_map.get(st) {
            // a item is found then replace
            let _ = mem::replace(st, ind.to_string());
            *ind
        } else {
            // insert and increament the next_counter by 1
            let key = mem::replace(st, self.next_index.to_string());
            self.s_map.insert(key, self.next_index);
            {
                let temp = self.next_index;
                self.next_index += 1;
                temp
            }
        }
    }
    pub fn to_xml(self) -> String {
        let mut writer = xmlwriter::XmlWriter::new(Options::default());
        // todo: write declarion manually.
        writer.write_declaration();
        writer.start_element("sst");
        writer.write_attribute("xmls", SST_XMLNS);
        writer.write_attribute("count", &self.total_counter.to_string());
        writer.write_attribute("uniqueCount", &self.s_map.len());
        for elem in self.s_map.keys() {
            writer.start_element("si");
            writer.start_element("t");
            writer.write_text(&elem);
            writer.end_element();
            writer.end_element();
        }
        writer.end_document()
    }
}
