/*
newdoc: Generate pre-populated documentation modules formatted with AsciiDoc.
Copyright (C) 2022  Marek Suchánek  <msuchane@redhat.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use color_eyre::eyre::Result;

use newdoc::{cmd_line, config, logging};

fn main() -> Result<()> {
    // Enable full-featured error logging.
    color_eyre::install()?;

    // Parse the command-line options
    let cmdline_args = cmd_line::get_args();

    // Initialize the logging system based on the set verbosity
    logging::initialize_logger(cmdline_args.common_options.verbosity)?;

    // Set current options based on the command-line options and config files.
    let options = config::merge_configs(&cmdline_args)?;

    // Run the main functionality
    newdoc::run(&options, &cmdline_args)?;

    Ok(())
}
