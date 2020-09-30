/// This module defines the `Module` struct, its builder struct, and methods on both structs.
use std::path::{Path, PathBuf};

use crate::Options;

/// All possible types of the AsciiDoc module
#[derive(Debug, Clone)]
pub enum ModuleType {
    Assembly,
    Concept,
    Procedure,
    Reference,
}

/// An initial representation of the module with input data, used to construct the `Module` struct
#[derive(Debug)]
pub struct Input {
    mod_type: ModuleType,
    title: String,
    options: Options,
    includes: Option<Vec<String>>,
}

/// A representation of the module with all its metadata and the generated AsciiDoc content
#[derive(Debug)]
pub struct Module {
    mod_type: ModuleType,
    title: String,
    id: String,
    pub file_name: String,
    pub include_statement: String,
    includes: Option<Vec<String>>,
    pub text: String,
}

// Load the AsciiDoc templates at build time
const ASSEMBLY_TEMPLATE: &str = include_str!("../templates/assembly.adoc");
const CONCEPT_TEMPLATE: &str = include_str!("../templates/concept.adoc");
const PROCEDURE_TEMPLATE: &str = include_str!("../templates/procedure.adoc");
const REFERENCE_TEMPLATE: &str = include_str!("../templates/reference.adoc");

/// Construct a basic builder for `Module`, storing information from the user input.
impl Input {
    pub fn new(mod_type: ModuleType, title: &str, options: &Options) -> Input {
        let title = String::from(title);
        let options = options.clone();

        Input {
            mod_type,
            title,
            options,
            includes: None,
        }
    }

    /// Set the optional include statements for files that this assembly includes
    pub fn include(mut self, include_statements: Vec<String>) -> Self {
        self.includes = Some(include_statements);
        self
    }

    /// Create an ID string that is derived from the human-readable title. The ID is usable as:
    ///
    /// * An AsciiDoc section ID
    /// * A DocBook section ID
    /// * A file name
    pub fn id(&self) -> String {
        let title = &self.title;
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

    /// Prepare the file name for the generated file.
    ///
    /// The file name is based on the module ID, with the `.adoc` extension and an optional prefix.
    pub fn file_name(&self) -> String {
        let prefix = if self.options.prefixes {
            // If prefixes are enabled, pick the right file prefix
            match self.mod_type {
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

        [prefix, &self.id(), suffix].join("")
    }

    /// Prepare an include statement that can be used to include the generated file from elsewhere.
    fn include_statement(&self) -> String {
        let path_placeholder = Path::new("<path>").to_path_buf();

        let include_path = if self.options.detect_directory {
            match self.infer_include_dir() {
                Some(path) => path,
                None => path_placeholder,
            }
        } else {
            path_placeholder
        };

        format!(
            "include::{}/{}[leveloffset=+1]",
            include_path.display(),
            &self.file_name()
        )
    }

    /// Determine the start of the include statement from the target path.
    /// Returns the relative path that can be used in the include statement, if it's possible
    /// to determine it automatically.
    fn infer_include_dir(&self) -> Option<PathBuf> {
        // The first directory in the include path is either `assemblies/` or `modules/`,
        // based on the module type.
        let include_root = match &self.mod_type {
            ModuleType::Assembly => "assemblies",
            _ => "modules",
        };

        // TODO: Maybe convert the path earlier in the module building.
        let target_path = Path::new(&self.options.target_dir).canonicalize().unwrap();

        // Split the target path into components
        let mut component_vec: Vec<_> = target_path
            .as_path()
            .components()
            .map(|c| c.as_os_str())
            .collect();

        // Find the position of the component that matches the root element,
        // searching from the end of the path forward.
        let root_position = component_vec.iter().rposition(|&c| c == include_root);

        // If there is such a root element in the path, construct the include path.
        // TODO: To be safe, check that the root path element still exists in a Git repository.
        if let Some(position) = root_position {
            let include_path = component_vec
                .split_off(position)
                .iter()
                .collect::<PathBuf>();
            Some(include_path)
        // If no appropriate root element was found, use a generic placeholder.
        } else {
            None
        }
    }

    /// Perform string replacements in the modular template that matches the `ModuleType`.
    /// Return the template text with all replacements.
    pub fn text(&self) -> String {
        // TODO: Add a comment in the generated file with a pre-filled include statement

        // Pick the right template
        let current_template = match self.mod_type {
            ModuleType::Assembly => ASSEMBLY_TEMPLATE,
            ModuleType::Concept => CONCEPT_TEMPLATE,
            ModuleType::Procedure => PROCEDURE_TEMPLATE,
            ModuleType::Reference => REFERENCE_TEMPLATE,
        };

        // Define the strings that will be replaced in the template
        let replacements = [
            ("${module_title}", &self.title),
            ("${module_id}", &self.id()),
        ];

        // Perform substitutions in the template
        // TODO: Create a separate function to perform a replacement
        let mut template_with_replacements = String::from(current_template);

        for (old, new) in replacements.iter() {
            template_with_replacements = template_with_replacements.replace(old, new);
        }

        if let Some(include_statements) = &self.includes {
            // The includes should never be empty thanks to the required group in clap
            assert!(!include_statements.is_empty());
            // Join the includes into a block of text, with blank lines in between to prevent
            // the AsciiDoc syntax to blend between modules
            let includes_text = include_statements.join("\n\n");

            template_with_replacements =
                template_with_replacements.replace("${include_statements}", &includes_text);
        } else {
            template_with_replacements = template_with_replacements
                .replace("${include_statements}", "Include modules here.");
        }

        // If comments are disabled via an option, delete comment lines from the content
        // TODO: This doesn't handle AsciiDoc comment blocks at all
        if !self.options.comments {
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
}

impl From<Input> for Module {
    /// Convert the `Input` builder struct into the finished `Module` struct.
    fn from(input: Input) -> Self {
        Module {
            mod_type: input.mod_type.clone(),
            title: input.title.clone(),
            id: input.id(),
            file_name: input.file_name(),
            include_statement: input.include_statement(),
            includes: input.includes.clone(),
            text: input.text(),
        }
    }
}

impl Module {
    /// The constructor for the Module struct. Creates a basic version of Module
    /// without any optional features.
    pub fn new(mod_type: ModuleType, title: &str, options: &Options) -> Module {
        let input = Input::new(mod_type, title, options);
        input.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::module::Module;
    use crate::module::ModuleType;
    use crate::Options;

    #[test]
    fn check_module_fields() {
        let options = Options {
            comments: false,
            prefixes: true,
            target_dir: ".".to_string(),
            detect_directory: true,
        };
        let module = Module::new(ModuleType::Assembly, "A testing assembly with /special-characters*", &options);

        // We can't test this until we implement a way to compare the enum values
        // assert_eq!(module.mod_type, ModuleType::Assembly);
        assert_eq!(module.title, "A testing assembly with /special-characters*".to_string());
        assert_eq!(module.id, "a-testing-assembly-with-special-characters".to_string());
    }
}
