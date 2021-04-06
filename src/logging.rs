use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TerminalMode, TermLogger};

/// This function initializes the `simplelog` logging system, which plugs into the `log`
/// infrastructure. The function returns nothing. It only affects the global state when it runs.
pub fn initialize_logger() {
    let config = ConfigBuilder::new()
        // Display a time stamp only for the debug and more verbose levels.
        .set_time_level(LevelFilter::Debug)
        .build();

    TermLogger::init(
        // The default verbosity level is Info because newdoc displays the include statement
        // at that level.
        LevelFilter::Info,
        config,
        // Mixed mode prints errors to stderr and info to stdout. Not sure about the other levels.
        TerminalMode::Mixed,
        // Try to use color if possible.
        ColorChoice::Auto
    ).expect("Failed to configure the terminal logging.");
}
