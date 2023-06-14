use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use matrix_lib::*;

// some constants
const CHAR_WIDTH: f32 = 80.;
const CHAR_WIDTH_I: i32 = 80;

const CHAR_HEIGHT: f32 = 40.;
const CHAR_HEIGHT_I: i32 = 40;

// struct Point3
// {
//   x: f32,
//   y: f32,
//   z: f32,
// }

fn draw_points(points: &mut Vec<Vec3>) -> ()
{
  // make line amount depend on window size later?

  // first need to find the minimum and maximum
  // this seems really inefficient, maybe there's a better way??
  let mut y_max: f32 = 0.;
  let mut y_min: f32 = 0.;
  let mut x_max: f32 = 0.;
  let mut x_min: f32 = 0.;

  for p in &mut *points
  {
    if p.y >= y_max
    {
      y_max = p.y;
    }
    if p.y <= y_min
    {
      y_min = p.y;
    }
    if p.x >= x_max
    {
      x_max = p.x;
    }
    if p.x <= x_min
    {
      x_min = p.x;
    }
  }

  let y_diff: f32 = y_max - y_min;
  let x_diff: f32 = x_max - x_min;

  for line in (0..CHAR_HEIGHT_I).rev()
  {
    // will store the points for this line by their char position
    let mut line_points: Vec<i32> = Vec::new();
    for p in &mut *points
    {
      if (p.y/y_diff * CHAR_HEIGHT).round() as i32 == line
      {
        // this point is on the line we are drawing
        // now, where is the point on this line?
        // pushing char place to vec
        line_points.push((p.x/x_diff * CHAR_WIDTH).round() as i32);
      }
    }

    // now creating the string
    let mut line_string = String::new();
    for i in 0..CHAR_WIDTH_I
    {
      if line_points.contains(&i)
      {
        line_string.push('@');
      }
      else
      {
        line_string.push(' ');
      }
    }

    // finally printing our line
    println!("{}", line_string);
  }

  // make a string
  // for each character in the string (0 through 80)
  // match the character to 
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
  let mut points: Vec<Vec3> = Vec::new();

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

    let new_point = Vec3::new
    (
      entries[1].parse::<f32>().unwrap(),
      entries[2].parse::<f32>().unwrap(),
      entries[3].parse::<f32>().unwrap(),
    );

    points.push(new_point);
  }

  let rads: f32 = radians(90.); 
  let mut tm: Mat3 = Mat3::new_id();
  tm.rotate(Axis::X, rads);


  let mut transformed_points: Vec<Vec3> = points
    .iter()
    .map(|point| tm * point.clone())
    .collect();

  for point in &transformed_points
  {
    println!("x: {} y: {} z: {}", point.x, point.y, point.z);
  }

  draw_points(&mut transformed_points);

  Ok(())
}