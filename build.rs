//! This script auto-generates a man page from the CLI configuration.

use bpaf::doc::Section;
use time::OffsetDateTime;

// We're reusing the module just for the Cli struct. Ignore the rest of the code
// and don't report it as "never used" in this build script.
#[allow(dead_code)]
#[path = "src/cmd_line.rs"]
mod cmd_line;

// Man page metadata
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const SECTION: Section = Section::General;
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> std::io::Result<()> {
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let date = current_date();

    let parser = cmd_line::cli();

    let man_page = parser.render_manpage(
        CARGO_PKG_NAME,
        SECTION,
        Some(&date),
        Some(CARGO_PKG_AUTHORS),
        None,
    );

    let man_name = format!("{CARGO_PKG_NAME}.1");
    let man_path = out_dir.join(man_name);

    std::fs::write(man_path, man_page)?;

    Ok(())
}

/// Generate the current date to mark the last update of the man page.
/// The format is "Month Year".
fn current_date() -> String {
    let now = OffsetDateTime::now_utc();
    let month = now.month();
    let year = now.year();

    format!("{month} {year}")
}
