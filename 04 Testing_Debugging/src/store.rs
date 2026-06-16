use std::fs;
use std::io;
use std::path::Path;

pub fn save_note(path: &Path, title: &str, body: &str) -> io::Result<()> {
    let content = format!("# {title}\n\n{body}\n");
    fs::write(path, content)
}

pub fn load_note(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn saves_and_loads_note() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("note.md");

        save_note(&path, "Test", "hello")?;
        let content = load_note(&path)?;

        assert!(content.contains("# Test"));
        assert!(content.contains("hello"));
        Ok(())
    }
}
