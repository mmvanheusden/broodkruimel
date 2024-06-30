use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time;

use sqlite::State;

use crate::api::geospatial::Location;
use crate::api::user::User;
use crate::logging;

/// Create a folder for the user, and initialize the database where the user data will be stored.
pub fn initialize_new_user(user: &User) {
    init_user_filestructure(&user);
    add_user_to_users_db(user);
    logging::info(format!("Created database for user {}.", &user.uuid), Some("database"));
}

/**
 * * Creates a directory structure for the specified [`User`]
 * * Creates empty database,gpx file for user.
 **/
fn init_user_filestructure(user: &User) {
    // DIRECTORIES
    // db
    let path_string = format!("./data/db/users/{}", &user.uuid);
    let path = Path::new(&path_string);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }

    // gpx
    let path_string = format!("./data/gpx/users/{}", &user.uuid);
    let path = Path::new(&path_string);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }


    // FILES
    // Create user empty gpx
    let file_location = format!("./data/gpx/users/{}/location_data.gpx", &user.uuid);
    let mut file = File::create(file_location).expect("create gpx error.");

    // write empty gpx track
    file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<gpx\n\tversion=\"1.1\"\n\tcreator=\"Broodkruimel\">\n\n<trk>\n\t<name>Breadcrumb</name>\n\t<desc>Broodkruimel location data</desc>\n\t<trkseg>\n\n\t</trkseg>\n</trk>\n</gpx>").unwrap();
    logging::info(format!("Created GPX file for user {}", &user.uuid), Some("database"));

    // Create user db
    let filename = format!("data/db/users/{}/location_data.db", &user.uuid);
    let connection = sqlite::open(filename).unwrap();
    let query = "CREATE TABLE location (latitude INTEGER, longitude INTEGER, gathered_at INTEGER);";

    connection.execute(query).unwrap();
}


/// Append the user to the users database. If the database does not exist, create it.
fn add_user_to_users_db(user: &User) {
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


/// Adds a [`Location`] to a user's database.
pub fn add_location_to_user_db(uuid: String, location: &Location) {
    let db_file = format!("data/db/users/{}/location_data.db", &uuid);
    let connection = sqlite::open(db_file).unwrap();
    connection.execute(format!("INSERT INTO location (latitude, longitude, gathered_at) VALUES ({}, {}, {})", location.lat(), location.lon(), time::UNIX_EPOCH.elapsed().unwrap().as_millis())).unwrap()
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
        return Err("The users database file was not found. Perhaps there are no users registered yet?");
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


/// Get the full user data from a user from the users DB. Returns error when the user doesn't exist in the db
/// ## Returns
/// ```
/// Ok((
///     name: String,
///     device_name: String,
///     created_at: u64
/// ))
/// ```
pub fn get_user_from_users_db(uuid: String) -> Result<(String, String, i64), &'static str> {
    let users_db = Path::new("./data/db/users/users.db");

    if users_db.exists() {
        let connection = sqlite::open(users_db).unwrap();
        let query = format!("SELECT * FROM users WHERE name = '{}'", &uuid);
        
        let mut statement = connection.prepare(query).unwrap();

        // println!("{}", statement.iter().count());
        // Error when statement lines is 0. Meaning the user was not found in the DB
        if statement.iter().count() < 1 {
            return Err("User not in users DB")
        }

        statement.next().unwrap();
        
        let name = statement.read::<String, _>("name").unwrap();
        let device_name = statement.read::<String, _>("device_name").unwrap();
        let created_at = statement.read::<String, _>("created_at").unwrap();

        Ok((name, device_name, created_at.parse().unwrap()))
    } else {
        Err("Users DB doesn't exist!")
    }
}