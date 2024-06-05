/*
newdoc: Generate pre-populated documentation modules formatted with AsciiDoc.
Copyright (C) 2024  Marek Suchánek  <msuchane@redhat.com>

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

//! # `config.rs`
//!
//! This module defines the global options merged from the command line,
//! the configuration files, and the defaults.

use std::path::{Path, PathBuf};

use color_eyre::eyre::{Result, WrapErr};
use directories::ProjectDirs;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::cmd_line::{
    AnchorPrefixes, Cli, Comments, Examples, FilePrefixes, Metadata, Simplified, Verbosity,
};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// This struct stores options based on the command-line arguments,
/// and is passed to various functions across the program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub comments: bool,
    pub file_prefixes: bool,
    pub anchor_prefixes: bool,
    pub examples: bool,
    pub metadata: bool,
    pub target_dir: PathBuf,
    pub simplified: bool,
    pub verbosity: Verbosity,
}

impl Options {
    /// Update the values in this instance from the command line, but only in cases
    /// where the command line's values are specified.
    /// Where the command line options are missing, preserve the value in self.
    fn update_from_cli(&mut self, cli: &Cli) {
        // This code is kinda ugly and could be solved by figment merging:
        // https://steezeburger.com/2023/03/rust-hierarchical-configuration/
        // However, given how few options there are and how special the figment
        // solution is, I prefer this more explicit approach that gives manual control.

        // Update the manually specified values:
        match cli.common_options.comments {
            Some(Comments::Comments) => {
                self.comments = true;
            }
            Some(Comments::NoComments) => {
                self.comments = false;
            }
            None => { /* Keep the existing value. */ }
        }
        match cli.common_options.file_prefixes {
            Some(FilePrefixes::FilePrefixes) => {
                self.file_prefixes = true;
            }
            Some(FilePrefixes::NoFilePrefixes) => {
                self.comments = false;
            }
            None => { /* Keep the existing value. */ }
        }
        match cli.common_options.anchor_prefixes {
            Some(AnchorPrefixes::AnchorPrefixes) => {
                self.anchor_prefixes = true;
            }
            Some(AnchorPrefixes::NoAnchorPrefixes) => {
                self.anchor_prefixes = false;
            }
            None => { /* Keep the existing value. */ }
        }
        match cli.common_options.examples {
            Some(Examples::Examples) => {
                self.examples = true;
            }
            Some(Examples::NoExamples) => {
                self.examples = false;
            }
            None => { /* Keep the existing value. */ }
        }
        match cli.common_options.metadata {
            Some(Metadata::Metadata) => {
                self.metadata = true;
            }
            Some(Metadata::NoMetadata) => {
                self.metadata = false;
            }
            None => { /* Keep the existing value. */ }
        }
        match cli.common_options.simplified {
            Some(Simplified::Simplified) => {
                self.simplified = true;
            }
            Some(Simplified::NotSimplified) => {
                self.simplified = false;
            }
            None => { /* Keep the existing value. */ }
        }
        // TODO: Because the verbosity field isn't optional on the CLI, but rather
        // defaults to the `Default` value, the CLI always overrides the config files,
        // even though the config files recognize the option in theory.
        // Consider if it's useful to configure verbosity, and if so,
        // change the behavior so that the config files have effect.
        match cli.common_options.verbosity {
            Verbosity::Verbose => {
                self.verbosity = Verbosity::Verbose;
            }
            Verbosity::Quiet => {
                self.verbosity = Verbosity::Quiet;
            }
            Verbosity::Default => { /* Keep the existing value. */ }
        }

        // These options only exist on the command line, not in config files.
        // Always use the value from CLI arguments.
        self.target_dir = cli.common_options.target_dir.clone();
    }
}

impl Default for Options {
    /// This is the canonical source of default runtime options.
    fn default() -> Self {
        Self {
            comments: false,
            file_prefixes: true,
            anchor_prefixes: false,
            examples: true,
            simplified: false,
            metadata: true,
            verbosity: Verbosity::Default,
            target_dir: ".".into(),
        }
    }
}

/// Provides the base name of the configuration file:
/// The `hidden` option controls whether this is a hidden file
/// with a dot at the start, such as `.newdoc.toml`, or
/// a regular, visible file, such as `newdoc.toml`.
fn config_file_name(hidden: bool) -> String {
    let prefix = if hidden { "." } else { "" };
    format!("{prefix}{PKG_NAME}.toml")
}

/// Try to locale the appropriate per-user configuration file on this platform.
fn home_conf_file() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "Red Hat", PKG_NAME)?;
    let conf_dir = proj_dirs.config_dir();
    let conf_file = conf_dir.join(config_file_name(false));

    Some(conf_file)
}

/// If the target location is in a Git repository, construct the path
/// to a configuration file at the repository's root.
/// Find all such configuration files if the Git repository is nested.
fn git_conf_files(target_dir: &Path) -> Vec<PathBuf> {
    // Find all ancestor directories that appear to be the root of a Git repo.
    let git_roots = target_dir.ancestors().filter(|dir| {
        // The simple heuristic is that the directory is the Git root if it contains
        // the `.git/` sub-directory.
        let git_dir = dir.join(".git");
        git_dir.is_dir()
    });

    let config_files: Vec<_> = git_roots
        .map(|root| root.join(config_file_name(true)))
        .collect();

    config_files
}

/// Combine the configuration found on the command line, in configuration files,
/// and in the defaults. Follows the standard hierarchy.
pub fn merge_configs(cli: &Cli) -> Result<Options> {
    // The default options are the base for further merging.
    let default_options = Options::default();

    // Prepare a figment instance to load config files.
    let mut figment = Figment::from(Serialized::defaults(default_options));

    // Load the home configuration file, if it exists:
    if let Some(home_conf_file) = home_conf_file() {
        log::debug!("Home configuration file: {}", home_conf_file.display());
        figment = figment.merge(Toml::file(home_conf_file));
    } else {
        // If the directory lookup fails because there's no home directory,
        // skip the processing of the home configuration file.
        log::warn!("Failed to locate a home directory. Skipping home configuration.");
    };

    // All config files in Git repo roots:
    let mut git_conf_files = git_conf_files(&cli.common_options.target_dir);
    // Reverse their order so that the inner repo configuration takes precedence over outer:
    git_conf_files.reverse();
    // Load each Git repo configuration file:
    for file in git_conf_files {
        log::info!("Git repo configuration file: {}", file.display());
        figment = figment.merge(Toml::file(file));
    }

    log::debug!("Figment configuration: {figment:#?}");

    let mut conf_options: Options = figment
        .extract()
        .wrap_err("Failed to load configuration files.")?;

    conf_options.update_from_cli(cli);

    Ok(conf_options)
}
