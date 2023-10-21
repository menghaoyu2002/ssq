use std::{collections::BTreeMap, fs::File, process::exit};

use colored::Colorize;

use crate::parser::{LogicalExpression, Query};

use super::{str_to_json_value, Executor};

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
        let headers = records
            .next()
            .unwrap()
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // verify that all columns in the query exist in the spreadsheet
        for column in &query.columns {
            if !headers.contains(&column.to_string()) {
                eprintln!(
                    "{} column '{}' does not exist in file '{}'",
                    "error:".red().bold(),
                    column,
                    query.file.path
                );
                exit(1);
            }
        }

        let mut rows: Vec<BTreeMap<String, serde_json::Value>> = vec![];

        for record in records {
            let record = record.unwrap();
            let mut row: BTreeMap<String, serde_json::Value> = BTreeMap::new();
            let mut full_row: BTreeMap<String, serde_json::Value> = BTreeMap::new();

            for (i, header) in headers.iter().enumerate() {
                full_row.insert(header.to_string(), str_to_json_value(&record[i]));
                if query.columns.contains(&header.as_str()) {
                    row.insert(header.to_string(), str_to_json_value(&record[i]));
                }
            }

            let should_add = match &query.conditions {
                Some(logical_expression) => {
                    LogicalExpression::evaluate_conditions(&logical_expression, &row)
                }
                None => true,
            };


            if row.len() > 0 && should_add {
                rows.push(row);
            }
        }

        serde_json::to_string_pretty(&rows)
    }
}
