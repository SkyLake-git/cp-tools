use crate::def::testcase::TestcaseUnit;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TestResult {
    WaitingForJudge,
    Accepted,
    WrongAnswer,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    CompileError,
    // irregular results
    OutputLimitExceeded,
    InternalError,
    WaitingForRejudge,
}

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestResult::WaitingForJudge => write!(f, "{}", " WJ ".bright_black().reversed()),
            TestResult::Accepted => write!(f, "{}", " AC ".bright_green().reversed()),
            TestResult::WrongAnswer => write!(f, "{}", " WA ".bright_yellow().reversed()),
            TestResult::RuntimeError => write!(f, "{}", " RE ".bright_yellow().reversed()),
            TestResult::TimeLimitExceeded => write!(f, "{}", " TLE ".bright_yellow().reversed()),
            TestResult::MemoryLimitExceeded => write!(f, "{}", " MLE ".bright_yellow().reversed()),
            TestResult::CompileError => write!(f, "{}", " CE ".bright_yellow().reversed()),
            TestResult::OutputLimitExceeded => write!(f, "{}", " OLE ".bright_red().reversed()),
            TestResult::InternalError => write!(f, "{}", " IE ".bright_red().reversed()),
            TestResult::WaitingForRejudge => write!(f, "{}", " WR ".bright_black().reversed()),
        }
    }
}

impl TryFrom<String> for TestResult {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "AC" => Ok(TestResult::Accepted),
            "WA" => Ok(TestResult::WrongAnswer),
            "TLE" => Ok(TestResult::TimeLimitExceeded),
            "MLE" => Ok(TestResult::MemoryLimitExceeded),
            "RE" => Ok(TestResult::RuntimeError),
            "CE" => Ok(TestResult::CompileError),
            "WJ" => Ok(TestResult::WaitingForJudge),
            "OLE" => Ok(TestResult::OutputLimitExceeded),
            "IE" => Ok(TestResult::InternalError),
            "WR" => Ok(TestResult::WaitingForRejudge),
            _ => Err(()),
        }
    }
}

pub trait UnitJudge {
    fn judge(&self, unit: &TestcaseUnit) -> bool;
}

pub struct ApproxFloatUnitJudger {
    pub error_threshold: f64,
}
impl ApproxFloatUnitJudger {
    pub fn new(error_threshold: f64) -> Self {
        Self { error_threshold }
    }
}
impl UnitJudge for ApproxFloatUnitJudger {
    fn judge(&self, unit: &TestcaseUnit) -> bool {
        let a: f64 = unit.actual.text.parse().unwrap();
        let b: f64 = unit.expected.text.parse().unwrap();

        (a - b).abs()
            <= f64::max(
                self.error_threshold * f64::max(a.abs(), b.abs()),
                self.error_threshold,
            )
    }
}

pub struct MatchUnitJudger {}
impl MatchUnitJudger {
    pub fn new() -> Self {
        Self {}
    }
}
impl UnitJudge for MatchUnitJudger {
    fn judge(&self, unit: &TestcaseUnit) -> bool {
        unit.actual.text == unit.expected.text
    }
}

pub trait SelectUnitJudge {
    fn select_judger(&self, unit: &TestcaseUnit) -> Box<dyn UnitJudge>;
}
