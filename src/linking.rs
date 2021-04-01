use std::{path::Path, process::Command};

use tempfile::TempDir;

const EXIT_CODE_FAILURE: i32 = 1;
const LINKER: &str = "rust-lld";

pub fn link_normally(args: &[String]) -> Result<(), i32> {
    let mut c = Command::new(LINKER);
    c.args(args);
    log::trace!("{:?}", c);

    success_or_exitstatus(c)
}

pub fn link_again(
    args: &[String],
    current_dir: &Path,
    new_origin: u64,
    tempdir: &TempDir,
) -> Result<(), i32> {
    let mut c = Command::new(LINKER);
    // add the current dir to the linker search path to include all unmodified scripts there
    // HACK `-L` needs to go after `-flavor gnu`; position is currently hardcoded
    c.args(&args[..2])
        .arg("-L".to_string())
        .arg(current_dir)
        .args(&args[2..])
        // we need to override `_stack_start` to make the stack start below fake RAM
        .arg(format!("--defsym=_stack_start={}", new_origin))
        // set working directory to temporary directory containing our new linker script
        // this makes sure that it takes precedence over the original one
        .current_dir(tempdir.path());
    log::trace!("{:?}", c);

    success_or_exitstatus(c)
}

fn success_or_exitstatus(mut c: Command) -> Result<(), i32> {
    let status = c.status().unwrap();
    if !status.success() {
        return Err(status.code().unwrap_or(EXIT_CODE_FAILURE));
    }
    Ok(())
}
