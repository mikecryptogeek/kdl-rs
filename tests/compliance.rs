use std::{
    fs,
    path::{Path, PathBuf},
};

use kdl::{KdlDocument, KdlError, KdlValue};
use miette::IntoDiagnostic;

#[test]
fn spec_compliance() -> miette::Result<()> {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_cases")
        .join("input");
    for test_name in fs::read_dir(&input).into_diagnostic()? {
        let test_path = test_name.into_diagnostic()?.path();
        println!(
            "parsing {}:",
            PathBuf::from(test_path.file_name().unwrap()).display()
        );
        let src = fs::read_to_string(&test_path).into_diagnostic()?;
        println!("src: {}", src);
        let res: Result<KdlDocument, KdlError> = src.parse();
        validate_res(res, &test_path)?;
    }
    Ok(())
}

fn validate_res(res: Result<KdlDocument, KdlError>, path: &Path) -> miette::Result<()> {
    let file_name = path.file_name().unwrap();
    let expected_path = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("expected_kdl")
        .join(file_name);
    if expected_path.exists() {
        let doc = res?;
        let expected = fs::read_to_string(&expected_path).into_diagnostic()?;
        println!("expected: {}", expected);
        let stringified = stringify_to_expected(doc);
        println!("stringified: {}", stringified);
        assert_eq!(stringified, expected);
    } else {
        assert!(res.is_err(), "parse should not have succeeded");
    }
    Ok(())
}

fn stringify_to_expected(mut doc: KdlDocument) -> String {
    doc.fmt_no_comments();
    normalize_numbers(&mut doc);
    normalize_strings(&mut doc);
    remove_empty_children(&mut doc);
    doc.to_string()
}

fn normalize_numbers(doc: &mut KdlDocument) {
    for node in doc.nodes_mut() {
        for entry in node.entries_mut() {
            if let Some(value) = entry.value().as_i64() {
                *entry.value_mut() = KdlValue::Base10(value);
            }
        }
        if let Some(children) = node.children_mut() {
            normalize_numbers(children);
        }
    }
}

fn normalize_strings(doc: &mut KdlDocument) {
    for node in doc.nodes_mut() {
        for entry in node.entries_mut() {
            if let Some(value) = entry.value().as_string() {
                *entry.value_mut() = KdlValue::String(value.to_string());
            }
        }
        if let Some(children) = node.children_mut() {
            normalize_strings(children);
        }
    }
}

fn remove_empty_children(doc: &mut KdlDocument) {
    for node in doc.nodes_mut() {
        let maybe_children = node.children_mut();
        if maybe_children.is_some() && maybe_children.as_ref().unwrap().nodes().is_empty() {
            *maybe_children = None;
        }
        if let Some(children) = maybe_children {
            remove_empty_children(children);
        }
    }
}
