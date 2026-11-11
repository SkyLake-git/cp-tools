use crate::atcoder::scrape::fetch_testcases;
use crate::tester::suite::TestSuite;
use crate::usecase::workspace::Workspace;

pub fn run(workspace: &mut Workspace, task_id: &str) -> anyhow::Result<TestSuite> {
    let suite = fetch_testcases(task_id)?;
    workspace.save_test_suite(&suite);

    Ok(suite)
}
