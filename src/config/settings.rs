use crate::def::contest_type::ContestType;
use crate::def::language::Language;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub general: GeneralSettings,
    pub paths: PathSettings,
    pub language: LanguageSettings,
    pub tester: TesterSettings,
    pub atcoder: AtCoderSettings,
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
pub struct LanguageSettings {
    pub cpp: CppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CppSettings {
    pub expand_include: bool,
    pub opt_begin_end: bool,
    pub opt_range_no_new_lines: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TesterSettings {
    /// 問題の実行時間制限にマージン分追加する
    pub timeout_margin: u128,
    pub ignore_trailing_whitespace: bool,
    pub error_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AtCoderSettings {
    pub watch_interval_seconds: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            paths: PathSettings::default(),
            language: LanguageSettings::default(),
            tester: TesterSettings::default(),
            atcoder: AtCoderSettings::default(),
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
            source: PathBuf::from("./main.cpp"),
            executable: PathBuf::from("./main"),
        }
    }
}

impl Default for LanguageSettings {
    fn default() -> Self {
        Self {
            cpp: CppSettings::default(),
        }
    }
}
impl Default for CppSettings {
    fn default() -> Self {
        Self {
            expand_include: false,
            opt_begin_end: false,
            opt_range_no_new_lines: false,
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
            watch_interval_seconds: 2,
        }
    }
}
