use regex::{Regex, RegexBuilder};

use crate::module::{Input, ModuleType};

// Load the AsciiDoc templates at build time
const ASSEMBLY_TEMPLATE: &str = include_str!("../data/templates/assembly.adoc");
const CONCEPT_TEMPLATE: &str = include_str!("../data/templates/concept.adoc");
const PROCEDURE_TEMPLATE: &str = include_str!("../data/templates/procedure.adoc");
const REFERENCE_TEMPLATE: &str = include_str!("../data/templates/reference.adoc");


impl Input {
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
        } else if self.options.examples {
            template_with_replacements = template_with_replacements
                .replace("${include_statements}", "Include modules here.");
        } else {
            template_with_replacements =
                template_with_replacements.replace("${include_statements}\n", "");
        }

        // If the `--no-examples` option is active, remove all lines between the <example> tags.
        if !self.options.examples {
            let examples: Regex = RegexBuilder::new(r"^// <example>\n[\s\S]*\n^// </example>\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template_with_replacements = examples
                .replace_all(&template_with_replacements, "")
                .to_string();
        // If the `--no-examples` option isn't active, remove just the <example> tags.
        } else {
            let example_tags: Regex = RegexBuilder::new(r"^// </?example>\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template_with_replacements = example_tags
                .replace_all(&template_with_replacements, "")
                .to_string();
        }

        // If comments are disabled via an option, delete comment lines from the content
        if !self.options.comments {
            // Delete multi-line (block) comments
            let multi_comments: Regex = RegexBuilder::new(r"^////[\s\S\n]*^////[\s]*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template_with_replacements = multi_comments
                .replace_all(&template_with_replacements, "")
                .to_string();

            // Delete single-line comments
            let single_comments: Regex = RegexBuilder::new(r"^//.*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template_with_replacements = single_comments
                .replace_all(&template_with_replacements, "")
                .to_string();

            // Delete leading white space left over by the deleted comments
            let leading_whitespace: Regex = RegexBuilder::new(r"^[\s\n]*")
                .multi_line(true)
                .build()
                .unwrap();
            template_with_replacements = leading_whitespace
                .replace(&template_with_replacements, "")
                .to_string();
        }

        template_with_replacements
    }
}
