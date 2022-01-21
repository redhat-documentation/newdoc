/// This module defines the command-line arguments and behavior of `newdoc`.
/// It relies on the `clap` crate.
use clap::{app_from_crate, AppSettings, Arg, ArgGroup, ArgMatches};

/// Define the command-line arguments and return them as the `clap::ArgMatches` struct.
pub fn get_args() -> ArgMatches {
    // Define command-line options
    let matches = app_from_crate!()
        // If no arguments are provided, print help
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("assembly")
                .short('a')
                .long("assembly")
                .takes_value(true)
                .value_name("title")
                .multiple_occurrences(true)
                .help("Create an assembly file"),
        )
        .arg(
            Arg::new("include-in")
                .short('i')
                .long("include-in")
                .takes_value(true)
                .value_name("title")
                .multiple_occurrences(false)
                .help("Create an assembly that includes the other specified modules"),
        )
        .arg(
            Arg::new("concept")
                .short('c')
                .long("concept")
                .takes_value(true)
                .value_name("title")
                .multiple_occurrences(true)
                .help("Create a concept module"),
        )
        .arg(
            Arg::new("procedure")
                .short('p')
                .long("procedure")
                .takes_value(true)
                .value_name("title")
                .multiple_occurrences(true)
                .help("Create a procedure module"),
        )
        .arg(
            Arg::new("reference")
                .short('r')
                .long("reference")
                .takes_value(true)
                .value_name("title")
                .multiple_occurrences(true)
                .help("Create a reference module"),
        )
        .arg(
            Arg::new("validate")
                .short('l')
                .long("validate")
                .takes_value(true)
                .value_name("file")
                .multiple_occurrences(true)
                .help("Validate (lint) an existing module or assembly file")
        )
        // This group specifies that you either generate modules or validate existing ones
        .group(
            ArgGroup::new("required")
                .args(&["assembly", "concept", "procedure", "reference", "validate"])
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::new("no-comments")
                .short('C')
                .long("no-comments")
                .help("Generate the file without any comments"),
        )
        .arg(
            Arg::new("no-examples")
                .short('E')
                .long("no-examples")
                .alias("expert-mode")
                .help("Generate the file without any example, placeholder content"),
        )
        .arg(
            Arg::new("detect-directory")
                .short('D')
                .long("detect-directory")
                .help("Detect the include path, rather than using the <path> placeholder. This is now the default behavior")
                // I'm enabling this option by default, without a way to disable it.
                // Let the users test the new behavior and evaluate.
                .hide(true),
        )
        .arg(
            Arg::new("no-prefixes")
                .short('P')
                .long("no-prefixes")
                .help("Do not use module type prefixes (such as `proc_`) in IDs and file names"),
        )
        .arg(
            Arg::new("target-dir")
                .short('T')
                .long("target-dir")
                .takes_value(true)
                .value_name("directory")
                .help("Save the generated files in this directory"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Display additional, debug messages")
                .conflicts_with("quiet"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Hide info-level messages. Display only warnings and errors")
                .conflicts_with("verbose"),
        )
        .get_matches();

    matches
}
