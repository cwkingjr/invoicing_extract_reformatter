use polars::prelude::*;
//use rust_xlsxwriter::{Workbook};
use calamine::{Reader, open_workbook, Xlsx, Data};
use std::path::{Path};
use std::env::args;
use std::process::exit;

fn main() -> Result<(), PolarsError> {
    // 1. Executable, 2. File path
    if args().count() < 2 {
        let executable_name = args().next().unwrap();
        eprintln!("File path command-line argument not provided. Usage: {executable_name} <file_path>");
        exit(-1);
    }

    let file_path = args().nth(1).unwrap();

    let path = Path::new(&file_path);


    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    if let Ok(range) = workbook.worksheet_range("Sheet1") {

        for row in range.rows(){
            println!("{:?}\n", &row);
        }
    }

    Ok(())
}
