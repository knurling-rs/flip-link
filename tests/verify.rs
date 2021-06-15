const CRATE: &str = "test-flip-link-app";

#[test]
fn should_link_example_firmware() -> anyhow::Result<()> {
    // Arrange
    cargo::check_flip_link();

    // Act
    cargo::build_example_firmware()?;

    // Assert

    // ---
    Ok(())
}

mod cargo {
    use std::process::Command;

    use assert_cmd::prelude::*;

    use super::CRATE;

    pub fn build_example_firmware() -> anyhow::Result<()> {
        let mut firmware_dir = std::env::current_dir()?;
        firmware_dir.push(CRATE);

        Command::new("cargo")
            .args(&["build", "--examples"])
            .current_dir(firmware_dir)
            .unwrap()
            .assert()
            .success();
        Ok(())
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
