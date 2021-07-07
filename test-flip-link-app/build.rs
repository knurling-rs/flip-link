fn main() -> anyhow::Result<()> {
    // include following if hal is excluded (aka. default features are disabled)
    #[cfg(not(feature = "lm3s6965"))]
    {
        use std::{env, fs::File, io::Write, path::PathBuf};

        let out = PathBuf::from(env::var_os("OUT_DIR").expect("`OUT_DIR` is not set"));

        File::create(out.join("memory.x"))?.write_all(include_bytes!(".memory.x"))?;
        File::create(out.join("empty.x"))?.write_all(include_bytes!(".empty.x"))?;

        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=.memory.x");
        println!("cargo:rerun-if-changed=.empty.x");
        println!("cargo:rerun-if-changed=build.rs");
    }

    Ok(())
}
