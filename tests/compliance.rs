use std::{
    fs,
    path::{Path, PathBuf},
};

use kdl::{KdlDocument, KdlError};
use miette::IntoDiagnostic;

#[test]
fn spec_compliance() -> miette::Result<()> {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_cases")
        .join("input");
    for test_name in fs::read_dir(&input).into_diagnostic()? {
        let test_path = test_name.into_diagnostic()?.path();
        let res: Result<KdlDocument, KdlError> =
            fs::read_to_string(&test_path).into_diagnostic()?.parse();
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
        let stringified = stringify_to_expected(doc);
        let expected = fs::read_to_string(&expected_path).into_diagnostic()?;
        println!("{}", stringified);
        assert_eq!(stringified, format!("{}\n", expected.trim()));
    } else {
        assert!(res.is_err());
    }
    Ok(())
}

fn stringify_to_expected(mut doc: KdlDocument) -> String {
    doc.fmt();
    doc.to_string()
}
