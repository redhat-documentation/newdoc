/// This module provides functionality to validate (lint) existing module and assembly files,
/// to check if the files meet the template structure and other requirements.
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::Path;

use color_eyre::eyre::{eyre, Context, Result};
use regex::{Regex, RegexBuilder};

use crate::module::ContentType;
use crate::REGEX_ERROR;

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
            Self::Information => "🔷 Information",
            Self::Warning => "🔶 Warning",
            Self::Error => "🔴 Error",
        };
        write!(f, "{}", display)
    }
}

impl IssueDefinition {
    /// This function checks a file content for the presence of an issue based on a regex.
    /// These issues are defined using the `IssueDefinition` struct.
    fn check(self, content: &str) -> Vec<IssueReport> {
        if self.multiline {
            let regex = RegexBuilder::new(self.pattern)
                .multi_line(true)
                .build()
                .expect(REGEX_ERROR);
            let findings = regex.find_iter(content);

            findings
                .map(|finding| IssueReport {
                    line_number: line_from_byte_no(content, finding.start()),
                    description: self.description,
                    severity: self.severity,
                })
                .collect()
        // If single-line:
        } else {
            let regex = Regex::new(self.pattern).expect(REGEX_ERROR);
            let findings = content
                .lines()
                .enumerate()
                .map(|(index, line)| (index, regex.find(line)))
                .filter(|(_index, found)| found.is_some());

            findings
                .map(|(index, _finding)| IssueReport {
                    line_number: Some(index),
                    description: self.description,
                    severity: self.severity,
                })
                .collect()
        }
    }
}

#[derive(Debug)]
pub struct IssueReport {
    // Not all issues have a line number
    line_number: Option<usize>,
    description: &'static str,
    severity: IssueSeverity,
}

impl fmt::Display for IssueReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stamp = if let Some(line_number) = self.line_number {
            // Add 1 to the line number because we store the line number as counted from 0,
            // but users want to count the first line as line number 1
            format!("{} at line {}: ", self.severity, line_number + 1)
        } else {
            format!("{}: ", self.severity)
        };
        let display = stamp + self.description;
        write!(f, "{}", display)
    }
}

/// The main validation function. Checks all possible issues in a single file, loaded from a file name.
/// Prints the issues to the standard output.
pub fn validate(file_name: &str) -> Result<()> {
    log::debug!("Validating file `{}`", file_name);

    let path = Path::new(file_name);
    let base_name: &OsStr = path
        .file_name()
        .ok_or_else(|| eyre!("Invalid file name: {:?}", path))?;

    let content =
        fs::read_to_string(path).context(format!("Error reading file `{}`.", file_name))?;

    let mod_type = determine_mod_type(base_name, &content);

    let reports = match mod_type {
        // If the file is an assembly, test the assembly requirements
        ModTypeOrReport::Type(ContentType::Assembly) => assembly::check(&content),
        // If the module type is indeterminate, test the requirements that don't depend on the type
        ModTypeOrReport::Report(type_report) => {
            let mut reports = check_common(&content);
            reports.push(type_report);
            reports
        }
        // In the remaining cases, the file is a module, so test module requirements
        ModTypeOrReport::Type(_) => module::check(&content),
    };

    report_issues(reports, file_name);

    Ok(())
}

/// Print a sorted, human-readable report about the issues found in the file
fn report_issues(mut issues: Vec<IssueReport>, file_path: &str) {
    if issues.is_empty() {
        // If there are no issues in the file, report that as info to avoid confusion over a blank output.
        issues.push(IssueReport {
            line_number: None,
            description: "No issues found in this file.",
            severity: IssueSeverity::Information,
        });
    }

    // Sort the reported issues by their line number
    issues.sort_by_key(|report| report.line_number);

    // Print the sorted reports for the file to the standard output
    println!("💾 File: {}", file_path);
    for issue in issues {
        println!("    {}", issue);
    }
}

/// This enum contains either the module type determined from a file, or an issue report saying
/// that the module type could not be determined.
enum ModTypeOrReport {
    Type(ContentType),
    Report(IssueReport),
}

/// Try to determine the module type of a file, using the file name and the file content.
fn determine_mod_type(base_name: &OsStr, content: &str) -> ModTypeOrReport {
    let mod_patterns = [
        (
            "assembly",
            ":_content-type: ASSEMBLY",
            ContentType::Assembly,
        ),
        ("con", ":_content-type: CONCEPT", ContentType::Concept),
        ("proc", ":_content-type: PROCEDURE", ContentType::Procedure),
        ("ref", ":_content-type: REFERENCE", ContentType::Reference),
    ];

    // Convert the OS file name to a string, replacing characters that aren't valid Unicode
    // with `�` placeholders.
    // OsStr itself is missing the `starts_with` method that we need here.
    let lossy_name = base_name.to_string_lossy();

    for pattern in &mod_patterns {
        if lossy_name.starts_with(pattern.0) || content.contains(pattern.1) {
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
    tests
        .iter()
        .flat_map(|&definition| definition.check(content))
        .collect()
}

/// This function collects all tests required regardless of the module or assembly type
fn check_common(content: &str) -> Vec<IssueReport> {
    let mut reports = Vec::new();

    reports.append(title::check(content).as_mut());
    reports.append(content::check(content).as_mut());
    reports.append(additional_resources::check(content).as_mut());

    reports
}

// This section groups all title requirements
mod title {
    use super::{
        find_first_occurrence, find_mod_id, perform_simple_tests, IssueDefinition, IssueReport,
        IssueSeverity, Regex, REGEX_ERROR,
    };

    const SIMPLE_TITLE_TESTS: [IssueDefinition; 1] = [
        // Test that there are no inline anchors in the title
        IssueDefinition {
            pattern: r"^=\s+.*\[\[\S+\]\].*",
            description: "The title contains an inline anchor.",
            severity: IssueSeverity::Error,
            multiline: false,
        },
    ];

    /// This function collects all tests that target both assembly and module files
    pub fn check(content: &str) -> Vec<IssueReport> {
        let mut reports = Vec::new();

        reports.append(perform_simple_tests(content, &SIMPLE_TITLE_TESTS).as_mut());

        if let Some(title_level_issue) = check_title_level(content) {
            reports.push(title_level_issue);
        }
        if let Some(id_attribute) = check_id_for_attributes(content) {
            reports.push(id_attribute);
        }

        reports
    }

    /// Find the first occurence of any heading in the file.
    /// Returns the line number of the occurence and the line.
    fn find_first_heading(content: &str) -> Option<(usize, &str)> {
        let any_heading_regex = Regex::new(r"^(\.|=+\s+)\S+.*").expect(REGEX_ERROR);

        find_first_occurrence(content, &any_heading_regex)
    }

    /// Check that the first heading found in the file is a title: a level-1, numbered heading
    fn check_title_level(content: &str) -> Option<IssueReport> {
        let title_regex = Regex::new(r"^=\s+\S+.*").expect(REGEX_ERROR);

        if let Some((line_no, heading)) = find_first_heading(content) {
            if let Some(_title) = title_regex.find(heading) {
                log::debug!("This is the title: {:?}", heading);
                None
            } else {
                log::debug!("This is the first heading: {:?}", heading);
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

    /// Detect attributes in module IDs. The only allowed attribute is {context}.
    fn check_id_for_attributes(content: &str) -> Option<IssueReport> {
        let (line_no, mod_id) = match find_mod_id(content) {
            Some(mod_id) => mod_id,
            // TODO: Refactor checking for the presence of ID to a dedicated function;
            // make other ID-related functions depend on it.
            None => {
                return Some(IssueReport {
                    line_number: None,
                    description: "The file is missing an ID.",
                    severity: IssueSeverity::Error,
                });
            }
        };

        let attribute_regex = Regex::new(r"\{((?:[[:alnum:]]|[-_])+)\}").expect(REGEX_ERROR);
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
}

// This section groups all content requirements
mod content {
    use super::{
        find_first_occurrence, find_mod_id, perform_simple_tests, IssueDefinition, IssueReport,
        IssueSeverity, Regex, REGEX_ERROR,
    };

    const SIMPLE_CONTENT_TESTS: [IssueDefinition; 2] = [
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
    ];

    /// This function collects all tests that target both assembly and module files
    pub fn check(content: &str) -> Vec<IssueReport> {
        let mut reports = Vec::new();

        reports.append(perform_simple_tests(content, &SIMPLE_CONTENT_TESTS).as_mut());

        if let Some(abstract_issue) = check_abstract_flag(content) {
            reports.push(abstract_issue);
        }

        reports.append(check_metadata_variable(content).as_mut());

        reports
    }

    /// Check that the module type variable is located before the module ID, as required.
    /// As a side effect, this function also checks that both the varible and the ID are present.
    fn check_metadata_variable(content: &str) -> Vec<IssueReport> {
        let metadata_var_pattern =
            r":_content-type:\s*(?:ASSEMBLY|PROCEDURE|CONCEPT|REFERENCE|SNIPPET)";
        let metadata_var_regex = Regex::new(metadata_var_pattern).expect(REGEX_ERROR);
        let metadata_var = find_first_occurrence(content, &metadata_var_regex);

        let mod_id = find_mod_id(content);

        // Prepare to store the reports about the module
        let mut results: Vec<IssueReport> = Vec::new();

        // Report if the metadata variable is completely missing.
        // A missing ID is already reported elsewhere.
        if metadata_var.is_none() {
            let report = IssueReport {
                line_number: None,
                description: "The module is missing the _content-type attribute.",
                severity: IssueSeverity::Warning,
            };
            results.push(report);
        }

        // If both elements are present, ensure their proper position in relation to each other
        if let (Some(mod_id), Some(metadata_var)) = (mod_id, metadata_var) {
            if mod_id.0 < metadata_var.0 {
                let report = IssueReport {
                    line_number: Some(metadata_var.0),
                    description: "The _content-type attribute is located after the module ID.",
                    severity: IssueSeverity::Error,
                };
                results.push(report);
            }
        }

        results
    }

    /// Check that the abstract flag is followed by a paragraph,
    /// if it exists at all. The abstract flag is not required.
    fn check_abstract_flag(content: &str) -> Option<IssueReport> {
        let abstract_regex = Regex::new(r#"^\[role="_abstract"\]"#).expect(REGEX_ERROR);
        let abstract_flag = find_first_occurrence(content, &abstract_regex);

        // If the file contains an abstract flag, test for the following paragraph
        if let Some((line_no, _line)) = abstract_flag {
            let no_paragraph_report = IssueReport {
                line_number: Some(line_no),
                description: "The _abstract flag is not immediately followed by a paragraph.",
                severity: IssueSeverity::Error,
            };

            // The next line number is the same as the line number for the abstract flag,
            // becase the number from the abstract flag report starts from 1
            // and this counting starts from 0.
            if let Some(next_line) = content.lines().nth(line_no) {
                // This regex is very inclusive for a 'paragraph', but that's intentional.
                // Consider that a paragraph can also start with an attribute, or with a role, such as:
                // ⁠[systemitem]`firewalld` can be used to (...)
                // Let's just check that the line starts with a non-whitespace character
                // and that a letter appears at least somewhere.
                let paragraph_regex = Regex::new(r"^\S+[[:alpha:]].*").expect(REGEX_ERROR);
                // If a line follows the flag but it doesn't appear as a paragraph, report the issue
                if paragraph_regex.find(next_line).is_none() {
                    log::debug!("The non-paragraph-line: {:?}", next_line);
                    Some(no_paragraph_report)
                } else {
                    None
                }
            // If no line follows the flag, also report the issue
            } else {
                log::debug!("No lines after the abstract.");
                Some(no_paragraph_report)
            }
        // If the file contains no abstract tag, report no error. This is supported format.
        } else {
            None
        }
    }
}

// This section groups all module requirements;
// they depend on title and content, and additional resources requirements
mod module {
    use super::{
        check_common, perform_simple_tests, IssueDefinition, IssueReport, IssueSeverity, Regex,
        REGEX_ERROR,
    };

    const SIMPLE_MODULE_TESTS: [IssueDefinition; 2] = [
        // Ensure the correct syntax for Additional resources
        IssueDefinition {
            pattern: r"^==\s*Additional resources\s*$",
            description: "In modules, 'Additional resources' must use the dot syntax.",
            severity: IssueSeverity::Error,
            multiline: false,
        },
        IssueDefinition {
            pattern: r"^={2,}\s+\S.*",
            description: "This heading is level-2 or greater. Be conscious of the heading level.",
            severity: IssueSeverity::Warning,
            multiline: false,
        },
    ];

    /// This function collects all tests required in module files
    pub fn check(content: &str) -> Vec<IssueReport> {
        let mut reports = Vec::new();

        reports.append(check_common(content).as_mut());
        reports.append(perform_simple_tests(content, &SIMPLE_MODULE_TESTS).as_mut());
        reports.append(check_include_except_snip(content).as_mut());

        reports
    }

    /// Test that modules include no other modules, except for snippets
    fn check_include_except_snip(content: &str) -> Vec<IssueReport> {
        let any_include_pattern = r"^include::.*\.adoc";
        let any_include_regex = Regex::new(any_include_pattern).expect(REGEX_ERROR);

        let snip_include_pattern = r"^include::((snip|.*/snip)[_-]|common-content/).*\.adoc";
        let snip_include_regex = Regex::new(snip_include_pattern).expect(REGEX_ERROR);

        let mut reports: Vec<IssueReport> = Vec::new();

        for (index, line) in content.lines().enumerate() {
            if let Some(include) = any_include_regex.find(line) {
                if let Some(_snippet) = snip_include_regex.find(include.as_str()) {
                    // In this case, the detected include is most likely a snippet. Report as Information
                    let report = IssueReport {
                    line_number: Some(index),
                    description: "This module includes a file that appears to be a snippet. This is supported.",
                    severity: IssueSeverity::Information,
                };
                    reports.push(report);
                } else {
                    let report = IssueReport {
                        line_number: Some(index),
                        description:
                            "This module includes a file that does not appear to be a snippet.",
                        severity: IssueSeverity::Error,
                    };
                    reports.push(report);
                }
            }
        }

        reports
    }
}

// This section groups all assembly requirements;
// they depend on title and content, and additional resources requirements
mod assembly {
    use super::{
        check_common, perform_simple_tests, IssueDefinition, IssueReport, IssueSeverity, Regex,
        REGEX_ERROR,
    };

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
            pattern: r"^\.Additional resources\s*$",
            description: "In assemblies, 'Additional resources' must use the == syntax.",
            severity: IssueSeverity::Error,
            multiline: false,
        },
    ];

    /// This function collects all tests required in assembly files
    pub fn check(content: &str) -> Vec<IssueReport> {
        // check_no_nesting(base_name, content);
        // check_supported_leveloffset(base_name, content);
        let mut reports = Vec::new();

        reports.append(check_common(content).as_mut());
        reports.append(perform_simple_tests(content, &SIMPLE_ASSEMBLY_TESTS).as_mut());
        reports.append(check_headings_in_assembly(content).as_mut());

        reports
    }

    /// Check for numbered headings of level 2 or greater in assemblies.
    /// This is more complicated than the same check in modules because the assembly template
    /// includes two such headings, which should _not_ be reported:
    /// * == Prerequisites
    /// * == Additional resources
    /// In addition, let's also assume that the legacy 'Related information' heading is fine. (TODO: Make sure.)
    fn check_headings_in_assembly(content: &str) -> Vec<IssueReport> {
        let heading_regex = Regex::new(r"^={2,}\s+\S.*").expect(REGEX_ERROR);
        let standard_headings_pattern =
            r"^==\s+(?:Prerequisites|Additional resources|Related information)";
        let standard_headings_regex = Regex::new(standard_headings_pattern).expect(REGEX_ERROR);

        let mut lines_with_heading: Vec<usize> = Vec::new();

        content.lines().enumerate().for_each(|(index, line)| {
            if let Some(_heading) = heading_regex.find(line) {
                if let Some(_standard_heading) = standard_headings_regex.find(line) {
                    // This line is a standard heading. No report.
                } else {
                    // This is a non-standard heading. Record the line number for a report.
                    lines_with_heading.push(index);
                }
            }
        });

        let reports = lines_with_heading
            .iter()
            .map(|line_no| IssueReport {
                line_number: Some(*line_no),
                description:
                    "This heading is level-2 or greater. Be conscious of the heading level.",
                severity: IssueSeverity::Warning,
            })
            .collect();

        reports
    }
}

mod additional_resources {
    use super::{
        find_first_occurrence, perform_simple_tests, IssueDefinition, IssueReport, IssueSeverity,
        Regex, REGEX_ERROR,
    };

    const SIMPLE_ADDITIONAL_RESOURCES_TESTS: [IssueDefinition; 0] = [
        // No simple tests at this point.
    ];

    /// Perform all available tests on the Additional resources section
    pub fn check(content: &str) -> Vec<IssueReport> {
        let heading = find_additional_resources(content);
        let mut issues = Vec::new();

        issues.append(perform_simple_tests(content, &SIMPLE_ADDITIONAL_RESOURCES_TESTS).as_mut());

        // Perform the tests only if the file actually has an additional resources heading.
        // If it doesn't, skip the tests.
        if let Some((index, _line)) = heading {
            // Prepare the lines vector in advance so that the following functions
            // don't have to split the files again on their own.
            let lines: Vec<&str> = content.lines().collect();

            // Collect the issues found by the particular functions.
            if let Some(issue) = check_add_res_flag(&lines, index) {
                issues.push(issue);
            }
            issues.append(check_paragraphs_in_add_res(&lines, index).as_mut());
            issues.append(check_link_labels_in_add_res(&lines, index).as_mut());
            issues.append(check_additional_resource_length(&lines, index).as_mut());
        }

        issues
    }

    /// Find the first heading that matches additional resources.
    /// This does not distinguish between the module and assembly format -- the simple tests check that.
    fn find_additional_resources(content: &str) -> Option<(usize, &str)> {
        let add_res_regex = Regex::new(
            r"^(?:==\s+|\.)(?:Additional resources|Related information|Additional information)\s*$",
        )
        .expect(REGEX_ERROR);

        find_first_occurrence(content, &add_res_regex)
    }

    /// See if the additional resources heading is missing the additional resources flag,
    /// or the flag is further away than the one preceding line.
    fn check_add_res_flag(lines: &[&str], heading_index: usize) -> Option<IssueReport> {
        let add_res_flag = r#"[role="_additional-resources"]"#;

        // If the line before the heading is the required flag, report no issue.
        if lines[heading_index - 1] == add_res_flag {
            None
        // If the preceding line is anything else than the flag, report the missing flag.
        } else {
            Some(IssueReport {
            line_number: Some(heading_index),
            description: "The additional resources heading is not immediately preceded by the _additional-resources flag.",
            severity: IssueSeverity::Error,
        })
        }
    }

    /// Check that the additional resources section is composed of list items, possibly with some ifdefs.
    fn check_paragraphs_in_add_res(lines: &[&str], heading_index: usize) -> Vec<IssueReport> {
        // This regex matches either a plain list item, or one that's embedded in an inline ifdef.
        let bullet_point_regex =
            Regex::new(r"(?:^\*+\s+\S+|^ifdef::\S+\[\*+\s+\S+.*\])").expect(REGEX_ERROR);
        // A paragraph that isn't a list item is allowed if it's an ifdef or a comment.
        let allowed_paragraph =
            Regex::new(r"^(?:ifdef::\S+\[.*]|endif::\[\]|//)").expect(REGEX_ERROR);
        // Let's try to use a loose definition of an empty paragraph as a whitespace paragraph.
        let empty_line_regex = Regex::new(r"^\s*$").expect(REGEX_ERROR);

        let mut issues = Vec::new();

        for (offset, &line) in lines[heading_index + 1..].iter().enumerate() {
            // If we find the first real list item, let's consider this a valid additional resources
            // section and return the issues up to this point.
            if bullet_point_regex.is_match(line) {
                return issues;
            // Report empty lines found before the first list item.
            } else if empty_line_regex.is_match(line) {
                issues.push(IssueReport {
                    // Add 1 because the offset starts counting the first line that follows the heading from 0
                    line_number: Some(heading_index + offset + 1),
                    description: "The additional resources section includes an empty line.",
                    severity: IssueSeverity::Error,
                });
            // Report unallowed paragraphs before the first list item.
            } else if !allowed_paragraph.is_match(line) {
                issues.push(IssueReport {
                    // Add 1 because the offset starts counting the first line that follows the heading from 0
                    line_number: Some(heading_index + offset + 1),
                    description: "The additional resources section includes a plain paragraph.",
                    severity: IssueSeverity::Error,
                });
            }
        }

        // If no list items have appeared until the end of the file, report that as the final issue.
        issues.push(IssueReport {
            line_number: Some(heading_index),
            description: "The additional resources section includes no list items.",
            severity: IssueSeverity::Error,
        });

        issues
    }

    /// Detect links with no labels after a certain point in the file,
    /// specifically after the additional resources heading.
    fn check_link_labels_in_add_res(lines: &[&str], heading_index: usize) -> Vec<IssueReport> {
        let link_regex = Regex::new(r"link:\S+\[]").expect(REGEX_ERROR);

        let mut issues = Vec::new();

        for (offset, &line) in lines[heading_index + 1..].iter().enumerate() {
            if link_regex.is_match(line) {
                issues.push(IssueReport {
                    line_number: Some(heading_index + offset + 1),
                    description:
                        "The additional resources section includes a link without a label.",
                    severity: IssueSeverity::Error,
                });
            }
        }

        issues
    }

    /// Check that the items in the additional resources section aren't too long, measured in words.
    fn check_additional_resource_length(lines: &[&str], heading_index: usize) -> Vec<IssueReport> {
        // This regex features capture groups to extract the content of the list item.
        let bullet_point_regex =
            Regex::new(r"^(?:\*+\s+(\S+.*)|ifdef::\S+\[\*+\s+(\S+.*)\])").expect(REGEX_ERROR);
        // This is the number of words you need to write:
        // * The `program(1)` man page
        // Let's use that as the approximate upper limit.
        let maximum_words = 4;

        let mut issues = Vec::new();

        for (offset, &line) in lines[heading_index + 1..].iter().enumerate() {
            if let Some(captures) = bullet_point_regex.captures(line) {
                let list_item_text = captures
                    .get(1)
                    .expect("Failed to extract text from a list item. This is a bug.");
                // Counting words by splitting by white space is a crude measurement, but it should be
                // close enough. The important thing is that it doesn't count long links as many words.
                let number_of_words = list_item_text.as_str().split_whitespace().count();
                log::debug!("Words in additional resources: {}", number_of_words);

                if number_of_words > maximum_words {
                    issues.push(IssueReport {
                    line_number: Some(heading_index + offset + 1),
                    description:
                        "The additional resource is long. Try to limit it to a couple of words.",
                    severity: IssueSeverity::Warning,
                });
                }
            }
        }

        issues
    }
}

/// Find the first occurence of an ID definition in the file.
/// Returns the line number of the occurence and the line.
fn find_mod_id(content: &str) -> Option<(usize, &str)> {
    let id_regex = Regex::new(r"^\[id=\S+\]").expect(REGEX_ERROR);

    find_first_occurrence(content, &id_regex)
}

/// Search for a predefined regex in a file. If found, return the line number and the line text.
fn find_first_occurrence<'a>(content: &'a str, regex: &Regex) -> Option<(usize, &'a str)> {
    for (index, line) in content.lines().enumerate() {
        if let Some(_occurrence) = regex.find(line) {
            return Some((index, line));
        }
    }
    None
}

/// The regex crate provides the byte number for matches in a multi-line search.
/// This function converts the byte number to a line number, which is much more
/// useful to a human. However, this is still WIP and inaccurate.
fn line_from_byte_no(content: &str, byte_no: usize) -> Option<usize> {
    // Debugging messages to help me pinpoint the byte offset
    log::debug!("Seeking byte: {}", byte_no);
    log::debug!("File size in bytes: {}", content.bytes().len());
    let mut line_bytes = 0;
    for line in content.lines() {
        line_bytes += line.bytes().len();
    }
    log::debug!("Lines size in bytes: {}", line_bytes);
    log::debug!("Number of lines: {}", content.lines().count());

    let mut total_bytes: usize = 0;

    for (line_index, line) in content.lines().enumerate() {
        total_bytes += 1;
        for _byte in line.bytes() {
            total_bytes += 1;
            if total_bytes == byte_no {
                return Some(line_index);
            }
        }
    }

    None
}
