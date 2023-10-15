use std::{collections::HashMap, fs::File};

use crate::parser::Query;

use super::Executor;

pub struct CsvExecutor {
    file: csv::Reader<File>,
}

impl CsvExecutor {
    pub fn new(path: &str) -> Self {
        Self {
            file: csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path(path)
                .unwrap(),
        }
    }
}

impl Executor for CsvExecutor {
    fn execute_query(&mut self, query: &Query) -> Result<String, serde_json::Error> {
        let mut records = self.file.records();
        let headers = records.next().unwrap().unwrap();
        let mut rows = vec![];

        for record in records {
            let record = record.unwrap();
            let mut row = HashMap::new();

            for (i, header) in headers.iter().enumerate() {
                if query.columns.contains(&header) {
                    row.insert(header.to_string(), record[i].to_string());
                }
            }

            if row.len() > 0 {
                rows.push(row);
            }
        }

        serde_json::to_string_pretty(&rows)
    }
}
