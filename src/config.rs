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

use std::path::PathBuf;

use color_eyre::eyre::{Result, WrapErr};
use directories::ProjectDirs;
use figment::{Figment, providers::{Format, Toml, Serialized}};
use serde::{Serialize, Deserialize};

use crate::cmd_line::{Cli, Comments, Verbosity};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// This struct stores options based on the command-line arguments,
/// and is passed to various functions across the program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub comments: bool,
    pub file_prefixes: bool,
    pub anchor_prefixes: bool,
    pub examples: bool,
    pub target_dir: PathBuf,
    pub simplified: bool,
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
            comments: match cli.common_options.comments {
                Comments::Comments => true,
                Comments::NoComments => false,
            },
            file_prefixes: !cli.common_options.no_file_prefixes,
            anchor_prefixes: cli.common_options.anchor_prefixes,
            examples: !cli.common_options.no_examples,
            // Set the target directory as specified or fall back on the current directory
            target_dir: cli.common_options.target_dir.clone(),
            simplified: cli.common_options.simplified,
            verbosity: cli.common_options.verbosity,
        }
    }

    /// Update the values in this instance from another instance, but only in cases
    /// where the other instance's values are non-default.
    /// Where the other instance keeps default values, preserve the value in self.
    fn update_from_non_default(&mut self, cli_options: &Self) {
        let default = Self::default();

        // This code is kinda ugly and could be solved by figment merging:
        // https://steezeburger.com/2023/03/rust-hierarchical-configuration/
        // However, given how few options there are and how special the figment
        // solution is, I prefer this more explicit approach that gives manual control.

        // Update the non-default values:
        if cli_options.comments != default.comments {
            self.comments = cli_options.comments;
        }
        if cli_options.file_prefixes != default.file_prefixes {
            self.file_prefixes = cli_options.file_prefixes;
        }
        if cli_options.anchor_prefixes != default.anchor_prefixes {
            self.anchor_prefixes = cli_options.anchor_prefixes;
        }
        if cli_options.examples != default.examples {
            self.examples = cli_options.examples;
        }
        if cli_options.simplified != default.simplified {
            self.simplified = cli_options.simplified;
        }
        if cli_options.verbosity != default.verbosity {
            self.verbosity = cli_options.verbosity;
        }

        // These options only exist on the command line, not in config files.
        // Always use the non-default value from CLI arguments.
        self.target_dir = cli_options.target_dir.clone();
    }
}

impl Default for Options {
    fn default() -> Self {
        // Synchronize the `Options` defaults with the default command-line arguments.
        let default_cli_args = Cli::default();

        Self::new(&default_cli_args)
    }
}

/// Provides the base name of the configuration file:
/// The `hidden` option controls whether this is a hidden file
/// with a dot at the start, such as `.newdoc.toml`, or
/// a regular, visible file, such as `newdoc.toml`.
fn config_file_name(hidden: bool) -> String {
    let prefix = if hidden {
        "."
    } else {
        ""
    };
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
fn git_conf_file(cli_options: &Options) -> Option<PathBuf> {
    let target_dir = &cli_options.target_dir;

    // Find the earliest ancestor directory that appears to be the root of a Git repo.
    let git_root = target_dir.ancestors().find(|dir| {
        // The simple heuristic is that the directory is the Git root if it contains
        // the `.git/` sub-directory.
        let git_dir = dir.join(".git");
        git_dir.is_dir()
    })?;

    let git_proj_config = git_root.join(config_file_name(true));

    Some(git_proj_config)
}

/// Combine the configuration found on the command line, in configuration files,
/// and in the defaults. Follows the standard hierarchy.
pub fn merge_configs(cli_options: Options) -> Result<Options> {
    let default_options = Options::default();
    let mut figment = Figment::from(Serialized::defaults(default_options));

    if let Some(home_conf_file) = home_conf_file() {
        log::info!("Home configuration file: {}", home_conf_file.display());
        figment = figment.merge(Toml::file(home_conf_file));
    } else {
        // If the directory lookup fails because there's no home directory,
        // skip the processing of the home configuration file.
        log::warn!("Failed to locate a home directory. Skipping home configuration.");
    };

    if let Some(git_conf_file) = git_conf_file(&cli_options) {
        log::info!("Git repo configuration file: {}", git_conf_file.display());
        figment =  figment.merge(Toml::file(git_conf_file));
    }

    log::debug!("Figment configuration: {figment:#?}");

    let mut conf_options: Options = figment.extract().wrap_err("Failed to load configuration files.")?;

    conf_options.update_from_non_default(&cli_options);

    Ok(conf_options)
}
