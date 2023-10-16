use std::{collections::{HashMap, BTreeMap}, fs::File, io::BufReader};

use crate::parser::Query;

use super::{Executor, JsonValue};
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

impl JsonValue for DataType {
    fn to_value(&self) -> serde_json::Value {
        match self {
            DataType::Empty => serde_json::Value::Null,
            DataType::Bool(b) => serde_json::Value::Bool(*b),
            DataType::Int(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            DataType::Float(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap())
            },
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
        let mut rows: Vec<BTreeMap<String, serde_json::Value>> = vec![];

        for row in iter {
            let mut record: BTreeMap<String, serde_json::Value> = BTreeMap::new();
            for (i, cell) in row.iter().enumerate() {
                if query.columns.contains(&headers[i].to_string().as_str()) {
                    record.insert(headers[i].to_string(), cell.to_value());
                }
            }

            if record.len() > 0 {
                rows.push(record);
            }
        }

        serde_json::to_string_pretty(&rows)
    }
}
