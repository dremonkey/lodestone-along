
// Standard lib packages

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

pub extern fn along(
    line: &FeatureLineString,
    dist: f64,
    units: &str) -> FeaturePoint {

  let coords = line.coordinates();

  let mut iter = coords.iter().peekable();
  let mut prev: Option<FeaturePoint> = None;
  let mut traveled = 0.0;

  loop {
    let coord = iter.next().unwrap().to_vec();
    let current = FeaturePoint::new(coord);

    // traverse the line until we have exceeded `dist` or hit the end
    match iter.peek() {
      Some(next_coord) => {
        if traveled >= dist {
          let delta = traveled - dist;
          if delta == 0.0 {
            return current;
          } else {
            // retrace our steps if we exceeded `dist`
            match prev {
              Some(prev) => {
                let brng = bearing(&current, &prev) - 180.0;
                let interpolated = destination(&current, delta, brng, &units);
                return interpolated;
              },
              None => return current
            } 
          }
        } else {
          let next = FeaturePoint::new(next_coord.to_vec());
          
          // save previous and calculate `traveled` before continuing
          prev = Some(current.clone());
          traveled += distance(&current, &next, &units);
        }
      },
      None => break
    }
  }

  // default
  let coord = coords.last().unwrap().to_vec();
  FeaturePoint::new(coord)
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
