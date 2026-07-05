extern crate rand;
extern crate num;
extern crate rayon;

use rand::Rng;
use num::Float;
use std::io::{self, BufWriter, BufReader, Read, Write};
use rayon::prelude::*;
use std::time::Instant;
use std::fs::File;
#[derive(Clone, Copy, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Float> Point<T> {
    fn distance(&self, other: &Self) -> T {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn closest_pair_with_thread<T: Float + Send + Sync>(points: &[Point<T>]) -> T {
    let len = points.len();
    if len <= 3 {
        let mut min_dist = T::infinity();
        for i in 0..len {
            for j in i + 1..len {
                let dist = points[i].distance(&points[j]);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        min_dist
    } else {
        let mid = len / 2;
        let left_min = rayon::join(|| closest_pair(&points[..mid]), || closest_pair(&points[mid..]));
        let min_dist = left_min.0.min(left_min.1);

        let strip: Vec<_> = points
            .par_iter()
            .filter(|&point| {
                let delta_x = point.x - points[mid].x;
                delta_x.abs() < min_dist
            })
            .collect();

        let strip_min = strip
            .par_windows(2)
            .map(|window| window[0].distance(&window[1]))
            .reduce_with(T::min)
            .unwrap_or(T::infinity());

        min_dist.min(strip_min)
    }
}


fn closest_pair<T: Float>(points: &[Point<T>]) -> T {
    let len = points.len();
    if len <= 3 {
        let mut min_dist = T::infinity();
        for i in 0..len {
            for j in i + 1..len {
                let dist = points[i].distance(&points[j]);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        min_dist
    } else {
        let mid = len / 2;
        let left_min = closest_pair(&points[..mid]);
        let right_min = closest_pair(&points[mid..]);
        let min_dist = left_min.min(right_min);

        let strip: Vec<_> = points
            .iter()
            .filter(|&point| {
                let delta_x = point.x - points[mid].x;
                delta_x.abs() < min_dist
            })
            .collect();

        let strip_min = strip
            .windows(2)
            .map(|window| window[0].distance(&window[1]))
            .fold(T::infinity(), T::min);

        min_dist.min(strip_min)
    }
}

fn main() {
    // Generate random points
  let mut rng = rand::thread_rng();
  let points: Vec<Point<f64>> = (0..1000000)
      .map(|_| Point {
          x: rng.gen_range(0.0..1000000.0),
          y: rng.gen_range(0.0..1000000.0),
      })
      .collect();

  // Save points to a file
  let mut file = File::create("points.txt").expect("Failed to create file");
  for point in &points {
      writeln!(file, "{},{}", point.x, point.y).expect("Failed to write to file");
  }

  // Read points from the file
  let mut file = File::open("points.txt").expect("Failed to open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents)
      .expect("Failed to read from file");

  // Parse points from the file
  let points: Vec<Point<f64>> = contents
      .lines()
      .map(|line| {
          let coords: Vec<&str> = line.split(',').collect();
          Point {
              x: coords[0].parse().unwrap(),
              y: coords[1].parse().unwrap(),
          }
      })
      .collect();
    // Compute closest pair without Rayon
    let start_time = Instant::now();
    let closest_pair_result_without_rayon = closest_pair(&points);
    let duration_without_rayon = start_time.elapsed();

    println!("Closest pair without Rayon: {}", closest_pair_result_without_rayon);
    println!("Time taken without Rayon: {:?}", duration_without_rayon);

    // Compute closest pair using Rayon
    let start_time = Instant::now();
    let closest_pair_result_with_rayon = closest_pair_with_thread(&points);
    let duration_with_rayon = start_time.elapsed();

    println!("Closest pair with Rayon: {}", closest_pair_result_with_rayon);
    println!("Time taken with Rayon: {:?}", duration_with_rayon);
}
