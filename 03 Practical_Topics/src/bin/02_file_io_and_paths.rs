use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    // PathBuf 适合构造和修改路径；&Path 适合只读借用路径。
    let mut dir = std::env::temp_dir();
    dir.push("rust_learn_practical");
    fs::create_dir_all(&dir).with_context(|| format!("create directory {}", dir.display()))?;

    let mut file_path = PathBuf::from(&dir);
    file_path.push("notes.txt");

    write_notes(&file_path, &["hello", "file io", "rust"])?;
    let lines = read_lines(&file_path)?;
    println!("read lines = {:?}", lines);

    let metadata = fs::metadata(&file_path)?;
    println!("file size = {} bytes", metadata.len());

    let copy_path = dir.join("notes-copy.txt");
    fs::copy(&file_path, &copy_path)
        .with_context(|| format!("copy {} to {}", file_path.display(), copy_path.display()))?;
    println!("copied to {}", copy_path.display());

    Ok(())
}

fn write_notes(path: &PathBuf, lines: &[&str]) -> Result<()> {
    let mut file =
        fs::File::create(path).with_context(|| format!("create file {}", path.display()))?;

    for line in lines {
        writeln!(file, "{line}")?;
    }

    Ok(())
}

fn read_lines(path: &PathBuf) -> Result<Vec<String>> {
    let file = fs::File::open(path).with_context(|| format!("open file {}", path.display()))?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("read lines from {}", path.display()))
}
