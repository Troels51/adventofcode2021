use std::env;
use std::fs::File;
use serde::{de::Error, Deserialize, Deserializer}; // 1.0.94

use crate::{day2::{Submarine, Command}, binary_diagnostics::diagnostic};
mod day1;
pub mod day2;
pub mod binary_diagnostics;

#[derive(Debug, PartialEq)]
struct DiagnosticLine(u64);

impl<'de> Deserialize<'de> for DiagnosticLine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        // do better hex decoding than this
        u64::from_str_radix(&s[0..], 2)
            .map(DiagnosticLine)
            .map_err(D::Error::custom)
    }
}

fn main() {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/input/day3.csv";
    let file = File::open(path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let list : Result<Vec<DiagnosticLine>, _ > = rdr.deserialize().collect();
    let list : Vec<u32> = list.unwrap().into_iter().map( |x| x.0 as u32).collect();
    let report = diagnostic(list.as_slice(), 12);
    println!("Result of day 3.1 is gamma = {}, epsilon = {}, power_consumption = {}", report.gamma, report.epsilon, report.gamma * report.epsilon);
    println!("Result of day 3.2 is oxygen = {}, co2 = {}, life support rating = {}", report.oxygen, report.co2_scrubber, report.oxygen * report.co2_scrubber);



}
