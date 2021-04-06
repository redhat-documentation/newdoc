use regex::{Regex, RegexBuilder};
use askama::Template; // bring trait in scope

use crate::module::{Input, ModuleType};


#[derive(Template)] // this will generate the code...
#[template(path = "assembly.adoc", escape = "none")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct AssemblyTemplate<'a> { // the name of the struct can be anything
    module_id: &'a str, // the field name should match the variable name
                   // in your template
    module_title: &'a str,
    include_statements: &'a str,
}

#[derive(Template)]
#[template(path = "concept.adoc", escape = "none")]
struct ConceptTemplate<'a> {
    module_id: &'a str,
    module_title: &'a str,
}

#[derive(Template)]
#[template(path = "procedure.adoc", escape = "none")]
struct ProcedureTemplate<'a> {
    module_id: &'a str,
    module_title: &'a str,
}

#[derive(Template)]
#[template(path = "reference.adoc", escape = "none")]
struct ReferenceTemplate<'a> {
    module_id: &'a str,
    module_title: &'a str,
}


impl Input {
    /// Perform string replacements in the modular template that matches the `ModuleType`.
    /// Return the template text with all replacements.
    pub fn text(&self) -> String {
        // TODO: Add a comment in the generated file with a pre-filled include statement

        let mut template = match self.mod_type {
            ModuleType::Assembly => AssemblyTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                include_statements: if let Some(include_statements) = &self.includes {
                    /* include_statements.join("\n\n") */
                    "Include this.\n\nInclude that."
                } else {
                    "Include modules here."
                }
            }.render(),
            ModuleType::Concept => ConceptTemplate { module_id: &self.id(), module_title: &self.title }.render(),
            ModuleType::Procedure => ProcedureTemplate { module_id: &self.id(), module_title: &self.title }.render(),
            ModuleType::Reference => ReferenceTemplate { module_id: &self.id(), module_title: &self.title }.render(),
        }.expect("Failed to cintruct the template");

        /*
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
        */

        // If the `--no-examples` option is active, remove all lines between the <example> tags.
        if !self.options.examples {
            let examples: Regex = RegexBuilder::new(r"^// <example>\n[\s\S]*\n^// </example>\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template = examples
                .replace_all(&template, "")
                .to_string();
        // If the `--no-examples` option isn't active, remove just the <example> tags.
        } else {
            let example_tags: Regex = RegexBuilder::new(r"^// </?example>\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template = example_tags
                .replace_all(&template, "")
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
            template = multi_comments
                .replace_all(&template, "")
                .to_string();

            // Delete single-line comments
            let single_comments: Regex = RegexBuilder::new(r"^//.*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            template = single_comments
                .replace_all(&template, "")
                .to_string();

            // Delete leading white space left over by the deleted comments
            let leading_whitespace: Regex = RegexBuilder::new(r"^[\s\n]*")
                .multi_line(true)
                .build()
                .unwrap();
            template = leading_whitespace
                .replace(&template, "")
                .to_string();
        }

        template
    }
}
