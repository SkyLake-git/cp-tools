use crate::tester::judge::TestResult;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Testcase {
    pub label: String,
    pub sanitized_label: String,
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct TestcaseResult {
    pub testcase: Testcase,
    pub actual: String,
    pub stderr: String,
    pub units: Option<Vec<TestcaseUnitResult>>,
    pub result: TestResult,
    pub execution_time: u128,
    pub time_limit: u128,
}

impl TestcaseResult {
    pub fn new(
        testcase: Testcase,
        actual: String,
        stderr: String,
        units: Option<Vec<TestcaseUnitResult>>,
        result: TestResult,
        execution_time: u128,
        time_limit: u128,
    ) -> Self {
        Self {
            testcase,
            actual,
            stderr,
            units,
            result,
            execution_time,
            time_limit,
        }
    }
}

impl Testcase {
    pub fn new(label: String, sanitized_label: String, input: String, output: String) -> Self {
        Self {
            label,
            sanitized_label,
            input,
            output,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TestcaseUnit {
    pub actual: Token,
    pub expected: Token,
}

#[derive(Debug, Clone)]
pub struct TestcaseUnitResult {
    pub unit: TestcaseUnit,
    pub judge: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separator {
    None,
    Newline,
    Whitespace,
}

impl Display for Separator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::Newline => write!(f, "\n"),
            Separator::Whitespace => write!(f, " "),
            Separator::None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub separator_after: Separator,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.text.as_str(), self.separator_after)
    }
}

fn split_with_separators(input: &str) -> Vec<Token> {
    if input.is_empty() {
        return Vec::new();
    }

    let lines: Vec<&str> = input.split_terminator('\n').collect();
    let mut result = Vec::new();

    for (i, raw_line) in lines.iter().enumerate() {
        let line = raw_line.trim_matches(|c| c == ' ' || c == '\t');

        let line_separator = if i + 1 < lines.len() {
            Separator::Newline
        } else {
            Separator::None
        };

        if line.is_empty() {
            result.push(Token {
                text: String::new(),
                separator_after: line_separator,
            });
            continue;
        }

        let mut start = 0;

        for (pos, ch) in line.char_indices() {
            if ch == ' ' || ch == '\t' {
                result.push(Token {
                    text: line[start..pos].to_owned(),
                    separator_after: Separator::Whitespace,
                });

                start = pos + ch.len_utf8();
            }
        }

        result.push(Token {
            text: line[start..].to_owned(),
            separator_after: line_separator,
        });
    }

    result
}

pub fn extract_units(testcase: &Testcase, actual: &str) -> Result<Vec<TestcaseUnit>, ()> {
    let mut units: Vec<TestcaseUnit> = vec![];
    let actual_lines: Vec<Token> = split_with_separators(actual);
    let output_lines: Vec<Token> = split_with_separators(&*testcase.output);

    if actual_lines.len() < output_lines.len() {
        return Err(());
    }

    let count = output_lines.len();

    for i in count..actual_lines.len() {
        if !actual_lines[i].text.is_empty() {
            return Err(());
        }
    }

    for i in 0..count {
        units.push(TestcaseUnit {
            actual: actual_lines[i].clone(),
            expected: output_lines[i].clone(),
        })
    }

    Ok(units)
}
