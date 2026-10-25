use crate::utils::is_directory;
use glob::glob;
use regex::Regex;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn fetch_gcc_includes() -> Vec<PathBuf> {
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

    let mut includes: Vec<PathBuf> = vec![];
    for dir in find_directories {
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
