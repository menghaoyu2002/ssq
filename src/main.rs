use std::process::exit;

use clap::Parser;
use colored::Colorize;

/// Run SQL queries on spreadsheets and outputs the result in JSON
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the spreadsheet
    #[arg(short, long)]
    path: String,

    /// Name of the sheet within the workbook
    #[arg(short, long)]
    sheet: Option<String>,

    /// SQL query to execute
    #[arg(short, long)]
    query: String,
}

const SHEET_REQUIRED: [&str; 7] = ["xlsx", "xls", "xlsm", "xlsb", "xla", "xlam", "ods"];
const SUPPORTED_FILETYPES: [&str; 8] = ["csv", "xlsx", "xls", "xlsm", "xlsb", "xla", "xlam", "ods"];

fn main() {
    let args = Args::parse();

    let path_parts: Vec<&str> = args.path.split(".").collect();
    let filetype = path_parts.last().unwrap_or(&"");

    if !SUPPORTED_FILETYPES.contains(filetype) {
        println!(
            "{} only {} files are supported",
            "error:".red().bold(),
            SUPPORTED_FILETYPES.join(", ").bold().yellow()
        );
        exit(1);
    }

    if SHEET_REQUIRED.contains(filetype) && args.sheet.is_none() {
        println!(
            "{} a sheet name {} is required for {} files",
            "error:".red().bold(),
            "'--sheet <SHEET>'".yellow(),
            filetype
        );
        exit(1);
    }
}
