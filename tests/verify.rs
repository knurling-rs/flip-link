use std::fs;

/// Path to test app
const CRATE: &str = "test-flip-link-app";
/// Example firmware in `$CRATE/examples`
const FILES: [&str; 4] = ["crash", "exception", "hello", "panic"];
/// Compilation target firmware is build for
const TARGET: &str = "thumbv7m-none-eabi";

#[test]
fn should_link_example_firmware() -> anyhow::Result<()> {
    // Arrange
    cargo::check_flip_link();

    // Act
    let cmd = cargo::build_example_firmware(CRATE);

    // Assert
    cmd.success();

    // ---
    Ok(())
}

#[test]
fn should_verify_memory_layout() -> anyhow::Result<()> {
    // Arrange
    cargo::check_flip_link();

    // Act
    cargo::build_example_firmware(CRATE).success();

    // Assert
    for elf_path in elf::paths() {
        // read and parse elf-file
        let elf = fs::read(elf_path)?;
        let object = object::File::parse(&*elf)?;

        // get the relevant sections
        let (bss, data, uninit, vector_table) = elf::get_sections(&object)?;
        // compute the initial stack-pointer from `.vector_table`
        let initial_sp = elf::compute_initial_sp(&vector_table)?;
        // get the bounds of 'static RAM'
        let bounds = elf::get_bounds(&[data, bss, uninit])?;

        // Is the initial stack-pointer inside 'static RAM'?
        assert!(bounds.contains(&initial_sp));
    }

    // ---
    Ok(())
}

mod cargo {
    use std::process::Command;

    use assert_cmd::{assert::Assert, prelude::*};

    /// Build all examples in `$REPO/$rel_path`
    #[must_use]
    pub fn build_example_firmware(rel_path: &str) -> Assert {
        // append `rel_path` to the current working directory
        let mut firmware_dir = std::env::current_dir().unwrap();
        firmware_dir.push(rel_path);

        Command::new("cargo")
            .args(&["build", "--examples"])
            .current_dir(firmware_dir)
            .unwrap()
            .assert()
    }

    /// Check that `flip-link` is present on the system
    pub fn check_flip_link() {
        Command::new("which")
            .arg("flip-link")
            .unwrap()
            .assert()
            .success();
    }
}

mod elf {
    use std::{convert::TryInto, ops::RangeInclusive};

    use anyhow::anyhow;
    use object::{File, Object, ObjectSection, Section};

    use super::{CRATE, FILES, TARGET};

    /// Get the initial stack pointer.
    ///
    /// It is the first 32-bit word in the `.vector_table` section,
    /// according to the "ARMv6-M Architecture Reference Manual".
    pub fn compute_initial_sp(vector_table: &Section) -> anyhow::Result<u64> {
        let data = vector_table.uncompressed_data()?;
        let sp = u32::from_le_bytes(data[..4].try_into()?);
        Ok(sp as u64)
    }

    /// Get [`RangeInclusive`] from lowest to highest address of all sections
    pub fn get_bounds(sections: &[Section]) -> anyhow::Result<RangeInclusive<u64>> {
        // get beginning and end of all sections
        let addresses = sections
            .iter()
            .flat_map(|sec| vec![sec.address(), sec.address() + sec.size()])
            .collect::<Vec<_>>();

        // get highest and lowest address of all sections
        let min = *addresses.iter().min().ok_or(anyhow!("empty iterator"))?;
        let max = *addresses.iter().max().ok_or(anyhow!("empty iterator"))?;

        Ok(min..=max)
    }

    /// Get the following sections from the elf-file:
    /// * `.bss`
    /// * `.data`
    /// * `.uninit`
    /// * `.vector_table`
    pub fn get_sections<'data, 'file>(
        object: &'file File<'data>,
    ) -> anyhow::Result<(
        Section<'data, 'file>,
        Section<'data, 'file>,
        Section<'data, 'file>,
        Section<'data, 'file>,
    )> {
        // try to get section, else error
        let get_section = |section_name| {
            object
                .section_by_name(section_name)
                .ok_or(anyhow!("error getting section `{}`", section_name))
        };

        Ok((
            get_section(".bss")?,
            get_section(".data")?,
            get_section(".uninit")?,
            get_section(".vector_table")?,
        ))
    }

    /// Paths to firmware binaries.
    pub fn paths() -> Vec<String> {
        FILES
            .iter()
            .map(|file_name| format!("{}/target/{}/debug/examples/{}", CRATE, TARGET, file_name))
            .collect()
    }
}
