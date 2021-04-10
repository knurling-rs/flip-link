use std::{borrow::Cow, path::PathBuf};

use anyhow::anyhow;

/// Get `output_path`, specified by `-o`
pub fn get_output_path(args: &[String]) -> anyhow::Result<&String> {
    args.windows(2)
        .find_map(|x| {
            if x[0] == "-o" {
                return Some(&x[1]);
            }
            None
        })
        .ok_or_else(|| anyhow!("(BUG?) `-o` flag not found"))
}

/// Get `search_paths`, specified by `-L`
pub fn get_search_paths(args: &[String]) -> Vec<PathBuf> {
    args.windows(2)
        .filter_map(|x| {
            if x[0] == "-L" {
                log::trace!("new search path: {}", x[1]);
                return Some(PathBuf::from(&x[1]));
            }
            None
        })
        .collect::<Vec<_>>()
}

/// Get `search_targets`, the names of the linker scripts, specified by `-T`
pub fn get_search_targets(args: &[String]) -> Vec<Cow<str>> {
    args.iter()
        .filter_map(|arg| {
            const FLAG: &str = "-T";
            if let Some(filename) = arg.strip_prefix(FLAG) {
                return Some(Cow::Borrowed(filename));
            }
            None
        })
        .collect::<Vec<_>>()
}
