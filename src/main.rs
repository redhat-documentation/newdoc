extern crate clap;
use clap::{Arg, App};

fn main() {
    // Define command-line options
    let matches = App::new("newdoc")
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
             .help("Generate the file without any comments"))
        .get_matches();

    // List the passed command-line options, just for debugging
    println!("{:#?}", matches);
}
