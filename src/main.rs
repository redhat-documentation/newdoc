use std::env;

extern crate clap;
use clap::{App, Arg, ArgMatches};

fn main() {
    let plain_args: Vec<String> = env::args().collect();
    println!("Plain args: {:#?}", plain_args);

    // Define command-line options
    let clap_app = App::new("newdoc")
        .version("v2.0.0")
        .author("Marek Suchánek")
        .about("Generate an AsciiDoc file using a modular template")
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

    // if plain_args.len() <= 1 {
    //     println!("No arguments, printing help.");
    //     let _clap_result = clap_app.print_help();
    // } else {
    //     let matches = clap_app.get_matches();

    // List the passed command-line options, just for debugging
    //     println!("{:#?}", matches);
    // }

    let matches = clap_app.get_matches();
    println!("{:#?}", matches);

    // Load the AsciiDoc templates and embed them into the binary, accessible as str slices
    let assembly_template = include_str!("../templates/assembly_title.adoc");
    let concept_template = include_str!("../templates/con_title.adoc");
    let procedure_template = include_str!("../templates/proc_title.adoc");
    let reference_template = include_str!("../templates/ref_title.adoc");

    // Just use the templates in a random way to check that they are there
    let templates_length: usize = [
        assembly_template,
        concept_template,
        procedure_template,
        reference_template,
    ]
    .iter()
    .map(|template| template.len())
    .sum();
    println!("Length of all the templates combined: {}", templates_length);

    for module_type in ["assembly", "concept", "procedure", "reference"].iter() {
        process_module_type(&matches, module_type);
    }
}

fn process_module_type(matches: &ArgMatches, module_type: &str) {
    if let Some(titles_iterator) = matches.values_of(module_type) {
        let titles: Vec<&str> = titles_iterator.collect();

        for title in titles {
            process_module(module_type, title)
        }
    }
}

fn process_module(module_type: &str, title: &str) {
    let module_id = convert_title_to_id(title);
    println!("We have a module of type {}, titled {}", module_type, title);
    println!("And the ID is: {}", module_id);
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
