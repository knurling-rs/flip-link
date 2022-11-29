use std::{borrow::Cow, path::PathBuf};

/// Get `output_path`, specified by `-o`
pub fn get_output_path(args: &[String]) -> crate::Result<&String> {
    args.windows(2)
        .find_map(|x| (x[0] == "-o").then(|| &x[1]))
        .ok_or_else(|| "(BUG?) `-o` flag not found".into())
}

/// Get `search_paths`, specified by `-L`
pub fn get_search_paths(args: &[String]) -> Vec<PathBuf> {
    args.windows(2)
        .filter_map(|x| (x[0] == "-L").then(|| PathBuf::from(&x[1])))
        .inspect(|path| log::trace!("new search path: {}", path.display()))
        .collect::<Vec<_>>()
}

/// Get `search_targets`, the names of the linker scripts, specified by `-T`
pub fn get_search_targets(args: &[String]) -> Vec<Cow<str>> {
    args.iter()
        .filter_map(|arg| arg.strip_prefix("-T").map(Cow::Borrowed))
        .collect::<Vec<_>>()
}
