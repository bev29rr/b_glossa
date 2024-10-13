use std::env;
use std::path::{Path, PathBuf};
use std::error::Error;

pub struct FileSystem;

impl FileSystem {
    pub fn get_template(string_path: String) -> Option<String> {
        let path = match Self::check_file_availability(string_path) {
            Some(path) => path,
            None => return None
        };
        match Self::read_file(&path) {
            Ok(file_contents) => Some(file_contents),
            Err(_) => None
        }
    }
    
    fn check_file_availability(string_path: String) -> Option<PathBuf> {
        let current_dir = env::current_dir();
        let path: PathBuf = match current_dir {
            Ok(dir) => dir.join("public").join(&string_path),
            Err(_) => {return None;},
        };
        if path.exists() {
            return Some(path);
        } else {
            return None;
        }
    }

    pub fn read_file(file_name: &Path) -> Result<String, Box<dyn Error>> {
        //println!("FILE PATH: {}", file_path.clone().display());
        std::fs::read_to_string(file_name)
            .map_err(|e| Box::new(e) as Box<dyn Error>) // Maps any errors to Box<dyn Error>
    }
}