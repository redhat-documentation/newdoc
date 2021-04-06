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
    examples: bool,
}

#[derive(Template)]
#[template(path = "concept.adoc", escape = "none")]
struct ConceptTemplate<'a> {
    module_id: &'a str,
    module_title: &'a str,
    examples: bool,
}

#[derive(Template)]
#[template(path = "procedure.adoc", escape = "none")]
struct ProcedureTemplate<'a> {
    module_id: &'a str,
    module_title: &'a str,
    examples: bool,
}

#[derive(Template)]
#[template(path = "reference.adoc", escape = "none")]
struct ReferenceTemplate<'a> {
    module_id: &'a str,
    module_title: &'a str,
    examples: bool,
}


impl Input {
    /// Render the include statements that appear inside an assembly
    /// into the final format. If the assembly includes nothing, use
    /// a placeholder, or an empty string if examples are disabled.
    fn includes_block(&self) -> String {
        if let Some(include_statements) = &self.includes {
            // The includes should never be empty thanks to the required group in clap
            assert!(!include_statements.is_empty());
            // Join the includes into a block of text, with blank lines in between to prevent
            // the AsciiDoc syntax to blend between modules
            include_statements.join("\n\n")
        } else if self.options.examples {
            "Include modules here.".to_string()
        } else {
            String::new()
        }
    }

    /// Perform string replacements in the modular template that matches the `ModuleType`.
    /// Return the template text with all replacements.
    pub fn text(&self) -> String {
        let mut document = match self.mod_type {
            ModuleType::Assembly => AssemblyTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                include_statements: &self.includes_block(),
                examples: self.options.examples,
            }.render(),
            ModuleType::Concept => ConceptTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                examples: self.options.examples,
            }.render(),
            ModuleType::Procedure => ProcedureTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                examples: self.options.examples,
            }.render(),
            ModuleType::Reference => ReferenceTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                examples: self.options.examples,
            }.render(),
        }.expect("Failed to construct the document from the template");

        // If comments are disabled via an option, delete comment lines from the content
        if !self.options.comments {
            // Delete multi-line (block) comments
            let multi_comments: Regex = RegexBuilder::new(r"^////[\s\S\n]*^////[\s]*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            document = multi_comments
                .replace_all(&document, "")
                .to_string();

            // Delete single-line comments
            let single_comments: Regex = RegexBuilder::new(r"^//.*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            document = single_comments
                .replace_all(&document, "")
                .to_string();

            // Delete leading white space left over by the deleted comments
            let leading_whitespace: Regex = RegexBuilder::new(r"^[\s\n]*")
                .multi_line(true)
                .build()
                .unwrap();
            document = leading_whitespace
                .replace(&document, "")
                .to_string();
        }

        // Add newlines at the end of the document to prevent potential issues
        // when including two AsciiDoc files right next to each other.
        document + "\n\n"
    }
}
