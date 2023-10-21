use std::collections::BTreeMap;

use crate::{
    filetypes::FileType,
    parser::{ComparisonOperator, LogicalExpression, LogicalOperator, Query},
};

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

impl LogicalExpression {
    fn evaluate_conditions(
        logical_expression: &LogicalExpression,
        row: &BTreeMap<String, serde_json::Value>,
    ) -> bool {
        match logical_expression {
            LogicalExpression::Predicate(predicate) => {
                let row_value = row.get(&predicate.column).unwrap();
                let compare_to = &str_to_json_value(&predicate.value);

                let left = json_to_number(row_value);
                let right = json_to_number(compare_to);

                match predicate.operator {
                    ComparisonOperator::Equal => match (left, right) {
                        (Some(left), Some(right)) => left == right,
                        _ => row_value.to_string() == compare_to.to_string(),
                    },
                    ComparisonOperator::NotEqual => match (left, right) {
                        (Some(left), Some(right)) => left != right,
                        _ => row_value.to_string() != compare_to.to_string(),
                    },
                    ComparisonOperator::GreaterThan => match (left, right) {
                        (Some(left), Some(right)) => left > right,
                        _ => row_value.to_string() > compare_to.to_string(),
                    },
                    ComparisonOperator::LessThan => match (left, right) {
                        (Some(left), Some(right)) => left < right,
                        _ => row_value.to_string() < compare_to.to_string(),
                    },
                    ComparisonOperator::GreaterThanOrEqual => match (left, right) {
                        (Some(left), Some(right)) => left >= right,
                        _ => row_value.to_string() >= compare_to.to_string(),
                    },
                    ComparisonOperator::LessThanOrEqual => match (left, right) {
                        (Some(left), Some(right)) => left <= right,
                        _ => row_value.to_string() <= compare_to.to_string(),
                    },
                }
            }
            LogicalExpression::Condition(condition) => {
                let left = LogicalExpression::evaluate_conditions(&condition.left, row);
                let right = LogicalExpression::evaluate_conditions(&condition.right, row);

                match condition.operator {
                    LogicalOperator::And => left && right,
                    LogicalOperator::Or => left || right,
                }
            }
        }
    }
}

pub fn str_to_json_value(value: &str) -> serde_json::Value {
    if value == "" {
        return serde_json::Value::Null;
    }

    if value == "true" || value == "false" {
        return serde_json::Value::Bool(value.parse::<bool>().unwrap());
    }

    if value == "null" {
        return serde_json::Value::Null;
    }

    match value.parse::<i64>() {
        Ok(v) => serde_json::Value::Number(serde_json::Number::from(v)),
        Err(_) => match value.parse::<f64>() {
            Ok(v) => match serde_json::Number::from_f64(v) {
                Some(n) => serde_json::Value::Number(n),
                None => serde_json::Value::String(value.to_string()),
            },
            Err(_) => serde_json::Value::String(value.to_string()),
        },
    }
}

fn json_to_number(value: &serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Number(n) => n.as_f64(),
        _ => None,
    }
}
