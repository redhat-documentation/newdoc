use std::fs;
use std::path::Path;

extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches};

// Load the AsciiDoc templates at build time
const ASSEMBLY_TEMPLATE: &str = include_str!("../templates/assembly_title.adoc");
const CONCEPT_TEMPLATE: &str = include_str!("../templates/con_title.adoc");
const PROCEDURE_TEMPLATE: &str = include_str!("../templates/proc_title.adoc");
const REFERENCE_TEMPLATE: &str = include_str!("../templates/ref_title.adoc");

// This struct will store options based on the command-line arguments,
// and will be passed to various functions across the program.
struct Options {
    comments: bool,
    prefixes: bool,
}

fn main() {
    // Define command-line options
    let cmdline_args = App::new("newdoc")
        .version("v2.0.0")
        .author("Marek Suchánek")
        .about("Generate an AsciiDoc file using a modular template")
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
        .arg(
            Arg::with_name("no-comments")
                .short("C")
                .long("no-comments")
                .help("Generate the file without any comments"),
        )
        .arg(
            Arg::with_name("no-prefixes")
                .short("-P")
                .long("no-prefixes")
                .help("Do not use module type prefixes (e.g. `proc_`) in file names"),
        ).get_matches();

    // Set current options based on the command-line options
    let options = Options {
        // Comments and prefixes are enabled (true) by default unless you disable them
        // on the command line. If the no-comments or no-prefixes option is passed
        // (occurences > 0), the feature is disabled, so the option is set to false.
        comments: cmdline_args.occurrences_of("no-comments") == 0,
        prefixes: cmdline_args.occurrences_of("no-prefixes") == 0,
    };

    // TODO: Streamline how we store metadata about module types. A struct, perhaps?
    // For each module type, see if it occurs on the command line and process it
    for module_type in ["assembly", "concept", "procedure", "reference"].iter() {
        process_module_type(&cmdline_args, module_type, &options);
    }
}

fn process_module_type(cmdline_args: &ArgMatches, module_type: &str, options: &Options) {
    // Check if the given module type occurs on the command line
    if let Some(titles_iterator) = cmdline_args.values_of(module_type) {
        // Porcess all module titles in the module type
        for title in titles_iterator {
            process_module(module_type, title, &options);
        }
    }
}

fn process_module(module_type: &str, title: &str, options: &Options) {
    // TODO: Add a comment in the generated file with a pre-filled include statement

    // Derive the module properties from its title
    let module_id = convert_title_to_id(title);
    let module_text = compose_module_text(title, &module_id, module_type, &options);
    let file_name = compose_file_name(&module_id, module_type, &options);

    // Write the module text into the file with the appropriate file name
    write_module(&file_name, &module_text);
}

fn convert_title_to_id(title: &str) -> String {
    // The ID is all lower-case
    let mut title_with_replacements = String::from(title).to_lowercase();

    // Replace characters that aren't allowed in the ID, usually with a dash or an empty string
    let substitutions = [
        (" ", "-"),
        ("(", ""),
        (")", ""),
        ("?", ""),
        ("!", ""),
        ("'", ""),
        ("\"", ""),
        ("#", ""),
        ("%", ""),
        ("&", ""),
        ("*", ""),
        (",", ""),
        (".", "-"),
        ("/", "-"),
        (":", "-"),
        (";", ""),
        ("@", "-at-"),
        ("\\", ""),
        ("`", ""),
        ("$", ""),
        ("^", ""),
        ("|", ""),
        // Remove known semantic markup from the ID:
        ("[package]", ""),
        ("[option]", ""),
        ("[parameter]", ""),
        ("[variable]", ""),
        ("[command]", ""),
        ("[replaceable]", ""),
        ("[filename]", ""),
        ("[literal]", ""),
        ("[systemitem]", ""),
        ("[application]", ""),
        ("[function]", ""),
        ("[gui]", ""),
        // Remove square brackets only after semantic markup:
        ("[", ""),
        ("]", ""),
        // TODO: Curly braces shouldn't appear in the title in the first place.
        // They'd be interpreted as attributes there.
        // Print an error in that case? Escape them with AciiDoc escapes?
        ("{", ""),
        ("}", ""),
    ];

    // Perform all the defined replacements on the title
    for (old, new) in substitutions.iter() {
        title_with_replacements = title_with_replacements.replace(old, new);
    }

    // Make sure the converted ID doesn't contain double dashes ("--"), because
    // that breaks references to the ID
    while title_with_replacements.contains("--") {
        title_with_replacements = title_with_replacements.replace("--", "-");
    }

    title_with_replacements
}

fn compose_module_text(
    title: &str,
    module_id: &str,
    module_type: &str,
    options: &Options,
) -> String {
    // Pick the right template
    let current_template = match module_type {
        "assembly" => ASSEMBLY_TEMPLATE,
        "concept" => CONCEPT_TEMPLATE,
        "procedure" => PROCEDURE_TEMPLATE,
        "reference" => REFERENCE_TEMPLATE,
        _ => unimplemented!(),
    };

    // Define the strings that will be replaced in the template
    let replacements = [("${module_title}", title), ("${module_id}", module_id)];

    // Perform substitutions in the template
    // TODO: Create a separate function to perform a replacement
    let mut template_with_replacements = String::from(current_template);

    for (old, new) in replacements.iter() {
        template_with_replacements = template_with_replacements.replace(old, new);
    }

    // If comments are disabled via an option, delete comment lines from the content
    // TODO: This doesn't handle AsciiDoc comment blocks at all
    if !options.comments {
        // Filter out comment lines in an iterator
        let lines = template_with_replacements
            .lines()
            .filter(|line| !line.starts_with("//"));
        // Connect the iterator back into a String, connecting with newlines
        template_with_replacements = lines.collect::<Vec<&str>>().join("\n");
        // Add a final newline at the end of the file
        template_with_replacements.push('\n');
    }

    template_with_replacements
}

fn compose_file_name(module_id: &str, module_type: &str, options: &Options) -> String {
    let prefix = if options.prefixes {
        // If prefixes are enabled, pick the right file prefix
        match module_type {
            "assembly" => "assembly_",
            "concept" => "con_",
            "procedure" => "proc_",
            "reference" => "ref_",
            _ => unimplemented!(),
        }
    } else {
        // If prefixes are disabled, use an empty string for the prefix
        ""
    };

    let suffix = ".adoc";

    [prefix, module_id, suffix].join("")
}

fn write_module(file_name: &str, content: &str) {
    // If the target file already exists, just print out an error
    if Path::new(file_name).exists() {
        // TODO: Add a prompt enabling the user to overwrite the existing file
        println!("File already exists: {}", file_name);
    } else {
        // If the target file doesn't exist, try to write to it
        let result = fs::write(file_name, content);
        match result {
            // If the write succeeds, print the include statement
            Ok(()) => {
                println!("File generated: {}", file_name);
                println!("include::<path>/{}[leveloffset=+1]", file_name);
            }
            // If the write fails, print why it failed
            Err(e) => {
                println!("Failed to write the file: {}", e);
            }
        }
    }
}
