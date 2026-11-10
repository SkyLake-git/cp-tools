use crate::def::testcase::{Testcase, TestcaseResult};
use crate::tester::execute::SimpleExecutor;
use crate::tester::suite::{TestSuite, TestSuiteResult};
use crate::tester::test::Tester;
use crate::usecase::workspace::Workspace;

pub enum TestProgressEvent<'ev> {
    Start { testcase: &'ev Testcase },
    Wait { elapsed_ms: u128 },
    Finish { result: &'ev TestcaseResult },
}

pub fn run(
    workspace: &Workspace,
    suite: TestSuite,
    mut progress_callback: impl FnMut(TestProgressEvent),
) -> TestSuiteResult {
    let mut tester = Tester::new(
        SimpleExecutor::new(),
        workspace.settings.paths.executable.clone(),
        workspace,
    );

    let mut results: Vec<TestcaseResult> = vec![];

    for testcase in suite.cases {
        progress_callback(TestProgressEvent::Start {
            testcase: &testcase,
        });
        let res = tester.test(
            &testcase,
            suite.time_limit,
            suite.time_limit + workspace.settings.tester.timeout_margin,
            |ms| progress_callback(TestProgressEvent::Wait { elapsed_ms: ms }),
        );
        progress_callback(TestProgressEvent::Finish { result: &res });
        results.push(res);
    }

    let res = TestSuiteResult::new(suite.problem_id, suite.time_limit, results);

    res
}
