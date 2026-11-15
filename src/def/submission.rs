use crate::tester::judge::TestResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Submission {
    pub submission_id: u64,
    pub created: i64,
    pub problem: String,
    pub task_id: String,
    pub result: TestResult,
    pub score: u32,
    pub execution_time: u32,
    pub memory_usage: u32,
    pub progress_current: Option<u32>,
    pub progress_max: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SubmissionUpdateQuery {
    pub submission_id: u64,
    pub result: TestResult,
    pub score: u32,
    pub execution_time: u32,
    pub memory_usage: u32,
    pub progress_current: Option<u32>,
    pub progress_max: Option<u32>,
}

pub fn apply_query(sub: &mut Submission, query: &SubmissionUpdateQuery) {
    sub.result = query.result.clone();
    sub.score = query.score;
    sub.execution_time = query.execution_time;
    sub.memory_usage = query.memory_usage;
}
