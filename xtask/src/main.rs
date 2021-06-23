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

    /// Install local revision of `flip-link`.
    pub fn install_flip_link() -> anyhow::Result<()> {
        let status = Command::new("cargo")
            .args(&["install", "--path", "."])
            .status()?;
        match status.success() {
            false => Err(anyhow::anyhow!("installing flip-link from path")),
            true => Ok(()),
        }
    }

    pub fn test() -> anyhow::Result<()> {
        let status = Command::new("cargo").arg("test").status()?;
        match status.success() {
            false => Err(anyhow::anyhow!("running `cargo test`")),
            true => Ok(()),
        }
    }
}
