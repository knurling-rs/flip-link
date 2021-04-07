use std::{
    io,
    path::Path,
    process::{Command, ExitStatus},
};

use tempfile::TempDir;

const LINKER: &str = "rust-lld";

/// Normal linking with just the arguments the user provides
pub fn link_normally(args: &[String]) -> io::Result<ExitStatus> {
    let mut c = Command::new(LINKER);
    c.args(args);
    log::trace!("{:?}", c);

    c.status()
}

/// Re-link with modified arguments, which is the whole point of `flip-link`
///
/// See inline comments for details of the modifications.
pub fn link_modified(
    args: &[String],
    current_dir: &Path,
    new_origin: u64,
    tempdir: &TempDir,
) -> io::Result<ExitStatus> {
    let mut c = Command::new(LINKER);
    c
        // HACK `-L` needs to go after `-flavor gnu`; position is currently hardcoded
        .args(&args[..2])
        // add the current dir to the linker search path to include all unmodified scripts there
        .arg("-L".to_string())
        .arg(current_dir)
        // rest of arguments, except `-flavor gnu`
        .args(&args[2..])
        // we need to override `_stack_start` to make the stack start below fake RAM
        .arg(format!("--defsym=_stack_start={}", new_origin))
        // set working directory to temporary directory containing our new linker script
        // this makes sure that it takes precedence over the original one
        .current_dir(tempdir.path());
    log::trace!("{:?}", c);

    c.status()
}
