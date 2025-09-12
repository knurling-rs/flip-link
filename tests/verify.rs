use std::fs;

use rstest::rstest;

/// Path to test app
const CRATE: &str = "test-flip-link-app";
/// Example firmware in `$CRATE/examples`
const FILES: [&str; 4] = ["crash", "exception", "hello", "panic"];
/// Compilation target firmware is build for
const TARGET: &str = "thumbv7em-none-eabi";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[rstest]
#[case::normal(true)]
#[case::custom_linkerscript(false)]
fn should_link_example_firmware(#[case] default_features: bool) {
    // Arrange
    cargo::check_flip_link();

    // Act
    let cmd = cargo::build_example_firmware(default_features);

    // Assert
    cmd.success();
}

#[test]
fn should_verify_memory_layout() -> Result<()> {
    // Arrange
    cargo::check_flip_link();

    // Act
    cargo::build_example_firmware(true).success();

    // Assert
    for elf_path in elf::paths() {
        // read and parse elf-file
        let elf = fs::read(&elf_path)?;
        let object = object::File::parse(&*elf)?;

        // get the relevant sections
        let [bss, data, uninit, vector_table] = elf::get_sections(&object);
        // compute the initial stack-pointer from `.vector_table`
        let initial_sp = elf::compute_initial_sp(&vector_table)?;
        // get the bounds of 'static RAM'
        let bounds = elf::get_bounds(&[data, bss, uninit])?;

        // Is the initial stack-pointer below 'static RAM'?
        assert!(initial_sp <= *bounds.start(),);
    }

    // ---
    Ok(())
}

mod cargo {
    use std::process::Command;

    use assert_cmd::{assert::Assert, prelude::*};

    use super::*;

    /// Build all examples in `$REPO/$rel_path`
    #[must_use]
    pub(crate) fn build_example_firmware(default_features: bool) -> Assert {
        // append `rel_path` to the current working directory
        let mut firmware_dir = std::env::current_dir().unwrap();
        firmware_dir.push(CRATE);

        // disable default features or use `-v` as a no-op
        let default_features = match default_features {
            false => "--no-default-features",
            true => "-v",
        };

        Command::new("cargo")
            .args(["build", "--examples", default_features])
            .current_dir(firmware_dir)
            .unwrap()
            .assert()
    }

    /// Check that `flip-link` is present on the system
    pub(crate) fn check_flip_link() {
        Command::new("which")
            .arg("flip-link")
            .unwrap()
            .assert()
            .success();
    }
}

mod elf {
    use std::{ops::RangeInclusive, path::PathBuf};

    use object::{File, Object, ObjectSection, Section};

    use super::*;

    /// Get the initial stack pointer.
    ///
    /// It is the first 32-bit word in the `.vector_table` section,
    /// according to the "ARMv6-M Architecture Reference Manual".
    pub(crate) fn compute_initial_sp(vector_table: &Section<'_, '_>) -> Result<u64> {
        let data = vector_table.uncompressed_data()?;
        let sp = u32::from_le_bytes(data[..4].try_into()?);
        Ok(sp as u64)
    }

    /// Get [`RangeInclusive`] from lowest to highest address of all sections
    pub(crate) fn get_bounds(sections: &[Section<'_, '_>]) -> Result<RangeInclusive<u64>> {
        // get beginning and end of all sections
        let addresses = sections
            .iter()
            .flat_map(|sec| [sec.address(), sec.address() + sec.size()])
            .collect::<Vec<_>>();

        // get highest and lowest address of all sections
        let min = *addresses.iter().min().ok_or("empty iterator".to_string())?;
        let max = *addresses.iter().max().ok_or("empty iterator".to_string())?;

        Ok(min..=max)
    }

    /// Get the following sections from the elf-file:
    /// * `.bss`
    /// * `.data`
    /// * `.uninit`
    /// * `.vector_table`
    pub(crate) fn get_sections<'file>(object: &'file File<'_>) -> [Section<'file, 'file>; 4] {
        // try to get section, else error
        let get_section = |section_name| object.section_by_name(section_name).expect(section_name);

        [
            get_section(".bss"),
            get_section(".data"),
            get_section(".uninit"),
            get_section(".vector_table"),
        ]
    }

    /// Paths to firmware binaries.
    pub(crate) fn paths() -> Vec<PathBuf> {
        FILES
            .into_iter()
            .map(|file_name| format!("{CRATE}/target/{TARGET}/debug/examples/{file_name}"))
            .map(PathBuf::from)
            .collect()
    }
}
