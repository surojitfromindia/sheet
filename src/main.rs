pub mod row;
pub mod traits;
pub mod work_book;
pub mod work_sheet;
pub mod xml_templates;

use row::Cell;
use work_book::WorkBook;
use work_sheet::WorkSheet;

fn main() {
    let mut work_book = WorkBook::new();

    // create work sheet
    let mut work_sheet_1 = WorkSheet::blank("sheet 1");

    // add rows and cells to the worksheets
    let row = work_sheet_1.add_blank_row();
    row.add_string("32".to_string());
    row.add_string("Hello".to_string());

    let manual_cell = Cell::from_string("Hey".to_string(), "D2".to_string());

    row.add_cell(manual_cell).unwrap();

    work_book.add_sheet(work_sheet_1);

    work_book.save();
}
