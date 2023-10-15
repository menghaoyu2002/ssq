use std::{collections::HashMap, fs::File, io::BufReader};

use crate::parser::Query;

use super::Executor;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use colored::Colorize;

pub struct XlsxExecutor {
    workbook: Xlsx<BufReader<File>>,
    tables: HashMap<String, Range<DataType>>,
}

impl XlsxExecutor {
    pub fn new(path: &str) -> Self {
        let workbook: Xlsx<_> = open_workbook(path).expect(&format!(
            "{} failed to open {}",
            "error".red().bold(),
            path
        ));

        Self {
            workbook,
            tables: HashMap::new(),
        }
    }
}

impl Executor for XlsxExecutor {
    fn execute_query(&mut self, query: &Query) -> Result<String, serde_json::Error> {
        let range;

        // this one feels particularly bad, please fix
        if self
            .tables
            .contains_key(&query.file.sheet.as_ref().unwrap().to_string())
        {
            range = self
                .tables
                .get(&query.file.sheet.as_ref().unwrap().to_string())
                .unwrap()
                .clone();
        } else {
            range = self
                .workbook
                .worksheet_range(&query.file.sheet.as_ref().unwrap())
                .expect(&format!(
                    "{} failed to open sheet {}",
                    "error".red().bold(),
                    query.file.sheet.as_ref().unwrap()
                ))
                .unwrap();

            self.tables.insert(
                query.file.sheet.as_ref().unwrap().to_string(),
                range.clone(),
            );
        }

        let mut iter = range.rows().into_iter();
        let headers = iter.next().unwrap();
        let mut rows = vec![];

        for row in iter {
            let mut record = HashMap::new();
            for (i, cell) in row.iter().enumerate() {
                if query.columns.contains(&headers[i].to_string().as_str()) {
                    record.insert(headers[i].to_string(), cell.to_string());
                }
            }
            
            if record.len() > 0 {
                rows.push(record);
            }
        }

        serde_json::to_string_pretty(&rows)
    }
}
