use std::fmt::Debug;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder};
use actix_web::web::{Json};
use chrono;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::filesystem::{fetch_users, initialize_new_user};
use crate::logging::{error, info};


/// The [`User`] struct represents a user in the system.
#[derive(Debug)]
pub struct User {
    pub uuid: Uuid,
    pub device_name: String,
    pub created_at: DateTime<Utc>,
}

/// The [`UserRequest`] struct represents the request body for creating a new user. The json that is received should have a device_id field.  
/// This struct is used to deserialize the request body into a UserRequest struct and then create a new [`User`].
#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub device_id: String,
}


impl User {
    /// Create a new [`User`] with a random UUIDv4 and the current time.
    pub fn new(device_identifier: String) -> User {
        User {
            uuid: Uuid::new_v4(),
            created_at: Utc::now(),
            device_name: device_identifier,
        }
    }
}

/// Create a new [`User`] and return the generated UUIDv4.
#[post("/create_user")]
pub async fn create_user(req_body: Json<UserRequest>) -> impl Responder {
    let new_user = User::new(req_body.into_inner().device_id);
    initialize_new_user(&new_user); // Create database.
    HttpResponse::Created().body(new_user.uuid.to_string())
}


/// Get a list of all users in the system.
/// # Returns
/// A JSON array of all the users in the system.
/// # Errors
/// If the lookup of users fails, an internal server error is returned.
#[get("/list_users")]
pub async fn get_user(request: HttpRequest) -> HttpResponse {
    // We build a shitty JSON array by hand. This is fine for now but not future-proof.
    let mut response = String::from("[");

    let users = match fetch_users() {
        Ok(users) => users,
        Err(_) => {
            error(format!("IP {} requested list of users, but the lookup failed.", request.peer_addr().unwrap().ip()), Some("list_users"));
            return HttpResponse::InternalServerError().body("Could not fetch users. Please try again later.");
        }
    };

    for user_uuid in users.iter() {
        // println!("User {:?}", user_uuid);
        response.push_str(format!("\"{}\",", user_uuid).as_str());
    }

    response.pop(); // Remove the last comma.
    response.push(']');

    info(format!("IP {} requested list of users", request.peer_addr().unwrap().ip()), Some("list_users"));
    HttpResponse::Ok().body(response)
}