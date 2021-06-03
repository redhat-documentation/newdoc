/// This module provides functionality to validate (lint) existing module and assembly files,
/// to check if the files meet the template structure and other requirements.

use std::fmt;
use std::fs;
use std::path::Path;
// use std::process::exit;
use log::{debug, error, info, warn};
use regex::{Regex, RegexBuilder};

use crate::module::ModuleType;

const ASSEMBLY_TESTS: [IssueDefinition; 3] = [
    // Test that an assembly includes no other assemblies
    IssueDefinition {
        pattern: r"^include::.*assembly[_-].*\.adoc",
        description: "This assembly includes another assembly.",
        severity: IssueSeverity::Error,
    },
    // Test that files don't use the unsupported leveloffset configuration
    IssueDefinition {
        pattern: r"^:leveloffset:\s*\+\d*",
        description: "Unsupported level offset configuration.",
        severity: IssueSeverity::Error,
    },
    IssueDefinition {
        pattern: r"^\.Additional resources",
        description: "In assemblies, 'Additional resources' must use the == syntax.",
        severity: IssueSeverity::Error,
    },
];

const MODULE_TESTS: [IssueDefinition; 1] = [
    // Test that modules include no other modules, except for snippets
    // This one doesn't work because the regex crate doesn't support lookahead
    /*
    IssueDefinition {
        pattern: r"^include::(?!(snip|.*\/snip)[_-]).*\.adoc",
        description: "This module includes another file that is not a snippet.",
        severity: IssueSeverity::Error,
    },
    */
    IssueDefinition {
        pattern: r"^==\s*Additional resources",
        description: "In modules, 'Additional resources' must use the dot syntax.",
        severity: IssueSeverity::Error,
    },
];

#[derive(Clone, Copy, Debug)]
struct IssueDefinition {
    pattern: &'static str,
    description: &'static str,
    severity: IssueSeverity,
}

#[derive(Clone, Copy, Debug)]
enum IssueSeverity {
    Information,
    Warning,
    Error,
}

#[derive(Debug)]
struct IssueReport {
    line_number: i32,
    description: &'static str,
    severity: IssueSeverity,
}

impl fmt::Display for IssueReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line_number, self.description)
    }
}

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
    } else if mod_type.is_some() {
        module_tests(base_name, &content);
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
    // check_no_nesting(base_name, content);
    // check_supported_leveloffset(base_name, content);
    let from_issues = ASSEMBLY_TESTS.iter()
        .map(|&definition| check_for_issue(definition, content));
    
    for reports in from_issues {
        for report in reports {
            println!("{}", report);
        }
    }
}

fn module_tests(base_name: &str, content: &str) {
    let from_issues = MODULE_TESTS.iter()
        .map(|&definition| check_for_issue(definition, content));
    
    for reports in from_issues {
        for report in reports {
            println!("{}", report);
        }
    }
}

/// This function checks a file content for the presence of an issue based on a regex.
/// These issues are defined using the IssueDefinition struct.
fn check_for_issue(issue: IssueDefinition, content: &str) -> Vec<IssueReport> {
    let regex = RegexBuilder::new(issue.pattern)
        .multi_line(true)
        .build()
        .unwrap();
    let findings = regex.find_iter(content);

    findings.map(|finding| {
        IssueReport {
            line_number: finding.start() as i32,
            description: issue.description,
            severity: issue.severity,
        }
    }).collect()
}
