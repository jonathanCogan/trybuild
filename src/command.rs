use std::process::{Command, Output};
use crate::error::{Error, Result};

pub fn build_test_exe(test_name: &str, exe_path: &str) -> Result<Output> {
    let mut cmd = Command::new(exe_path);
    cmd.arg(test_name).output().map_err(Error::Cargo)
}