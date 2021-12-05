use std::env;
use std::fs::File;


fn times_it_increased( floor_height : Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last = i32::MAX;
    for i in floor_height {
        if i > last {
            count = count + 1;
        }
        last = i;
    }
    count
}
fn sliding_window(floor_height : Vec<i32>) -> Vec<i32> {
    floor_height.into_boxed_slice().windows(3).map(|window| window.into_iter().sum()).collect()
}
#[test]
fn sliding_window_test() {
    let example = vec![199, 200, 208, 210, 200 , 207, 240, 269, 260 , 263];
    let slide_window = [607, 618, 618, 617, 647, 716, 769, 792];
    assert_eq!(sliding_window(example), slide_window);
}
#[test]
fn times_it_increased_example() {
    let example = vec![199, 200, 208, 210, 200 , 207, 240, 269, 260 , 263];
    assert_eq!(times_it_increased(example), 7);
}
#[test]
fn day1_1_test() {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/input/day1-1";
    let file = File::open(path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let list : Result<Vec<i32>, _ > = rdr.deserialize().collect();
    let result = times_it_increased(list.unwrap());
    
    println!("Result of day 1.1 is {}", result);
    assert_eq!(result, 1477);
}
#[test]
fn day1_2_test() {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/input/day1-1";
    let file = File::open(path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let list : Result<Vec<i32>, _ > = rdr.deserialize().collect();
    let result = times_it_increased(sliding_window(list.unwrap()));
    
    println!("Result of day 1.2 is {}", result);
    assert_eq!(result, 1523);
}
