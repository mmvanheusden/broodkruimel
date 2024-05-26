use std::path::Path;
use crate::api::user::User;
use crate::logging;



/// Create a folder for the user, and initialize the database where the user data will be stored.
pub fn initialize_new_user(user: &User) {
    create_user_dir(user.uuid.to_string().as_str());
    let filename = format!("data/db/users/{}/location-data.db", &user.uuid.to_string().as_str());
    let connection = sqlite::open(filename).unwrap();
    let query = "CREATE TABLE location (latitude INTEGER, longitude INTEGER);";

    connection.execute(query).unwrap();
    logging::info(format!("Created database for user {}.", &user.uuid).as_str(), Some("database"));
    integrate_user(user);
}

/// Create user dir
fn create_user_dir(name: &str) {
    let path_string = format!("./data/db/users/{}", name);
    let path = Path::new(&path_string);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}


/// Append the user to the users' database. If the database does not exist, create it.
fn integrate_user(user: &User) {
    let file = Path::new("./data/db/users/users.db");
    let query: String;

    // If the database does not exist, modify the query to also create the table.
    if !file.exists() {
        // Writes header and user.
        query = format!("CREATE TABLE users (name TEXT, device_name TEXT, created_at INTEGER); INSERT INTO users (name, device_name, created_at) VALUES ('{}','{}', {});", user.uuid,user.device_id, user.created_at.timestamp());
    } else {
        // Only writes user.
        query = format!("INSERT INTO users (name, device_name, created_at) VALUES ('{}','{}', {});", user.uuid,user.device_id, user.created_at.timestamp());
    }

    let connection = sqlite::open("data/db/users/users.db").unwrap();
    connection.execute(query).unwrap();
}

/// Create the folder structure where we write stuff later on.
pub async fn initialize_file_structure() {
    let path = Path::new("./data/db/users/");
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}
