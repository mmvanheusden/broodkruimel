use std::path::Path;
use sqlite::State;

use crate::api::location::Location;
use crate::api::user::User;
use crate::logging;

/// Create a folder for the user, and initialize the database where the user data will be stored.
pub fn initialize_new_user(user: &User) {
    create_user_dir(user.uuid.to_string().as_str());
    let filename = format!("data/db/users/{}/location_data.db", &user.uuid.to_string().as_str());
    let connection = sqlite::open(filename).unwrap();
    let query = "CREATE TABLE location (latitude INTEGER, longitude INTEGER, gathered_at INTEGER);";

    connection.execute(query).unwrap();
    logging::info(format!("Created database for user {}. (device name: {})", &user.uuid, &user.device_name).as_str(), Some("database"));
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
        query = format!("CREATE TABLE users (name TEXT, device_name TEXT, created_at INTEGER); INSERT INTO users (name, device_name, created_at) VALUES ('{}','{}', {});", user.uuid, user.device_name, user.created_at.timestamp());
    } else {
        // Only writes user.
        query = format!("INSERT INTO users (name, device_name, created_at) VALUES ('{}','{}', {});", user.uuid, user.device_name, user.created_at.timestamp());
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

/// Adds a [`Location`] to a user's database.
pub fn add_location_to_user_db(data: Location) {
    let db_file = format!("data/db/users/{}/location_data.db", data.get_uuid());
    let connection = sqlite::open(db_file).unwrap();
    connection.execute(format!("INSERT INTO location (latitude, longitude, gathered_at) VALUES ({}, {}, {})", data.get_lat_long().0, data.get_lat_long().1, data.get_gathered_at())).unwrap()
}

// TODO: Learn more about lifetimes!
/// Fetches the list of users from the database.
///
/// # Returns
/// Either
/// * [`Ok`] With a [`Vec<String>`] with a list of user UUIDs.
/// * [`Err`] With an error message.
pub fn fetch_users() -> Result<Vec<String>, &'static str> {
    let users_db = Path::new("./data/db/users/users.db");
    let mut users: Vec<String> = Vec::new();
    // Check if the file even exists
    if !users_db.exists() {
        return Err("The users database file was not found. Perhaps there are no users registered yet?")
    }

    let query = "SELECT name FROM users";
    let connection = sqlite::open("data/db/users/users.db").unwrap();

    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let uuid = statement.read::<String, _>("name").unwrap();
        users.push(uuid);
    }

    Ok(users)
}