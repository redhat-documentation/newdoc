use std::path::Path;
use std::fs;

extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches};

// Load the AsciiDoc templates at build time
const ASSEMBLY_TEMPLATE: &str = include_str!("../templates/assembly_title.adoc");
const CONCEPT_TEMPLATE: &str = include_str!("../templates/con_title.adoc");
const PROCEDURE_TEMPLATE: &str = include_str!("../templates/proc_title.adoc");
const REFERENCE_TEMPLATE: &str = include_str!("../templates/ref_title.adoc");

struct Options {
    comments: bool,
    prefixes: bool,
}

fn main() {

    // Define command-line options
    let clap_app = App::new("newdoc")
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
        );

    let matches = clap_app.get_matches();

    // Set current options based on the command-line options
    let options = Options {
        // Comments and prefixes are enabled (true) by default unless you disable them
        // on the command line
        comments: if matches.occurrences_of("no-comments") > 0 { false } else { true },
        prefixes: if matches.occurrences_of("no-prefixes") > 0 { false } else { true },
    };

    for module_type in ["assembly", "concept", "procedure", "reference"].iter() {
        process_module_type(&matches, module_type, &options);
    }
}

fn process_module_type(matches: &ArgMatches, module_type: &str, options: &Options) {
    if let Some(titles_iterator) = matches.values_of(module_type) {
        for title in titles_iterator {
            process_module(module_type, title, &options);
        }
    }
}

fn process_module(module_type: &str, title: &str, options: &Options) {
    let module_id = convert_title_to_id(title);
    println!("We have a module of type {}, titled {}", module_type, title);
    println!("And the ID is: {}", module_id);

    let module_text = compose_module_text(title, &module_id, module_type, &options);
    println!("The applied template:\n{}", module_text);

    let file_name = compose_file_name(&module_id, module_type, &options);
    println!("And the file name is {}", file_name);

    write_module(&file_name, &module_text);
}

fn convert_title_to_id(title: &str) -> String {
    let mut title = String::from(title).to_lowercase();

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

    for (old, new) in substitutions.iter() {
        title = title.replace(old, new);
    }

    // Make sure the converted ID doesn't contain double dashes ("--"), because
    // that breaks references to the ID
    while title.contains("--") {
        title = title.replace("--", "-");
    }

    title
}

fn compose_module_text(title: &str, module_id: &str, module_type: &str, options: &Options) -> String {
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
    if !options.prefixes {
        println!("No prefix");
    }
    // Pick the right file prefix
    let prefix = match module_type {
        "assembly" => "assembly_",
        "concept" => "con_",
        "procedure" => "proc_",
        "reference" => "ref_",
        _ => unimplemented!(),
    };

    let suffix = ".adoc";

    let file_name = [prefix, module_id, suffix].join("");

    file_name
}

fn write_module(file_name: &str, content: &String) {
    // If the target file already exists, just print out an error
    if Path::new(file_name).exists() {
        println!("File already exists: {}", file_name);
    } else {
        // If the target file doesn't exist, try to write to it
        let result = fs::write(file_name, content);
        match result {
            // If the write succeeds, print the include statement
            Ok(()) => {
                println!("include::<path>/{}[leveloffset=+1]", file_name);
            },
            // If the write fails, print why it failed
            Err(e) => {
                println!("Failed to write the file: {}", e);
            }
        }
    }
}
