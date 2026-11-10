use crate::cli::express::print_testcase_result;
use crate::config::settings::FormatOutMethod;
use crate::constants::{
    CPP_FACADE_DIR_NAME, PROJECT_DIR_NAME, SMOOTH_PROGRESS_CHARS, assume_home_dir,
    assume_project_dir, get_ms_since_unix,
};
use crate::def::contest_type::ContestType;
use crate::format::cpp::expand::glob_gcc_includes;
use crate::format::format::format_source_code;
use crate::modern_log::{GradientTick, gradient_string, status};
use crate::tester::judge::TestResult;
use crate::usecase::test::TestProgressEvent;
use crate::usecase::workspace::Workspace;
use crate::usecase::{start, submit, test};
use crate::utils::touch_files_relative;
use anyhow::bail;
use arboard::Clipboard;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::cell::Cell;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// super hardcode-er
/// todo: fetch from github?, get from system?
/// todo: move these to usecase
static ATCODER_ACL_PATHS: &[&str] = &[
    "atcoder/all",
    "atcoder/all.hpp",
    "atcoder/convolution",
    "atcoder/convolution.hpp",
    "atcoder/dsu",
    "atcoder/dsu.hpp",
    "atcoder/fenwicktree",
    "atcoder/fenwicktree.hpp",
    "atcoder/internal_bit",
    "atcoder/internal_bit.hpp",
    "atcoder/internal_csr",
    "atcoder/internal_csr.hpp",
    "atcoder/internal_math",
    "atcoder/internal_math.hpp",
    "atcoder/internal_queue",
    "atcoder/internal_queue.hpp",
    "atcoder/internal_scc",
    "atcoder/internal_scc.hpp",
    "atcoder/internal_type_traits",
    "atcoder/internal_type_traits.hpp",
    "atcoder/lazysegtree",
    "atcoder/lazysegtree.hpp",
    "atcoder/LICENSE",
    "atcoder/math",
    "atcoder/math.hpp",
    "atcoder/maxflow",
    "atcoder/maxflow.hpp",
    "atcoder/mincostflow",
    "atcoder/mincostflow.hpp",
    "atcoder/modint",
    "atcoder/modint.hpp",
    "atcoder/scc",
    "atcoder/scc.hpp",
    "atcoder/segtree",
    "atcoder/segtree.hpp",
    "atcoder/string",
    "atcoder/string.hpp",
    "atcoder/twosat",
    "atcoder/twosat.hpp",
];

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

pub fn initialize_gcc_facades(workspace: &Workspace) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::with_template("{msg}")?);
    let handler =
        spinner.enable_gradient_tick("Scanning gcc includes...", Duration::from_millis(10));
    let mut res = glob_gcc_includes();
    drop(handler);
    match workspace.settings.general.contest_type {
        ContestType::AtCoder => {
            for p in ATCODER_ACL_PATHS {
                res.push(PathBuf::from(p));
            }
        }
    }

    spinner.suspend(|| {
        status(
            "Scanned".bright_green().bold(),
            format!("detected {} files", res.len()),
        )
    });

    spinner.set_length(res.len() as u64);
    spinner.set_style(
        ProgressStyle::with_template("{msg} [{bar:20}] {pos}/{len}: {prefix}")?
            .progress_chars(SMOOTH_PROGRESS_CHARS),
    );
    let handler =
        spinner.enable_gradient_tick("Touching facade files...", Duration::from_millis(10));

    let base_path = assume_home_dir()
        .join(PROJECT_DIR_NAME)
        .join(CPP_FACADE_DIR_NAME);
    touch_files_relative(base_path.clone(), res, |p| {
        spinner.inc(1);
        spinner.set_prefix(format!(
            "{}",
            pathdiff::diff_paths(&p, &base_path).unwrap().display()
        ));
        spinner.tick();
    });

    drop(handler);
    spinner.suspend(|| status("Initialized".bright_green().bold(), "cpp facade files"));
    spinner.finish_and_clear();

    Ok(())
}

pub fn dispatch_purge() -> anyhow::Result<()> {
    assume_project_dir();
    status("Purging".bright_green().bold(), "gcc facades");
    fs::remove_dir_all(
        &assume_home_dir()
            .join(PROJECT_DIR_NAME)
            .join(CPP_FACADE_DIR_NAME),
    )
    .ok();

    status("Succeeded".bright_green().bold(), "purged cp-tools files");
    Ok(())
}

pub fn dispatch_format(workspace: &Workspace, manual: bool) -> anyhow::Result<String> {
    if !fs::exists(&workspace.settings.paths.source)? {
        bail!("source file doesn't exists");
    }

    if !fs::exists(
        &assume_home_dir()
            .join(PROJECT_DIR_NAME)
            .join(CPP_FACADE_DIR_NAME),
    )? {
        status("Initializing".bright_green().bold(), "gcc facades");
        initialize_gcc_facades(&workspace)?;
    }

    let source_code = workspace.read_source_code();
    status("Formatting".bright_green().bold(), "the source code");
    let start_ms = get_ms_since_unix();
    let arranged_source_code = format_source_code(&workspace, &source_code)?;
    let took_ms = get_ms_since_unix() - start_ms;
    status(
        "Finished".bright_green().bold(),
        format!("formatted code in {took_ms}ms"),
    );

    if !manual {
        match &workspace.settings.format.out.method {
            FormatOutMethod::Clipboard => {
                let mut clip = Clipboard::new()?;
                clip.set_text(&arranged_source_code)?;
                status("Succeeded".bright_green().bold(), "copied to clipboard")
            }
            FormatOutMethod::File => {
                if let Some(target_file) = workspace.settings.format.out.target_file.to_owned() {
                    fs::write(target_file, &arranged_source_code)?;
                    status("Succeeded".bright_green().bold(), "wrote to file")
                } else {
                    status(
                        "Failed".bright_red().bold(),
                        "to write formatted code to file: file is none (please specify path `format.out.target_file`)",
                    )
                }
            }
        }
    }

    Ok(arranged_source_code)
}

pub fn dispatch_submit(workspace: &Workspace, no_test: bool) -> anyhow::Result<()> {
    if !no_test {
        let result = dispatch_test(workspace)?;

        if !result {
            return Ok(());
        }
    }
    let source_code = dispatch_format(&workspace, true)?;
    status("Submitting".bright_green().bold(), "the source code");
    if submit::run(&workspace, &source_code)? {
        status("Successfully".bright_green().bold(), "submitted");
        // todo: watch
    } else {
        status(
            "Failed".bright_red().bold(),
            "to submit, make sure you're participating in a contest",
        )
    }

    Ok(())
}

pub fn dispatch_test(workspace: &Workspace) -> anyhow::Result<bool> {
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
        return Ok(false);
    }

    let suite = fetch_res?;

    test::run(&workspace, suite.clone(), |p| match p {
        TestProgressEvent::Start { .. } => {
            spinner.set_message("Judging...");
            spinner.tick();
        }
        TestProgressEvent::Wait { elapsed_ms } => {
            if elapsed_ms - last_ticked.get() >= 10 {
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
                let judge_gradient =
                    gradient_string(judge_msg.as_str(), elapsed_ms as f32 / 1200.0);
                let msg = format!("{} ┃ {}", judge_gradient, res);
                spinner.set_message(msg);

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

    spinner.finish_and_clear();

    let mut head_str: String = "".to_owned();
    for r in &head_results {
        head_str.push_str(format!("{}  ", r).as_str());
    }

    println!(
        "({}/{}) ┃ {} ┃",
        accepted_count,
        head_results.len(),
        head_str.trim()
    );

    if accepted_count == head_results.len() {
        println!();
        println!("{}", "┃".bright_green().bold());
        println!("{}", "┃ Fully Accepted, Good job".bright_green().bold());
        println!("{}", "┃".bright_green().bold());
        println!();
    }

    Ok(accepted_count == head_results.len())
}
