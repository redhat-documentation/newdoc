//! These are integration tests. They let the top-level functions generate
//! each module type and then they compare the generated content with a pre-generated specimen
//! to check that we introduce no changes unknowingly.

use std::path::PathBuf;

use newdoc::*;

// These values represent the default newdoc options.
fn basic_options() -> Options {
    Options {
        comments: true,
        file_prefixes: true,
        anchor_prefixes: false,
        examples: true,
        target_dir: PathBuf::from("."),
        verbosity: Verbosity::Default,
    }
}

/// Test that we generate the assembly that we expect.
#[test]
fn test_assembly() {
    let mod_type = ContentType::Assembly;
    let mod_title = "Testing that an assembly forms properly";
    let options = basic_options();
    let assembly = Module::new(mod_type, mod_title, &options);

    let pre_generated =
        include_str!("./generated/assembly_testing-that-an-assembly-forms-properly.adoc");

    assert_eq!(assembly.text, pre_generated);
}

/// Test that we generate the concept module that we expect.
#[test]
fn test_concept_module() {
    let mod_type = ContentType::Concept;
    let mod_title = "A title that tests a concept";
    let options = basic_options();
    let concept = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/con_a-title-that-tests-a-concept.adoc");

    assert_eq!(concept.text, pre_generated);
}

/// Test that we generate the procedure module that we expect.
#[test]
fn test_procedure_module() {
    let mod_type = ContentType::Procedure;
    let mod_title = "Testing a procedure";
    let options = basic_options();
    let procedure = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/proc_testing-a-procedure.adoc");

    assert_eq!(procedure.text, pre_generated);
}

/// Test that we generate the reference module that we expect.
#[test]
fn test_reference_module() {
    let mod_type = ContentType::Reference;
    let mod_title = "The lines in a reference module";
    let options = basic_options();
    let reference = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/ref_the-lines-in-a-reference-module.adoc");

    assert_eq!(reference.text, pre_generated);
}

/// Test that we generate the snippet file that we expect.
#[test]
fn test_snippet_file() {
    let mod_type = ContentType::Snippet;
    let mod_title = "Some notes in a snippet file";
    let options = basic_options();
    let snippet = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/snip_some-notes-in-a-snippet-file.adoc");

    assert_eq!(snippet.text, pre_generated);
}

// These values strip down the modules to the bare minimum.
fn minimal_options() -> Options {
    Options {
        comments: false,
        file_prefixes: true,
        anchor_prefixes: false,
        examples: false,
        target_dir: PathBuf::from("."),
        verbosity: Verbosity::Default,
    }
}

/// Test that we generate the assembly that we expect.
#[test]
fn test_minimal_assembly() {
    let mod_type = ContentType::Assembly;
    let mod_title = "Minimal assembly";
    let options = minimal_options();
    let assembly = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/minimal-assembly.adoc");

    assert_eq!(assembly.text, pre_generated);
}

/// Test that we generate the concept module that we expect.
#[test]
fn test_minimal_concept() {
    let mod_type = ContentType::Concept;
    let mod_title = "Minimal concept";
    let options = minimal_options();
    let concept = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/minimal-concept.adoc");

    assert_eq!(concept.text, pre_generated);
}

/// Test that we generate the procedure module that we expect.
#[test]
fn test_minimal_procedure() {
    let mod_type = ContentType::Procedure;
    let mod_title = "Minimal procedure";
    let options = minimal_options();
    let procedure = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/minimal-procedure.adoc");

    assert_eq!(procedure.text, pre_generated);
}

/// Test that we generate the reference module that we expect.
#[test]
fn test_minimal_reference() {
    let mod_type = ContentType::Reference;
    let mod_title = "Minimal reference";
    let options = minimal_options();
    let reference = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/minimal-reference.adoc");

    assert_eq!(reference.text, pre_generated);
}

/// Test that we generate the snippet file that we expect.
#[test]
fn test_minimal_snippet() {
    let mod_type = ContentType::Snippet;
    let mod_title = "Minimal snippet";
    let options = minimal_options();
    let snippet = Module::new(mod_type, mod_title, &options);

    let pre_generated = include_str!("./generated/minimal-snippet.adoc");

    assert_eq!(snippet.text, pre_generated);
}
