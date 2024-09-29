use std::env;
use std::path::Path;

pub struct FileSystem;

impl FileSystem {
    /*
    pub fn get_template(string_path: String) Option<> {
        let path = match Self::check_file_availability(string_path) => {
            true => {
                env::current_dir()
                    .expect("Error occured with loading path")
                    .join("public")
                    .join(file_name)
            }
            false => {
                
            }
        };
    }

    //fn read_dir() -> Vec<String> {}
    
    fn check_file_availability(string_path: String) -> bool {
        let path = Path::new(string_path);
        if path.exists() {
            return true;
        } else {
            return false;
        }
    } */

    pub fn read_file(file_name: String) -> String {
        let file_path = env::current_dir()
            .expect("Failed to get current directory")
            .join("public")
            .join(file_name);

        //println!("FILE PATH: {}", file_path.clone().display());
        let file_contents = std::fs::read_to_string(file_path.clone())
                .expect(&format!("Error reading {}", file_path.clone().display()));
        
        file_contents
    }
}