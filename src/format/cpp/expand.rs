use crate::constants::{CPP_FACADE_DIR_NAME, PROJECT_DIR_NAME, assume_home_dir};
use crate::utils::is_directory;
use glob::glob;
use regex::Regex;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

pub static GCC_INCLUDE_DIRECTORIES: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| fetch_gcc_include_directories());

pub fn fetch_gcc_include_directories() -> Vec<PathBuf> {
    let output = Command::new("g++")
        .arg("-E")
        .arg("-Wp,-v")
        .arg("-x")
        .arg("c++")
        .arg("-")
        .stdin(Stdio::null())
        .output()
        .expect("failed to execute command");

    let res = String::from_utf8_lossy(&output.stderr).to_string();
    let re = Regex::new(r"#include <...> search starts here:([\s\S]*)End of search list").unwrap();

    let cap = re
        .captures(&res)
        .expect("failed to detect include directory");

    let mut find_directories: Vec<PathBuf> = vec![];
    for g in cap[1].trim().lines() {
        let g_trimmed = g.trim();

        if is_directory(&g_trimmed) {
            find_directories.push(PathBuf::from(g_trimmed));
        }
    }

    find_directories
}

pub fn glob_gcc_includes() -> Vec<PathBuf> {
    let mut includes: Vec<PathBuf> = vec![];
    for dir in (*GCC_INCLUDE_DIRECTORIES).clone() {
        for f in glob(dir.clone().join("**/*").display().to_string().as_str())
            .unwrap()
            .filter_map(Result::ok)
        {
            if !is_directory(&f) {
                includes
                    .push(pathdiff::diff_paths(f, dir.clone()).expect("failed to get diff path"));
            }
        }
    }

    includes
}

pub fn make_expand_compiler_args() -> Vec<String> {
    vec![
        "-nostdinc".to_owned(),
        "-nostdinc++".to_owned(),
        "-CC".to_owned(),
        "-P".to_owned(),
        "-I".to_owned(),
        format!(
            "{}",
            assume_home_dir()
                .join(PROJECT_DIR_NAME)
                .join(CPP_FACADE_DIR_NAME)
                .display()
        )
        .to_owned(),
    ]
}
