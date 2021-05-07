use askama::Template;
use regex::{Regex, RegexBuilder};

use crate::module::{Input, ModuleType};

// A note on the structure of this file:
// This file repeats a lot of code when it configures the Askama templates.
// Either we could fix it by using a generics trick, which I haven't learned yet,
// or the repetition is inherent to the way the templates share some properties, but only
// by accident, not as a rule.
// For now, the code works as intended, and the file is short enough that I'm not bothered
// to see the questionable esthetics.

// Specify a template in terms of the Askama engine
#[derive(Template)]
// Askama loads the template files from the `data/templates` directory,
// which is configured in the `askama.toml` file.
#[template(path = "assembly.adoc", escape = "none")]
struct AssemblyTemplate<'a> {
    // The field name must match the variable name in the template
    module_id: &'a str,
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

// We're implementing the template functions on the Input struct, not on Module,
// because the templating happens at the point when newdoc composes the text of the module,
// which is part of the module creation. The module then stores the rendered template.
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
            }
            .render(),
            ModuleType::Concept => ConceptTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                examples: self.options.examples,
            }
            .render(),
            ModuleType::Procedure => ProcedureTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                examples: self.options.examples,
            }
            .render(),
            ModuleType::Reference => ReferenceTemplate {
                module_id: &self.id(),
                module_title: &self.title,
                examples: self.options.examples,
            }
            .render(),
        }
        .expect("Failed to construct the document from the template");

        // If comments are disabled via an option, delete comment lines from the content
        if !self.options.comments {
            // Delete multi-line (block) comments
            let multi_comments: Regex = RegexBuilder::new(r"^////[\s\S\n]*^////[\s]*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            document = multi_comments.replace_all(&document, "").to_string();

            // Delete single-line comments
            let single_comments: Regex = RegexBuilder::new(r"^//.*\n")
                .multi_line(true)
                .swap_greed(true)
                .build()
                .unwrap();
            document = single_comments.replace_all(&document, "").to_string();

            // Delete leading white space left over by the deleted comments
            let leading_whitespace: Regex = RegexBuilder::new(r"^[\s\n]*")
                .multi_line(true)
                .build()
                .unwrap();
            document = leading_whitespace.replace(&document, "").to_string();
        }

        // Remove excess blank lines that might have been left by the verious
        // replacement stages. Make sure that the result contains no more than one
        // consecutive blank line.
        let two_blanks = "\n\n\n";
        let one_blank = "\n\n";

        while document.contains(two_blanks) {
            document = document.replace(two_blanks, one_blank);
        }

        // Add newlines at the end of the document to prevent potential issues
        // when including two AsciiDoc files right next to each other.
        document + one_blank
    }
}
