use crate::def::language::Language;
use crate::format::cpp::format::gcc_preprocess;
use crate::usecase::workspace::Workspace;
use anyhow::bail;
use regex::Regex;

pub fn escape_comment(input: &str) -> String {
    let re = fancy_regex::Regex::new(r"(?<!:)//(.*)\n").unwrap();
    let result = re.replace_all(&input, "/*$1*/\n");

    result.into_owned()
}

pub fn remove_newlines(input: &str) -> String {
    let re = Regex::new(r"\n[ \t]*").unwrap();
    let result = re.replace_all(&input, " ");

    result.into_owned()
}

/// LF only
pub fn format_source_code(workspace: &Workspace, source_code: &str) -> anyhow::Result<String> {
    let mut arranged_source_code: String = source_code.to_owned();

    // todo: support multi comment char (#, //)

    if workspace.settings.format.opt_begin_end {
        let beg_re = Regex::new(r#"// compete BOF([\s\S]*)// compete EOF"#)?;

        if let Some(cap) = beg_re.captures(&arranged_source_code) {
            arranged_source_code = cap[0].to_owned();
        }
    }

    arranged_source_code = match workspace.settings.general.language {
        Language::CppGcc => {
            if !workspace.settings.format.cpp.expand_include {
                arranged_source_code
            } else {
                gcc_preprocess(
                    &arranged_source_code,
                    workspace.settings.format.cpp.expand_library_paths.clone(),
                )?
            }
        }
        _ => bail!("unknown workspace language"),
    };

    if workspace.settings.format.opt_range_no_new_lines {
        if let Some((before, rest)) =
            arranged_source_code.split_once("// compete BEGIN_NO_NEW_LINES")
        {
            if let Some((middle, after)) = rest.split_once("// compete END_NO_NEW_LINES") {
                arranged_source_code = (before.trim().to_owned() + "\n").to_owned()
                    + &remove_newlines(&escape_comment(middle)).trim()
                    + "\n"
                    + after.trim();
            }
        }
    }

    Ok(arranged_source_code)
}
