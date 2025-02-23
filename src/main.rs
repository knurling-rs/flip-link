mod argument_parser;
mod linking;

use std::{
    borrow::Cow,
    env,
    fs::{self, File},
    io::Write,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    process,
};

use object::{elf, Object as _, ObjectSection, SectionFlags};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const EXIT_CODE_FAILURE: i32 = 1;
/// Stack Pointer alignment required by the ARM architecture
const SP_ALIGN: u64 = 8;

fn main() -> Result<()> {
    notmain().map(|code| process::exit(code))
}

fn notmain() -> Result<i32> {
    env_logger::init();

    // NOTE `skip` the name/path of the binary (first argument)
    let raw_args = env::args().skip(1).collect::<Vec<_>>();

    // If there's no argument provided, print the help message
    if let None | Some("--help" | "-h") = raw_args.first().map(String::as_str) {
        eprintln!(
            "flip-link: adds zero-cost stack overflow protection to your \
            embedded programs\nYou should not use flip-link directly from \
            command line, use flip-link as your default linker instead. \n\n\
            For detailed usage, check https://github.com/knurling-rs/flip-link"
        );
        return Ok(0);
    }

    {
        let exit_status = linking::link_normally(&raw_args)?;
        if !exit_status.success() {
            eprintln!(
                "\nflip-link: the native linker failed to link the program normally; \
                please check your project configuration and linker scripts"
            );
            return Ok(exit_status.code().unwrap_or(EXIT_CODE_FAILURE));
        }
        // if linking succeeds then linker scripts are well-formed; we'll rely on that in the parser
    }

    let expanded_args = argument_parser::expand_files(&raw_args);
    let current_dir = env::current_dir()?;
    let linker_scripts = get_linker_scripts(&expanded_args, &current_dir)?;

    // here we assume that we'll end with the same linker script as LLD
    // I'm unsure about how LLD picks a linker script when there are multiple candidates in the
    // library search path
    let mut ram_path_entry = None;
    for linker_script in linker_scripts {
        let script_contents = fs::read_to_string(linker_script.path())?;
        if let Some(entry) = find_ram_in_linker_script(&script_contents) {
            log::info!("found {entry} in {}", linker_script.path().display());
            ram_path_entry = Some((linker_script, entry));
            break;
        }
    }
    let (ram_linker_script, ram_entry) =
        ram_path_entry.ok_or("MEMORY.RAM not found after scanning linker scripts")?;

    let output_path = argument_parser::get_output_path(&expanded_args)?;
    let elf = fs::read(output_path)?;
    let object = object::File::parse(elf.as_slice())?;

    // TODO assert that `_stack_start == ORIGIN(RAM) + LENGTH(RAM)`
    // if that's not the case the user has specified a custom location for the stack; we should
    // error in that case (e.g. the stack may have been placed in CCRAM)

    // compute the span of RAM sections
    let (used_ram_length, used_ram_align) = compute_span_of_ram_sections(ram_entry, object);

    // the idea is to push `used_ram` all the way to the end of the RAM region
    // to do this we'll use a fake ORIGIN and LENGTH for the RAM region
    // this fake RAM region will be at the end of real RAM region
    let new_origin = round_down_to_nearest_multiple(
        ram_entry.end() - used_ram_length,
        used_ram_align.max(SP_ALIGN),
    );
    let new_length = ram_entry.end() - new_origin;

    log::info!("new RAM region: ORIGIN={new_origin:#x}, LENGTH={new_length}");

    // to overwrite RAM we'll create a new linker script in a temporary directory
    let exit_status = in_tempdir(|tempdir| {
        let original_linker_script = fs::read_to_string(ram_linker_script.path())?;
        // XXX in theory could collide with a user-specified linker script
        let mut new_linker_script = File::create(tempdir.join(ram_linker_script.file_name()))?;

        for (index, line) in original_linker_script.lines().enumerate() {
            if index == ram_entry.line {
                writeln!(
                    new_linker_script,
                    "  RAM : ORIGIN = {new_origin:#x}, LENGTH = {new_length}"
                )?
            } else {
                writeln!(new_linker_script, "{line}")?
            }
        }
        new_linker_script.flush()?;

        let exit_status = linking::link_modified(&raw_args, &current_dir, tempdir, new_origin)?;
        Ok(exit_status)
    })?;

    if !exit_status.success() {
        return Ok(exit_status.code().unwrap_or(EXIT_CODE_FAILURE));
    }

    Ok(0)
}

fn in_tempdir<T>(callback: impl FnOnce(&Path) -> Result<T>) -> Result<T> {
    // We avoid the `tempfile` crate because it pulls in quite a few dependencies.

    let mut random = [0; 8];
    getrandom::getrandom(&mut random).map_err(|e| e.to_string())?;

    let mut path = std::env::temp_dir();
    path.push(format!("flip-link-{random:02x?}"));
    fs::create_dir(&path)?;

    let res = callback(&path);

    // Just in case https://github.com/rust-lang/rust/issues/29497 hits us, we ignore the error from
    // removing the directory and just let it linger until the machine reboots ¯\_(ツ)_/¯.
    let _ = fs::remove_dir_all(&path);

    res
}

/// Returns `(used_ram_length, used_ram_align)`
fn compute_span_of_ram_sections(ram_entry: MemoryEntry, object: object::File) -> (u64, u64) {
    let mut used_ram_start = u64::MAX;
    let mut used_ram_end = 0;
    let mut used_ram_align = 0;
    let ram_region_span = ram_entry.span();
    let mut found_a_section = false;
    for section in object.sections() {
        if let SectionFlags::Elf { sh_flags } = section.flags() {
            if (sh_flags & elf::SHF_ALLOC as u64) != 0 {
                let start = section.address();
                let size = section.size();
                let end = start + size;

                if ram_region_span.contains(&start) && ram_region_span.contains(&end) {
                    found_a_section = true;
                    log::debug!(
                        "{} resides in RAM",
                        section.name().unwrap_or("nameless section")
                    );
                    used_ram_align = used_ram_align.max(section.align());

                    if used_ram_start > start {
                        used_ram_start = start;
                    }

                    if used_ram_end < end {
                        used_ram_end = end;
                    }
                }
            }
        }
    }

    let used_ram_length = if !found_a_section {
        used_ram_start = ram_entry.origin;
        0
    } else {
        used_ram_end - used_ram_start
    };

    log::info!("used RAM spans: origin={used_ram_start:#x}, length={used_ram_length}, align={used_ram_align}");

    (used_ram_length, used_ram_align)
}

fn round_down_to_nearest_multiple(x: u64, multiple: u64) -> u64 {
    x - (x % multiple)
}

struct LinkerScript(PathBuf);

impl LinkerScript {
    fn new(path: PathBuf) -> Self {
        assert!(path.is_file());
        Self(path)
    }

    fn file_name(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

fn get_linker_scripts(args: &[String], current_dir: &Path) -> Result<Vec<LinkerScript>> {
    let mut search_paths = vec![current_dir.into()];
    search_paths.extend(argument_parser::get_search_paths(args));

    let mut search_targets = argument_parser::get_search_targets(args);

    // try to find all linker scripts from `search_list` in the `search_paths`
    let mut linker_scripts = vec![];
    while let Some(filename) = search_targets.pop() {
        for dir in &search_paths {
            let full_path = dir.join(&*filename);

            if full_path.exists() {
                log::trace!("found {filename} in {}", dir.display());
                let contents = fs::read_to_string(&full_path)?;

                // also load linker scripts `INCLUDE`d by other scripts
                for include in get_includes_from_linker_script(&contents) {
                    log::trace!("{filename} INCLUDEs {include}");
                    search_targets.push(Cow::Owned(include.to_string()));
                }

                linker_scripts.push(LinkerScript::new(full_path));
                break;
            }
        }
    }

    Ok(linker_scripts)
}

/// Entry under the `MEMORY` section in a linker script
#[derive(Clone, Copy, Debug, PartialEq)]
struct MemoryEntry {
    line: usize,
    origin: u64,
    length: u64,
}

impl MemoryEntry {
    fn end(&self) -> u64 {
        self.origin + self.length
    }

    fn span(&self) -> RangeInclusive<u64> {
        self.origin..=self.end()
    }
}

impl std::fmt::Display for MemoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemoryEntry(line={}, origin=0x{:08x}, length=0x{:x})",
            self.line, self.origin, self.length
        )
    }
}

/// Rm `token` from beginning of `line`, else `continue` loop iteration
macro_rules! eat {
    ($line:expr, $token:expr) => {
        if let Some(a) = $line.strip_prefix($token) {
            a.trim()
        } else {
            continue;
        }
    };
}

/// This macro takes any expression which evaluates to a `Result`, returns the `Ok` value, or continues in case of an `Err`.
macro_rules! tryc {
    ($expr:expr) => {
        if let Ok(x) = $expr {
            x
        } else {
            continue;
        }
    };
}

fn get_includes_from_linker_script(linker_script: &str) -> Vec<&str> {
    linker_script
        .lines()
        .filter_map(|line| line.trim().strip_prefix("INCLUDE").map(str::trim))
        .collect()
}

/// Looks for "RAM : ORIGIN = $origin, LENGTH = $length"
// FIXME this is a dumb line-by-line parser
fn find_ram_in_linker_script(linker_script: &str) -> Option<MemoryEntry> {
    for (index, mut line) in linker_script.lines().enumerate() {
        line = line.trim();
        line = eat!(line, "RAM");

        // jump over attributes like (xrw) see parse_attributes()
        if let Some(i) = line.find(':') {
            line = line[i..].trim();
        }

        line = eat!(line, ':');
        line = eat!(line, "ORIGIN");
        line = eat!(line, '=');

        let boundary_pos = tryc!(line.find(',').ok_or(()));
        let origin = evaluate_expression(&line[..boundary_pos]) as u64;
        line = line[boundary_pos..].trim();

        line = eat!(line, ',');
        line = eat!(line, "LENGTH");
        line = eat!(line, '=');

        let length = evaluate_expression(line) as u64;

        return Some(MemoryEntry {
            line: index,
            origin,
            length,
        });
    }

    None
}

/// Evaluate a linker-script expression.
///
/// Panics if the expression is not understood
fn evaluate_expression(line: &str) -> i64 {
    log::debug!("Evaluating expression {:?}", line);

    let line = line.replace("K", "*1024");
    let line = line.replace("M", "*(1024*1024)");
    let line = line.replace("G", "*(1024*1024*1024)");

    log::debug!("Now evaluating unpacked expression {:?}", line);

    let value = match evalexpr::eval(&line) {
        Ok(evalexpr::Value::Int(n)) => n,
        Ok(val) => {
            panic!("Failed to parse expression {line:?} ({line:?}), got {val:?}?");
        }
        Err(e) => {
            panic!("Failed to parse expression {line:?} ({line:?}), got error {e:?}");
        }
    };

    log::debug!("Evaluated expression as {:#x}", value);

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x00000000, LENGTH = 256K
            RAM : ORIGIN = 0x20000000, LENGTH = 64K
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20000000,
                length: 64 * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    #[test]
    fn parse_no_units() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x00000000, LENGTH = 262144
            RAM : ORIGIN = 0x20000000, LENGTH = 65536
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20000000,
                length: 64 * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    #[test]
    fn ignore_comment() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x00000000, LENGTH = 256K
            RAM : ORIGIN = 0x20000000, LENGTH = 64K /* This is a comment */
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20000000,
                length: 64 * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    #[test]
    fn test_perform_addition_hex_and_number() {
        _ = env_logger::try_init();
        const ADDITION: &str = "0x20000000 + 1000";
        let expected: i64 = 0x20000000 + 1000;

        assert_eq!(evaluate_expression(ADDITION), expected);
    }

    #[test]
    fn test_perform_complex_maths() {
        _ = env_logger::try_init();
        const ADDITION: &str = "(0x20000000+1K)-512";
        let expected: i64 = 0x20000000 + 512;

        assert_eq!(evaluate_expression(ADDITION), expected);
    }

    #[test]
    fn test_perform_addition_returns_number() {
        _ = env_logger::try_init();
        const NO_ADDITION: &str = "0x20000000";
        let expected: i64 = 0x20000000;

        assert_eq!(evaluate_expression(NO_ADDITION), expected);
    }

    #[test]
    fn parse_plus() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x08000000, LENGTH = 2M
            RAM : ORIGIN = 0x20000000 + 0x20000, LENGTH = 512K - 256K
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20020000,
                length: (512 - 256) * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    #[test]
    fn parse_plus_origin_k() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x08000000, LENGTH = 2M
            RAM : ORIGIN = 0x20020000 + 100K, LENGTH = 368K
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20020000 + (100 * 1024),
                length: 368 * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    #[test]
    fn parse_plus_origin_no_units() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x08000000, LENGTH = 2M
            RAM : ORIGIN = 0x20020000 + 1000, LENGTH = 368K
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20020000 + 1000,
                length: 368 * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    #[test]
    fn parse_plus_origin_m() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            FLASH : ORIGIN = 0x08000000, LENGTH = 2M
            RAM : ORIGIN = 0x20020000 + 100M, LENGTH = 368K
        }

        INCLUDE device.x";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20020000 + (100 * 1024 * 1024),
                length: 368 * 1024,
            })
        );

        assert_eq!(
            get_includes_from_linker_script(LINKER_SCRIPT),
            vec!["device.x"]
        );
    }

    // test attributes https://sourceware.org/binutils/docs/ld/MEMORY.html
    #[test]
    fn parse_attributes() {
        _ = env_logger::try_init();
        const LINKER_SCRIPT: &str = "MEMORY
        {
            /* NOTE 1 K = 1 KiBi = 1024 bytes */
            FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
            RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 128K
        }";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 4,
                origin: 0x20000000,
                length: 128 * 1024,
            })
        );
    }
}
