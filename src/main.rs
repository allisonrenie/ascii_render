use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};
//use crossterm::{input, InputEvent, KeyEvent, RawScreen};

use matrix_lib::*;

// some constants
const CHAR_WIDTH: f32 = 80.;
const CHAR_WIDTH_I: i32 = 80;

const CHAR_HEIGHT: f32 = 40.;
const CHAR_HEIGHT_I: i32 = 40;

// TO DO: separate into a read points and draw points?
fn draw_points(points: &mut Vec<Vec3>) -> ()
{
  // make line amount depend on window size later?

  // first need to find the minimum and maximum
  // this seems really inefficient, maybe there's a better way??
  let mut y_max: f32 = points[0].y;
  let mut y_min: f32 = points[0].y;
  let mut x_max: f32 = points[0].x;
  let mut x_min: f32 = points[0].x;

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


  // println!("min y: {} max y: {} y diff: {}", y_min, y_max, y_diff);
  // io::stdout().flush().unwrap();
  // println!("min x: {} max x: {} x diff: {}", x_min, x_max, x_diff);
  // io::stdout().flush().unwrap();

  // this is so cursed LOL
  // there are 40 right now
  let mut line_points_arr: [Vec<i32>; CHAR_HEIGHT_I as usize] = 
  [
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  Vec::new(), 
  Vec::new(),
  ];

  for p in &mut *points
  {
    // finding y line for this point
    let curr_line: i32;
    if p.y < 0.
    {
      curr_line = ((p.y + y_min.abs())/y_diff * (CHAR_HEIGHT - 1.)).round() as i32;
    }
    else
    {
      curr_line = ((p.y - y_min)/y_diff * (CHAR_HEIGHT - 1.)).round() as i32;
    }
    // println!("current y: {} current line{}", p.y, curr_line);
    // io::stdout().flush().unwrap();

    let x_pos: i32;
    if p.x < 0.
    {
      x_pos = ((p.x + x_min.abs())/x_diff * CHAR_WIDTH).round() as i32;
    }
    else
    {
      x_pos = ((p.x - x_min)/x_diff * CHAR_WIDTH).round() as i32;
    }
    // println!("current x: {} current x pos: {}", p.x, x_pos);
    // io::stdout().flush().unwrap();
    // push x char pos into vector for char line y
    line_points_arr[curr_line as usize].push(x_pos);
  }

  // println!("min y: {} max y: {} y diff: {}", y_min, y_max, y_diff);
  // io::stdout().flush().unwrap();
  // println!("min x: {} max x: {} x diff: {}", x_min, x_max, x_diff);
  // io::stdout().flush().unwrap();

  let mut i: i32 = CHAR_HEIGHT_I; // counter for labelling lines

  // iterates over line_point vectors in reverse order
  // meaning it draws the last line first and goes down to 0
  for line_points in line_points_arr.iter().rev()
  {
    let mut line_string = String::new();

    // iterating over each character in the line
    for i in 0..CHAR_WIDTH_I
    {
      // if this character is contained in the vector of the x positions
      // of the points that are on this line, it is "drawn"
      if line_points.contains(&i)
      {
        line_string.push('=');
      }
      else
      {
        line_string.push('.');
      }
    }

    // finally printing our line
    println!("{}{}", i, line_string);
    i -= 1;
  }
    
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  //let _screen = RawScreen::into_raw_mode()?;

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
  // into a giant array of vec3 structs
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

  // time for the graphics loop!
  // some time setup
  let fps: u32 = 5;
  let frame_dur: Duration = Duration::from_secs(1)/fps; // sets duration of frame
  let mut last_frame_time: Instant = Instant::now(); // will hold time at last frame
  let ani_dur: Duration = Duration::from_secs(2); // duration of animation
  let start_time: Instant = Instant::now(); // start time 
  let ani_loops: u32 = 5; // how many times animation should run
  
  loop
  {
    // checking elapsed time and breaking out if has been entire enimation
    let elapsed = start_time.elapsed();
    if elapsed >= ani_dur * ani_loops
    {
      break;
    }

    let curr_frame_time: Instant = Instant::now();
    let elapsed_time: Duration = curr_frame_time - last_frame_time;

    // seeing if it is time for the next frame
    if elapsed_time >= frame_dur 
    {
      // creating our transformation matrix
      let rads: f32 = radians((elapsed.as_secs_f32() % ani_dur.as_secs_f32())/ani_dur.as_secs_f32() * 90.); 
      let mut tm: Mat3 = Mat3::new_id();
      tm.rotate(Axis::Y, rads);

      // transforming all of our points
      let mut transformed_points: Vec<Vec3> = points
        .iter()
        .map(|point| tm * point.clone())
        .collect();

      // for point in &transformed_points
      // {
      //   println!("x: {} y: {} z: {}", point.x, point.y, point.z);
      //   io::stdout().flush().unwrap();
      // }

      // drawing!
      print!("\n\n\n\n\n\n\n\n\n\n\n");
      draw_points(&mut transformed_points);

      //draw_points(&mut points);

      // making last frame time our current time after drawing frame
      last_frame_time = curr_frame_time;

      // breaking out of loop with a key press
      // let input = input();
      // let mut sync_stdin = input.read_sync();
      // if let Some(event) = sync_stdin.next()
      // {
      //   if event == InputEvent::Keyboard(KeyEvent::Esc)
      //   {
      //     break;
      //   }
      // }
    }
  }

  Ok(())
}