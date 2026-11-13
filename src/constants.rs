use reqwest::Url;
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::env::home_dir;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const PROJECT_NAME: &'static str = "cp-tools";
pub const PROJECT_NAME_LEN: usize = PROJECT_NAME.len();
pub const CUTE_TICK_STRINGS: &'static [&'static str] =
    &["(-.-)", "(-.-)", "(o.o)", "(o.o)", "(-.^)/"];
pub const SMOOTH_PROGRESS_CHARS: &'static str = "█▉▊▋▌▍▎▏ ";

pub const USER_AGENT: &'static str = "cp-tools/0.1.0";
pub const ATCODER_DOMAIN: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://atcoder.jp/").unwrap());

pub const PROJECT_DIR_NAME: &'static str = ".cp-tools";
pub const CPP_FACADE_DIR_NAME: &'static str = "cpp_facade_files";
pub const HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| make_http_client(true));

pub fn make_http_client(redirect: bool) -> Client {
    let mut builder = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(5));

    if !redirect {
        builder = builder.redirect(Policy::none());
    }

    builder.build().expect("Failed to create HTTP client")
}

pub fn get_ms_since_unix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get unix timestamp")
        .as_millis()
}

pub fn assume_home_dir() -> PathBuf {
    home_dir().expect("failed to get home dir")
}

pub fn assume_project_dir() {
    fs::create_dir(assume_home_dir().join(PROJECT_DIR_NAME)).ok();
}

pub fn format_progress(current: u32, max: u32, col: usize) -> String {
    let ratio = current as f64 / max as f64;
    let count = ((ratio * col as f64).floor() as usize).min(col);

    let rem = ratio - count as f64 / col as f64;

    let codes = ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];

    let partial = if count < col {
        let index = ((rem * col as f64) * 7.0).round() as usize;
        codes[index.min(7)]
    } else {
        ""
    };

    format!(
        "{}{}{}",
        "█".repeat(count),
        partial,
        " ".repeat(col.saturating_sub(count + usize::from(!partial.is_empty()))),
    )
}
