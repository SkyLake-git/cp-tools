use crate::cli::express::print_testcase_result;
use crate::modern_log::{gradient_string, status};
use crate::tester::judge::TestResult;
use crate::usecase::test::TestProgressEvent;
use crate::usecase::workspace::Workspace;
use crate::usecase::{start, test};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::cell::Cell;

pub fn dispatch_start(mut workspace: Workspace, task_id: &str) -> anyhow::Result<()> {
    status(
        "Fetching".bright_green().bold(),
        "from ".to_owned() + task_id,
    );
    let suite = start::run(&mut workspace, task_id)?;

    status(
        "Succeeded".bright_green().bold(),
        format!("saved {} testcases", suite.cases.len()),
    );
    status("Good Luck".bright_cyan().bold(), "");

    Ok(())
}

pub fn dispatch_test(mut workspace: Workspace) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{msg}")?);

    let last_ticked: Cell<u128> = Cell::new(0);
    let mut head_results: Vec<TestResult> = vec![];
    let mut accepted_count = 0;
    let fetch_res = workspace.fetch_test_suite();

    if fetch_res.is_err() {
        status(
            "Failed".bright_red().bold(),
            "to load testcases, try run `start`",
        );
        return Ok(());
    }

    let suite = fetch_res?;

    test::run(&mut workspace, suite.clone(), |p| match p {
        TestProgressEvent::Start { .. } => {
            spinner.set_message("Judging...");
            spinner.tick();
        }
        TestProgressEvent::Wait { elapsed_ms } => {
            let res: TestResult;
            if elapsed_ms >= suite.time_limit {
                res = TestResult::TimeLimitExceeded;
            } else {
                res = TestResult::WaitingJudge;
            }
            let judge_msg = format!(
                "Judging... {:>4} ms",
                ((elapsed_ms as f64 / 100.0).round() * 100.0) as u128
            );
            let judge_gradient = gradient_string(judge_msg.as_str(), elapsed_ms as f32 / 1200.0);
            let msg = format!("{} ┃ {}", judge_gradient, res);
            spinner.set_message(msg);

            if elapsed_ms - last_ticked.get() >= 10 {
                spinner.tick();
                last_ticked.set(elapsed_ms);
            }
        }
        TestProgressEvent::Finish { result } => {
            spinner.disable_steady_tick();
            spinner.suspend(|| {
                print_testcase_result((*result).clone());
            });
            spinner.reset();
            last_ticked.set(0);
            head_results.push(result.result.clone());
            if result.result == TestResult::Accepted {
                accepted_count += 1
            };
        }
    });

    let mut head_str: String = "".to_owned();
    for r in &head_results {
        head_str.push_str(format!("{}  ", r).as_str());
    }

    print!(
        "({}/{}) ┃ {} ┃",
        accepted_count,
        head_results.len(),
        head_str.trim()
    );

    Ok(())
}
