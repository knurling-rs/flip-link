use std::path::PathBuf;

/// Get `output_path`, specified by `-o`
pub fn get_output_path(args: &[String]) -> Option<&str> {
    let mut next_is_output = false;
    for arg in args {
        if arg == "-o" {
            next_is_output = true;
        } else if next_is_output {
            return Some(arg);
        }
    }

    None
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
