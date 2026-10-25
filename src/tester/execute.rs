use crate::def::testcase::Testcase;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ExecutionResult {
    pub exit_code: ExitStatus,
    pub stdout: String,
    pub stderr: String,
    pub start_time: u128,
    pub terminated: bool,
}

impl ExecutionResult {
    pub fn new(
        exit_code: ExitStatus,
        stdout: String,
        stderr: String,
        start_time: u128,
        terminated: bool,
    ) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
            start_time,
            terminated,
        }
    }
}

pub trait Execute {
    fn spawn(&mut self, path: PathBuf, testcase: &Testcase);
    fn is_finished(&mut self) -> bool;
    fn get_result(&mut self) -> Result<ExecutionResult, String>;
    fn terminate(&mut self) -> Result<(), String>;
}

pub struct SimpleExecutor {
    pub proc: Option<Child>,
    pub input_stream: Option<File>,
    pub spawn_time: u128,
    pub terminated: bool,
}
impl SimpleExecutor {
    pub fn new() -> Self {
        Self {
            proc: None,
            input_stream: None,
            spawn_time: 0u128,
            terminated: false,
        }
    }
}
impl Execute for SimpleExecutor {
    fn spawn(&mut self, path: PathBuf, testcase: &Testcase) {
        let mut proc: Child = Command::new(path)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn process");

        proc.stdin
            .as_mut()
            .unwrap()
            .write_all((testcase.input.to_owned() + "\n").as_bytes())
            .expect("failed to write to stdin");

        self.proc = Some(proc);
        self.spawn_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("failed to get unix time")
            .as_millis();
    }

    fn is_finished(&mut self) -> bool {
        self.proc
            .as_mut()
            .expect("process is not spawned")
            .try_wait()
            .expect("failed to check status")
            .is_some()
    }

    fn get_result(&mut self) -> Result<ExecutionResult, String> {
        let child = self.proc.as_mut().ok_or("process is not spawned")?;
        let mut read_stdout = String::new();
        let mut read_stderr = String::new();

        child
            .stdout
            .as_mut()
            .ok_or("failed take stdout")?
            .read_to_string(&mut read_stdout)
            .expect("failed to read stdout");

        child
            .stderr
            .as_mut()
            .ok_or("failed take stderr")?
            .read_to_string(&mut read_stderr)
            .expect("failed to read stderr");

        Ok(ExecutionResult::new(
            child
                .try_wait()
                .unwrap()
                .ok_or("failed to check status")?
                .to_owned(),
            read_stdout.replace("\r\n", "\n"),
            read_stderr.replace("\r\n", "\n"),
            self.spawn_time,
            self.terminated,
        ))
    }

    fn terminate(&mut self) -> Result<(), String> {
        self.proc
            .as_mut()
            .ok_or("process is not spawned")?
            .kill()
            .expect("failed to kill");

        self.terminated = true;

        Ok(())
    }
}
