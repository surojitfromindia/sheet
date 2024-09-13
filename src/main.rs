pub mod cell;
pub mod row;
pub mod traits;
pub mod work_book;
pub mod work_sheet;
pub mod xml_templates;

use work_book::WorkBook;
use work_sheet::WorkSheet;
use xml_templates::style::{FontStyle, UnderLine};

struct StudentMarks {
    name: String,
    english: String,
    maths: String,
    science: String,
}
impl StudentMarks {
    fn new(name: &str, english: &str, maths: &str, science: &str) -> Self {
        StudentMarks {
            name: name.to_string(),
            english: english.to_string(),
            maths: maths.to_string(),
            science: science.to_string(),
        }
    }
}

fn main() {
    // create a work book
    let mut work_book = WorkBook::new();

    // create work sheet
    let mut work_sheet_1 = WorkSheet::blank("sheet 1");

    // add headers
    let header = work_sheet_1.add_blank_row();
    header.add_string("Name".to_string());
    header.add_string("English".to_string());
    header.add_string("Maths".to_string());
    header.add_string("Science".to_string());

    // add student marks
    let marks = vec![
        StudentMarks::new("Copper", "90", "80", "70"),
        StudentMarks::new("Gold", "80", "70", "60"),
        StudentMarks::new("Silver", "70", "60", "50"),
    ];
    for mark in marks {
        let row = work_sheet_1.add_blank_row();
        row.add_string(mark.name);
        row.add_number(mark.english).unwrap();
        row.add_number(mark.maths).unwrap();
        row.add_number(mark.science).unwrap();
    }

    // add this work sheet to the work book
    work_book.add_sheet(work_sheet_1);

    let mut work_sheet_2 = WorkSheet::blank("sheet 2");
    let row_1 = work_sheet_2.add_blank_row();

    // bold cell
    let bold_cell = row_1.add_string("Bold word".to_string());
    let font_style = FontStyle::new().bold(true);
    bold_cell.set_font_style(font_style);

    // strike cell
    let strike_cell = row_1.add_string("Crossed this word".to_string());
    let s_style = FontStyle::new().strike(true);
    strike_cell.set_font_style(s_style);

    let under_line_cell = row_1.add_string("Has undeline".to_string());
    let und_style = FontStyle::new().underline(Some(UnderLine::Double));
    under_line_cell.set_font_style(und_style);

    


    // add this work sheet to the work book
    work_book.add_sheet(work_sheet_2);

    // save the work book
    work_book.save();
}
