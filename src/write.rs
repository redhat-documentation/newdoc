use std::fs;
use std::io;
use std::path::PathBuf;

use log::{debug, error, info, warn};

use crate::module::Module;
use crate::Options;

impl Module {
    /// Write the generated module content to the path specified in `options` with the set file name.
    pub fn write_file(&self, options: &Options) {
        // Compose the full (but still relative) file path from the target directory and the file name
        let full_path_buf: PathBuf = [&options.target_dir, &self.file_name].iter().collect();
        let full_path = full_path_buf.as_path();

        debug!("Writing file `{}`", &full_path.display());

        // If the target file already exists, just print out an error
        if full_path.exists() {
            // A prompt enabling the user to overwrite the existing file
            warn!("File already exists: {}", full_path.display());
            warn!("Do you want to overwrite it? [y/N] ");

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read your response");

            match answer.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    warn!("→ Rewriting the file.");
                }
                _ => {
                    info!("→ Preserving the existing file.");
                    // Break from generating this particular module.
                    // Other modules that might be in the queue will be generated on next iteration.
                    return;
                }
            };
        }

        // If the target file doesn't exist, try to write to it
        let result = fs::write(full_path, &self.text);
        match result {
            // If the write succeeds, print the include statement
            Ok(()) => {
                debug!("Successfully written file `{}`", &full_path.display());
                info!("‣ File generated: {}", full_path.display());
                info!("  {}", self.include_statement);
            }
            // If the write fails, print why it failed
            Err(e) => {
                error!("Failed to write the `{}` file: {}", &full_path.display(), e);
            }
        }
    }
}
