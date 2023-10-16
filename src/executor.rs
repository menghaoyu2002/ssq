use crate::{filetypes::FileType, parser::Query};

use self::{csv::CsvExecutor, xlsx::XlsxExecutor};

mod csv;
mod xlsx;

pub trait Executor {
    fn execute_query(&mut self, query: &Query) -> Result<String, serde_json::Error>;
}

pub trait JsonValue {
    fn to_value(&self) -> serde_json::Value;
}

pub fn get_executor(path: &str, filetype: FileType) -> Box<dyn Executor> {
    match filetype {
        FileType::SingleSheetFileType(_) => Box::new(CsvExecutor::new(path)),
        FileType::MultiSheetFiletype(_) => Box::new(XlsxExecutor::new(path)),
    }
}
