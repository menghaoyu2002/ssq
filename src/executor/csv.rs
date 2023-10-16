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
        let mut rows: Vec<HashMap<String, serde_json::Value>> = vec![];

        for record in records {
            let record = record.unwrap();
            let mut row: HashMap<String, serde_json::Value> = HashMap::new();

            for (i, header) in headers.iter().enumerate() {
                if query.columns.contains(&header) {
                    row.insert(header.to_string(), to_json_value(&record[i]));
                }
            }

            if row.len() > 0 {
                rows.push(row);
            }
        }

        serde_json::to_string_pretty(&rows)
    }
}

fn to_json_value(value: &str) -> serde_json::Value {
    match value.parse::<i64>() {
        Ok(v) => serde_json::Value::Number(serde_json::Number::from(v)),
        Err(_) => match value.parse::<f64>() {
            Ok(v) => match serde_json::Number::from_f64(v) {
                Some(n) => serde_json::Value::Number(n),
                None => serde_json::Value::String(value.to_string()),
            },
            Err(_) => serde_json::Value::String(value.to_string()),
        },
    }
}
