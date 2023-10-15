use core::fmt;

#[derive(Debug)]
pub enum MultiSheetFileType {
    ODS,
    XLA,
    XLAM,
    XLS,
    XLSB,
    XLSM,
    XLSX,
}

#[derive(Debug)]
pub enum SingleSheetFileType {
    CSV,
}

pub enum FileType {
    SingleSheetFileType(SingleSheetFileType),
    MultiSheetFiletype(MultiSheetFileType),
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::MultiSheetFiletype(s) => write!(f, "{}", s.to_string()),
            FileType::SingleSheetFileType(s) => write!(f, "{}", s.to_string()),
        }
    }
}

impl fmt::Display for SingleSheetFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SingleSheetFileType::CSV => write!(f, "csv"),
        }
    }
}

impl fmt::Display for MultiSheetFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultiSheetFileType::ODS => write!(f, "ods"),
            MultiSheetFileType::XLA => write!(f, "xla"),
            MultiSheetFileType::XLAM => write!(f, "xlam"),
            MultiSheetFileType::XLS => write!(f, "xls"),
            MultiSheetFileType::XLSB => write!(f, "xlsb"),
            MultiSheetFileType::XLSM => write!(f, "xlsm"),
            MultiSheetFileType::XLSX => write!(f, "xlsx"),
        }
    }
}

impl FileType {
    pub fn parse_to_filetype(candidate: Option<&str>) -> Option<FileType> {
        match candidate?.to_lowercase().as_str() {
            "csv" => Some(FileType::SingleSheetFileType(SingleSheetFileType::CSV)),
            "xlsx" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::XLSX)),
            "ods" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::ODS)),
            "xla" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::XLA)),
            "xlam" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::XLAM)),
            "xls" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::XLS)),
            "xlsb" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::XLSB)),
            "xlsm" => Some(FileType::MultiSheetFiletype(MultiSheetFileType::XLSM)),
            _ => None,
        }
    }
}
