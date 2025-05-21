use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Get `output_path`, specified by `-o`
pub fn get_output_path(args: &[String]) -> crate::Result<&String> {
    args.windows(2)
        .find_map(|x| (x[0] == "-o").then(|| &x[1]))
        .ok_or_else(|| "(BUG?) `-o` flag not found".into())
}

/// Get `search_paths`, specified by `-L`
pub fn get_search_paths(args: &[String]) -> Vec<PathBuf> {
    args.windows(2)
        .filter(|&x| (x[0] == "-L"))
        .map(|x| PathBuf::from(&x[1]))
        .inspect(|path| log::trace!("new search path: {}", path.display()))
        .collect()
}

/// Get `search_targets`, the names of the linker scripts, specified by `-T`
pub fn get_search_targets(args: &[String]) -> Vec<Cow<str>> {
    args.iter()
        .filter_map(|arg| arg.strip_prefix("-T").map(Cow::Borrowed))
        .collect()
}

/// Expands @file arguments into the file's contents
pub fn expand_files(args: &[String]) -> Vec<String> {
    let mut expanded = Vec::with_capacity(args.len());

    for arg in args {
        if let Some(arg) = arg.strip_prefix('@') {
            // The normal linker was able to open the file, so this *should* never panic
            let file = File::open(arg).unwrap_or_else(|e| {
                panic!("Unable to open {arg}, this should never happen and should be reported: {e}")
            });
            let reader = BufReader::new(file);
            for line in reader.lines() {
                // Same as above, normal linker succeeded so we should too
                let line = line.unwrap_or_else(|e| {
                    panic!(
                        "Invalid file {arg}, this should never happen and should be reported: {e}"
                    )
                });
                // Remove quotes if they exist
                if line.starts_with('"') && line.ends_with('"') {
                    expanded.push(line[1..line.len() - 1].to_owned());
                } else {
                    expanded.push(line.to_owned());
                }
            }
        } else {
            expanded.push(arg.clone());
        }
    }

    expanded
}
