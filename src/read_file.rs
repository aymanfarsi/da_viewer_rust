use std::{fs, io::Cursor, path::Path};

use calamine::{open_workbook, DataType, Reader, Xlsx};
use polars::prelude::{JsonReader, LazyCsvReader, LazyFileListReader, LazyFrame, SerReader};

use crate::models::TableStruct;

pub fn load_data_from_file(file_path: String) -> Result<TableStruct, String> {
    let path = Path::new(&file_path);
    match path.extension().unwrap().to_str().unwrap() {
        "csv" => read_csv(file_path),
        "parquet" => read_parquet(file_path),
        "json" => read_json(file_path),
        "xlsx" => read_excel(file_path),
        _ => Err("File extension not supported yet".to_string()),
    }
}

fn read_csv(file_path: String) -> Result<TableStruct, String> {
    let res = LazyCsvReader::new(file_path).finish();
    match res {
        Ok(lf) => {
            let df = lf.collect().unwrap();

            let columns = df
                .get_column_names()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let mut rows: Vec<Vec<String>> = vec![];
            for row in df.iter() {
                let mut row_vec: Vec<String> = vec![];
                for col in row.iter() {
                    row_vec.push(col.to_string());
                }
                rows.push(row_vec);
            }

            Ok(TableStruct { columns, rows })
        }
        Err(err) => Err(err.to_string()),
    }
}

fn read_excel(file_path: String) -> Result<TableStruct, String> {
    let mut workbook: Xlsx<_> = open_workbook(file_path).expect("Cannot open file");

    let res = workbook.worksheet_range_at(0);

    match res {
        None => Err("No sheet found".to_string()),
        Some(range) => {
            let range = range.unwrap();

            let columns = range
                .rows()
                .next()
                .unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let mut rows: Vec<Vec<String>> = vec![];
            for (idx, r) in range.rows().skip(1).enumerate() {
                rows.push(vec![]);
                for (_, c) in r.iter().enumerate() {
                    match *c {
                        DataType::Empty => {
                            rows[idx].push("null".to_string());
                        }
                        DataType::String(ref s)
                        | DataType::DateTimeIso(ref s)
                        | DataType::DurationIso(ref s) => {
                            rows[idx].push(s.to_string());
                        }
                        DataType::Float(ref f_type)
                        | DataType::DateTime(ref f_type)
                        | DataType::Duration(ref f_type) => {
                            rows[idx].push(f_type.to_string());
                        }
                        DataType::Int(ref i) => {
                            rows[idx].push(i.to_string());
                        }
                        DataType::Error(ref e) => {
                            return Err(e.to_string());
                        }
                        DataType::Bool(ref b) => {
                            rows[idx].push(b.to_string());
                        }
                    };
                }
            }

            Ok(TableStruct { columns, rows })
        }
    }
}

fn read_parquet(file_path: String) -> Result<TableStruct, String> {
    let res = LazyFrame::scan_parquet(file_path, Default::default());
    match res {
        Ok(lf) => {
            let df = lf.collect().unwrap();

            let columns = df
                .get_column_names()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let mut rows: Vec<Vec<String>> = vec![];
            for row in df.iter() {
                let mut row_vec: Vec<String> = vec![];
                for col in row.iter() {
                    row_vec.push(col.to_string());
                }
                rows.push(row_vec);
            }

            Ok(TableStruct { columns, rows })
        }
        Err(err) => Err(err.to_string()),
    }
}

fn read_json(file_path: String) -> Result<TableStruct, String> {
    let cursor = Cursor::new(fs::read_to_string(file_path).unwrap());
    let res = JsonReader::new(cursor).finish();
    match res {
        Ok(df) => {
            let columns = df
                .get_column_names()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let mut rows: Vec<Vec<String>> = vec![];
            for row in df.iter() {
                let mut row_vec: Vec<String> = vec![];
                for col in row.iter() {
                    row_vec.push(col.to_string());
                }
                rows.push(row_vec);
            }

            Ok(TableStruct { columns, rows })
        }
        Err(err) => Err(err.to_string()),
    }
}
