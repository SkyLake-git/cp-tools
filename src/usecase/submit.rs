use crate::atcoder::submit::{submit, translate_language_id};
use crate::def::contest_type::ContestType;
use crate::usecase::workspace::Workspace;
use anyhow::Context;

pub fn run(workspace: &mut Workspace, source_code: &str) -> anyhow::Result<bool> {
    let suite = workspace.fetch_test_suite()?;
    Ok(match workspace.settings.general.contest_type {
        ContestType::AtCoder => submit(
            &suite.problem_id,
            translate_language_id(workspace.settings.general.language.clone())
                .context("Language is invalid")?,
            &source_code,
        ),
    }?)
}
