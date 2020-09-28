use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use colored::*;

mod cmd_line;
mod module;

use module::{Module, ModuleType};


/// This struct stores options based on the command-line arguments,
/// and is passed to various functions across the program.
pub struct Options {
    comments: bool,
    prefixes: bool,
    target_dir: String,
}

fn main() {
    let cmdline_args = cmd_line::get_args();

    // Set current options based on the command-line options
    let options = Options {
        // Comments and prefixes are enabled (true) by default unless you disable them
        // on the command line. If the no-comments or no-prefixes option is passed
        // (occurences > 0), the feature is disabled, so the option is set to false.
        comments: cmdline_args.occurrences_of("no-comments") == 0,
        prefixes: cmdline_args.occurrences_of("no-prefixes") == 0,
        // Set the target directory as specified or fall back on the current directory
        target_dir: if let Some(dir) = cmdline_args.value_of("target-dir") {
            String::from(dir)
        } else {
            String::from(".")
        },
    };

    // Store all modules except for the populated assembly that will be created in this Vec
    let mut non_populated: Vec<Module> = Vec::new();

    // TODO: Maybe attach these strings to the ModuleType enum somehow
    // For each module type, see if it occurs on the command line and process it
    for module_type_str in ["assembly", "concept", "procedure", "reference"].iter() {
        // Check if the given module type occurs on the command line
        if let Some(titles_iterator) = cmdline_args.values_of(module_type_str) {
            let mut modules = process_module_type(titles_iterator, module_type_str, &options);

            // Move all the newly created modules into the common Vec
            non_populated.append(&mut modules);
        }
    }

    // Write all non-populated modules to the disk
    for module in non_populated.iter() {
        write_module(&module, &options);
    }

    // Treat the populated assembly module as a special case:
    // * There can be only one populated assembly
    // * It must be generated after the other modules so that it can use their include statements
    if let Some(title) = cmdline_args.value_of("include-in") {
        // Gather all include statements for the other modules
        // TODO: Figure out if this can be done without calling .to_owned on all the Strings
        let include_statements: Vec<String> = non_populated
            .iter()
            .map(|module| module.include_statement.to_owned())
            .collect();

        // The include_statements should never be empty thanks to the required group in clap
        assert!(!include_statements.is_empty());

        // Generate the populated assembly module
        let populated = Module::new(ModuleType::Assembly, title, &options)
            .includes(include_statements);

        write_module(&populated, &options);
    }
}

/// Process all titles that have been specified on the command line and that belong to a single
/// module type.
fn process_module_type(titles: clap::Values, module_type_str: &str, options: &Options) -> Vec<Module> {
    let mut modules_from_type = Vec::new();

    for title in titles {
        // Convert the string module type to an enum.
        // This must be done for each title separately so that the title can own the ModuleType.
        let module_type = match module_type_str {
            "assembly" => ModuleType::Assembly,
            "include-in" => ModuleType::Assembly,
            "concept" => ModuleType::Concept,
            "procedure" => ModuleType::Procedure,
            "reference" => ModuleType::Reference,
            _ => unimplemented!(),
        };

        let module = Module::new(module_type, title, &options);

        modules_from_type.push(module);
    }

    modules_from_type
}


/// Write the generated module content to the path specified in `options` with the set file name.
// fn write_module(file_name: &str, content: &str, options: &Options) {
fn write_module(module: &Module, options: &Options) {
    // Compose the full (but still relative) file path from the target directory and the file name
    let full_path_buf: PathBuf = [&options.target_dir, &module.file_name].iter().collect();
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
    let result = fs::write(full_path, &module.compose_text(&options));
    match result {
        // If the write succeeds, print the include statement
        Ok(()) => {
            eprintln!("‣  File generated: {}", full_path.display());
            eprintln!("   {}", module.include_statement);
        }
        // If the write fails, print why it failed
        Err(e) => {
            eprintln!("{}", format!("E: Failed to write the file: {}", e).red());
        }
    }
}

