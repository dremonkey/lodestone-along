/// The main crate for lodestone-along
/// 
/// ## Overview
/// 
/// Takes a FeatureLineString and returns a FeaturePoint at a specified distance 
/// along the line.

// Third party crates
extern crate lodestone_bearing;
extern crate lodestone_destination;
extern crate lodestone_distance;
extern crate lodestone_point;
extern crate lodestone_linestring;

use lodestone_bearing::bearing;
use lodestone_destination::destination;
use lodestone_distance::distance;
use lodestone_linestring::FeatureLineString;
use lodestone_point::FeaturePoint;

/// Returns a FeaturePoint at a specified distance along the line.
pub extern fn along(
    line: &FeatureLineString,
    dist: f64,
    units: &str) -> FeaturePoint {

  let (current, prev, delta) = traverse(&line, dist, &units);

  if delta == 0.0 {
    return current;
  } else {
    let brng = bearing(&current, &prev);
    let interpolated = destination(&current, delta, brng, &units);
    return interpolated;
  }
}

// Method to traverse a line until distance has been reached or exceeded
// At that point it returns the last two coordinates and the overshoot delta
fn traverse(
    line: &FeatureLineString,
    dist: f64,
    units: &str) -> (FeaturePoint, FeaturePoint, f64) {

  let mut coords = line.coordinates();
  let mut current = FeaturePoint::new(coords.remove(0));
  let mut prev = current.clone();
  let mut traveled = 0.0;

  for coord in coords {
    current = FeaturePoint::new(coord.clone());
    traveled += distance(&prev, &current, &units);

    if traveled >= dist {
      let delta = traveled - dist;
      return (current, prev, delta);
    } else {
      prev = current.clone();
    }
  }

  (current, prev, 0.0)
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use lodestone_linestring::FeatureLineString;
  use super::traverse;

  #[test]
  fn test_traverse() {
    let coords = vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![1.0, 1.0], vec![2.0, 1.0]];
    let line = FeatureLineString::new(coords);
    
    let (current, prev, delta) = traverse(&line, 200.0, "km");
    
    assert_eq!(current, FeaturePoint::new(vec![1.0, 1.0]));
    assert_eq!(prev, FeaturePoint::new(vec![1.0, 0.0]));
    assert_eq!(delta, 22.638981586547118);
  }
}
