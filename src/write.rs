use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use colored::*;

use crate::Options;
use crate::module::Module;

impl Module {
    /// Write the generated module content to the path specified in `options` with the set file name.
    pub fn write_file(&self) {
        // Compose the full (but still relative) file path from the target directory and the file name
        let full_path_buf: PathBuf = [&self.options.target_dir, &self.file_name].iter().collect();
        let full_path = full_path_buf.as_path();

        // If the target file already exists, just print out an error
        if full_path.exists() {
            // A prompt enabling the user to overwrite the existing file
            eprintln!(
                "{}",
                format!("W: File already exists: {}", full_path.display()).yellow()
            );
            eprint!("   Do you want to overwrite it? [y/N] ");
            // We must manually flush the buffer or else the printed string doesn't appear.
            // The buffer otherwise waits for a newline.
            io::stdout().flush().unwrap();

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read the response");

            match answer.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    eprintln!("   → Rewriting the file.");
                }
                _ => {
                    eprintln!("   → Preserving the existing file.");
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
                eprintln!("‣  File generated: {}", full_path.display());
                eprintln!("   {}", self.include_statement);
            }
            // If the write fails, print why it failed
            Err(e) => {
                eprintln!("{}", format!("E: Failed to write the file: {}", e).red());
            }
        }
    }
}
