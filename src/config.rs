use std::path::PathBuf;

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

pub fn merge_configs(cli_options: Options) -> Options {
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

    let mut conf_options: Options = figment.extract().unwrap();

    conf_options.update_from_non_default(&cli_options);

    println!("complete options: {:#?}", conf_options);

    conf_options
}
