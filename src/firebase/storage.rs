pub mod uploads {
    use log::warn;
    use std::path::PathBuf;

    pub fn upload_file_to_storage(file_path: &PathBuf) -> Result<(), &'static str> {
        if !file_path.exists() {
            warn!("Invalid file path: {:?}", file_path);
            return Err("Invalid file path");
        }

        // upload logic...
        Ok(())
    }

    pub async fn get_storage_buckets() {}
}
