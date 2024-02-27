use std::path::PathBuf;

use directories::ProjectDirs;
use figment::{Figment, providers::{Format, Toml, Serialized}};
use serde::{Serialize, Deserialize};

use crate::cmd_line::{Cli, Verbosity};

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
            comments: cli.common_options.comments,
            file_prefixes: !cli.common_options.no_file_prefixes,
            anchor_prefixes: cli.common_options.anchor_prefixes,
            examples: !cli.common_options.no_examples,
            // Set the target directory as specified or fall back on the current directory
            target_dir: cli.common_options.target_dir.clone(),
            simplified: cli.common_options.simplified,
            verbosity: cli.common_options.verbosity,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            comments: false,
            file_prefixes: true,
            anchor_prefixes: false,
            examples: true,
            target_dir: PathBuf::from("."),
            simplified: false,
            verbosity: Verbosity::Default,
        }
    }
}

pub fn todo(cli_options: &Options) {
    let proj_dirs = ProjectDirs::from("com", "Red Hat", PKG_NAME)
        .expect("Failed to find a home directory.");
    let conf_dir = proj_dirs.config_dir();
    let conf_file = conf_dir.join(format!("{PKG_NAME}.toml"));
    println!("Configuration file:  {}", conf_file.display());

    let target_dir = &cli_options.target_dir;

    let default_options = Options::default();

    let mut figment = Figment::from(Serialized::defaults(default_options))
        .merge(Toml::file(conf_file));

    // Find the earliest ancestor directory that appears to be the root of a Git repo.
    let git_root = target_dir.ancestors().find(|dir| {
        let git_dir = dir.join(".git");
        git_dir.is_dir()
    });

    if let Some(git_root) = git_root {
        let git_proj_config = git_root.join(format!(".{PKG_NAME}.toml"));
        println!("git project config: {}", git_proj_config.display());
        figment =  figment.merge(Toml::file(git_proj_config));
    }

    println!("figment: {:#?}", figment);
}
