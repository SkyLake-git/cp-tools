use crate::constants::{CPP_FACADE_DIR_NAME, PROJECT_DIR_NAME, assume_home_dir};
use crate::format::cpp::expand::make_expand_compiler_args;
use anyhow::bail;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn detect_original_includes(source_code: &str) -> Vec<String> {
    let reg = Regex::new(r#"#include\s+["<](.+)[">]"#).unwrap();
    let captures = reg.captures_iter(source_code);

    let mut detected: Vec<String> = vec![];
    let base = assume_home_dir()
        .join(PROJECT_DIR_NAME)
        .join(CPP_FACADE_DIR_NAME);

    for cap in captures {
        let v = cap[1].trim();
        if let Ok(exists) = fs::exists(base.join(v))
            && exists
        {
            detected.push(cap[0].trim().to_owned());
        }
    }

    detected
}

pub fn gcc_preprocess(source_code: &str, include_dirs: Vec<PathBuf>) -> anyhow::Result<String> {
    let options: Vec<String> = make_expand_compiler_args();
    let mut path_options: Vec<String> = vec![];
    for path in include_dirs {
        path_options.push("-isystem".to_owned());
        path_options.push(format!("{}", path.display()));
    }
    let mut cmd = Command::new("g++")
        .arg("-E")
        .arg("-x")
        .arg("c++")
        .args(path_options)
        .arg("-D")
        .arg("ONLINE_JUDGE")
        .arg("-D")
        .arg("ATCODER")
        .args(options)
        .arg("-")
        .arg("-o")
        .arg("-")
        .current_dir(
            assume_home_dir()
                .join(PROJECT_DIR_NAME)
                .join(CPP_FACADE_DIR_NAME),
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    cmd.stdin.take().unwrap().write(source_code.as_bytes())?;

    let original_includes = detect_original_includes(source_code);
    let output = cmd.wait_with_output()?;
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    if !stderr.trim().is_empty() {
        bail!("failed to preprocess source code: ".to_owned() + stderr.trim());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    let res = original_includes.join("\n") + "\n" + &stdout;

    Ok(res)
}
