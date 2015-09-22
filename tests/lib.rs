
// Standard lib crates
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

// Third party crates
extern crate lodestone_along;
extern crate lodestone_bearing;
extern crate lodestone_destination;
extern crate lodestone_distance;
extern crate lodestone_linestring;
extern crate lodestone_point;

use lodestone_along::along;
use lodestone_destination::destination;
use lodestone_bearing::bearing;
use lodestone_distance::distance;
use lodestone_linestring::FeatureLineString;
use lodestone_point::FeaturePoint;

#[test]
fn test_along_dist_between() {  
  let line = get_linestring();

  // find a point between coordinates at index 0 and 1
  // ... do prep work
  let coords = line.coordinates();
  let pt1 = FeaturePoint::new(coords[0].clone());
  let pt2 = FeaturePoint::new(coords[1].clone());
  let brng = bearing(&pt1, &pt2);
  let distance = distance(&pt1, &pt2, "m") - 200.0;

  // ... set the expected value
  let expected_point = destination(&pt1, distance, brng, "m");

  // ... test point calculated by along
  let pt3 = along(&line, distance, "m");
  assert!(expected_point == pt3);
}

#[test]
fn test_along_dist_exact() {  
  let line = get_linestring();

  // test `along` for match with existing vertice
  // ... do prep work
  let coords = line.coordinates();
  let pt1 = FeaturePoint::new(coords[0].clone());
  let pt2 = FeaturePoint::new(coords[1].clone());
  let brng = bearing(&pt1, &pt2);
  let distance = distance(&pt1, &pt2, "m");

  // ... set the expected value
  let expected_point = destination(&pt1, distance, brng, "m");

  // ... test point calculated by along
  let pt3 = along(&line, distance, "m");
  assert!(expected_point == pt3);
}

// Helper method to build the FeatureLineString
fn get_linestring() -> FeatureLineString {
  let path = Path::new("tests/support/line.geojson");
  let display = path.display();

  let mut file = match File::open(&path) {
    Ok(file) => file,
    Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
  };

  let mut geostring = String::new();
  let _ = file.read_to_string(&mut geostring);
  
  match FeatureLineString::from_str(&geostring) {
    Ok(line) => line,
    Err(..) => panic!(..) 
  }
}