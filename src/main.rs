use std::env;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let plain_args: Vec<String> = env::args().collect();
    println!("Plain args: {:#?}", plain_args);

    // Define command-line options
    let mut clap_app = App::new("newdoc")
        .version("v2.0.0")
        .author("Marek Suchánek")
        .about("Generate an AsciiDoc file using a modular template")
        .arg(Arg::with_name("assembly")
             .short("a")
             .long("assembly")
             .takes_value(true)
             .value_name("title")
             .multiple(true)
             .help("Create an assembly file"))
        .arg(Arg::with_name("concept")
             .short("c")
             .long("concept")
             .takes_value(true)
             .value_name("title")
             .multiple(true)
             .help("Create a concept module"))
        .arg(Arg::with_name("procedure")
             .short("p")
             .long("procedure")
             .takes_value(true)
             .value_name("title")
             .multiple(true)
             .help("Create a procedure module"))
        .arg(Arg::with_name("reference")
             .short("r")
             .long("reference")
             .takes_value(true)
             .value_name("title")
             .multiple(true)
             .help("Create a reference module"))
        .arg(Arg::with_name("no-comments")
             .short("C")
             .long("no-comments")
             .help("Generate the file without any comments"));


    if plain_args.len() <= 1 {
        println!("No arguments, printing help.");
        let _clap_result = clap_app.print_help();
    } else {
        let matches = clap_app.get_matches();

        // List the passed command-line options, just for debugging
        println!("{:#?}", matches);
    }

    let test_title = "This is a testing title.";
    println!("Test title: {}", test_title);
    let converted_to_id = convert_title_to_id(test_title);
    println!("Converted ID: {}", converted_to_id);
}

fn convert_title_to_id(title: &str) -> String {
    String::from(title)
}

