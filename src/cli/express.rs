use crate::def::testcase::{TestcaseResult, Token};
use owo_colors::OwoColorize;

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
