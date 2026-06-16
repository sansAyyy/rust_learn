mod common;

use pretty_assertions::assert_eq;
use tempfile::tempdir;
use testing_debugging::{calculator, is_palindrome, parser, store};

#[test]
fn calculator_and_parser_work_together() {
    let numbers = parser::parse_csv_numbers("10,20,30").expect("csv should parse");
    let average = calculator::average(&numbers);

    assert_eq!(numbers, common::sample_numbers());
    assert_eq!(average, Some(20.0));
}

#[test]
fn public_palindrome_api_is_available() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
}

#[test]
fn store_writes_files_in_temp_dir() -> std::io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("note.md");

    store::save_note(&path, "Integration", common::sample_note_body())?;
    let content = store::load_note(&path)?;

    assert!(content.contains("Integration"));
    assert!(content.contains(common::sample_note_body()));
    Ok(())
}
