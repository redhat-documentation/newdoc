use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

extern crate clap;
use clap::{App, AppSettings, Arg, Values};

// Load the AsciiDoc templates at build time
const ASSEMBLY_TEMPLATE: &str = include_str!("../templates/assembly.adoc");
const CONCEPT_TEMPLATE: &str = include_str!("../templates/concept.adoc");
const PROCEDURE_TEMPLATE: &str = include_str!("../templates/procedure.adoc");
const REFERENCE_TEMPLATE: &str = include_str!("../templates/reference.adoc");

/// All possible types of the AsciiDoc module
#[derive(Debug)]
enum ModuleType {
    Assembly,
    Concept,
    Procedure,
    Reference,
}

/// A representation of the module with all its metadata and the generated AsciiDoc content
#[derive(Debug)]
struct Module {
    mod_type: ModuleType,
    title: String,
    id: String,
    file_name: String,
    text: String,
}

impl Module {
    /// The constructor for the Module struct
    pub fn new(mod_type: ModuleType, title: &str, options: &Options) -> Module {
        let title = String::from(title);
        let id = Module::convert_title_to_id(&title);
        let file_name = Module::compose_file_name(&id, &mod_type, &options);
        let text = Module::compose_text(&title, &id, &mod_type, &options);

        Module {
            mod_type,
            title,
            id,
            file_name,
            text,
        }
    }
}

/// This struct stores options based on the command-line arguments,
/// and is passed to various functions across the program.
struct Options {
    comments: bool,
    prefixes: bool,
    target_dir: String,
}

fn main() {
    // Define command-line options
    let cmdline_args = App::new("newdoc")
        .version("v2.1.0")
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
                .short("P")
                .long("no-prefixes")
                .help("Do not use module type prefixes (e.g. `proc_`) in file names"),
        )
        .arg(
            Arg::with_name("target-dir")
                .short("-T")
                .long("target-dir")
                .takes_value(true)
                .value_name("directory")
                .help("Save the generated files in this directory"),
        )
        .get_matches();

    // Set current options based on the command-line options
    let options = Options {
        // Comments and prefixes are enabled (true) by default unless you disable them
        // on the command line. If the no-comments or no-prefixes option is passed
        // (occurences > 0), the feature is disabled, so the option is set to false.
        comments: cmdline_args.occurrences_of("no-comments") == 0,
        prefixes: cmdline_args.occurrences_of("no-prefixes") == 0,
        // Set the target directory as specified or fall back on the current directory
        target_dir: if let Some(dir) = cmdline_args.value_of("target-dir") {
            String::from(dir)
        } else {
            String::from(".")
        },
    };

    // TODO: Maybe attach these strings to the ModuleType enum somehow
    // For each module type, see if it occurs on the command line and process it
    for module_type_str in ["assembly", "concept", "procedure", "reference"].iter() {
        // Check if the given module type occurs on the command line
        if let Some(titles_iterator) = cmdline_args.values_of(module_type_str) {
            let modules = process_module_type(titles_iterator, module_type_str, &options);

            for module in modules.iter() {
                write_module(&module.file_name, &module.text, &options);
            }
        }
    }
}

/// Process all titles that have been specified on the command line and that belong to a single
/// module type.
fn process_module_type(titles: Values, module_type_str: &str, options: &Options) -> Vec<Module> {
    let mut modules_from_type = Vec::new();

    for title in titles {
        // Convert the string module type to an enum.
        // This must be done for each title separately so that the title can own the ModuleType.
        let module_type = match module_type_str {
            "assembly" => ModuleType::Assembly,
            "concept" => ModuleType::Concept,
            "procedure" => ModuleType::Procedure,
            "reference" => ModuleType::Reference,
            _ => unimplemented!(),
        };

        let module = Module::new(module_type, title, &options);

        modules_from_type.push(module);
    }

    modules_from_type
}

impl Module {
    /// Create an ID string that is derived from the human-readable title. The ID is usable as:
    ///
    /// * An AsciiDoc section ID
    /// * A DocBook section ID
    /// * A file name
    ///
    /// ## Examples
    ///
    /// ```
    /// let title = "Testing newdoc";
    /// assert_eq!(String::from("testing-newdoc"), Module::convert_title_to_id(title));
    /// ```
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

    /// Perform string replacements in the modular template that matches the `ModuleType`.
    /// Return the template text with all replacements.
    fn compose_text(
        title: &str,
        module_id: &str,
        module_type: &ModuleType,
        options: &Options,
    ) -> String {
        // TODO: Add a comment in the generated file with a pre-filled include statement

        // Pick the right template
        let current_template = match module_type {
            ModuleType::Assembly => ASSEMBLY_TEMPLATE,
            ModuleType::Concept => CONCEPT_TEMPLATE,
            ModuleType::Procedure => PROCEDURE_TEMPLATE,
            ModuleType::Reference => REFERENCE_TEMPLATE,
        };

        // Define the strings that will be replaced in the template
        let replacements = [
            ("${module_title}", title),
            ("${module_id}", module_id),
            ("${include_statements}", "Include modules here."),
        ];

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

    /// Prepare the file name for the generated file.
    ///
    /// The file name is based on the module ID, with the `.adoc` extension and an optional prefix.
    fn compose_file_name(module_id: &str, module_type: &ModuleType, options: &Options) -> String {
        let prefix = if options.prefixes {
            // If prefixes are enabled, pick the right file prefix
            match module_type {
                ModuleType::Assembly => "assembly_",
                ModuleType::Concept => "con_",
                ModuleType::Procedure => "proc_",
                ModuleType::Reference => "ref_",
            }
        } else {
            // If prefixes are disabled, use an empty string for the prefix
            ""
        };

        let suffix = ".adoc";

        [prefix, module_id, suffix].join("")
    }
}

/// Write the generated module content to the path specified in `options` with the set file name.
fn write_module(file_name: &str, content: &str, options: &Options) {
    // Compose the full (but still relative) file path from the target directory and the file name
    let full_path_buf: PathBuf = [&options.target_dir, file_name].iter().collect();
    let full_path = full_path_buf.as_path();

    // If the target file already exists, just print out an error
    if full_path.exists() {
        // A prompt enabling the user to overwrite the existing file
        eprintln!("File already exists: {}", full_path.display());
        eprint!("Do you want to overwrite it? [y/N] ");
        // We must manually flush the buffer or else the printed string doesn't appear.
        // The buffer otherwise waits for a newline.
        io::stdout().flush().unwrap();

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read the response");

        match answer.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                eprintln!("Rewriting the file.");
            }
            _ => {
                eprintln!("Preserving the existing file.");
                // Break from generating this particular module.
                // Other modules that might be in the queue will be generated on next iteration.
                return;
            }
        };
    }

    // If the target file doesn't exist, try to write to it
    let result = fs::write(full_path, content);
    match result {
        // If the write succeeds, print the include statement
        Ok(()) => {
            eprintln!("File generated: {}", full_path.display());
            eprintln!("include::<path>/{}[leveloffset=+1]", file_name);
        }
        // If the write fails, print why it failed
        Err(e) => {
            eprintln!("Failed to write the file: {}", e);
        }
    }
}
