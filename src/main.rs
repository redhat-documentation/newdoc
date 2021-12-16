use log::{debug, info};

mod cmd_line;
mod logging;
mod module;
mod templating;
mod validation;
mod write;

use module::{Input, Module, ModuleType};

/// This struct stores options based on the command-line arguments,
/// and is passed to various functions across the program.
#[derive(Debug, Clone)]
pub struct Options {
    comments: bool,
    prefixes: bool,
    examples: bool,
    target_dir: String,
    detect_directory: bool,
}

fn main() {
    // Parse the command-line options
    let cmdline_args = cmd_line::get_args();
    // Determine the configured verbosity level
    let verbose = cmdline_args.is_present("verbose");
    let quiet = cmdline_args.is_present("quiet");
    // Initialize the logging system based on the set verbosity
    logging::initialize_logger(verbose, quiet);

    if cmdline_args.is_present("detect-directory") {
        info!("The `--detect-directory` (`-D`) option is now enabled by default.");
    }

    // Set current options based on the command-line options
    let options = Options {
        // Comments and prefixes are enabled (true) by default unless you disable them
        // on the command line. If the no-comments or no-prefixes option is passed
        // (occurences > 0), the feature is disabled, so the option is set to false.
        comments: !cmdline_args.is_present("no-comments"),
        prefixes: !cmdline_args.is_present("no-prefixes"),
        examples: !cmdline_args.is_present("no-examples"),
        // Set the target directory as specified or fall back on the current directory
        target_dir: if let Some(dir) = cmdline_args.value_of("target-dir") {
            String::from(dir)
        } else {
            String::from(".")
        },
        // I'm turning this into the default behavior.
        // For now, I'm going to keep the switch as is, just always true,
        // So that I can easily revert the changes later if needed.
        // After this change has had proper user testing, let's remove the switch
        // and the dead code.
        detect_directory: true,
    };

    debug!("Active options:\n{:#?}", &options);

    // Store all modules except for the populated assembly that will be created in this Vec
    let mut non_populated: Vec<Module> = Vec::new();

    // TODO: Maybe attach these strings to the ModuleType enum somehow
    // For each module type, see if it occurs on the command line and process it
    for module_type_str in &["assembly", "concept", "procedure", "reference"] {
        // Check if the given module type occurs on the command line
        if let Some(titles_iterator) = cmdline_args.values_of(module_type_str) {
            let mut modules = process_module_type(titles_iterator, module_type_str, &options);

            // Move all the newly created modules into the common Vec
            non_populated.append(&mut modules);
        }
    }

    // Write all non-populated modules to the disk
    for module in &non_populated {
        module.write_file(&options);
    }

    // Treat the populated assembly module as a special case:
    // * There can be only one populated assembly
    // * It must be generated after the other modules so that it can use their include statements
    if let Some(title) = cmdline_args.value_of("include-in") {
        // Gather all include statements for the other modules
        // TODO: Figure out if this can be done without calling .to_owned on all the Strings
        let include_statements: Vec<String> = non_populated
            .iter()
            .map(|module| module.include_statement.clone())
            .collect();

        // The include_statements should never be empty thanks to the required group in clap
        assert!(!include_statements.is_empty());

        // Generate the populated assembly module
        let populated: Module = Input::new(ModuleType::Assembly, title, &options)
            .include(include_statements)
            .into();

        populated.write_file(&options);
    }

    // Validate all file names specified on the command line
    if let Some(files_iterator) = cmdline_args.values_of("validate") {
        for file in files_iterator {
            validation::validate(file);
        }
    }
}

/// Process all titles that have been specified on the command line and that belong to a single
/// module type.
fn process_module_type(
    titles: clap::Values,
    module_type_str: &str,
    options: &Options,
) -> Vec<Module> {
    let module_type = match module_type_str {
        "assembly" | "include-in" => ModuleType::Assembly,
        "concept" => ModuleType::Concept,
        "procedure" => ModuleType::Procedure,
        "reference" => ModuleType::Reference,
        _ => unimplemented!(),
    };

    let modules_from_type = titles.map(|title| Module::new(module_type, title, options));

    modules_from_type.collect()
}

// These tests act as pseudo-integration tests. They let the top-level functions generate
// each module type and then they compare the generated content with a pre-generated specimen
// to check that we introduce no changes unknowingly.
#[cfg(test)]
mod tests {
    use crate::module::Module;
    use crate::module::ModuleType;
    use crate::Options;

    // These values represent the default newdoc options.
    fn basic_options() -> Options {
        Options {
            comments: true,
            prefixes: true,
            examples: true,
            target_dir: ".".to_string(),
            detect_directory: true,
        }
    }

    /// Test that we generate the assembly that we expect.
    #[test]
    fn test_assembly() {
        let mod_type = ModuleType::Assembly;
        let mod_title = "Testing that an assembly forms properly";
        let options = basic_options();
        let assembly = Module::new(mod_type, mod_title, &options);

        let pre_generated =
            include_str!("../data/generated/assembly_testing-that-an-assembly-forms-properly.adoc");

        assert_eq!(assembly.text, pre_generated);
    }

    /// Test that we generate the concept module that we expect.
    #[test]
    fn test_concept_module() {
        let mod_type = ModuleType::Concept;
        let mod_title = "A title that tests a concept";
        let options = basic_options();
        let concept = Module::new(mod_type, mod_title, &options);

        let pre_generated = include_str!("../data/generated/con_a-title-that-tests-a-concept.adoc");

        assert_eq!(concept.text, pre_generated);
    }

    /// Test that we generate the procedure module that we expect.
    #[test]
    fn test_procedure_module() {
        let mod_type = ModuleType::Procedure;
        let mod_title = "Testing a procedure";
        let options = basic_options();
        let procedure = Module::new(mod_type, mod_title, &options);

        let pre_generated = include_str!("../data/generated/proc_testing-a-procedure.adoc");

        assert_eq!(procedure.text, pre_generated);
    }

    /// Test that we generate the reference module that we expect.
    #[test]
    fn test_reference_module() {
        let mod_type = ModuleType::Reference;
        let mod_title = "The lines in a reference module";
        let options = basic_options();
        let reference = Module::new(mod_type, mod_title, &options);

        let pre_generated =
            include_str!("../data/generated/ref_the-lines-in-a-reference-module.adoc");

        assert_eq!(reference.text, pre_generated);
    }

    // These values strip down the modules to the bare minimum.
    fn minimal_options() -> Options {
        Options {
            comments: false,
            prefixes: false,
            examples: false,
            target_dir: ".".to_string(),
            detect_directory: true,
        }
    }

    /// Test that we generate the assembly that we expect.
    #[test]
    fn test_minimal_assembly() {
        let mod_type = ModuleType::Assembly;
        let mod_title = "Minimal assembly";
        let options = minimal_options();
        let assembly = Module::new(mod_type, mod_title, &options);

        let pre_generated = include_str!("../data/generated/minimal-assembly.adoc");

        assert_eq!(assembly.text, pre_generated);
    }

    /// Test that we generate the concept module that we expect.
    #[test]
    fn test_minimal_concept() {
        let mod_type = ModuleType::Concept;
        let mod_title = "Minimal concept";
        let options = minimal_options();
        let concept = Module::new(mod_type, mod_title, &options);

        let pre_generated = include_str!("../data/generated/minimal-concept.adoc");

        assert_eq!(concept.text, pre_generated);
    }

    /// Test that we generate the procedure module that we expect.
    #[test]
    fn test_minimal_procedure() {
        let mod_type = ModuleType::Procedure;
        let mod_title = "Minimal procedure";
        let options = minimal_options();
        let procedure = Module::new(mod_type, mod_title, &options);

        let pre_generated = include_str!("../data/generated/minimal-procedure.adoc");

        assert_eq!(procedure.text, pre_generated);
    }

    /// Test that we generate the reference module that we expect.
    #[test]
    fn test_minimal_reference() {
        let mod_type = ModuleType::Reference;
        let mod_title = "Minimal reference";
        let options = minimal_options();
        let reference = Module::new(mod_type, mod_title, &options);

        let pre_generated = include_str!("../data/generated/minimal-reference.adoc");

        assert_eq!(reference.text, pre_generated);
    }
}
