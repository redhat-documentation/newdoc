//! This script auto-generates a man page from the CLI configuration.

use bpaf::Section;

// We're reusing the module just for the Cli struct. Ignore the rest of the code
// and don't report it as "never used" in this build script.
#[allow(dead_code)]
#[path = "src/cmd_line.rs"]
mod cmd_line;

// Man page metadata
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const SECTION: Section = Section::General;
const DATE: &str = "February 2023";
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const CARGO_PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const CARGO_PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

fn main() -> std::io::Result<()> {
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let parser = cmd_line::cli();

    let man_page = parser.as_manpage(
        CARGO_PKG_NAME,
        SECTION,
        DATE,
        CARGO_PKG_AUTHORS,
        CARGO_PKG_HOMEPAGE,
        CARGO_PKG_REPOSITORY,
    );

    let man_name = format!("{CARGO_PKG_NAME}.1");
    let man_path = out_dir.join(man_name);

    std::fs::write(man_path, man_page)?;

    Ok(())
}
