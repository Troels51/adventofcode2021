use serde::{de::Error, Deserialize, Deserializer}; // 1.0.94



fn add_to_vec(bitset: &mut[i32; 32], number: u32, size: usize) {
    for i in 0..size {
        let bit = (number >> i) & 1;
        if bit == 1 {
            bitset[i] += 1;
        }
        else {
            bitset[i] -= 1;
        }
    }
}
fn accumulator_to_uint(input: &[i32; 32], size: usize) -> u32 {
    let input : Vec<u32> = input.iter().map(|i| 
        if *i > 1 {
            1
        }
        else {
            0
        }).collect(); //Normalize to 1 or 0 or -1
    let mut accumulator: u32 = 0;
    for i in 0..size {
        accumulator += input[i] << i;
    }
    accumulator
}
fn accumulated_array(input: &[u32], size: usize) -> [i32; 32] {
    let mut accumulator:  [i32;32] = [0; 32];
    for i in input {
        add_to_vec(&mut accumulator, *i, size);
    };
    accumulator
} 
pub struct Report {
    pub oxygen : u32,
    pub co2_scrubber : u32,
    pub gamma : u32,
    pub epsilon : u32,
}
pub fn diagnostic(diagnostic_report: &[u32] , size: usize) -> Report {
    let mut accumulator = accumulated_array(diagnostic_report, size);
    let oxygen = oxygen_generator_rating(diagnostic_report, size);
    let co2 = co2_scrubber_rating(diagnostic_report, size);
    let gamma = accumulator_to_uint(&accumulator, size);
    let epsilon = !gamma & !(std::u32::MAX << size);
    Report {
        oxygen : oxygen,
        co2_scrubber: co2,
        gamma : gamma,
        epsilon : epsilon }
}

fn get_common_value(diagnostic_report: &[u32], position: usize) -> i32 {
    let mut acc = 0;
    for number in diagnostic_report {
        if is_bit_set(*number, position) {
            acc += 1;
        }
        else {
            acc -= 1;
        }
    }
    acc
}

pub fn oxygen_generator_rating(diagnostic_report: &[u32], size: usize) -> u32 {
    let mut position = size - 1;
    let mut list = diagnostic_report.clone().to_vec();
    while list.len() > 1 {
        let accumulated = get_common_value(list.clone().as_ref(), position);
        if  accumulated < 0 { //if 1 is more common
            list = list.into_iter().filter(|val| !is_bit_set(*val, position) ).collect();
        }
        else if accumulated > 0 {
            list = list.into_iter().filter(|val| is_bit_set(*val, position) ).collect();
        } else {
            list = list.into_iter().filter(|val| is_bit_set(*val, position) ).collect();
        }
        if list.len() == 1 {
            return list[0];
        }
        position -= 1;
    }
    return list[0];
}

fn is_bit_set(number: u32, position : usize) -> bool {
    (number & (1 << position)) != 0
}

pub fn co2_scrubber_rating(diagnostic_report: &[u32], size: usize) -> u32 {
    let mut position = size - 1;
    let mut list = diagnostic_report.clone().to_vec();
    while list.len() > 1 {

        let accumulated = get_common_value(list.clone().as_ref(), position);
        if accumulated < 0 { //1 is least common
            list = list.into_iter().filter(|val| is_bit_set(*val, position) ).collect();
        }
        else if accumulated > 0 { // 0 least common
            list = list.into_iter().filter(|val| !is_bit_set(*val, position) ).collect();
        } else {
            list = list.into_iter().filter(|val| !is_bit_set(*val, position) ).collect();
        }
        if list.len() == 1 {
            return list[0];
        }
        position -= 1;
    }
    return list[0];
}

#[test]
fn to_vec_test() {
    let test_data = 0b1;
    let mut result = [0; 32];
    add_to_vec(&mut result, test_data, 1);
    assert_eq!(result[0], 1);
}

#[test]
fn diagnostic_test() {
    let example_data = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
    let report = diagnostic(example_data.as_ref(), 5);
    assert_eq!(report.gamma, 22);
    assert_eq!(report.epsilon, 9);
    assert_eq!(report.co2_scrubber, 10);
    assert_eq!(report.oxygen, 23);
}

#[test]
fn is_bit_set_test() {
    assert_eq!(is_bit_set(1, 0), true);
    assert_eq!(is_bit_set(0b01010, 0), false);
    assert_eq!(is_bit_set(0b01010, 1), true);
    assert_eq!(is_bit_set(0b01010, 2), false);
}

#[test]
fn get_common_value_test() {
    let example_data = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
    let acc = get_common_value(&example_data, 4);
    assert_eq!(acc, 2);
}
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

// Main, TODO: put into test
// let path = env!("CARGO_MANIFEST_DIR").to_string() + "/input/day3.csv";
// let file = File::open(path).unwrap();
// let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
// let list : Result<Vec<DiagnosticLine>, _ > = rdr.deserialize().collect();
// let list : Vec<u32> = list.unwrap().into_iter().map( |x| x.0 as u32).collect();
// let report = diagnostic(list.as_slice(), 12);
// println!("Result of day 3.1 is gamma = {}, epsilon = {}, power_consumption = {}", report.gamma, report.epsilon, report.gamma * report.epsilon);
// println!("Result of day 3.2 is oxygen = {}, co2 = {}, life support rating = {}", report.oxygen, report.co2_scrubber, report.oxygen * report.co2_scrubber);
