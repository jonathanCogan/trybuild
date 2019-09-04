use std::process::{Command, Output};
use crate::error::{Error, Result};

pub fn build_test_exe(test_name: &str, exe_path: &str, args: &[&str]) -> Result<Output> {
    
    let mut cmd = Command::new(exe_path);
    cmd.args(args).arg(test_name);
    cmd.output().map_err(Error::Cargo)
}