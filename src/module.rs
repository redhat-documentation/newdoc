use crate::Options;

/// All possible types of the AsciiDoc module
#[derive(Debug)]
pub enum ModuleType {
    Assembly,
    Concept,
    Procedure,
    Reference,
}

/// A representation of the module with all its metadata and the generated AsciiDoc content
#[derive(Debug)]
pub struct Module {
    pub mod_type: ModuleType,
    pub title: String,
    pub id: String,
    pub file_name: String,
    pub include_statement: String,
    pub text: String,
    included: Option<Vec<String>>,
}

// Load the AsciiDoc templates at build time
const ASSEMBLY_TEMPLATE: &str = include_str!("../templates/assembly.adoc");
const CONCEPT_TEMPLATE: &str = include_str!("../templates/concept.adoc");
const PROCEDURE_TEMPLATE: &str = include_str!("../templates/procedure.adoc");
const REFERENCE_TEMPLATE: &str = include_str!("../templates/reference.adoc");

impl Module {
    /// The constructor for the Module struct
    pub fn new(
        mod_type: ModuleType,
        title: &str,
        options: &Options,
    ) -> Module {
        let title = String::from(title);
        let id = Module::convert_title_to_id(&title);
        let file_name = Module::compose_file_name(&id, &mod_type, &options);
        let include_statement = Module::compose_include_statement(&file_name);
        let text = Module::compose_text(&title, &id, &mod_type, None, &options);

        Module {
            mod_type,
            title,
            id,
            file_name,
            include_statement,
            text,
            included: None,
        }
    }

    pub fn includes(mut self, include_statements: Vec<String>) -> Self {
        self.included = Some(include_statements);
        self
    }

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
        includes: Option<&[String]>,
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
        let replacements = [("${module_title}", title), ("${module_id}", module_id)];

        // Perform substitutions in the template
        // TODO: Create a separate function to perform a replacement
        let mut template_with_replacements = String::from(current_template);

        for (old, new) in replacements.iter() {
            template_with_replacements = template_with_replacements.replace(old, new);
        }

        if let Some(includes) = includes {
            // The includes should never be empty thanks to the required group in clap
            assert!(!includes.is_empty());
            // Join the includes into a block of text, with blank lines in between to prevent
            // the AsciiDoc syntax to blend between modules
            let includes_text = includes.join("\n\n");

            template_with_replacements =
                template_with_replacements.replace("${include_statements}", &includes_text);
        } else {
            template_with_replacements = template_with_replacements
                .replace("${include_statements}", "Include modules here.");
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

    /// Prepare an include statement that can be used to include the generated file from elsewhere.
    fn compose_include_statement(file_name: &str) -> String {
        format!("include::<path>/{}[leveloffset=+1]", file_name)
    }
}
