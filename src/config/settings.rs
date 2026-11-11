use crate::def::contest_type::ContestType;
use crate::def::language::Language;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub general: GeneralSettings,
    pub paths: PathSettings,
    pub format: FormatSettings,
    pub tester: TesterSettings,
    pub atcoder: AtCoderSettings,
    pub validation: ValidationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GeneralSettings {
    pub contest_type: ContestType,
    pub log_level: String,
    pub language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PathSettings {
    pub source: PathBuf,
    pub executable: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FormatSettings {
    pub opt_begin_end: bool,
    pub opt_range_no_new_lines: bool,
    pub out: FormatOutSettings,
    pub cpp: CppSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FormatOutSettings {
    pub method: FormatOutMethod,
    pub target_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormatOutMethod {
    Clipboard,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CppSettings {
    pub expand_include: bool,
    pub expand_library_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TesterSettings {
    pub timeout_margin: u128,
    pub ignore_trailing_whitespace: bool,
    pub error_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AtCoderSettings {
    pub watch_interval_millis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ValidationSettings {
    pub git: ValidationGitSettings,
    pub check_same_submission: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ValidationGitSettings {
    pub check_uncommitted_changes: bool,
    pub check_unpushed_commits: bool,
    pub target_paths: Vec<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            paths: PathSettings::default(),
            format: FormatSettings::default(),
            tester: TesterSettings::default(),
            atcoder: AtCoderSettings::default(),
            validation: ValidationSettings::default(),
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            language: Language::None,
            contest_type: ContestType::AtCoder,
        }
    }
}

impl Default for PathSettings {
    fn default() -> Self {
        Self {
            source: PathBuf::from("./source_path.ext"),
            executable: PathBuf::from("./executable_file.exe"),
        }
    }
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            opt_begin_end: false,
            opt_range_no_new_lines: false,
            out: FormatOutSettings::default(),
            cpp: CppSettings::default(),
        }
    }
}
impl Default for FormatOutSettings {
    fn default() -> Self {
        Self {
            method: FormatOutMethod::Clipboard,
            target_file: None,
        }
    }
}

impl Default for CppSettings {
    fn default() -> Self {
        Self {
            expand_include: false,
            expand_library_paths: vec![],
        }
    }
}

impl Default for TesterSettings {
    fn default() -> Self {
        Self {
            timeout_margin: 500,
            ignore_trailing_whitespace: true,
            error_threshold: 1e-6,
        }
    }
}

impl Default for AtCoderSettings {
    fn default() -> Self {
        Self {
            watch_interval_millis: 2000,
        }
    }
}

impl Default for ValidationSettings {
    fn default() -> Self {
        Self {
            git: ValidationGitSettings::default(),
            check_same_submission: true,
        }
    }
}

impl Default for ValidationGitSettings {
    fn default() -> Self {
        Self {
            check_uncommitted_changes: true,
            check_unpushed_commits: true,
            target_paths: vec![],
        }
    }
}
