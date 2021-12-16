#![feature(array_zip)]
#[macro_use]
extern crate bmp;
use std::env;
use std::fs::File;
use serde::{de::Error, Deserialize, Deserializer}; // 1.0.94

use bmp::{Image, Pixel};

use crate::{day2::{Submarine, Command}, binary_diagnostics::diagnostic};
mod day1;
pub mod day2;
pub mod binary_diagnostics;
pub mod bingo;

pub mod day4_data;
pub mod hydrothermal;
pub mod day5_data;

fn main() {
    let mut img = Image::new(1000, 1000);
    let data:Vec<hydrothermal::Line> = day5_data::day5data();
    let mut map: hydrothermal::Map = hydrothermal::Map{overlappings: vec![vec![0;1000];1000] };
    for line in data {
        if line.into_iter().count() == 0{
        }
        for p in line {
            img.set_pixel(p.x as u32, p.y as u32, px!(255, 255, 255));
        }
        map.draw_points_that_cover_line(line);
    }
    let result = map.count_above(2);

    
    let _ = img.save("img.bmp");
    println!("Result is : {}", result);

}
