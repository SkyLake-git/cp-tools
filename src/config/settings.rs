use crate::def::contest_type::ContestType;
use crate::def::language::Language;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub general: GeneralSettings,
    pub paths: PathSettings,
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
