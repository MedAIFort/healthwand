use std::fs::{self, File};
use std::io::{self, BufReader, Read};
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
            self.allowed_extensions
                .iter()
                .any(|x| x.eq_ignore_ascii_case(ext))
        } else {
            false
        }
    }

    fn collect_files_recursive(
        &self,
        dir: &Path,
        files: &mut Vec<PathBuf>,
        depth: usize,
    ) -> Result<(), FileSourceError> {
        const MAX_DEPTH: usize = 100;
        if depth > MAX_DEPTH {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_symlink() {
                continue;
            }
            let path = entry.path();
            if file_type.is_dir() {
                self.collect_files_recursive(&path, files, depth + 1)?;
            } else if file_type.is_file() && self.is_text_file(&path) {
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
            self.collect_files_recursive(&self.root, &mut files, 0)?;
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

    #[test]
    fn test_files_nonexistent_root() {
        let fs = LocalFileSource::new("/this/path/should/not/exist", vec!["txt".to_string()]);
        let result = fs.files();
        assert!(result.is_err());
        match result {
            Err(FileSourceError::Io(_)) => {}
            _ => panic!("Expected Io error for nonexistent root"),
        }
    }

    #[test]
    fn test_files_on_regular_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("single.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test").unwrap();
        let fs = LocalFileSource::new(&file_path, vec!["txt".to_string()]);
        let files = fs.files().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }

    #[test]
    fn test_read_file_not_allowed_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("notallowed.bin");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "data").unwrap();
        let fs = LocalFileSource::new(dir.path(), vec!["txt".to_string()]);
        let result = fs.read_file(&file_path);
        assert!(result.is_err());
        match result {
            Err(FileSourceError::NotTextFile(p)) => assert_eq!(p, file_path),
            _ => panic!("Expected NotTextFile error for disallowed extension"),
        }
    }

    #[test]
    fn test_read_file_permission_denied() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("private.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "secret").unwrap();
        drop(file);
        // Remove read permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o000);
            fs::set_permissions(&file_path, perms).unwrap();
            let fs = LocalFileSource::new(dir.path(), vec!["txt".to_string()]);
            let result = fs.read_file(&file_path);
            assert!(result.is_err());
            match result {
                Err(FileSourceError::Io(e)) => {
                    assert!(e.kind() == std::io::ErrorKind::PermissionDenied);
                }
                _ => panic!("Expected Io error for permission denied"),
            }
            // Restore permissions so tempdir can clean up
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o644);
            fs::set_permissions(&file_path, perms).unwrap();
        }
    }
}
