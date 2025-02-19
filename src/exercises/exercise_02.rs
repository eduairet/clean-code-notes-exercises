/// Eduardo Aire Torres - <eduardo.aire.torres@gmail>
use std::io::Write;

/// DiskStorage struct
///
/// # Examples
///
/// ```
/// use clean_code_notes_exercises::exercises::exercise_02::DiskStorage;
///
/// let log_storage = DiskStorage::new("logs");
///
/// log_storage.create_directory();
/// log_storage.insert_file("test.txt", "Test");
/// ```
#[derive(Debug)]
pub struct DiskStorage {
    pub storage_directory: String,
}

impl DiskStorage {
    pub fn new(directory_name: &str) -> Self {
        Self {
            storage_directory: directory_name.to_string(),
        }
    }

    pub fn get_directory_path(&self) -> std::path::PathBuf {
        std::path::Path::new(&self.storage_directory).to_path_buf()
    }

    pub fn create_directory(&self) -> std::io::Result<()> {
        if !self.get_directory_path().exists() {
            std::fs::create_dir_all(&self.storage_directory)?;
        }
        Ok(())
    }

    /// # Warning
    ///
    /// Directory must exist in advance
    pub fn insert_file(&self, file_name: &str, content: &str) -> std::io::Result<()> {
        let file_path = self.get_directory_path().join(file_name);
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        // TODO: Add error handling
        Ok(())
    }
}
