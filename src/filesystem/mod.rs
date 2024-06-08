use std::path::Path;

pub mod database;
pub mod gps;


/// Create the folder structure where we write stuff later on.
pub async fn initialize_file_structure() {
    let path = Path::new("./data/db/users/");
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}