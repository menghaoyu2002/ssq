use clap::Parser;

/// Run SQL queries on spreadsheets
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path of the spreadsheet
    #[arg(short, long)]
    path: Option<String>,

    /// name of the sheet within the workbook (for xlsx files only)
    #[arg(short, long)]
    sheet: Option<String>,

    /// the query to execute
    #[arg(short, long)]
    query: String,
}

fn main() {
    let args = Args::parse();
}
