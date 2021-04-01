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