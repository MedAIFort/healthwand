use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum FileSourceError {
    Io(io::Error),
    NotTextFile(PathBuf),
}

impl std::fmt::Display for FileSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSourceError::Io(e) => write!(f, "IO error: {}", e),
            FileSourceError::NotTextFile(p) => write!(f, "Not a text file: {}", p.display()),
        }
    }
}

impl std::error::Error for FileSourceError {}

impl From<io::Error> for FileSourceError {
    fn from(e: io::Error) -> Self {
        FileSourceError::Io(e)
    }
}

pub trait FileSource {
    fn files(&self) -> Result<Vec<PathBuf>, FileSourceError>;
    fn read_file(&self, path: &Path) -> Result<String, FileSourceError>;
}

pub struct LocalFileSource {
    pub root: PathBuf,
    pub allowed_extensions: Vec<String>,
}

impl LocalFileSource {
    pub fn new<P: Into<PathBuf>>(root: P, allowed_extensions: Vec<String>) -> Self {
        Self {
            root: root.into(),
            allowed_extensions,
        }
    }

    fn is_text_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            self.allowed_extensions.iter().any(|x| x.eq_ignore_ascii_case(ext))
        } else {
            false
        }
    }

    fn collect_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), FileSourceError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.collect_files_recursive(&path, files)?;
            } else if self.is_text_file(&path) {
                files.push(path);
            }
        }
        Ok(())
    }
}

impl FileSource for LocalFileSource {
    fn files(&self) -> Result<Vec<PathBuf>, FileSourceError> {
        let mut files = Vec::new();
        let meta = fs::metadata(&self.root)?;
        if meta.is_dir() {
            self.collect_files_recursive(&self.root, &mut files)?;
        } else if meta.is_file() && self.is_text_file(&self.root) {
            files.push(self.root.clone());
        }
        Ok(files)
    }

    fn read_file(&self, path: &Path) -> Result<String, FileSourceError> {
        if !self.is_text_file(path) {
            return Err(FileSourceError::NotTextFile(path.to_path_buf()));
        }
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_file_traversal_and_reading() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let fs = LocalFileSource::new(dir.path(), vec!["txt".to_string()]);
        let files = fs.files().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].extension().unwrap(), "txt");

        let content = fs.read_file(&files[0]).unwrap();
        assert!(content.contains("Hello, world!"));
    }

    #[test]
    fn test_non_text_file_filtered() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("image.png");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "not really an image").unwrap();

        let fs = LocalFileSource::new(dir.path(), vec!["txt".to_string()]);
        let files = fs.files().unwrap();
        assert!(files.is_empty());
    }
}
