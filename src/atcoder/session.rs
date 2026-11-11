use crate::config::credentials::fetch_atcoder;
use crate::constants::{ATCODER_DOMAIN, HTTP_CLIENT};
use anyhow::Context;
use percent_encoding::percent_decode_str;
use regex::Regex;
use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use std::sync::LazyLock;

pub fn make_cookie_from(session: &SecretString) -> String {
    format!("REVEL_SESSION={};", session.expose_secret()).to_owned()
}

pub fn get_csrf_token(session: &SecretString) -> String {
    let re = Regex::new(r"csrf_token:(([0-9a-zA-Z]|\+|/|=)+)").unwrap();
    let decoded = percent_decode_str(session.expose_secret()).decode_utf8_lossy();
    let cap = re.captures(&decoded).expect("failed to detect csrf_token");

    cap[1].to_owned()
}

pub fn is_logged_in(cookie: String) -> anyhow::Result<bool> {
    let response = HTTP_CLIENT
        .get(ATCODER_DOMAIN.join("settings/")?)
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .context("Failed to access AtCoder settings")?
        .error_for_status()
        .context("AtCoder returned an unsuccessful HTTP status")?;

    let final_url = response.url();

    Ok(final_url.path() != "/login/" && final_url.path() == "/settings/")
}

pub fn get_atcoder_task_url(task_id: &str) -> anyhow::Result<Url> {
    let contest_id = task_id.rsplit_once('_').expect("invalid task ID").0;

    Ok(ATCODER_DOMAIN
        .join("contests/")?
        .join(&format!("{}/", contest_id))?
        .join("tasks/")?
        .join(&format!("{}/", task_id))?)
}

pub fn get_atcoder_submission_url(task_id: &str) -> anyhow::Result<Url> {
    let contest_id = task_id.rsplit_once('_').expect("invalid task ID").0;

    Ok(ATCODER_DOMAIN
        .join("contests/")?
        .join(&format!("{}/", contest_id))?
        .join("submit/")?)
}

pub fn get_atcoder_submissions_me_url(task_id: &str) -> anyhow::Result<Url> {
    let contest_id = task_id.rsplit_once('_').expect("invalid task ID").0;

    Ok(ATCODER_DOMAIN
        .join("contests/")?
        .join(&format!("{}/", contest_id))?
        .join("submissions/")?
        .join("me/")?)
}
pub fn get_atcoder_submissions_json_url(
    task_id: &str,
    submission_ids: &Vec<u64>,
) -> anyhow::Result<Url> {
    let contest_id = task_id.rsplit_once('_').expect("invalid task ID").0;

    let mut url = ATCODER_DOMAIN
        .join("contests/")?
        .join(&format!("{}/", contest_id))?
        .join("submissions/")?
        .join("me/")?
        .join("status/")?
        .join("json/")?;

    {
        let mut query = url.query_pairs_mut();
        query.append_pair("reload", "true");

        for submission_id in submission_ids {
            query.append_pair("sids[]", &submission_id.to_string());
        }
    }

    Ok(url)
}

pub static SESSION_COOKIE: LazyLock<String> = LazyLock::new(|| {
    let session = fetch_atcoder()
        .expect("failed to fetch atcoder credentials")
        .revel_session;

    make_cookie_from(&session)
});
pub static SESSION_CSRF_TOKEN: LazyLock<String> = LazyLock::new(|| {
    let session = fetch_atcoder()
        .expect("failed to fetch atcoder credentials")
        .revel_session;

    get_csrf_token(&session)
});
