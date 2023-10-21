use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    process::exit,
};

use crate::parser::{LogicalExpression, Query};

use super::{Executor, JsonValue};
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use colored::Colorize;

pub struct XlsxExecutor {
    workbook: Xlsx<BufReader<File>>,
    tables: HashMap<String, Range<DataType>>,
}

impl XlsxExecutor {
    pub fn new(path: &str) -> Self {
        let workbook = match open_workbook(path) {
            Ok(wb) => wb,
            Err(e) => {
                eprintln!(
                    "{} failed to open {}, {}",
                    "error:".red().bold(),
                    path.to_string().bold(),
                    e
                );
                exit(1);
            }
        };

        Self {
            workbook,
            tables: HashMap::new(),
        }
    }
}

impl JsonValue for DataType {
    fn to_value(&self) -> serde_json::Value {
        match self {
            DataType::Empty => serde_json::Value::Null,
            DataType::Bool(b) => serde_json::Value::Bool(*b),
            DataType::Int(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DataType::Float(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap())
            }
            DataType::String(s) => serde_json::Value::String(s.to_string()),
            DataType::DateTime(dt) => serde_json::Value::String(dt.to_string()),
            DataType::Error(e) => serde_json::Value::String(e.to_string()),
            DataType::Duration(d) => serde_json::Value::String(d.to_string()),
            DataType::DateTimeIso(dt) => serde_json::Value::String(dt.to_string()),
            DataType::DurationIso(d) => serde_json::Value::String(d.to_string()),
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
            range = match self
                .workbook
                .worksheet_range(&query.file.sheet.as_ref().unwrap())
            {
                Some(Ok(range)) => range,
                Some(Err(e)) => {
                    eprintln!(
                        "{} failed to open sheet '{}', {}",
                        "error:".red().bold(),
                        query.file.sheet.as_ref().unwrap(),
                        e
                    );
                    exit(1);
                },
                None => {
                    eprintln!(
                        "{} sheet '{}' does not exist in file '{}'",
                        "error:".red().bold(),
                        query.file.sheet.as_ref().unwrap(),
                        query.file.path
                    );
                    exit(1);
                }
            };
            self.tables.insert(
                query.file.sheet.as_ref().unwrap().to_string(),
                range.clone(),
            );
        }

        let mut iter = range.rows().into_iter();
        let headers = iter
            .next()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>();

        // verify that every column in the query exists in the sheet
        for column in &query.columns {
            if !headers.contains(&column.to_string()) {
                eprintln!(
                    "{} column '{}' does not exist in sheet '{}'",
                    "error:".red().bold(),
                    column,
                    query.file.sheet.as_ref().unwrap()
                );
                exit(1);
            }
        }

        let mut rows: Vec<BTreeMap<String, serde_json::Value>> = vec![];

        for row in iter {
            let mut record: BTreeMap<String, serde_json::Value> = BTreeMap::new();
            let mut full_row: BTreeMap<String, serde_json::Value> = BTreeMap::new();
            for (i, cell) in row.iter().enumerate() {
                if query.columns.contains(&headers[i].to_string().as_str()) {
                    record.insert(headers[i].to_string(), cell.to_value());
                }
                full_row.insert(headers[i].to_string(), cell.to_value());
            }

            let should_include = match &query.conditions {
                Some(logical_expression) => {
                    LogicalExpression::evaluate_conditions(&logical_expression, &full_row)
                }
                None => true,
            };
            if record.len() > 0 && should_include {
                rows.push(record);
            }
        }

        serde_json::to_string_pretty(&rows)
    }
}
