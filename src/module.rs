/*
newdoc: Generate pre-populated documentation modules formatted with AsciiDoc.
Copyright (C) 2022  Marek Suchánek  <msuchane@redhat.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! This module defines the `Module` struct, its builder struct, and methods on both structs.

use std::fmt;
use std::path::{Component, Path, PathBuf};

use crate::Options;

/// All possible types of the AsciiDoc module
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Assembly,
    Concept,
    Procedure,
    Reference,
    Snippet,
}

// Implement human-readable string display for the module type
impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Assembly => "assembly",
            Self::Concept => "concept",
            Self::Procedure => "procedure",
            Self::Reference => "reference",
            Self::Snippet => "snippet",
        };
        write!(f, "{}", name)
    }
}

/// An initial representation of the module with input data, used to construct the `Module` struct
#[derive(Debug)]
pub struct Input {
    pub mod_type: ContentType,
    pub title: String,
    pub options: Options,
    pub includes: Option<Vec<String>>,
}

/// A representation of the module with all its metadata and the generated AsciiDoc content
#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    mod_type: ContentType,
    title: String,
    anchor: String,
    pub file_name: String,
    pub include_statement: String,
    includes: Option<Vec<String>>,
    pub text: String,
}

/// Construct a basic builder for `Module`, storing information from the user input.
impl Input {
    #[must_use]
    pub fn new(mod_type: ContentType, title: &str, options: &Options) -> Input {
        log::debug!("Processing title `{}` of type `{:?}`", title, mod_type);

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
    #[must_use]
    pub fn include(mut self, include_statements: Vec<String>) -> Self {
        self.includes = Some(include_statements);
        self
    }

    /// Create an ID string that is derived from the human-readable title. The ID is usable as:
    ///
    /// * An AsciiDoc section ID
    /// * A DocBook section ID
    /// * A file name
    ///
    /// # Examples
    ///
    /// ```
    /// use newdoc::{ContentType, Input, Options};
    ///
    /// let mod_type = ContentType::Concept;
    /// let title = "A test -- with #problematic ? characters";
    /// let options = Options::default();
    /// let input = Input::new(mod_type, title, &options);
    ///
    /// assert_eq!("a-test-with-problematic-characters", input.id());
    /// ```
    #[must_use]
    pub fn id(&self) -> String {
        let title = &self.title;
        // The ID is all lower-case
        let mut title_with_replacements: String = String::from(title).to_lowercase();

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
            (",", "-"),
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
            ("=", "-"),
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
            // Print an error in that case? Escape them with AsciiDoc escapes?
            ("{", ""),
            ("}", ""),
        ];

        // Perform all the defined replacements on the title
        for (old, new) in substitutions {
            title_with_replacements = title_with_replacements.replace(old, new);
        }

        // Replace remaining characters that aren't ASCII, or that are non-alphanumeric ASCII,
        // with dashes. For example, this replaces diacritics and typographic quotation marks.
        title_with_replacements = title_with_replacements
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect();

        // Ensure the converted ID doesn't contain double dashes ("--"), because
        // that breaks references to the ID
        while title_with_replacements.contains("--") {
            title_with_replacements = title_with_replacements.replace("--", "-");
        }

        // Ensure that the ID doesn't end with a dash
        if title_with_replacements.ends_with('-') {
            let len = title_with_replacements.len();
            title_with_replacements = title_with_replacements[..len - 1].to_string();
        }

        title_with_replacements
    }

    /// Prepare the file name for the generated file.
    ///
    /// The file name is based on the module ID,
    /// with an optional prefix and the `.adoc` extension.
    #[must_use]
    pub fn file_name(&self) -> String {
        // Add a prefix only if they're enabled.
        let prefix = if self.options.file_prefixes {
            self.prefix()
        } else {
            ""
        };

        let id = self.id();

        let suffix = ".adoc";

        [prefix, &id, suffix].join("")
    }

    /// Prepare the AsciiDoc anchor or ID.
    ///
    /// The anchor is based on the module ID, with an optional prefix.
    #[must_use]
    pub fn anchor(&self) -> String {
        // Add a prefix only if they're enabled.
        let prefix = if self.options.anchor_prefixes {
            self.prefix()
        } else {
            ""
        };

        let id = self.id();

        [prefix, &id].join("")
    }

    /// Pick the right file and ID prefix depending on the content type.
    fn prefix(&self) -> &'static str {
        match self.mod_type {
            ContentType::Assembly => "assembly_",
            ContentType::Concept => "con_",
            ContentType::Procedure => "proc_",
            ContentType::Reference => "ref_",
            ContentType::Snippet => "snip_",
        }
    }

    /// Prepare an include statement that can be used to include the generated file from elsewhere.
    fn include_statement(&self) -> String {
        let path_placeholder = Path::new("<path>").to_path_buf();

        let include_path = match self.infer_include_dir() {
            Some(path) => path,
            None => path_placeholder,
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
        // based on the module type, or `snippets/` for snippet files.
        let include_root = match &self.mod_type {
            ContentType::Assembly => "assemblies",
            ContentType::Snippet => "snippets",
            _ => "modules",
        };

        // TODO: Maybe convert the path earlier in the module building.
        let relative_path = Path::new(&self.options.target_dir);
        // Try to find the root element in an absolute path.
        // If the absolute path cannot be constructed due to an error, search the relative path instead.
        let target_path = match relative_path.canonicalize() {
            Ok(path) => path,
            Err(_) => relative_path.to_path_buf(),
        };

        // Split the target path into components
        let component_vec: Vec<_> = target_path
            .as_path()
            .components()
            .map(Component::as_os_str)
            .collect();

        // Find the position of the component that matches the root element,
        // searching from the end of the path forward.
        let root_position = component_vec.iter().rposition(|&c| c == include_root);

        // If there is such a root element in the path, construct the include path.
        // TODO: To be safe, check that the root path element still exists in a Git repository.
        if let Some(position) = root_position {
            let include_path = component_vec[position..].iter().collect::<PathBuf>();
            Some(include_path)
        // If no appropriate root element was found, use a generic placeholder.
        } else {
            None
        }
    }
}

impl From<Input> for Module {
    /// Convert the `Input` builder struct into the finished `Module` struct.
    fn from(input: Input) -> Self {
        let module = Module {
            mod_type: input.mod_type,
            title: input.title.clone(),
            anchor: input.anchor(),
            file_name: input.file_name(),
            include_statement: input.include_statement(),
            includes: input.includes.clone(),
            text: input.text(),
        };

        log::debug!("Generated module properties:");
        log::debug!("Type: {:?}", &module.mod_type);
        log::debug!("Anchor: {}", &module.anchor);
        log::debug!("File name: {}", &module.file_name);
        log::debug!("Include statement: {}", &module.include_statement);
        log::debug!(
            "Included modules: {}",
            if let Some(includes) = &module.includes {
                includes.join(", ")
            } else {
                "none".to_string()
            }
        );

        module
    }
}

impl Module {
    /// The constructor for the Module struct. Creates a basic version of Module
    /// without any optional features.
    #[must_use]
    pub fn new(mod_type: ContentType, title: &str, options: &Options) -> Module {
        let input = Input::new(mod_type, title, options);
        input.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Options, Verbosity};

    fn basic_options() -> Options {
        Options {
            comments: false,
            file_prefixes: true,
            anchor_prefixes: false,
            examples: true,
            target_dir: PathBuf::from("."),
            verbosity: Verbosity::Default,
        }
    }

    fn path_options() -> Options {
        Options {
            comments: false,
            file_prefixes: true,
            anchor_prefixes: false,
            examples: true,
            target_dir: PathBuf::from("repo/modules/topic/"),
            verbosity: Verbosity::Default,
        }
    }

    #[test]
    fn check_basic_assembly_fields() {
        let options = basic_options();
        let assembly = Module::new(
            ContentType::Assembly,
            "A testing assembly with /special-characters*",
            &options,
        );

        assert_eq!(assembly.mod_type, ContentType::Assembly);
        assert_eq!(
            assembly.title,
            "A testing assembly with /special-characters*"
        );
        assert_eq!(
            assembly.anchor,
            "a-testing-assembly-with-special-characters"
        );
        assert_eq!(
            assembly.file_name,
            "assembly_a-testing-assembly-with-special-characters.adoc"
        );
        assert_eq!(assembly.include_statement, "include::<path>/assembly_a-testing-assembly-with-special-characters.adoc[leveloffset=+1]");
        assert_eq!(assembly.includes, None);
    }

    #[test]
    fn check_module_builder_and_new() {
        let options = basic_options();
        let from_new: Module = Module::new(
            ContentType::Assembly,
            "A testing assembly with /special-characters*",
            &options,
        );
        let from_builder: Module = Input::new(
            ContentType::Assembly,
            "A testing assembly with /special-characters*",
            &options,
        )
        .into();
        assert_eq!(from_new, from_builder);
    }

    #[test]
    fn check_detected_path() {
        let options = path_options();

        let module = Module::new(
            ContentType::Procedure,
            "Testing the detected path",
            &options,
        );

        assert_eq!(
            module.include_statement,
            "include::modules/topic/proc_testing-the-detected-path.adoc[leveloffset=+1]"
        );
    }
}
