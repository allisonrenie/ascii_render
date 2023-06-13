use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Point3
{
  x: f32,
  y: f32,
  z: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  // in the future this should probably take command line args? is that a thing?

  // create path
  let path = Path::new("assets/teapot.obj");
  let display = path.display();

  // opening file for reading
  let file= match File::open(&path)
  {
    // pattern matching for error checking
    Err(why) => panic!("couldn't open {}: {}, ", display, why),
    Ok(file) => file,
  };

  let reader = BufReader::new(file);

  // empty vec of points
  let mut points: Vec<Point3> = Vec::new();

  // next, must get this giant array (or at least parts that start with v)
  // into a giant array of point3 structs

  for line in reader.lines()
  {
    let line = line?;
    let entries: Vec<&str> = line.split_whitespace().collect();

    if entries.len() < 4
    {
      continue;
    }
    // for now, skipping everything that isn't a point
    if entries[0] != "v"
    {
      continue;
    }

    let new_point = Point3
    {
      x: entries[1].parse::<f32>().unwrap(),
      y: entries[2].parse::<f32>().unwrap(),
      z: entries[3].parse::<f32>().unwrap(),
    };

    points.push(new_point);
  }

  for p in points
  {
    print!("here's a point: {}, {}, {}\n", p.x, p.y, p.z);
  }

  Ok(())
}