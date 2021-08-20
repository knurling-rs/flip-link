fn main() -> anyhow::Result<()> {
    println!("\n⏳ install latest flip-link");
    cargo::install_flip_link()?;

    println!("\n🧪 cargo test");
    cargo::test()?;

    // ---
    Ok(())
}

mod cargo {
    use std::process::Command;

    use anyhow::anyhow;

    /// Install local revision of `flip-link`.
    pub fn install_flip_link() -> anyhow::Result<()> {
        let status = Command::new("cargo")
            .args(&["install", "--debug", "--force", "--path", "."])
            .status()?;
        match status.success() {
            false => Err(anyhow!("installing flip-link from path")),
            true => Ok(()),
        }
    }

    pub fn test() -> anyhow::Result<()> {
        let status = Command::new("cargo")
            // `--test-threads=1` prevents race conditions accessing the elf-file
            .args(&["test", "--", "--test-threads=1"])
            .status()?;
        match status.success() {
            false => Err(anyhow!("running `cargo test`")),
            true => Ok(()),
        }
    }
}
