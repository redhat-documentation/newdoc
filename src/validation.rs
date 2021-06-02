/// This module provides functionality to validate (lint) existing module and assembly files,
/// to check if the files meet the template structure and other requirements.

use std::fs;
use std::path::Path;
// use std::process::exit;
use log::{debug, error, info, warn};
use regex::{Regex, RegexBuilder};

use crate::module::ModuleType;

pub fn validate(file_name: &str) {
    debug!("Validating file `{}`", file_name);

    let path = Path::new(file_name);
    let base_name = path.file_name().unwrap().to_str().unwrap();

    let read_result = fs::read_to_string(path);
    let content = match read_result {
        Ok(content) => content,
        Err(err) => {
            error!("Error reading file `{}`: {}", file_name, err);
            return;
        }
    };

    let mod_type = determine_mod_type(base_name, &content);

    if mod_type == Some(ModuleType::Assembly) {
        assembly_tests(base_name, &content);
    }
}

fn determine_mod_type(base_name: &str, content: &str) -> Option<ModuleType> {
    // An inner function to encapsulate the logic.
    // This can't be a simple expression block because the logic involves
    // stuff like for loops.
    let inner = || {
        if base_name.starts_with("assembly_") || base_name.starts_with("assembly-") {
            return Some(ModuleType::Assembly);
        }
        let mod_patterns = [
            ("con", ":_module-type: CONCEPT", ModuleType::Concept),
            ("proc", ":_module-type: PROCEDURE", ModuleType::Procedure),
            ("ref", ":_module-type: REFERENCE", ModuleType::Reference),
        ];
        for pattern in mod_patterns.iter() {
            if base_name.starts_with(pattern.0) || content.contains(pattern.1) {
                return Some(pattern.2);
            }
        }
        error!("`{}`: Cannot determine the module type.", base_name);
        return None;
    };

    // Run the inner function
    let mod_type = inner();

    // Report on the value received from the inner function
    match mod_type {
        None => { error!("`{}`: Cannot determine the module type.", base_name); },
        Some(mod_type) => { info!("`{}`: Module type is {}.", base_name, mod_type); }
    }

    mod_type
}

/// This function collects all tests that target only assembly files
fn assembly_tests(base_name: &str, content: &str) {
    check_no_nesting(base_name, content);
    check_supported_leveloffset(base_name, content);
}

/// Test that an assembly includes no other assemblies
fn check_no_nesting(base_name: &str, content: &str) {
    let include_pattern = r"^include::.*assembly[_-].*\.adoc";

    // Currently, we're using a multi-line regex over the whole file.
    // It would be better to use single-line regexes and interate over the lines
    // in the file individually. The main benefit would be that we could report
    // the exact line where this occurs, rather than the character.
    let include_regex = RegexBuilder::new(include_pattern)
        .multi_line(true)
        .build()
        .unwrap();
    let included_assemblies = include_regex.find_iter(content);

    for assembly in included_assemblies {
        let position = assembly.start();
        let text = assembly.as_str();
        error!("`{}`: Includes another assembly at character {}: `{}`", base_name, position, text);
    }
}

/// Test that files don't use the unsupported leveloffset configuration
fn check_supported_leveloffset(base_name: &str, content: &str) {
    let unsupported_pattern = r"^:leveloffset:\s*\+\d*";
    let unsupported_regex = RegexBuilder::new(unsupported_pattern)
        .multi_line(true)
        .build()
        .unwrap();
    let leveloffsets = unsupported_regex.find_iter(content);
    for leveloffset in leveloffsets {
        let position = leveloffset.start();
        let text = leveloffset.as_str();
        error!("`{}`: Unsupported include configuration at character {}: `{}`", base_name, position, text);
    }
}