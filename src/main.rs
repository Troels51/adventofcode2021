use std::env;
use std::fs::File;

use crate::day2::{Submarine, Command};
mod day1;
pub mod day2;
pub mod binary_diagnostics;

type InputCmd = (String, i32);
fn main() {
    let mut sub : Submarine = Submarine::default();
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/input/day2.csv";
    let file = File::open(path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).delimiter(b' ').from_reader(file);
    let list : Result<Vec<InputCmd>, _ > = rdr.deserialize().collect();
    let cmdlist : Vec<Command> = list.unwrap().into_iter().map(|x| Command::from_inputcmd(x)).collect();
    sub.take_cmd_list(cmdlist.as_slice());
    let result = sub.depth * sub.horizontal_position;
    println!("Result of day 2.1 is {}", result);
    //assert_eq!(result, 1488669);
}
