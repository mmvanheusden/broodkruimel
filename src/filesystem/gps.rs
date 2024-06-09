use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::OpenOptions;

use uuid::Uuid;
use crate::api::geospatial::Location;

/// Add a [`Location`] to a user's gpx.
pub fn add_location_to_gpx(uuid: Uuid, location: &Location) {
    let filepath = format!("./data/gpx/users/{}/location_data.gpx", uuid);
    let gpx_file = File::options().read(true).open(&filepath).unwrap();
    let line_count = BufReader::new(&gpx_file).lines().count();

    // Adds an <trkpt> segment to the gpx
    write_at_line(&filepath, line_count - 3, format!("\t\t<trkpt lat=\"{}\" lon=\"{}\"></trkpt>\n", &location.lat(), &location.lon())).unwrap();
}

/// Writes data to file at a specified line. Overrides the line
fn write_at_line<S: AsRef<str>>(file_path: &str, line_num: usize, content: S) -> std::io::Result<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    if line_num > lines.len() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "line number too large"));
    }

    lines[line_num - 1] = content.as_ref().into();

    let file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}