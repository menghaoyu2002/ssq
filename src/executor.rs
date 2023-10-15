use crate::{filetypes::FileType, parser::Query};

use self::{csv::CsvExecutor, xlsx::XlsxExecutor};

mod csv;
mod xlsx;

pub trait Executor {
    fn execute_query(&self, query: &Query);
}

pub fn get_executor(path: &str, filetype: FileType) -> Box<dyn Executor> {
    match filetype {
        FileType::SingleSheetFileType(_) => Box::new(CsvExecutor::new(path)),
        FileType::MultiSheetFiletype(_) => Box::new(XlsxExecutor::new(path)),
    }
}
