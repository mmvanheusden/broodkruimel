use actix_web::{HttpResponse, post, Responder};
use actix_web::web::Json;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::filesystem::add_location_to_user_db;
use crate::logging::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    uuid: String,
    latitude: i32,
    longitude: i32,
    gathered_at: i64, // Unix timestamp
}

impl Location {
    /// Get the UUID of the user associated with this location
    pub fn get_uuid(&self) -> &String {
        &self.uuid
    }

    /// Get the latitude and longitude of this location
    pub fn get_lat_long(&self) -> (i32, i32) {
        (self.latitude, self.longitude)
    }

    /// Get the timestamp of when this location was gathered
    pub fn get_gathered_at(&self) -> i64 {
        self.gathered_at
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationRequest {
    uuid: String,
    latitude: i32,
    longitude: i32,
    gathered_at: i64, // Unix timestamp
}

impl Location {
    pub fn new(uuid: String, latitude: i32, longitude: i32, gathered_at: i64) -> Location {
        Location {
            uuid,
            latitude,
            longitude,
            gathered_at,
        }
    }
}

/// Adds a location to the user's location database
#[post("/push_location")]
pub async fn add_location_to_user(req_body: Json<LocationRequest>) -> impl Responder {
    let location = Location::new(req_body.uuid.clone(), req_body.latitude, req_body.longitude, req_body.gathered_at);
    add_location_to_user_db(location);
    info(format!("User {} has send their location at {}", req_body.uuid, DateTime::from_timestamp(req_body.gathered_at, 0).unwrap()), Some("push_location"));
    HttpResponse::Ok().finish()
}