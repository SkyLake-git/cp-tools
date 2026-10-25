use crate::constants::get_ms_since_unix;
use crate::def::testcase::{Testcase, TestcaseResult, TestcaseUnitResult, extract_units};
use crate::tester::execute::Execute;
use crate::tester::judge::{SelectUnitJudge, TestResult};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

pub struct Tester<'tl, E: Execute, J: SelectUnitJudge> {
    pub execute_path: PathBuf,
    pub executor: E,
    pub judge_selector: &'tl J,
}

impl<'ttl, E: Execute, J: SelectUnitJudge> Tester<'ttl, E, J> {
    pub fn new(executor: E, execute_path: PathBuf, judge_selector: &'ttl J) -> Self {
        Self {
            execute_path,
            executor,
            judge_selector,
        }
    }

    /// With blocking
    pub fn test(
        &mut self,
        testcase: &Testcase,
        time_limit_ms: u128,
        terminate_threshold_ms: u128,
        mut wait_callback: impl FnMut(u128),
    ) -> TestcaseResult {
        self.executor.spawn(self.execute_path.clone(), testcase);

        let start_time: u128 = get_ms_since_unix();
        while !self.executor.is_finished() {
            let ms = get_ms_since_unix() - start_time;
            wait_callback(ms);
            sleep(Duration::from_millis(1));

            if ms > terminate_threshold_ms {
                self.executor.terminate().ok();
                break;
            }
        }

        let execution_time: u128 = get_ms_since_unix() - start_time;
        if execution_time > terminate_threshold_ms {
            return TestcaseResult::new(
                (*testcase).clone(),
                "".to_owned(),
                "".to_owned(),
                None,
                TestResult::TimeLimitExceeded,
                execution_time,
                time_limit_ms,
            );
        }

        let result = self
            .executor
            .get_result()
            .expect("failed to get result from executor");

        if !result.exit_code.success() {
            return TestcaseResult::new(
                (*testcase).clone(),
                result.stdout.to_owned(),
                result.stderr.to_owned(),
                None,
                TestResult::RuntimeError,
                execution_time,
                time_limit_ms,
            );
        }

        let units = extract_units(testcase, &result.stdout);

        if units.is_err() {
            return TestcaseResult::new(
                (*testcase).clone(),
                result.stdout.to_owned(),
                result.stderr.to_owned(),
                None,
                TestResult::WrongAnswer,
                execution_time,
                time_limit_ms,
            );
        }

        let mut unit_res: Vec<TestcaseUnitResult> = vec![];
        let mut res = TestResult::Accepted;

        for unit in units.unwrap() {
            // todo: do not instantiate judger everytime
            let judge = self.judge_selector.select_judger(&unit).judge(&unit);

            unit_res.push(TestcaseUnitResult {
                unit: unit.clone(),
                judge,
            });

            if !judge {
                res = TestResult::WrongAnswer;
            }
        }

        if execution_time > time_limit_ms {
            return TestcaseResult::new(
                (*testcase).clone(),
                result.stdout.to_owned(),
                result.stderr.to_owned(),
                Some(unit_res),
                TestResult::TimeLimitExceeded,
                execution_time,
                time_limit_ms,
            );
        } else {
            TestcaseResult::new(
                (*testcase).clone(),
                result.stdout.to_owned(),
                result.stderr.to_owned(),
                Some(unit_res),
                res,
                execution_time,
                time_limit_ms,
            )
        }
    }
}
