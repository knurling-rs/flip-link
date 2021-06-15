const CRATE: &str = "test-flip-link-app";

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
