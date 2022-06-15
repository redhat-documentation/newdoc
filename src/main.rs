use color_eyre::eyre::Result;

use newdoc::{cmd_line, Options};

fn main() -> Result<()> {
    // Parse the command-line options
    let cmdline_args = cmd_line::get_args();

    // Set current options based on the command-line options
    let options = Options::new(&cmdline_args);

    // Run the main functionality
    newdoc::run(options, cmdline_args)?;

    Ok(())
}
