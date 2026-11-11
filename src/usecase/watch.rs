use crate::atcoder::scrape::{fetch_submission_update_queries, fetch_submissions};
use crate::def::contest_type::ContestType;
use crate::def::submission::{Submission, apply_query};
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

    let mut submissions = fetch_submissions(&suite.problem_id)?;

    loop {
        let watch_submission_ids: Vec<u64> = submissions
            .iter()
            .filter(|a| a.result == TestResult::WaitingForJudge)
            .map(|a| a.submission_id)
            .collect();

        let mut waiting_count = 0usize;
        if !watch_submission_ids.is_empty() || true {
            let queries =
                fetch_submission_update_queries(&suite.problem_id, &watch_submission_ids)?;
            waiting_count += watch_submission_ids.len();

            for q in queries {
                if let Some(sub) = submissions
                    .iter_mut()
                    .find(|a| a.submission_id == q.submission_id)
                {
                    apply_query(sub, &q);

                    if q.result == TestResult::Accepted && waiting_count > 0 {
                        waiting_count -= 1;
                    }
                }
            }
        }

        on_update(&submissions);

        if !watch_submission_ids.is_empty() && waiting_count == 0 && !persistent {
            break;
        }

        sleep(Duration::from_millis(max(
            1000u64,
            workspace.settings.atcoder.watch_interval_millis,
        )))
    }

    Ok(())
}
