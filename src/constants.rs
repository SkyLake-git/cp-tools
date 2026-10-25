use reqwest::Url;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub const PROJECT_NAME: &'static str = "cp-tools";
pub const PROJECT_NAME_LEN: usize = PROJECT_NAME.len();
pub const CUTE_TICK_STRINGS: &'static [&'static str] =
    &["(-.-)", "(-.-)", "(o.o)", "(o.o)", "(-.^)/"];
pub const SMOOTH_PROGRESS_CHARS: &'static str = "█▉▊▋▌▍▎▏ ";

pub const USER_AGENT: &'static str = "cp-tools/0.1.0";
pub const ATCODER_DOMAIN: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://atcoder.jp/").unwrap());

pub const WORKSPACE_DIR_NAME: &'static str = ".cp-tools";

pub fn get_ms_since_unix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get unix timestamp")
        .as_millis()
}

pub fn get_atcoder_task_url(task_id: &str) -> anyhow::Result<Url> {
    let contest_id = task_id.rsplit_once('_').expect("invalid task ID").0;

    Ok(ATCODER_DOMAIN
        .join("contests/")?
        .join(&format!("{}/", contest_id))?
        .join("tasks/")?
        .join(&format!("{}/", task_id))?)
}
