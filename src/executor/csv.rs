use std::fs::File;

use crate::parser::Query;

use super::Executor;

pub struct CsvExecutor {
    file: csv::Reader<File>,
}

impl CsvExecutor {
    pub fn new(path: &str) -> Self {
        Self {
            file: csv::Reader::from_path(path).unwrap(),
        }
    }
}

impl Executor for CsvExecutor {
    fn execute_query(&self, query: &Query) {}
}
