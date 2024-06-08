use std::fs::File;

use uuid::Uuid;

pub fn add_location_to_gpx(uuid: Uuid, point: (f64, f64)) {
    let gpx_file = format!("./data/gpx/users/{}/location_data.gpx", uuid);
    let mut file = File::options().read(true).write(true).open(&gpx_file).unwrap();

    todo!();
}