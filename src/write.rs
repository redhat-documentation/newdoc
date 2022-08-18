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

use std::fs;
use std::io;
use std::path::PathBuf;

use color_eyre::eyre::{eyre, Result, WrapErr};

use crate::module::Module;
use crate::Options;

impl Module {
    /// Write the generated module content to the path specified in `options` with the set file name.
    pub fn write_file(&self, options: &Options) -> Result<()> {
        // Compose the full (but still relative) file path from the target directory and the file name
        let full_path_buf: PathBuf = [&options.target_dir, &self.file_name].iter().collect();
        let full_path = full_path_buf.as_path();

        log::debug!("Writing file `{}`", &full_path.display());

        // If the target file already exists, just print out an error
        if full_path.exists() {
            // A prompt enabling the user to overwrite the existing file
            log::warn!("File already exists: {}", full_path.display());
            log::warn!("Do you want to overwrite it? [y/N] ");

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .wrap_err_with(|| eyre!("Failed to read your response: {:?}", answer))?;

            match answer.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    log::warn!("→ Rewriting the file.");
                }
                _ => {
                    log::info!("→ Preserving the existing file.");
                    // Break from generating this particular module.
                    // Other modules that might be in the queue will be generated on next iteration.
                    return Ok(());
                }
            };
        }

        // If the target file doesn't exist, try to write to it
        fs::write(full_path, &self.text)
            .wrap_err_with(|| eyre!("Failed to write the `{}` file.", &full_path.display()))?;

        // If the write succeeds, print the include statement
        log::debug!("Successfully written file `{}`", &full_path.display());
        log::info!("‣ File generated: {}", full_path.display());
        log::info!("  {}", self.include_statement);

        Ok(())
    }
}
