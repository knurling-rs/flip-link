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

/// Link using a custom linker script and stack starting point. _(This is the whole point of `flip-link`)_
///
/// * `args` are arguments passed to the linker invocation
/// * `current_dir` is the directory from which the linker was invoked
/// * `stack_start` is the new, custom starting point from which our stack grows downwards–
///   * this should be right *below* the `.bss+.data` region that we've moved to the top, e.g.:
///     ```
///      +-------------+
///      | .bss+.data  |
///      +-------------+ <-- `stack_start`
///      |    stack    |
///      |      |      |
///      |      v      |
///      | ~~~~~~~~~~~ |
///      |             |
///      +-------------+
///     ```
/// * `custom_linker_script_location` is the directory in which the linker script to be used is located
pub fn link_modified(
    args: &[String],
    current_dir: &Path,
    custom_linker_script_location: &TempDir,
    stack_start: u64,
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
        .arg(format!("--defsym=_stack_start={}", stack_start))
        // set working directory to temporary directory containing our new linker script
        // this makes sure that it takes precedence over the original one
        .current_dir(custom_linker_script_location.path());
    log::trace!("{:?}", c);

    c.status()
}
