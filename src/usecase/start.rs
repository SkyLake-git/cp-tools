use crate::atcoder::scrape::fetch_testcases;
use crate::atcoder::session::get_atcoder_task_url;
use crate::tester::suite::TestSuite;
use crate::usecase::workspace::Workspace;

pub fn run(workspace: &mut Workspace, task_id: &str) -> anyhow::Result<TestSuite> {
    let url = get_atcoder_task_url(task_id)?;
    let suite = fetch_testcases(&url)?;
    workspace.save_test_suite(&suite);

    Ok(suite)
}
