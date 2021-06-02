/// This module provides functionality to validate (lint) existing module and assembly files,
/// to check if the files meet the template structure and other requirements.

use std::fs;
use std::path::Path;
// use std::process::exit;
use log::{debug, error, info, warn};

use crate::module::ModuleType;

pub fn validate(file_name: &str) {
    info!("Validating file `{}`", file_name);

    let path = Path::new(file_name);

    let read_result = fs::read_to_string(path);
    let content = match read_result {
        Ok(content) => content,
        Err(err) => {
            error!("Error reading file `{}`: {}", file_name, err);
            return;
        }
    };

    let base_name = path.file_name().unwrap().to_str().unwrap();

    let mod_type = determine_mod_type(base_name, &content);
    info!("`{}`: Module type is {:?}.", file_name, mod_type.unwrap());
}

fn determine_mod_type(file_name: &str, content: &str) -> Option<ModuleType> {
    if file_name.starts_with("assembly_") || file_name.starts_with("assembly-") {
        return Some(ModuleType::Assembly);
    }
    let mod_patterns = [
        ("con", ":_module-type: CONCEPT", ModuleType::Concept),
        ("proc", ":_module-type: PROCEDURE", ModuleType::Procedure),
        ("ref", ":_module-type: REFERENCE", ModuleType::Reference),
    ];
    for pattern in mod_patterns.iter() {
        if file_name.starts_with(pattern.0) || content.contains(pattern.1) {
            return Some(pattern.2);
        }
    }
    error!("`{}`: Cannot determine the module type.", file_name);
    return None;
}