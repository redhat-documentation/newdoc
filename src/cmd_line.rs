/// This module defines the command-line arguments and behavior of `newdoc`.
/// It relies on the `clap` crate.
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgGroup,
    ArgMatches,
};

/// Define the command-line arguments and return them as the `clap::ArgMatches` struct.
pub fn get_args() -> ArgMatches<'static> {
    // Define command-line options
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        // If no arguments are provided, print help
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("assembly")
                .short("a")
                .long("assembly")
                .takes_value(true)
                .value_name("title")
                .multiple(true)
                .help("Create an assembly file"),
        )
        .arg(
            Arg::with_name("include-in")
                .short("i")
                .long("include-in")
                .takes_value(true)
                .value_name("title")
                .multiple(false)
                .help("Create an assembly that includes the other specified modules"),
        )
        .arg(
            Arg::with_name("concept")
                .short("c")
                .long("concept")
                .takes_value(true)
                .value_name("title")
                .multiple(true)
                .help("Create a concept module"),
        )
        .arg(
            Arg::with_name("procedure")
                .short("p")
                .long("procedure")
                .takes_value(true)
                .value_name("title")
                .multiple(true)
                .help("Create a procedure module"),
        )
        .arg(
            Arg::with_name("reference")
                .short("r")
                .long("reference")
                .takes_value(true)
                .value_name("title")
                .multiple(true)
                .help("Create a reference module"),
        )
        // This group ensures that at least one of the assembly or module inputs is present
        .group(
            ArgGroup::with_name("modules")
                .args(&["assembly", "concept", "procedure", "reference"])
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("no-comments")
                .short("C")
                .long("no-comments")
                .help("Generate the file without any comments"),
        )
        .arg(
            Arg::with_name("no-examples")
                .short("E")
                .long("no-examples")
                .alias("expert-mode")
                .help("Generate the file without any example, placeholder content"),
        )
        .arg(
            Arg::with_name("detect-directory")
                .short("D")
                .long("detect-directory")
                .help("Detect the include path, rather than using the <path> placeholder"),
        )
        .arg(
            Arg::with_name("no-prefixes")
                .short("P")
                .long("no-prefixes")
                .help("Do not use module type prefixes (such as `proc_`) in IDs and file names"),
        )
        .arg(
            Arg::with_name("target-dir")
                .short("-T")
                .long("target-dir")
                .takes_value(true)
                .value_name("directory")
                .help("Save the generated files in this directory"),
        )
        .get_matches()
}
