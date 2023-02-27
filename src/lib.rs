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

// Enable additional clippy lints by default.
#![warn(
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::clone_on_ref_ptr,
    clippy::todo
)]
// Disable the documentation clippy lint, so that it stops suggesting backticks around AsciiDoc.
#![allow(clippy::doc_markdown)]
// Forbid unsafe code in this program.
#![forbid(unsafe_code)]

//! # newdoc
//! The `newdoc` tool generates pre-populated module and assembly files formatted with AsciiDoc,
//! which are used in Red Hat and Fedora documentation. The generated files follow
//! the Modular Documentation guidelines: <https://redhat-documentation.github.io/modular-docs/>.

use std::path::PathBuf;

use color_eyre::eyre::{bail, eyre, Result, WrapErr};

pub mod cmd_line;
mod logging;
mod module;
mod templating;
mod validation;
mod write;

use cmd_line::{Cli, Verbosity};
pub use module::{ContentType, Input, Module};

/// newdoc uses many regular expressions at several places. Constructing them should never fail,
/// because the pattern doesn't change at runtime, but in case it does, present a unified
/// error message through `expect`.
const REGEX_ERROR: &str = "Failed to construct a regular expression. Please report this as a bug";

/// This struct stores options based on the command-line arguments,
/// and is passed to various functions across the program.
#[derive(Debug, Clone)]
pub struct Options {
    pub comments: bool,
    pub file_prefixes: bool,
    pub anchor_prefixes: bool,
    pub examples: bool,
    pub target_dir: PathBuf,
    pub verbosity: Verbosity,
}

impl Options {
    /// Set current options based on the command-line options
    #[must_use]
    pub fn new(cli: &Cli) -> Self {
        Self {
            // Comments and prefixes are enabled (true) by default unless you disable them
            // on the command line. If the no-comments or no-prefixes option is passed,
            // the feature is disabled, so the option is set to false.
            comments: !cli.no_comments,
            file_prefixes: !cli.no_file_prefixes,
            anchor_prefixes: cli.anchor_prefixes,
            examples: !cli.no_examples,
            // Set the target directory as specified or fall back on the current directory
            target_dir: cli.target_dir.clone().unwrap_or_else(|| PathBuf::from(".")),
            verbosity: cli.verbosity,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            comments: true,
            file_prefixes: true,
            anchor_prefixes: false,
            examples: true,
            target_dir: PathBuf::from("."),
            verbosity: Verbosity::Default,
        }
    }
}

pub fn run(options: &Options, cli: &Cli) -> Result<()> {
    // Initialize the logging system based on the set verbosity
    logging::initialize_logger(options.verbosity)?;

    log::debug!("Active options:\n{:#?}", &options);

    // Attach titles from the CLI to content types.
    let content_types = [
        (ContentType::Assembly, &cli.action.assembly),
        (ContentType::Concept, &cli.action.concept),
        (ContentType::Procedure, &cli.action.procedure),
        (ContentType::Reference, &cli.action.reference),
        (ContentType::Snippet, &cli.action.snippet),
    ];

    // Store all modules except for the populated assembly that will be created in this Vec
    let mut non_populated: Vec<Module> = Vec::new();

    // For each module type, see if it occurs on the command line and process it
    for (content_type, titles) in content_types {
        // Check if the given module type occurs on the command line
        let mut modules = process_module_type(titles, content_type, options);

        // Move all the newly created modules into the common Vec
        non_populated.append(&mut modules);
    }

    // Write all non-populated modules to the disk
    for module in &non_populated {
        module.write_file(options)?;
    }

    // Treat the populated assembly module as a special case:
    // * There can be only one populated assembly
    // * It must be generated after the other modules so that it can use their include statements
    if let Some(title) = &cli.action.include_in {
        // Gather all include statements for the other modules
        let include_statements: Vec<String> = non_populated
            .into_iter()
            .map(|module| module.include_statement)
            .collect();

        // The include_statements should never be empty thanks to the required group in clap.
        // Make sure once more, though.
        if include_statements.is_empty() {
            bail!("The populated assembly includes no other files.");
        }

        // Generate the populated assembly module
        let populated: Module = Input::new(ContentType::Assembly, title, options)
            .include(include_statements)
            .into();

        populated.write_file(options)?;
    }
    // Validate all file names specified on the command line
    for file in &cli.action.validate {
        validation::validate(file).wrap_err_with(|| eyre!("Failed to validate file {:?}", file))?;
    }

    Ok(())
}

/// Process all titles that have been specified on the command line and that belong to a single
/// module type.
fn process_module_type(
    titles: &[String],
    content_type: ContentType,
    options: &Options,
) -> Vec<Module> {
    let modules_from_type = titles
        .iter()
        .map(|title| Module::new(content_type, title, options));

    modules_from_type.collect()
}
