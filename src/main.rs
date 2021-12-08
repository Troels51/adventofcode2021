#![feature(array_zip)]

use std::env;
use std::fs::File;
use serde::{de::Error, Deserialize, Deserializer}; // 1.0.94

use crate::{day2::{Submarine, Command}, binary_diagnostics::diagnostic};
mod day1;
pub mod day2;
pub mod binary_diagnostics;
pub mod bingo;

pub mod day4_data;


fn main() {
    let numbers  = vec![93,18,74,26,98,52,94,23,15,2,34,75,13,31,39,76,96,16,84,12,38,27,8,85,86,43,4,79,57,19,40,59,14,21,35,0,90,11,32,17,78,83,54,42,66,82,99,45,55,63,24,5,89,46,80,49,3,48,67,47,50,60,81,51,71,33,72,6,9,30,56,20,77,29,28,69,25,36,91,92,65,22,62,58,64,88,10,7,87,41,44,37,73,70,68,97,61,95,53,1];
    let mut bingodata : bingo::BingoData = bingo::BingoData{numbers_drawn: numbers, boards: day4_data::data4data()};
    let (score, winning_board) = bingodata.get_first_winning_board().unwrap();
    let (last_score, last_winning_board) = bingodata.get_last_winning_board().unwrap();
    //dbg!(last_score, last_winning_board);
    println!("Winning score is : {}", winning_board.score(score));
    println!("Last Winning score is : {}", last_winning_board.score(last_score));
}
