use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_until},
    character::complete::{alphanumeric1, multispace1},
    combinator::{opt, verify},
    multi::separated_list1,
    sequence::tuple,
    Err, IResult,
};

#[derive(Debug)]
pub struct FileInfo<'a> {
    pub path: &'a str,
    pub sheet: Option<&'a str>,
}

#[derive(Debug)]
pub struct Query<'a> {
    pub columns: Vec<&'a str>,
    pub file: FileInfo<'a>,
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let (remaining, (_, columns, (table, sheet))) =
        tuple((parse_select, parse_columns, parse_from))(input).unwrap();

    Ok((
        remaining,
        Query {
            columns,
            file: FileInfo { path: table, sheet },
        },
    ))
}

fn parse_sheet(input: &str) -> IResult<&str, Option<&str>> {
    let (remaining, sheet) = opt(tuple((
        multispace1,
        tag_no_case("SHEET"),
        multispace1,
        parse_until_next_keyword,
    )))(input)?;

    match sheet {
        Some((_, _, _, sheet)) => Ok((remaining, Some(sheet))),
        None => Ok((remaining, None)),
    }
}

fn parse_select(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag_no_case("SELECT"), multispace1))(input)
}

fn parse_from(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let (remaining, (_, _, _, table)) = tuple((
        multispace1,
        tag_no_case("FROM"),
        multispace1,
        parse_until_next_keyword,
    ))(input)?;

    if table.split(".").last().unwrap() == "csv" {
        verify(parse_sheet, |s| s.is_none())(remaining)?;

        Ok((remaining, (table, None)))
    } else {
        let (remaining, sheet) = parse_sheet(remaining)?;
        Ok((remaining, (table, sheet)))
    }
}

fn parse_until_next_keyword(input: &str) -> IResult<&str, &str> {
    alt((take_until(";"), take_until(" "), take_till(|c| c == '\0')))(input)
}

fn parse_columns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        tuple((
            take_until(","),
            tag(","),
            take_till(|c: char| c.is_alphanumeric()),
        )),
        alt((alphanumeric1, tag("*"))),
    )(input)
}
