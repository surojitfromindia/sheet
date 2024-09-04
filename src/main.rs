pub mod cell;
pub mod row;
pub mod traits;
pub mod work_book;
pub mod work_sheet;
pub mod xml_templates;

use cell::Cell;
use work_book::WorkBook;
use work_sheet::WorkSheet;

fn main() {
    let mut work_book = WorkBook::new();

    // create work sheet
    let mut work_sheet_1 = WorkSheet::blank("we");

    // add rows and cells to the worksheets
    work_sheet_1
        .add_blank_row()
        .add_cells(vec![
            Cell::of_string("Octobar".to_string()),
            Cell::of_string("Nov".to_string()),
            Cell::of_string("Nov".to_string()),
        ])
        .unwrap();

    work_book.add_sheet(work_sheet_1);

    work_book.save();
}
