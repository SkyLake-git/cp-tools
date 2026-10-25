use crate::def::contest_type::ContestType::AtCoder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ContestType {
    AtCoder,
}

impl Into<String> for ContestType {
    fn into(self) -> String {
        match self {
            AtCoder => "atcoder".to_owned(),
        }
    }
}

impl TryFrom<String> for ContestType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "atcoder" => Ok(AtCoder),
            _ => Err("failed".to_owned()),
        }
    }
}
