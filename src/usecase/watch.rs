use crate::atcoder::scrape::fetch_submissions;
use crate::def::contest_type::ContestType;
use crate::def::submission::Submission;
use crate::tester::judge::TestResult;
use crate::usecase::workspace::Workspace;
use std::cmp::max;
use std::thread::sleep;
use std::time::Duration;

pub fn run(
    workspace: &Workspace,
    persistent: bool,
    on_update: impl FnMut(&Vec<Submission>),
) -> anyhow::Result<()> {
    match &workspace.settings.general.contest_type {
        ContestType::AtCoder => run_atcoder(&workspace, persistent, on_update),
    }
}

fn run_atcoder(
    workspace: &Workspace,
    persistent: bool,
    mut on_update: impl FnMut(&Vec<Submission>),
) -> anyhow::Result<()> {
    let suite = workspace.fetch_test_suite()?;
    let mut waiting = false;

    loop {
        let submissions = fetch_submissions(&suite.problem_id)?;
        let watch_submission_ids: Vec<u64> = submissions
            .iter()
            .filter(|a| a.progress_current.is_some() || a.result == TestResult::WaitingForJudge)
            .map(|a| a.submission_id)
            .collect();

        on_update(&submissions);
        if !watch_submission_ids.is_empty() {
            waiting = true;
        }

        if watch_submission_ids.is_empty() && waiting && !persistent {
            break;
        }

        sleep(Duration::from_millis(max(
            1000u64,
            workspace.settings.atcoder.watch_interval_millis,
        )))
    }

    Ok(())
}
