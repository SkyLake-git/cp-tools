use crate::config::credentials::fetch_atcoder;
use crate::constants::{ATCODER_DOMAIN, USER_AGENT};
use crate::def::testcase::Testcase;
use crate::tester::suite::TestSuite;
use anyhow::{Context, anyhow, bail};
use reqwest::Url;
use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};
use secrecy::{ExposeSecret, SecretString};
use std::collections::BTreeMap;
use std::sync::LazyLock;
use std::time::Duration;

pub fn make_cookie_from(session: &SecretString) -> String {
    format!("REVEL_SESSION={}", session.expose_secret()).to_owned()
}

static COOKIE: LazyLock<String> = LazyLock::new(|| {
    let session = fetch_atcoder()
        .expect("failed to fetch atcoder credentials")
        .revel_session;

    format!("REVEL_SESSION={}", session.expose_secret()).to_owned()
});

pub fn is_logged_in(cookie: String) -> anyhow::Result<bool> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(15))
        .build()
        .context("Failed to create HTTP client")?;
    let response = client
        .get(ATCODER_DOMAIN.join("settings/")?)
        .header(reqwest::header::COOKIE, &cookie)
        .send()
        .context("Failed to access AtCoder settings")?
        .error_for_status()
        .context("AtCoder returned an unsuccessful HTTP status")?;

    let final_url = response.url();

    Ok(final_url.path() != "/login/" && final_url.path() == "/settings/")
}

pub fn fetch_testcases(parsed_url: &Url) -> anyhow::Result<TestSuite> {
    if parsed_url.scheme() != "https"
        || parsed_url.host_str() != Some("atcoder.jp")
        || !parsed_url.path().contains("/tasks/")
    {
        bail!("Expected an AtCoder task URL: {parsed_url}");
    }

    let client = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(15))
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(parsed_url.clone())
        .header(reqwest::header::COOKIE, COOKIE.as_str())
        .send()
        .context("Failed to fetch problem page")?
        .error_for_status()
        .context("Problem page returned an HTTP error")?;

    let html = response.text().context("Failed to read HTML response")?;

    parse_testcases(parse_problem_id(&parsed_url)?.as_str(), &html)
}

fn parse_testcases(problem_id: &str, html: &str) -> anyhow::Result<TestSuite> {
    let document = Html::parse_document(html);
    let heading_selector = Selector::parse("h3").expect("valid CSS selector");

    let mut cases: BTreeMap<usize, (Option<String>, Option<String>)> = BTreeMap::new();

    for heading in document.select(&heading_selector) {
        let title = heading.text().collect::<String>();
        let title = title.trim();

        let (is_input, number) = if let Some(number) = title.strip_prefix("Sample Input ") {
            (true, number)
        } else if let Some(number) = title.strip_prefix("Sample Output ") {
            (false, number)
        } else {
            continue;
        };

        let number: usize = number
            .trim()
            .parse()
            .context("Invalid sample number in heading")?;

        let pre = heading
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .find(|element| element.value().name() == "pre")
            .with_context(|| format!("Missing <pre> after {title}"))?;

        let content = pre.text().collect::<String>();
        let content = content.replace("\r\n", "\n").trim().to_string();

        let case = cases.entry(number).or_default();

        if is_input {
            if case.0.replace(content).is_some() {
                bail!("Duplicate sample input: {number}");
            }
        } else if case.1.replace(content).is_some() {
            bail!("Duplicate sample output: {number}");
        }
    }

    if cases.is_empty() {
        bail!(
            "No sample test cases found. \
             The page structure may have changed or access may be restricted."
        );
    }

    let mut result = Vec::with_capacity(cases.len());

    for (number, case) in cases {
        let input = case
            .0
            .with_context(|| format!("Sample input {number} is missing"))?;

        let output = case
            .1
            .with_context(|| format!("Sample output {number} is missing"))?;

        result.push(Testcase {
            label: format!("Sample {number}"),
            sanitized_label: format!("sample_{number}"),
            input,
            output,
        });
    }

    Ok(TestSuite::new(
        problem_id.parse()?,
        parse_time_limit(&document)?,
        result,
    ))
}

pub fn parse_problem_id(url: &Url) -> anyhow::Result<String> {
    if url.scheme() != "https" || url.host_str() != Some("atcoder.jp") {
        bail!("URL must be an HTTPS AtCoder URL");
    }

    let segments: Vec<&str> = url
        .path_segments()
        .context("URL has no path segments")?
        .collect();

    let task_index = segments
        .iter()
        .position(|segment| *segment == "tasks")
        .context("URL does not contain /tasks/")?;

    let problem_id = segments
        .get(task_index + 1)
        .filter(|id| !id.is_empty())
        .context("Problem ID not found in URL")?;

    Ok((*problem_id).to_string())
}
fn parse_time_limit(document: &Html) -> anyhow::Result<u128> {
    let text = document.root_element().text().collect::<String>();

    let (_, tail) = text
        .split_once("Time Limit:")
        .ok_or_else(|| anyhow!("Time limit not found"))?;

    let value = tail
        .trim_start()
        .split_once(" sec")
        .ok_or_else(|| anyhow!("Invalid time limit format"))?
        .0
        .trim()
        .parse::<f64>()?;

    if !value.is_finite() || value <= 0.0 {
        bail!("Invalid time limit value: {value}");
    }

    Ok((value * 1000.0) as u128)
}
