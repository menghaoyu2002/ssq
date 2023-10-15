use std::{collections::HashMap, fs::File, io::BufReader};

use crate::parser::Query;

use super::Executor;
use calamine::{open_workbook, DataType, Range, Xlsx};
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
    fn execute_query(&self, query: &Query) {}
}
