use std::{
    io::{self, Read},
    process::exit,
};

use clap::Parser;
use colored::Colorize;
use executor::get_executor;
use filetypes::FileType;
use parser::parse_query;

mod executor;
mod filetypes;
pub mod parser;

/// Run SQL queries on spreadsheets and outputs the result in JSON
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// SQL query to execute
    #[arg(short, long)]
    query: Option<String>,
}

fn main() {
    let mut optional_query_string = Args::parse().query;

    // read from stdin if no query string is given
    if optional_query_string.is_none() && !atty::is(atty::Stream::Stdin) {
        let mut q = String::new();
        io::stdin().read_to_string(&mut q).unwrap();
        optional_query_string = Some(q);
    }

    match optional_query_string {
        Some(query_string) => {
            let parsed = parse_query(&query_string);
            let query = match parsed {
                Ok((_, query)) => {
                    query
                }
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            };


            match FileType::parse_to_filetype(query.file.path.split(".").last()) {
                Some(filetype) => {
                    if matches!(&filetype, FileType::MultiSheetFiletype(_))
                        && query.file.sheet.is_none()
                    {
                        eprintln!(
                            "{} a sheet is required for {} files",
                            "error:".red().bold(),
                            filetype.to_string().bold()
                        );
                        exit(1);
                    }

                    let mut executor = get_executor(&query.file.path, filetype);
                    let data = executor.execute_query(&query);

                    match data {
                        Ok(data) => println!("{}", data),
                        Err(e) => {
                            eprintln!("{} {}", "error:".red().bold(), e);
                            exit(1);
                        }
                    }
                }
                None => {
                    eprintln!("{} unsupported filetype", "error:".red().bold());
                    exit(1)
                }
            }
        }
        None => {
            eprintln!(
                "{} missing {} argument",
                "error:".red().bold(),
                "\'--query <QUERY>\'".yellow()
            );
            exit(1);
        }
    }
}
