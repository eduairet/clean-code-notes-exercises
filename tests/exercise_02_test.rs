use clean_code_notes_exercises::exercises::exercise_02::*;

#[cfg(test)]
mod exercise_02_test {
    use super::*;

    #[test]
    fn test_disk_storage_create_directory() {
        let log_storage = DiskStorage::new("logs");
        assert_eq!(log_storage.create_directory().is_ok(), true);
        log_storage.insert_file("test.txt", "Test").unwrap();
        let file_path = log_storage.get_directory_path().join("test.txt");
        let file_content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(file_content, "Test");
    }
}
