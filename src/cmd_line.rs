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

//! # `cmd_line.rs`
//!
//! This module defines the command-line arguments and behavior of `newdoc`.
//! It relies on the `clap` crate.

use std::path::PathBuf;

use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help(true))]
#[clap(group(
    ArgGroup::new("required")
                .args(&[
                    "assembly",
                    "concept",
                    "procedure",
                    "reference",
                    "snippet",
                    "validate",
                ])
                .required(true)
                .multiple(true)
))]
pub struct Cli {
    /// Create an assembly file
    #[clap(short, long, value_name = "TITLE")]
    pub assembly: Option<Vec<String>>,

    /// Create an assembly that includes the other specified modules
    #[clap(short, long = "include-in", value_name = "TITLE")]
    pub include_in: Option<String>,

    /// Create a concept module
    #[clap(short, long, value_name = "TITLE")]
    pub concept: Option<Vec<String>>,

    /// Create a procedure module
    #[clap(short, long, value_name = "TITLE")]
    pub procedure: Option<Vec<String>>,

    /// Create a reference module
    #[clap(short, long, value_name = "TITLE")]
    pub reference: Option<Vec<String>>,

    /// Create a snippet file
    #[clap(short, long, value_name = "TITLE")]
    pub snippet: Option<Vec<String>>,

    /// Validate (lint) an existing module or assembly file
    #[clap(short = 'l', long, value_name = "FILE")]
    pub validate: Option<Vec<PathBuf>>,

    /// Generate the file without any comments
    #[clap(short = 'C', long = "no-comments")]
    pub no_comments: bool,

    /// Generate the file without any example, placeholder content
    #[clap(short = 'E', long = "no-examples", alias = "expert-mode")]
    pub no_examples: bool,

    /// Do not use module type prefixes (such as `proc_`) in file names
    #[clap(short = 'P', long, alias = "no-prefixes")]
    pub no_file_prefixes: bool,

    /// Add use module type prefixes (such as `proc_`) in AsciiDoc anchors
    #[clap(short = 'A', long)]
    pub anchor_prefixes: bool,

    /// Save the generated files in this directory
    #[clap(short = 'T', long = "target-dir", value_name = "DIRECTORY")]
    pub target_dir: Option<PathBuf>,

    /// Display additional, debug messages
    #[clap(short, long, conflicts_with = "quiet")]
    pub verbose: bool,

    /// Hide info-level messages. Display only warnings and errors
    #[clap(short, long, conflicts_with = "verbose")]
    pub quiet: bool,
}

/// Get command-line arguments as the `Cli` struct.
#[must_use]
pub fn get_args() -> Cli {
    Cli::parse()
}
