# ssq
use sql to query spreadsheets.

Takes in a SQL query and returns an array of JSON objects representing each row.

Only supports the syntax

`SELECT [COLUMNS] FROM [PATH_TO_SPREADSHEET] SHEET [SHEET_WITHIN_SPREADSHEET] WHERE [CONDITIONS]`

`SHEET` keyword is optional and only required for filetypes that can contain multiple sheets.

<sub><sup>take it easy on me, this is my first time writing rust. i feel like im doing something wrong but i don't know the right way to do it so i'm leaving it like this. i'm sorry for the rust crimes i'm committing</sup></sub>
