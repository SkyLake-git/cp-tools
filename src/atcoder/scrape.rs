use crate::atcoder::session::{
    SESSION_COOKIE, get_atcoder_submissions_json_url, get_atcoder_submissions_me_url,
    get_atcoder_task_url,
};
use crate::constants::HTTP_CLIENT;
use crate::def::submission::{Submission, SubmissionUpdateQuery};
use crate::def::testcase::Testcase;
use crate::tester::judge::TestResult;
use crate::tester::suite::TestSuite;
use anyhow::{Context, anyhow, bail};
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use std::collections::BTreeMap;

fn parse_resource_value(text: &str, unit: &str) -> anyhow::Result<u32> {
    let value = text
        .trim()
        .strip_suffix(unit)
        .context("Invalid resource value")?
        .trim()
        .trim_start_matches('>')
        .trim();

    value.parse::<u32>().context("Invalid resource value")
}

pub fn fetch_testcases(task_id: &str) -> anyhow::Result<TestSuite> {
    let parsed_url = get_atcoder_task_url(task_id)?;

    let response = HTTP_CLIENT
        .get(parsed_url.clone())
        .header(reqwest::header::COOKIE, SESSION_COOKIE.as_str())
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

pub fn fetch_submissions(task_id: &str) -> anyhow::Result<Vec<Submission>> {
    let url = get_atcoder_submissions_me_url(task_id)?;
    let response = HTTP_CLIENT
        .get(url.clone())
        .header(reqwest::header::COOKIE, SESSION_COOKIE.as_str())
        .send()
        .context("Failed to fetch problem page")?
        .error_for_status()
        .context("Problem page returned an HTTP error")?;

    let data = response.text().context("Failed to read HTML response")?;

    extract_submissions(&data)
}

pub fn fetch_submission_update_queries(
    task_id: &str,
    submission_ids: &Vec<u64>,
) -> anyhow::Result<Vec<SubmissionUpdateQuery>> {
    let url = get_atcoder_submissions_json_url(task_id, &submission_ids)?;
    let response = HTTP_CLIENT
        .get(url.clone())
        .header(reqwest::header::COOKIE, SESSION_COOKIE.as_str())
        .send()
        .context("Failed to fetch problem page")?
        .error_for_status()
        .context("Problem page returned an HTTP error")?;

    let data: Value = response.json().context("Failed to read HTML response")?;

    decode_submission_json(&data)
}

fn parse_result(input: &str) -> anyhow::Result<(TestResult, Option<u32>, Option<u32>)> {
    let input = input.trim();

    if input.is_empty() {
        bail!("Empty result");
    }

    let mut parts = input.split_whitespace();
    let first = parts.next().unwrap();

    if let Some((current, max)) = first.split_once('/') {
        let progress_current = current
            .parse::<u32>()
            .map_err(|_| anyhow!("Invalid progress current: {current}"))?;

        let progress_max = max
            .parse::<u32>()
            .map_err(|_| anyhow!("Invalid progress max: {max}"))?;

        if progress_max == 0 || progress_current > progress_max {
            bail!("Invalid progress: {first}");
        }

        let result = parts.next().unwrap_or("WJ");

        if parts.next().is_some() {
            bail!("Invalid result format: {input}");
        }

        return Ok((
            TestResult::try_from(result.to_owned())
                .map_err(|_| anyhow!("Unknown result: {}", result))?,
            Some(progress_current),
            Some(progress_max),
        ));
    }

    if parts.next().is_some() {
        bail!("Invalid result format: {input}");
    }

    Ok((
        TestResult::try_from(first.to_owned()).map_err(|_| anyhow!("Unknown result: {}", first))?,
        None,
        None,
    ))
}

pub fn extract_submissions(html: &str) -> anyhow::Result<Vec<Submission>> {
    use anyhow::{Context, anyhow};
    use chrono::DateTime;
    use scraper::{Html, Selector};

    let document = Html::parse_document(html);

    let row_selector = Selector::parse("div.panel-submission table tbody tr")
        .map_err(|e| anyhow!("Invalid row selector: {e}"))?;

    let time_selector =
        Selector::parse("time").map_err(|e| anyhow!("Invalid time selector: {e}"))?;

    let problem_selector = Selector::parse("a[href*='/tasks/']")
        .map_err(|e| anyhow!("Invalid problem selector: {e}"))?;

    let submission_selector = Selector::parse("a.submission-details-link[href*='/submissions/']")
        .map_err(|e| anyhow!("Invalid submission selector: {e}"))?;

    let span_selector =
        Selector::parse("span").map_err(|e| anyhow!("Invalid span selector: {e}"))?;

    let td_selector = Selector::parse("td").map_err(|e| anyhow!("Invalid td selector: {e}"))?;

    let mut submissions = Vec::new();

    for row in document.select(&row_selector) {
        let cells: Vec<_> = row.select(&td_selector).collect();

        // Submission ID
        let submission_id = row
            .select(&submission_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .and_then(|href| href.rsplit('/').next())
            .filter(|s| !s.is_empty())
            .context("Missing submission ID")?
            .parse::<u64>()
            .context("Invalid submission ID")?;

        // Submission datetime
        // Example: 2026-11-11 02:35:12+0900
        let datetime = row
            .select(&time_selector)
            .next()
            .map(|time| time.text().collect::<String>())
            .context("Missing submission datetime")?;

        let created = DateTime::parse_from_str(datetime.trim(), "%Y-%m-%d %H:%M:%S%z")
            .context("Invalid submission datetime")?
            .timestamp();

        // Problem name
        let problem = row
            .select(&problem_selector)
            .next()
            .map(|a| a.text().collect::<String>().trim().to_string())
            .context("Missing problem name")?;

        let task_id = row
            .select(&problem_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .and_then(|href| href.rsplit('/').next())
            .filter(|s| !s.is_empty())
            .context("Missing task ID")?
            .to_string();

        // Score (5th column)
        let score = cells[4]
            .text()
            .collect::<String>()
            .trim()
            .parse::<u32>()
            .context("Invalid score")?;

        // Result (7th column)
        let waiting_judge = row
            .select(
                &Selector::parse("td.waiting-judge")
                    .map_err(|e| anyhow!("Invalid waiting-judge selector: {e}"))?,
            )
            .next()
            .is_some();

        let (result, progress_current, progress_max) = if waiting_judge {
            let result_cell = cells
                .iter()
                .find(|cell| cell.value().classes().any(|class| class == "waiting-judge"))
                .context("Missing waiting-judge column")?;

            let result_text = result_cell.text().collect::<String>().trim().to_string();

            parse_result(&result_text)?
        } else {
            let result_cell = cells.get(6).context("Missing result column")?;

            let result_text = result_cell
                .select(&span_selector)
                .next()
                .map(|span| span.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| result_cell.text().collect::<String>().trim().to_string());

            parse_result(&result_text)?
        };

        // Execution time and memory usage
        let (execution_time, memory_usage) = if waiting_judge {
            (0, 0)
        } else {
            let execution_time = parse_resource_value(
                &cells
                    .get(7)
                    .context("Missing execution time column")?
                    .text()
                    .collect::<String>(),
                "ms",
            )?;

            let memory_usage = parse_resource_value(
                &cells
                    .get(8)
                    .context("Missing memory usage column")?
                    .text()
                    .collect::<String>(),
                "KiB",
            )?;

            (execution_time, memory_usage)
        };

        submissions.push(Submission {
            submission_id,
            created,
            problem,
            task_id,
            result,
            score,
            execution_time,
            memory_usage,
            progress_current,
            progress_max,
        });
    }

    Ok(submissions)
}

fn decode_submission_json(data: &Value) -> anyhow::Result<Vec<SubmissionUpdateQuery>> {
    let results = data
        .get("Result")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing Result"))?;

    let td_selector = Selector::parse("td").map_err(|e| anyhow!("Invalid selector: {e}"))?;

    let waiting_selector = Selector::parse("td.waiting-judge")
        .map_err(|e| anyhow!("Invalid waiting-judge selector: {e}"))?;

    let span_selector =
        Selector::parse("span").map_err(|e| anyhow!("Invalid span selector: {e}"))?;

    let mut submissions = Vec::with_capacity(results.len());

    for (submission_id, data) in results {
        let html = data["Html"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing Html"))?;

        let score = data["Score"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing Score"))?
            .parse::<u32>()?;

        let document =
            Html::parse_document(&format!("<table><tbody><tr>{}</tr></tbody></table>", html));

        let cells: Vec<_> = document.select(&td_selector).collect();

        let waiting_judge = document.select(&waiting_selector).next().is_some();

        let (result, progress_current, progress_max) = if waiting_judge {
            let cell = cells
                .iter()
                .find(|cell| cell.value().classes().any(|class| class == "waiting-judge"))
                .ok_or_else(|| {
                    anyhow!("Missing waiting-judge cell for submission {submission_id}")
                })?;

            let result_text = cell.text().collect::<String>().trim().to_string();

            parse_result(&result_text)?
        } else {
            if cells.len() != 3 {
                bail!(
                    "Invalid HTML structure for submission {submission_id}: expected 3 cells, got {}",
                    cells.len()
                );
            }

            let result_text = cells[0]
                .select(&span_selector)
                .next()
                .map(|span| span.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| cells[0].text().collect::<String>().trim().to_string());

            parse_result(&result_text)?
        };

        let (execution_time, memory_usage) = if waiting_judge {
            (0, 0)
        } else {
            let execution_time = parse_resource_value(&cells[1].text().collect::<String>(), "ms")?;

            let memory_usage = parse_resource_value(&cells[2].text().collect::<String>(), "KiB")?;

            (execution_time, memory_usage)
        };

        submissions.push(SubmissionUpdateQuery {
            submission_id: submission_id.parse::<u64>()?,
            result,
            score,
            execution_time,
            memory_usage,
            progress_current,
            progress_max,
        });
    }

    Ok(submissions)
}
