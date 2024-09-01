use xmlwriter::*;

pub trait XMLString {
    fn to_xml(self, writer: &mut XmlWriter) ;
}