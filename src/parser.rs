use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_until},
    character::complete::{alphanumeric1, multispace1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub struct Filetype<'a> {
    pub path: &'a str,
    pub sheet: Option<&'a str>,
}

pub struct Query<'a> {
    pub columns: Vec<&'a str>,
    pub file: Filetype<'a>,
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let (remaining, (_, columns, (_, _, _, table), sheet)) =
        tuple((parse_select, parse_columns, parse_from, parse_sheet))(input).unwrap();

    Ok((
        remaining,
        Query {
            columns,
            file: Filetype { path: table, sheet },
        },
    ))
}

fn parse_sheet(input: &str) -> IResult<&str, Option<&str>> {
    match tuple((
        multispace1,
        tag_no_case("SHEET"),
        multispace1,
        parse_until_next_keyword,
    ))(input)
    {
        Ok((remaining, (_, _, _, s))) => Ok((remaining, Some(s))),
        _ => Ok((input, None)),
    }
}

fn parse_select(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag_no_case("SELECT"), multispace1))(input)
}

fn parse_from(input: &str) -> IResult<&str, (&str, &str, &str, &str)> {
    tuple((
        multispace1,
        tag_no_case("FROM"),
        multispace1,
        parse_until_next_keyword,
    ))(input)
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
        alphanumeric1,
    )(input)
}
