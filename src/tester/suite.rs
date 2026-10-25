use crate::def::testcase::{Testcase, TestcaseResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TestSuite {
    pub problem_id: String,
    pub time_limit: u128,
    pub cases: Vec<Testcase>,
}

#[derive(Debug)]
pub struct TestSuiteResult {
    pub problem_id: String,
    pub time_limit: u128,
    pub cases: Vec<TestcaseResult>,
}

impl TestSuite {
    pub fn new(problem_id: String, time_limit: u128, cases: Vec<Testcase>) -> Self {
        Self {
            problem_id,
            time_limit,
            cases,
        }
    }
}

impl TestSuiteResult {
    pub fn new(problem_id: String, time_limit: u128, cases: Vec<TestcaseResult>) -> Self {
        Self {
            problem_id,
            time_limit,
            cases,
        }
    }
}
