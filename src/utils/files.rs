pub mod files {
    use std::{fs::File, io::Read, path::PathBuf};

    pub fn read_file(file_path: &str) -> std::io::Result<Vec<u8>> {
        let mut file: File = File::open(file_path)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn walk_dir(dir: PathBuf) {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            println!("\nWalking: {:?}\n", &dir);

            for entry in entries {
                if let Ok(entry) = entry {
                    let path: PathBuf = entry.path();
                    if path.is_dir() {
                        println!("Directory: {:?}", path);
                    } else {
                        println!("File: {:?}", path);
                    }
                }
            }
        } else {
            println!("\nFailed to read directory: {:?}", &dir);
        }
    }
}
