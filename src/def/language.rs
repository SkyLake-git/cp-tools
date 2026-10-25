use crate::def::language::Language::{CppClang, CppGcc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    CppGcc,
    CppClang,
    None,
}

impl TryFrom<i32> for Language {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CppGcc),
            1 => Ok(CppClang),
            _ => Err(format!("unknown language id: {value}").to_owned()),
        }
    }
}
