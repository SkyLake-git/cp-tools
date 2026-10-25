use crate::config::settings::Settings;
use crate::constants::PROJECT_DIR_NAME;
use crate::def::testcase::TestcaseUnit;
use crate::tester::judge::{ApproxFloatUnitJudger, MatchUnitJudger, SelectUnitJudge, UnitJudge};
use crate::tester::suite::TestSuite;
use std::fs;
use std::path::PathBuf;

pub const WORKSPACE_SETTINGS_FN: &'static str = "settings.toml";
pub const WORKSPACE_TESTCASES_FN: &'static str = "testcases.toml";

pub struct Workspace {
    pub path: PathBuf,
    pub settings: Settings,
    pub test_suite: Option<TestSuite>,
}

impl Workspace {
    pub fn init(path: PathBuf) {
        fs::create_dir(path.join(PROJECT_DIR_NAME)).ok();
        fs::write(
            path.join(PROJECT_DIR_NAME).join(WORKSPACE_SETTINGS_FN),
            toml::to_string(&Settings::default()).expect("failed to serialize settings"),
        )
        .expect("failed to write settings");
    }
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let r = fs::read_to_string(path.join(PROJECT_DIR_NAME).join(WORKSPACE_SETTINGS_FN))?;
        Ok(Self {
            path,
            settings: toml::from_str(&r)?,
            test_suite: None,
        })
    }

    pub fn save_test_suite(&mut self, suite: &TestSuite) {
        fs::write(
            self.path
                .join(PROJECT_DIR_NAME)
                .join(WORKSPACE_TESTCASES_FN),
            toml::to_string(suite).expect("failed to serialize settings"),
        )
        .expect("failed to write file");
    }

    pub fn fetch_test_suite(&mut self) -> anyhow::Result<TestSuite> {
        if self.test_suite.is_some() {
            return Ok(self.test_suite.as_mut().unwrap().clone());
        }

        let r = fs::read_to_string(
            self.path
                .join(PROJECT_DIR_NAME)
                .join(WORKSPACE_TESTCASES_FN),
        )?;

        let suite: TestSuite = toml::from_str(&r).expect("failed to parse testcases");

        self.test_suite.replace(suite.clone());

        Ok(suite)
    }
}

impl SelectUnitJudge for Workspace {
    fn select_judger(&self, unit: &TestcaseUnit) -> Box<dyn UnitJudge> {
        if unit.expected.text.contains('.')
            && unit.actual.text.parse::<f64>().is_ok()
            && unit.expected.text.parse::<f64>().is_ok()
        {
            Box::new(ApproxFloatUnitJudger::new(
                self.settings.tester.error_threshold,
            ))
        } else {
            Box::new(MatchUnitJudger::new())
        }
    }
}
