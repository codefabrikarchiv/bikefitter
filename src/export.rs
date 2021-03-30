use crate::dataframe::Dataframe;

extern crate nfd;
use nfd::Response;

use xlsxwriter::Workbook;
use xlsxwriter::XlsxError;

pub fn export_data(snapshots: &Vec<Dataframe>) -> Result<(), XlsxError> {
    let result = nfd::open_save_dialog(Some("xlsx"), None).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    match result {
        Response::Cancel => {
            println!("User canceled");
            Ok(())
        }
        Response::Okay(file_path) => {
            let fp = file_path + ".xlsx";
            let workbook = Workbook::new(&fp);
            let mut sheet = workbook.add_worksheet(None)?;
            sheet.write_string(0, 0, "Nummer", None)?;
            sheet.write_string(0, 1, "x", None)?;
            sheet.write_string(0, 2, "y", None)?;

            let mut row = 1;
            for snapshot in snapshots {
                sheet.write_number(row, 0, row.into(), None)?;
                sheet.write_number(row, 1, snapshot.x.into(), None)?;
                sheet.write_number(row, 2, snapshot.y.into(), None)?;
                row += 1;
            }
            workbook.close()?;
            Ok(())
        }
        Response::OkayMultiple(files) => {
            println!("Files {:?}", files);
            Ok(())
        }
    }
}
