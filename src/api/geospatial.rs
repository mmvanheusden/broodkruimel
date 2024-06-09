use std::str::FromStr;

use actix_web::{HttpResponse, post, Responder};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::filesystem::database::add_location_to_user_db;
use crate::filesystem::gps::add_location_to_gpx;
use crate::logging::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationRequest {
    uuid: String,
    latitude: f64,
    longitude: f64,
}

impl Location {
    pub fn new(lat: f64, lon: f64) -> Location {
        Location {
            latitude: lat,
            longitude: lon,
        }
    }

    pub fn lat(&self) -> f64 {
        self.latitude
    }
    pub fn lon(&self) -> f64 {
        self.longitude
    }
}

/// Adds a location to the user's location database
#[post("/push_location")]
pub async fn push_location(req_body: Json<LocationRequest>) -> impl Responder {
    let location = Location::new(req_body.latitude, req_body.longitude);
    add_location_to_user_db(&Uuid::from_str(&req_body.uuid.as_str()).unwrap(), &location);
    add_location_to_gpx(Uuid::from_str(req_body.uuid.clone().as_str()).unwrap(), &location);
    info(format!("User {} has send their location.", req_body.uuid), Some("push_location"));
    HttpResponse::Ok().finish()
}