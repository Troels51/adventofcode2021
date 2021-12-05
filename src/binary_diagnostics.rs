
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

fn get_common_value(diagnostic_report: &[u32], position: usize) -> i32{
    let mut acc = 0;
    for number in diagnostic_report {
        let bit = (number >> position) & 1;
        if bit == 1 {
            acc += 1;
        }
        else {
            acc -= 1;
        }
    }
    acc
}

pub fn oxygen_generator_rating(diagnostic_report: &[u32], size: usize) -> u32 {
    let mut position = 0;
    let mut list = diagnostic_report.clone().to_vec();
    while list.len() > 1 {
        let accumulated = get_common_value(diagnostic_report, position);
        if  accumulated < 0 { //if 1 is more common
            list = list.into_iter().filter(|val| (val >> position) & 1 == 0 ).collect();
        }
        else if accumulated > 0 {
            list = list.into_iter().filter(|val| (val >> position) & 1 == 0 ).collect();
        } else {
            list = list.into_iter().filter(|val| (val >> position) & 1 == 1 ).collect();
        }
        position += 1;
    }
    return list[0];
}

pub fn co2_scrubber_rating(diagnostic_report: &[u32], size: usize) -> u32 {
    let mut position = 0;
    let mut list = diagnostic_report.clone().to_vec();
    while list.len() > 1 {

        let accumulated = get_common_value(diagnostic_report, position);

        dbg!(accumulated);
        let test_list : Vec<bool> = list.clone().into_iter().map(|val| ((val >> position) & 1) > 1 ).collect();
        dbg!(list.clone());
        dbg!(test_list);

        if accumulated < 0 { //if 1 is more common
            list = list.into_iter().filter(|val| (*val >> position) & 1 == 0 ).collect();
        }
        else if accumulated > 0 { // Zero more common
            list = list.into_iter().filter(|val| ((*val >> position) & 1) == 1 ).collect();
        } else {
            list = list.into_iter().filter(|val| (*val >> position) & 1 == 0 ).collect();
        }
        position += 1;

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
    assert_eq!(report.oxygen, 23);
    assert_eq!(report.co2_scrubber, 10);



}