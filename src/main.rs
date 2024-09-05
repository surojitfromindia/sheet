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
    let mut work_sheet_1 = WorkSheet::blank("i");

    // add rows and cells to the worksheets
    let row = work_sheet_1.add_blank_row();
    row.add_cell_of_number("32".to_string()).unwrap();
    row.add_cell_of_string("Hello".to_string());

    // let cell3 = Cell::of_string_with_reference("World".to_string(), "A1".to_string());
    // row.add_cell(cell3);

    work_book.add_sheet(work_sheet_1);

    work_book.save();
}
