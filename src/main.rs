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
    let mut work_sheet_1 = WorkSheet::blank("Sheet1");

    // add rows and cells to the worksheets
    work_sheet_1.add_blank_row().add_cells(vec![
        Cell::new("Octobar".to_string(), "A1".to_string()),
        Cell::new("Nov".to_string(), "A2".to_string()),
        Cell::new("Dec".to_string(), "A3".to_string()),
        Cell::new("Nov".to_string(), "A4".to_string()),
        Cell::new("Jan".to_string(), "A5".to_string()),
    ]);

    // another blank row and more cells to that row.
    work_sheet_1.add_blank_row().add_cells(vec![
        Cell::new("Pen".to_string(), "A1".to_string()),
        Cell::new("Dan".to_string(), "A2".to_string()),
        Cell::new("Dec".to_string(), "A3".to_string()),
        Cell::new("Copy cat".to_string(), "A4".to_string()),
        Cell::new("Jan".to_string(), "A5".to_string()),
    ]);

    let work_sheet_2 = WorkSheet::blank("Hey bro");


    work_book.add_sheet(work_sheet_1);
    work_book.add_sheet(work_sheet_2);

    work_book.save();
}
