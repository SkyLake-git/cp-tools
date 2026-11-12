use crate::def::submission::Submission;
use crate::def::testcase::{TestcaseResult, Token};
use chrono::{Local, TimeZone};
use dialoguer::console::{Alignment, pad_str};
use owo_colors::OwoColorize;
use unicode_width::UnicodeWidthStr;

pub fn make_ascii_escape_move(row: i32) -> String {
    let suf = if row > 0 { "F" } else { "E" };
    format!("\x1b[{}{}", row.abs(), suf)
}
pub fn print_testcase_result(res: TestcaseResult) {
    println!(
        "┏━ {}",
        format!(
            "{}: {} {}",
            format!("{}", res.testcase.label).bright_white(),
            res.result,
            format!("{:>4}ms / {} ms", res.execution_time, res.time_limit).cyan()
        )
    );

    let mut expected_str: String;
    let mut actual_str: String;

    if res.units.is_some() {
        expected_str = "".to_owned();
        actual_str = "".to_owned();
        for unit_res in res.units.unwrap() {
            let ex = format!("{}", unit_res.unit.expected);
            let ac = format!(
                "{}",
                Token {
                    text: unit_res.unit.actual.text,
                    separator_after: unit_res.unit.expected.separator_after
                }
            );
            if unit_res.judge {
                expected_str.push_str(ex.bright_green().to_string().as_str());
                actual_str.push_str(ac.bright_green().to_string().as_str());
            } else {
                expected_str.push_str(ex.bright_red().to_string().as_str());
                actual_str.push_str(ac.bright_red().to_string().as_str());
            }
        }
        expected_str += "\n";
        actual_str += "\n";
    } else {
        expected_str = res.testcase.output.to_owned() + "\n";
        actual_str = res.actual.trim_end().to_owned() + "\n";
    }

    println!("{}", "IN:".bright_cyan().underline());
    println!("{}", res.testcase.input);
    println!("{}", "EXPECTED:".bright_magenta().underline());
    print!("{}", expected_str);
    println!("{}", "ACTUAL:".bright_magenta().underline());
    print!("{}", actual_str);

    if !res.stderr.is_empty() {
        println!("{}", "ERR:".bright_red().underline());
        print!("{}", res.stderr);
    }
    println!();
}

pub fn print_submissions(submissions: &Vec<Submission>, limit: usize) {
    let mut max_sizes = vec![0; 5];
    let mut contents = Vec::new();
    let alignments = vec![
        Alignment::Left,
        Alignment::Left,
        Alignment::Right,
        Alignment::Right,
        Alignment::Left,
    ];

    for submission in submissions {
        let created = Local
            .timestamp_opt(submission.created, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let content = vec![
            created,
            submission.problem.clone(),
            submission.execution_time.clone().to_string() + " ms",
            submission.memory_usage.clone().to_string() + " KiB",
            submission.result.clone().to_string(),
        ];

        for (i, value) in content.iter().enumerate() {
            let visible = dialoguer::console::strip_ansi_codes(value).into_owned();
            let width = UnicodeWidthStr::width(visible.as_str());
            max_sizes[i] = max_sizes[i].max(width);
        }

        contents.push(content);

        if contents.len() >= limit {
            break;
        }
    }

    for content in contents {
        print!("┃ ");

        let padded: Vec<String> = content
            .iter()
            .zip(&max_sizes)
            .zip(&alignments)
            .map(|((value, &width), &align)| pad_str(value, width, align, None).into_owned())
            .collect();

        println!("{} ┃", padded.join(" ┃ "));
    }
}
