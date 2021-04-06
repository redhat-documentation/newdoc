use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TerminalMode, TermLogger};

/// This function initializes the `simplelog` logging system, which plugs into the `log`
/// infrastructure. The function returns nothing. It only affects the global state when it runs.
pub fn initialize_logger(verbose: bool, quiet: bool) {
    // Set the verbosity level based on the command-line options.
    // Our `clap` configuration ensures that `verbose` and `quiet` can never be both true.
    let verbosity = if verbose {
        LevelFilter::Debug
    } else if quiet {
        LevelFilter::Warn
    } else {
        // The default verbosity level is Info because newdoc displays the include statement
        // at that level.
        LevelFilter::Info
    };

    let config = ConfigBuilder::new()
        // Display a time stamp only for the most verbose level.
        .set_time_level(LevelFilter::Trace)
        .build();

    TermLogger::init(
        verbosity,
        config,
        // Mixed mode prints errors to stderr and info to stdout. Not sure about the other levels.
        TerminalMode::Mixed,
        // Try to use color if possible.
        ColorChoice::Auto
    ).expect("Failed to configure the terminal logging.");
}
