use std::str::FromStr;

use actix_web::{HttpRequest, HttpResponse, put, Responder, web};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

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

/// Appends a location to the user's location database
#[put("/api/users/{uuid}/location")]
pub async fn push_location(path: web::Path<String>, req_body: Json<LocationRequest>, request: HttpRequest) -> impl Responder {
    let uuid = path.into_inner().to_string();
    let location = Location::new(req_body.latitude, req_body.longitude);

    add_location_to_user_db(uuid.clone(), &location);
    add_location_to_gpx(uuid.clone(), &location);

    info(format!("IP: {} has send their location.", &request.peer_addr().unwrap().to_string()), Some(format!("PUT: /api/users/{}/location", &uuid)));
    HttpResponse::Ok().finish()
}