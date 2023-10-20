use std::process::exit;

use colored::Colorize;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_until},
    character::complete::{alphanumeric1, multispace0, multispace1},
    combinator::{opt, verify},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
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
    pub conditions: Option<LogicalExpression>,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl ComparisonOperator {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "=" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            "<>" => Some(Self::NotEqual),
            ">" => Some(Self::GreaterThan),
            "<" => Some(Self::LessThan),
            ">=" => Some(Self::GreaterThanOrEqual),
            "<=" => Some(Self::LessThanOrEqual),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "AND" => Some(Self::And),
            "OR" => Some(Self::Or),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Predicate {
    pub column: String,
    pub operator: ComparisonOperator,
    pub value: String,
}

#[derive(Debug)]
pub struct Condition {
    pub left: Box<LogicalExpression>,
    pub right: Box<LogicalExpression>,
    pub operator: LogicalOperator,
}

#[derive(Debug)]
pub enum LogicalExpression {
    Predicate(Predicate),
    Condition(Condition),
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let (remaining, (_, columns, (table, sheet))) =
        tuple((parse_select, parse_columns, parse_from))(input).unwrap();

    let (remaining, conditions) = parse_where(remaining, &columns)?;

    Ok((
        remaining,
        Query {
            columns: columns.clone(),
            file: FileInfo { path: table, sheet },
            conditions,
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
    alt((take_until(" "), take_until(";"), take_till(|c| c == '\0')))(input)
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

fn parse_where<'a>(
    input: &'a str,
    columns: &Vec<&str>,
) -> IResult<&'a str, Option<LogicalExpression>> {
    let (remaining, where_claus)= opt(tuple((multispace1, tag_no_case("WHERE"))))(input)?;

    match where_claus {
        Some((_, _)) => {
            let (remaining, conditions) = parse_conditions(remaining, columns)?;
            Ok((remaining, Some(conditions)))
        }
        None => Ok((remaining, None)),
    }

}

fn parse_logical_operator(input: &str) -> IResult<&str, &str> {
    alt((tag_no_case("OR"), tag_no_case("AND")))(input)
}

fn parse_conditions<'a>(
    input: &'a str,
    columns: &Vec<&str>,
) -> IResult<&'a str, LogicalExpression> {
    let (remaining, (predicate1, maybe_predicate2, _)) = tuple((
        |input| parse_predicate(columns, input),
        opt(tuple((multispace1, parse_logical_operator, |input| {
            parse_predicate(columns, input)
        }))),
        multispace0,
    ))(input)?;

    let left = match maybe_predicate2 {
        Some((_, operator, predicate2)) => LogicalExpression::Condition(Condition {
            left: Box::new(LogicalExpression::Predicate(predicate1)),
            right: Box::new(LogicalExpression::Predicate(predicate2)),
            operator: LogicalOperator::from_str(operator).unwrap(),
        }),
        None => LogicalExpression::Predicate(predicate1),
    };

    let additional_conditions = parse_logical_operator(remaining);
    match additional_conditions {
        Ok((remaining, operator)) => {
            let (remaining, right) = parse_conditions(remaining, columns)?;
            Ok((
                remaining,
                LogicalExpression::Condition(Condition {
                    left: Box::new(left),
                    right: Box::new(right),
                    operator: LogicalOperator::from_str(operator).unwrap(),
                }),
            ))
        }
        Err(_) => Ok((remaining, left)),
    }
}

fn parse_string_value(input: &str) -> IResult<&str, &str> {
    alt((
        alphanumeric1,
        delimited(tag("'"), alphanumeric1, tag("'")),
        delimited(tag("\""), alphanumeric1, tag("\"")),
    ))(input)
}
fn parse_predicate<'a>(columns: &Vec<&str>, input: &'a str) -> IResult<&'a str, Predicate> {
    let (remaining, (_, s1, _, comp, _, s2)) = tuple((
        multispace1,
        alt((
            alphanumeric1,
            delimited(tag("'"), alphanumeric1, tag("'")),
            delimited(tag("\""), alphanumeric1, tag("\"")),
        )),
        multispace0,
        alt((
            tag(">="),
            tag(">"),
            tag("<="),
            tag("<"),
            tag("="),
            tag("!="),
            tag("<>"),
        )),
        multispace0,
        parse_string_value,
    ))(input)?;

    if columns.contains(&s1) {
        Ok((
            remaining,
            Predicate {
                column: s1.to_string(),
                operator: ComparisonOperator::from_str(comp).unwrap(),
                value: s2.to_string(),
            },
        ))
    } else if columns.contains(&s2) {
        Ok((
            remaining,
            Predicate {
                column: s2.to_string(),
                operator: ComparisonOperator::from_str(comp).unwrap(),
                value: s1.to_string(),
            },
        ))
    } else {
        eprintln!("{}: column {} not found", "error".bold().red(), s1);
        exit(1);
    }
}
