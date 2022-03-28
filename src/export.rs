use crate::dataframe::Dataframe;

extern crate simple_excel_writer;
use simple_excel_writer as excel;

use excel::*;

use std::path::Path;

pub fn export_data(snapshots: &Vec<Dataframe>) -> Result<(), i32> {
    let fp = Path::new("./bikefitting.xlsx").to_str().unwrap();
    let mut wb = Workbook::create(fp);
    let mut sheet = wb.create_sheet("Bikefitting");

    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Nummer", "x", "y"])?;
        let mut row = 1;
        for snapshot in snapshots {
            sw.append_row(row![row.to_string(), snapshot.x.to_string(), snapshot.y.to_string()]);
            row += 1;
        }
        Ok(())
    }).expect("write error!");

    wb.close().expect("close error!");
    Ok(())
}
