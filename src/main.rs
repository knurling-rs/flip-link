use std::{
    borrow::Cow,
    env,
    fs::{self, File},
    io::Write,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    process::{self, Command},
};

use anyhow::anyhow;
use object::{elf, Object as _, ObjectSection, SectionFlags};

const EXIT_CODE_FAILURE: i32 = 1;
// TODO make this configurable (via command-line flag or similar)
const LINKER: &str = "rust-lld";
/// Stack Pointer alignment required by the ARM architecture
const SP_ALIGN: u64 = 8;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // NOTE `skip` the name/path of the binary (first argument)
    let args = env::args().skip(1).collect::<Vec<_>>();

    // if linking succeeds then linker scripts are well-formed; we'll rely on that in the parser
    link_normally(&args).unwrap_or_else(|code| process::exit(code));

    let current_dir = env::current_dir()?;
    let linker_scripts = get_linker_scripts(&args, &current_dir)?;
    let output_path =
        get_output_path(&args).ok_or_else(|| anyhow!("(BUG?) `-o` flag not found"))?;

    // here we assume that we'll end with the same linker script as LLD
    // I'm unsure about how LLD picks a linker script when there are multiple candidates in the
    // library search path
    let mut ram_path_entry = None;
    for linker_script in linker_scripts {
        let script_contents = fs::read_to_string(linker_script.path())?;
        if let Some(entry) = find_ram_in_linker_script(&script_contents) {
            log::debug!("found {:?} in {}", entry, linker_script.path().display());
            ram_path_entry = Some((linker_script, entry));
            break;
        }
    }
    let (ram_linker_script, ram_entry) = ram_path_entry
        .ok_or_else(|| anyhow!("MEMORY.RAM not found after scanning linker scripts"))?;

    let elf = fs::read(output_path)?;
    let object = object::File::parse(&elf)?;

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

    log::info!(
        "new RAM region: ORIGIN={:#x}, LENGTH={}",
        new_origin,
        new_length
    );

    // to overwrite RAM we'll create a new linker script in a temporary directory
    let tempdir = tempfile::tempdir()?;
    let original_linker_script = fs::read_to_string(ram_linker_script.path())?;
    // XXX in theory could collide with a user-specified linker script
    let mut new_linker_script = File::create(tempdir.path().join(ram_linker_script.filename()))?;

    for (index, line) in original_linker_script.lines().enumerate() {
        if index == ram_entry.line {
            writeln!(
                new_linker_script,
                "  RAM : ORIGIN = {:#x}, LENGTH = {}",
                new_origin, new_length
            )?;
        } else {
            writeln!(new_linker_script, "{}", line)?;
        }
    }
    // commit file to disk
    drop(new_linker_script);

    // invoke the linker a second time
    let mut c2 = Command::new(LINKER);
    // add the current dir to the linker search path to include all unmodified scripts there
    // HACK `-L` needs to go after `-flavor gnu`; position is currently hardcoded
    c2.args(&args[..2])
        .arg("-L".to_string())
        .arg(current_dir)
        .args(&args[2..])
        // we need to override `_stack_start` to make the stack start below fake RAM
        .arg(format!("--defsym=_stack_start={}", new_origin))
        // set working directory to temporary directory containing our new linker script
        // this makes sure that it takes precedence over the original one
        .current_dir(tempdir.path());
    log::trace!("{:?}", c2);
    let status = c2.status()?;
    if !status.success() {
        process::exit(status.code().unwrap_or(EXIT_CODE_FAILURE));
    }

    Ok(())
}

fn link_normally(args: &[String]) -> Result<(), i32> {
    let mut c = Command::new(LINKER);
    c.args(args);
    log::trace!("{:?}", c);

    let status = c.status().unwrap();
    if !status.success() {
        return Err(status.code().unwrap_or(EXIT_CODE_FAILURE));
    }
    Ok(())
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
            if sh_flags & elf::SHF_ALLOC as u64 != 0 {
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

    log::info!(
        "used RAM spans: origin={:#x}, length={}, align={}",
        used_ram_start,
        used_ram_length,
        used_ram_align
    );

    (used_ram_length, used_ram_align)
}

fn round_down_to_nearest_multiple(x: u64, multiple: u64) -> u64 {
    x - (x % multiple)
}

struct LinkerScript {
    filename: String,
    full_path: PathBuf,
}

impl LinkerScript {
    fn filename(&self) -> &str {
        &self.filename
    }

    fn path(&self) -> &Path {
        &self.full_path
    }
}

fn get_linker_scripts(args: &[String], current_dir: &Path) -> anyhow::Result<Vec<LinkerScript>> {
    // search paths are the current dir and args passed by `-L`
    let mut search_paths = args
        .windows(2)
        .filter_map(|x| {
            if x[0] == "-L" {
                log::trace!("new search path: {}", x[1]);
                return Some(Path::new(&x[1]));
            }
            None
        })
        .collect::<Vec<_>>();
    search_paths.push(current_dir);

    // get names of linker scripts, passed via `-T`
    // FIXME this doesn't handle "-T memory.x" (as two separate CLI arguments)
    let mut search_list = args
        .iter()
        .filter_map(|arg| {
            const FLAG: &str = "-T";
            if arg.starts_with(FLAG) {
                let filename = &arg[FLAG.len()..];
                return Some(Cow::Borrowed(filename));
            }
            None
        })
        .collect::<Vec<_>>();

    // try to find all linker scripts from `search_list` in the `search_paths`
    let mut linker_scripts = vec![];
    while let Some(filename) = search_list.pop() {
        for dir in &search_paths {
            let full_path = dir.join(&*filename);

            if full_path.exists() {
                log::trace!("found {} in {}", filename, dir.display());
                let contents = fs::read_to_string(&full_path)?;

                // also load linker scripts `INCLUDE`d by other scripts
                for include in get_includes_from_linker_script(&contents) {
                    log::trace!("{} INCLUDEs {}", filename, include);
                    search_list.push(Cow::Owned(include.to_string()));
                }

                linker_scripts.push(LinkerScript {
                    filename: filename.into_owned(),
                    full_path,
                });
                break;
            }
        }
    }

    Ok(linker_scripts)
}

fn get_output_path(args: &[String]) -> Option<&str> {
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

/// Rm `token` from beginning of `line`, else `continue` loop iteration
macro_rules! eat {
    ($line:expr, $token:expr) => {
        if $line.starts_with($token) {
            $line[$token.len()..].trim()
        } else {
            continue;
        }
    };
}

fn get_includes_from_linker_script(linker_script: &str) -> Vec<&str> {
    let mut includes = vec![];
    for mut line in linker_script.lines() {
        line = line.trim();
        line = eat!(line, "INCLUDE");
        includes.push(line);
    }

    includes
}

/// Looks for "RAM : ORIGIN = $origin, LENGTH = $length"
// FIXME this is a dumb line-by-line parser
fn find_ram_in_linker_script(linker_script: &str) -> Option<MemoryEntry> {
    macro_rules! tryc {
        ($expr:expr) => {
            if let Some(x) = $expr {
                x
            } else {
                continue;
            }
        };
    }

    for (index, mut line) in linker_script.lines().enumerate() {
        line = line.trim();
        line = eat!(line, "RAM");

        // jump over attributes like (xrw) see parse_attributes()
        if let Some(i) = line.find(":") {
            line = line[i..].trim();
        }

        line = eat!(line, ":");
        line = eat!(line, "ORIGIN");
        line = eat!(line, "=");

        let boundary_pos = tryc!(line.find(|c| c == ',' || c == ' '));
        const HEX: &str = "0x";
        let origin = if line.starts_with(HEX) {
            tryc!(u64::from_str_radix(&line[HEX.len()..boundary_pos], 16).ok())
        } else {
            tryc!(line[..boundary_pos].parse().ok())
        };
        line = &line[boundary_pos..].trim();

        line = eat!(line, ",");
        line = eat!(line, "LENGTH");
        line = eat!(line, "=");

        let segments: Vec<&str> = line.split('+').map(|s| s.trim().trim_end()).collect();
        let mut total_length = 0;
        for segment in segments {
            let boundary_pos = segment
                .find(|c| c == 'K' || c == 'M')
                .unwrap_or(segment.len());
            let length: u64 = tryc!(segment[..boundary_pos].parse().ok());
            let raw = &segment[boundary_pos..];
            let mut chars = raw.chars();
            let unit = chars.next();
            if unit == Some('K') {
                total_length += length * 1024;
            } else if unit == Some('M') {
                total_length += length * 1024 * 1024;
            }
        }
        return Some(MemoryEntry {
            line: index,
            origin,
            length: total_length,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        const LINKER_SCRIPT: &str = "MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

INCLUDE device.x
";

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
    fn parse_plus() {
        const LINKER_SCRIPT: &str = "MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2M
  RAM : ORIGIN = 0x20020000, LENGTH = 368K + 16K
}

INCLUDE device.x
";

        assert_eq!(
            find_ram_in_linker_script(LINKER_SCRIPT),
            Some(MemoryEntry {
                line: 3,
                origin: 0x20020000,
                length: (368 + 16) * 1024,
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
        const LINKER_SCRIPT: &str = "MEMORY
{
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
    RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 128K
}
";

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
