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

use std::path::PathBuf;

use bpaf::Bpaf;
use serde::{Serialize, Deserialize};

/// Generate pre-populated module files formatted with AsciiDoc that are used in Red Hat and Fedora documentation.
#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
pub struct Cli {
    #[bpaf(
        external,
        group_help("Generate or validate files:"),
        guard(at_least_one_file, SOME_FILES)
    )]
    pub action: Action,

    #[bpaf(external, group_help("Common options:"))]
    pub common_options: CommonOptions,
}

#[derive(Clone, Debug, Bpaf)]
pub struct CommonOptions {
    /// Add module type prefixes (such as `proc_`) in AsciiDoc anchors
    #[bpaf(short('A'), long)]
    pub anchor_prefixes: bool,

    /// Generate the file without any example, placeholder content
    #[bpaf(short('E'), long, long("expert-mode"))]
    pub no_examples: bool,

    /// Do not use module type prefixes (such as `proc_`) in file names
    #[bpaf(short('P'), long, long("no-prefixes"))]
    pub no_file_prefixes: bool,

    /// Generate the file without conditionals for the Red Hat documentation pipeline. Suitable for upstream.
    #[bpaf(short('S'), long)]
    pub simplified: bool,

    /// Save the generated files in this directory
    #[bpaf(short('T'), long, argument("DIRECTORY"), fallback(".".into()))]
    pub target_dir: PathBuf,

    #[bpaf(external, fallback(Verbosity::default()))]
    pub verbosity: Verbosity,

    #[bpaf(external, fallback(Comments::default()))]
    pub comments: Comments,
}

#[derive(Clone, Debug, Bpaf)]
pub struct Action {
    /// Create an assembly file
    #[bpaf(short, long, argument("TITLE"))]
    pub assembly: Vec<String>,

    /// Create a concept module
    #[bpaf(short, long, argument("TITLE"))]
    pub concept: Vec<String>,

    /// Create a procedure module
    #[bpaf(short, long, argument("TITLE"))]
    pub procedure: Vec<String>,

    /// Create a reference module
    #[bpaf(short, long, argument("TITLE"))]
    pub reference: Vec<String>,

    /// Create a snippet file
    #[bpaf(short, long, argument("TITLE"))]
    pub snippet: Vec<String>,

    /// Create an assembly that includes the other specified modules
    #[bpaf(short, long, argument("TITLE"))]
    pub include_in: Option<String>,

    /// REMOVED: Validate (lint) an existing module or assembly file
    /// The option is hidden, has no effect, and exists only for compatibility
    /// with previous releases.
    #[bpaf(short('l'), long, argument("FILE"), hide)]
    pub validate: Vec<PathBuf>,
}

/// The verbosity level set on the command line.
/// The default option is invisible as a command-line argument.
#[derive(Clone, Copy, Debug, Bpaf, Serialize, Deserialize, Default, PartialEq)]
pub enum Verbosity {
    /// Display additional, debug messages
    #[bpaf(short, long)]
    Verbose,
    /// Hide info-level messages. Display only warnings and errors
    #[bpaf(short, long)]
    Quiet,
    #[default]
    #[bpaf(hide)]
    Default,
}

#[derive(Clone, Copy, Debug, Bpaf, Serialize, Deserialize, Default, PartialEq)]
pub enum Comments {
    /// Generate the file without any comments.
    /// This option is now the default.
    #[default]
    #[bpaf(short('C'), long)]
    NoComments,
    /// Generate the file with explanatory comments
    #[bpaf(short('M'), long)]
    Comments,
}

/// Check that the current command generates or validates at least one file.
fn at_least_one_file(action: &Action) -> bool {
    !action.assembly.is_empty()
        || !action.concept.is_empty()
        || !action.procedure.is_empty()
        || !action.reference.is_empty()
        || !action.snippet.is_empty()
        || !action.validate.is_empty()
        || action.include_in.is_some()
}

/// The error message if the command does not generate or validate files.
const SOME_FILES: &str = "Specify at least one file to generate or validate.";

/// Get command-line arguments as the `Cli` struct.
#[must_use]
pub fn get_args() -> Cli {
    cli().run()
}
