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
        multiline: false,
    },
    // Test that files don't use the unsupported leveloffset configuration
    IssueDefinition {
        pattern: r"^:leveloffset:\s*\+\d*",
        description: "Unsupported level offset configuration.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
    // Ensure the correct syntax for Additional resources
    IssueDefinition {
        pattern: r"^\.Additional resources",
        description: "In assemblies, 'Additional resources' must use the == syntax.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
];

const MODULE_TESTS: [IssueDefinition; 1] = [
    // Ensure the correct syntax for Additional resources
    IssueDefinition {
        pattern: r"^==\s*Additional resources",
        description: "In modules, 'Additional resources' must use the dot syntax.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
];

const TITLE_TESTS: [IssueDefinition; 2] =[
    // Test that there are no inline anchors in the title
    IssueDefinition {
        pattern: r"^=\s+.*\[\[\S+\]\].*",
        description: "The title contains an inline anchor.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
    IssueDefinition {
        pattern: r"^=\s+.*\{\S+\}.*",
        description: "The title contains an attribute.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
];

#[derive(Clone, Copy, Debug)]
struct IssueDefinition {
    pattern: &'static str,
    description: &'static str,
    severity: IssueSeverity,
    multiline: bool,
}

#[derive(Clone, Copy, Debug)]
enum IssueSeverity {
    Information,
    Warning,
    Error,
}

impl fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Self::Information => "Information",
            Self::Warning => "Warning",
            Self::Error => "Error",
        };
        write!(f, "{}", display)
    }
}

#[derive(Debug)]
struct IssueReport {
    // Not all issues have a line number
    line_number: Option<usize>,
    description: &'static str,
    severity: IssueSeverity,
}

impl fmt::Display for IssueReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stamp = if let Some(line_number) = self.line_number {
            format!("{} at line {}: ", self.severity, line_number)
        } else {
            format!("{}: ", self.severity)
        };
        let display = stamp + self.description;
        write!(f, "{}", display)
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

    let reports = if mod_type == Some(ModuleType::Assembly) {
        assembly_tests(base_name, &content)
    } else if mod_type.is_some() {
        module_tests(base_name, &content)
    } else {
        warn!("Cannot determine module type for {}", file_name);
        Vec::new()
    };

    report_issues(reports, file_name);
}

/// Print a human-readable report about the issues found in the file
fn report_issues(issues: Vec<IssueReport>, file_path: &str) {
    if !issues.is_empty() {
        println!("File: {}", file_path);
        for issue in issues {
            println!("  * {}", issue);
        }
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
        Some(mod_type) => { debug!("`{}`: Module type is {}.", base_name, mod_type); }
    }

    mod_type
}

/// This function collects all tests that target only assembly files
fn assembly_tests(base_name: &str, content: &str) -> Vec<IssueReport> {
    // check_no_nesting(base_name, content);
    // check_supported_leveloffset(base_name, content);
    let mut reports: Vec<IssueReport> = ASSEMBLY_TESTS.iter()
        .map(|&definition| check_for_issue(definition, content))
        .flatten()
        .collect();

    reports.append(TITLE_TESTS.iter()
        .map(|&definition| check_for_issue(definition, content))
        .flatten()
        .collect::<Vec<_>>()
        .as_mut());
    
    reports
}

fn module_tests(base_name: &str, content: &str) -> Vec<IssueReport> {
    let mut reports: Vec<IssueReport> = MODULE_TESTS.iter()
        .map(|&definition| check_for_issue(definition, content))
        .flatten()
        .collect();
    
    reports.append(TITLE_TESTS.iter()
        .map(|&definition| check_for_issue(definition, content))
        .flatten()
        .collect::<Vec<_>>()
        .as_mut());
    
    reports.append(check_metadata_variable(content).as_mut());
    reports.append(check_include_except_snip(content).as_mut());
    
    reports
}

/// This function checks a file content for the presence of an issue based on a regex.
/// These issues are defined using the IssueDefinition struct.
fn check_for_issue(issue: IssueDefinition, content: &str) -> Vec<IssueReport> {
    if issue.multiline {
        let regex = RegexBuilder::new(issue.pattern)
            .multi_line(true)
            .build()
            .unwrap();
        let findings = regex.find_iter(content);
        
        findings.map(|finding| {
            IssueReport {
                line_number: line_from_byte_no(content, finding.start()),
                description: issue.description,
                severity: issue.severity,
            }
        }).collect()
    } else {
        let regex = Regex::new(issue.pattern).unwrap();
        let findings = content.lines().enumerate()
            .map(|(index, line)| (index, regex.find(line)))
            .filter(|(_index, found)| found.is_some());
        
        findings.map(|(index, _finding)| {
            IssueReport {
                // Line numbers start from 1, but the enumeration starts from 0. Add 1 to the index
                line_number: Some(index + 1),
                description: issue.description,
                severity: issue.severity,
            }
        }).collect()
    }
}

/// Check that the module type variable is located before the module ID, as required.
/// As a side effect, this function also checks that both the varible and the ID are present.
/// TODO: Refactor the ID check separately so that it can also be used for assemblies.
fn check_metadata_variable(content: &str) -> Vec<IssueReport> {
    let metadata_var_pattern = r":_module-type:\s*(?:PROCEDURE|CONCEPT|REFERENCE)";
    let metadata_var_regex = Regex::new(metadata_var_pattern).unwrap();
    let mut metadata_var_position = None;

    let mod_id_pattern = r"^\[id=";
    let mod_id_regex = Regex::new(mod_id_pattern).unwrap();
    let mut mod_id_position = None;

    // Browse all lines in the module, and if we find any of the two elements, record the line number
    for (index, line) in content.lines().enumerate() {
        if let Some(_metadata_var) = metadata_var_regex.find(line) {
            metadata_var_position = Some(index);
        }
        if let Some(_mod_id) = mod_id_regex.find(line) {
            mod_id_position = Some(index);
        }
    }

    // Prepare to store the reports about the module
    let mut results: Vec<IssueReport> = Vec::new();

    // Report if any of the two elements is completely missing
    if mod_id_position.is_none() {
        let report = IssueReport {
            line_number: None,
            description: "The module is missing an ID.",
            severity: IssueSeverity::Error,
        };
        results.push(report);
    }
    if metadata_var_position.is_none() {
        let report = IssueReport {
            line_number: None,
            description: "The module is missing the module type variable.",
            severity: IssueSeverity::Error,
        };
        results.push(report);
    }

    // If both elements are present, ensure their proper position in relation to each other
    if let (Some(mod_id_position), Some(metadata_var_position)) = (mod_id_position, metadata_var_position) {
        if mod_id_position < metadata_var_position {
            let report = IssueReport {
                line_number: Some(metadata_var_position),
                description: "The module type variable is located after the module ID.",
                severity: IssueSeverity::Error,
            };
            results.push(report);
        }
    }

    results
}

/// Test that modules include no other modules, except for snippets
fn check_include_except_snip(content: &str) -> Vec<IssueReport> {
    let any_include_pattern = r"^include::.*\.adoc";
    let any_include_regex = Regex::new(any_include_pattern).unwrap();

    let snip_include_pattern = r"^include::((snip|.*/snip)[_-]|common-content/).*\.adoc";
    let snip_include_regex = Regex::new(snip_include_pattern).unwrap();

    let mut reports: Vec<IssueReport> = Vec::new();

    for (index, line) in content.lines().enumerate() {
        if let Some(include) = any_include_regex.find(line) {
            if let Some(_snippet) = snip_include_regex.find(include.as_str()) {
                // In this case, the detected include is most likely a snippet. Report as Information
                let report = IssueReport {
                    line_number: Some(index + 1),
                    description: "This module includes a file that appears to be a snippet. This is supported.",
                    severity: IssueSeverity::Information,
                };
                reports.push(report);
            } else {
                let report = IssueReport {
                    line_number: Some(index + 1),
                    description: "This module includes a file that does not appear to be a snippet.",
                    severity: IssueSeverity::Error,
                };
                reports.push(report);
            }
        }
    }

    reports
}

/// The regex crate provides the byte number for matches in a multi-line search.
/// This function converts the byte number to a line number, which is much more
/// useful to a human. However, this is still WIP and inaccurate.
fn line_from_byte_no(content: &str, byte_no: usize) -> Option<usize> {
    // Debugging messages to help me pinpoint the byte offset
    debug!("Seeking byte: {}", byte_no);
    debug!("File size in bytes: {}", content.bytes().len());
    let mut line_bytes = 0;
    for line in content.lines() {
        line_bytes += line.bytes().len();
    }
    debug!("Lines size in bytes: {}", line_bytes);
    debug!("Number of lines: {}", content.lines().count());

    let mut total_bytes: usize = 0;

    for (line_index, line) in content.lines().enumerate() {
        total_bytes += 1;
        for _byte in line.bytes() {
            total_bytes += 1;
            if total_bytes == byte_no {
                // Line numbers start from 1, but the enumeration starts from 0. Add 1 to the index
                return Some(line_index + 1);
            }
        }
    }

    // TODO: Convert this return value into returing a Result
    // panic!("Cannot locate the line where the issue occurs.");
    None
}