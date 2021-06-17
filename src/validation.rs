/// This module provides functionality to validate (lint) existing module and assembly files,
/// to check if the files meet the template structure and other requirements.

use std::fmt;
use std::fs;
use std::path::Path;
// use std::process::exit;
use log::{debug, error};
use regex::{Regex, RegexBuilder};

use crate::module::ModuleType;

const SIMPLE_ASSEMBLY_TESTS: [IssueDefinition; 3] = [
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

const SIMPLE_MODULE_TESTS: [IssueDefinition; 1] = [
    // Ensure the correct syntax for Additional resources
    IssueDefinition {
        pattern: r"^==\s*Additional resources",
        description: "In modules, 'Additional resources' must use the dot syntax.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
];

const SIMPLE_TITLE_TESTS: [IssueDefinition; 2] = [
    // Test that there are no inline anchors in the title
    IssueDefinition {
        pattern: r"^=\s+.*\[\[\S+\]\].*",
        description: "The title contains an inline anchor.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
    // Test that titles contain no attributes (variables)
    IssueDefinition {
        pattern: r"^=\s+.*\{\S+\}.*",
        description: "The title contains an attribute.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
];

const SIMPLE_CONTENT_TESTS: [IssueDefinition; 3] = [
    IssueDefinition {
        pattern: r"<[[:alpha:]]+>.*</[[:alpha:]]+>",
        description: "The file seems to contain HTML markup",
        severity: IssueSeverity::Error,
        multiline: false,
    },
    IssueDefinition {
        pattern: r"(?:xref:\S+\[\]|<<\S+>>|<<\S+,.+>>)",
        description: "The file contains an unsupported cross-reference.",
        severity: IssueSeverity::Error,
        multiline: false,
    },
    IssueDefinition {
        pattern: r"^={2,}\s+\S+",
        description: "This heading is level-2 or greater. Be conscious of the heading level.",
        severity: IssueSeverity::Warning,
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

    let reports = match mod_type {
        ModTypeOrReport::Type(ModuleType::Assembly) => test_assemblies(base_name, &content),
        ModTypeOrReport::Report(r) => vec![r],
        _ => test_modules(base_name, &content),
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

/// This enum contains either the module type determined from a file, or an issue report saying
/// that the module type could not be determined.
enum ModTypeOrReport {
    Type(ModuleType),
    Report(IssueReport),
}

/// Try to determine the module type of a file, using the file name and the file content.
fn determine_mod_type(base_name: &str, content: &str) -> ModTypeOrReport {
    if base_name.starts_with("assembly_") || base_name.starts_with("assembly-") {
        return ModTypeOrReport::Type(ModuleType::Assembly);
    }
    let mod_patterns = [
        ("con", ":_module-type: CONCEPT", ModuleType::Concept),
        ("proc", ":_module-type: PROCEDURE", ModuleType::Procedure),
        ("ref", ":_module-type: REFERENCE", ModuleType::Reference),
    ];
    for pattern in mod_patterns.iter() {
        if base_name.starts_with(pattern.0) || content.contains(pattern.1) {
            return ModTypeOrReport::Type(pattern.2);
        }
    }
    let report = IssueReport {
        line_number: None,
        description: "Cannot determine the module type.",
        severity: IssueSeverity::Error,
    };
    ModTypeOrReport::Report(report)
}

/// Run all tests defined in an array on a file content
fn perform_simple_tests(content: &str, tests: &[IssueDefinition]) -> Vec<IssueReport> {
    tests.iter()
        .map(|&definition| check_for_simple_issue(definition, content))
        .flatten()
        .collect()
}

/// This function collects all tests that target only assembly files
fn test_assemblies(_base_name: &str, content: &str) -> Vec<IssueReport> {
    // check_no_nesting(base_name, content);
    // check_supported_leveloffset(base_name, content);
    let mut reports = perform_simple_tests(content, &SIMPLE_ASSEMBLY_TESTS);

    reports.append(perform_simple_tests(content, &SIMPLE_TITLE_TESTS).as_mut());
    reports.append(perform_simple_tests(content, &SIMPLE_CONTENT_TESTS).as_mut());

    if let Some(title_level_issue) = check_title_level(content) {
        reports.push(title_level_issue);
    }
    if let Some(id_attribute) = check_id_for_attributes(content) {
        reports.push(id_attribute);
    }
    if let Some(abstract_issue) = check_abstract_flag(content) {
        reports.push(abstract_issue);
    }
    if let Some(experimental_issue) = check_experimental_flag(content) {
        reports.push(experimental_issue);
    }
    
    reports
}

/// This function collects all tests that target all module files
fn test_modules(_base_name: &str, content: &str) -> Vec<IssueReport> {
    let mut reports = perform_simple_tests(content, &SIMPLE_MODULE_TESTS);
    
    reports.append(perform_simple_tests(content, &SIMPLE_TITLE_TESTS).as_mut());
    reports.append(perform_simple_tests(content, &SIMPLE_CONTENT_TESTS).as_mut());
    
    reports.append(check_metadata_variable(content).as_mut());
    reports.append(check_include_except_snip(content).as_mut());

    if let Some(title_level_issue) = check_title_level(content) {
        reports.push(title_level_issue);
    }
    if let Some(id_attribute) = check_id_for_attributes(content) {
        reports.push(id_attribute);
    }
    if let Some(abstract_issue) = check_abstract_flag(content) {
        reports.push(abstract_issue);
    }
    if let Some(experimental_issue) = check_experimental_flag(content) {
        reports.push(experimental_issue);
    }
    
    reports
}

/// This function checks a file content for the presence of an issue based on a regex.
/// These issues are defined using the IssueDefinition struct.
fn check_for_simple_issue(issue: IssueDefinition, content: &str) -> Vec<IssueReport> {
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
fn check_metadata_variable(content: &str) -> Vec<IssueReport> {
    let metadata_var_pattern = r":_module-type:\s*(?:PROCEDURE|CONCEPT|REFERENCE)";
    let metadata_var_regex = Regex::new(metadata_var_pattern).unwrap();
    let metadata_var = find_first_occurrence(content, metadata_var_regex);

    let mod_id = find_mod_id(content);

    // Prepare to store the reports about the module
    let mut results: Vec<IssueReport> = Vec::new();

    // Report if the metadata variable is completely missing.
    // A missing ID is already reported elsewhere.
    if metadata_var.is_none() {
        let report = IssueReport {
            line_number: None,
            description: "The module is missing the module type variable.",
            severity: IssueSeverity::Warning,
        };
        results.push(report);
    }

    // If both elements are present, ensure their proper position in relation to each other
    if let (Some(mod_id), Some(metadata_var)) = (mod_id, metadata_var) {
        if mod_id.0 < metadata_var.0 {
            let report = IssueReport {
                line_number: Some(metadata_var.0),
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

/// Check that the first heading found in the file is a title: a level-1, numbered heading
fn check_title_level(content: &str) -> Option<IssueReport> {
    let title_regex = Regex::new(r"^=\s+\S+.*").unwrap();

    if let Some((line_no, heading)) = find_first_heading(content) {
        if let Some(_title) = title_regex.find(heading) {
            debug!("This is the title: {:?}", heading);
            None
        } else {
            debug!("This is the first heading: {:?}", heading);
            Some(IssueReport {
                line_number: Some(line_no),
                description: "The first heading in the file is not level 1.",
                severity: IssueSeverity::Error,
            })
        }
    } else {
        Some(IssueReport {
            line_number: None,
            description: "The file has no title or headings.",
            severity: IssueSeverity::Error,
        })
    }
}

/// Find the first occurence of any heading in the file.
/// Returns the line number of the occurence and the line.
fn find_first_heading(content: &str) -> Option<(usize, &str)> {
    let any_heading_regex = Regex::new(r"^(\.|=+\s+)\S+.*").unwrap();

    find_first_occurrence(content, any_heading_regex)
}

/// Find the first occurence of an ID definition in the file.
/// Returns the line number of the occurence and the line.
fn find_mod_id(content: &str) -> Option<(usize, &str)> {
    let id_regex = Regex::new(r"^\[id=\S+\]").unwrap();

    find_first_occurrence(content, id_regex)
}

/// Search for a predefined regex in a file. If found, return the line number and the line text.
fn find_first_occurrence(content: &str, regex: Regex) -> Option<(usize, &str)> {
    for (index, line) in content.lines().enumerate() {
        if let Some(_occurrence) = regex.find(line) {
            return Some((index + 1, line));
        }
    }
    None
}

/// Detect attributes in module IDs. The only allowed attribute is {context}.
fn check_id_for_attributes(content: &str) -> Option<IssueReport> {
    let (line_no, mod_id) = match find_mod_id(content) {
        Some(mod_id) => mod_id,
        // TODO: Refactor checking for teh presence of ID to a dedicated function;
        // make other ID-related functions depend on it.
        None => {
            return Some(IssueReport {
                line_number: None,
                description: "The file is missing an ID.",
                severity: IssueSeverity::Error,
            });
        }
    };

    let attribute_regex = Regex::new(r"\{((?:[[:alnum:]]|[-_])+)\}").unwrap();
    let attribute = attribute_regex.captures(mod_id)?;

    if attribute.get(1).unwrap().as_str() == "context" {
        // The context attribute is allowed
        None
    } else {
        Some(IssueReport {
            line_number: Some(line_no),
            description: "The ID includes an attribute.",
            severity: IssueSeverity::Error,
        })
    }
}

/// Check that the abstract flag exists in the file and that it's followed by a paragraph.
fn check_abstract_flag(content: &str) -> Option<IssueReport> {
    let abstract_regex = Regex::new(r#"^\[role="_abstract"\]"#).unwrap();
    let abstract_flag = find_first_occurrence(content, abstract_regex);

    // If the file contains an abstract flag, test for the following paragraph
    if let Some((line_no, _line)) = abstract_flag {
        let no_paragraph_report = IssueReport {
            line_number: Some(line_no + 1),
            description: "The abstract flag is not immediately followed by a paragraph.",
            severity: IssueSeverity::Error,
        };

        // The next line number is the same as the line number for the abstract flag,
        // becase the number from the abstract flag report starts from 1
        // and this counting starts from 0.
        if let Some(next_line) = content.lines().nth(line_no) {
            let paragraph_regex = Regex::new(r"^(?:[[:alnum:]]|\{).+").unwrap();
            // If a line follows the flag but it doesn't appear as a paragraph, report the issue
            if paragraph_regex.find(next_line).is_none() {
                debug!("The non-paragraph-line: {:?}", next_line);
                Some(no_paragraph_report)
            } else {
                None
            }
        // If no line follows the flag, also report the issue
        } else {
            debug!("No lines after the abstract.");
            Some(no_paragraph_report)
        }
    } else {
        Some(IssueReport {
            line_number: None,
            description: "The file is missing the abstract flag.",
            severity: IssueSeverity::Error,
        })
    }
}

/// Check that if the file uses any UI macros, it also contains the :experimental: attribute
fn check_experimental_flag(content: &str) -> Option<IssueReport> {
    let ui_macros_regex = Regex::new(r"(?:btn:\[.+\]|menu:\S+\[.+\]|kbd:\[.+\])").unwrap();
    let experimental_regex = Regex::new(r"^:experimental:").unwrap();

    if let Some((line_no, _line)) = find_first_occurrence(content, ui_macros_regex) {
        if let Some(_experimental) = experimental_regex.find(content) {
            // This is fine. The file has both a UI macro and the experimental attribute.
            None
        } else {
            Some(IssueReport {
                line_number: Some(line_no),
                description: "The file uses a UI macro but the `:experimental:` attribute is missing.",
                severity: IssueSeverity::Error,
            })
        }
    } else {
        // No UI macro found, means no issue
        None
    }
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