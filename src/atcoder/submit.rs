use crate::atcoder::session::{SESSION_COOKIE, SESSION_CSRF_TOKEN, get_atcoder_submission_url};
use crate::constants::make_http_client;
use crate::def::language::Language;
use anyhow::Context;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct AtCoderSubmissionBody {
    csrf_token: String,
    #[serde(rename = "data.TaskScreenName")]
    task_screen_name: String,
    #[serde(rename = "data.LanguageId")]
    language_id: u64,
    #[serde(rename = "sourceCode")]
    source_code: String,
}

pub fn translate_language_id(language: Language) -> Option<u64> {
    match language {
        Language::CppGcc => Some(6017),
        Language::None => None,
    }
}

pub fn submit(problem_id: &str, language_id: u64, source_code: &str) -> anyhow::Result<bool> {
    let body = AtCoderSubmissionBody {
        csrf_token: SESSION_CSRF_TOKEN.to_owned(),
        task_screen_name: problem_id.to_owned(),
        language_id,
        source_code: source_code.to_owned(),
    };
    let response = make_http_client(false)
        .post(get_atcoder_submission_url(problem_id)?)
        .header(reqwest::header::COOKIE, SESSION_COOKIE.as_str())
        .form(&body)
        .send()
        .context("Failed to post submission")?;

    Ok(response.status().is_redirection())
}
