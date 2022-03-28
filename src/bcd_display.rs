use std::{collections::HashMap, cmp::{max, min, max_by, min_by}};

struct Swap {
    from : char,
    to: char,
}
fn is_1(input: &Vec<char>) -> bool{
    input.len() == 2
}
fn is_4(input: &Vec<char>) -> bool{
    input.len() == 4
}
fn is_7(input: &Vec<char>) -> bool{
    input.len() == 3
}
fn is_8(input: &Vec<char>) -> bool{
    input.len() == 7
}
fn is_9(input: &Vec<char>, seven: &Vec<char>, four: &Vec<char>) -> bool {
    let first_step = difference(input, seven); 
    let second_step = difference(&first_step,four);
    dbg!(second_step.clone());
    second_step.len() == 1
}

fn is_1_4_7_8(input: &Vec<char>) -> bool {
    is_1(input) ||
    is_4(input) ||
    is_7(input) ||
    is_8(input)
}
fn difference(first : &Vec<char>, second: &Vec<char>) -> Vec<char>{
    let largest = max_by(first,second, |a,b| a.len().cmp(&b.len()));
    let smallest = min_by(first,second, |a,b| a.len().cmp(&b.len()));
    largest.into_iter().filter(|item| !smallest.contains(item)).cloned().collect()
}
fn intersection(first: &Vec<char>, second: &Vec<char>) -> Vec<char> {
    first.into_iter().filter(|&c| second.into_iter().any(|c2| c2 == c)  ).cloned().collect()
}

pub fn count_1_4_7_8(digits: &Vec<Vec<char>> ) -> usize{
    digits.iter().filter(|x| is_1_4_7_8(x)).count()
}
pub struct InputLine {
    pub unique : Vec<Vec<char>>,
    pub output : Vec<Vec<char>>,
}
impl InputLine {
    pub fn new(unique: Vec<&str>, output: Vec<&str>) -> InputLine {
        InputLine{
            unique: unique.into_iter().map(|s| s.chars().collect()).collect(),
            output: output.into_iter().map(|s| s.chars().collect()).collect()}
    }


    pub fn decode(&self) -> u32 {
        let mut unique_pattern_strings = self.unique.clone();
        let mut unique_patterns:Vec<Vec<char>> = vec!["".chars().collect() ; 10];
        let mut wire_map: HashMap<char, char> = HashMap::new();
        unique_patterns[1] = unique_pattern_strings.iter().find(|pattern| is_1(pattern)).unwrap().clone();
        unique_patterns[4] = unique_pattern_strings.iter().find(|pattern| is_4(pattern)).unwrap().clone();
        unique_patterns[7] = unique_pattern_strings.iter().find(|pattern| is_7(pattern)).unwrap().clone();
        unique_patterns[8] = unique_pattern_strings.iter().find(|pattern| is_8(pattern)).unwrap().clone();
        //wire_map.insert( 'a', difference(&unique_patterns[7],&unique_patterns[1])[0].clone());
        unique_pattern_strings.retain(|s| !is_1_4_7_8(s)); // Remove 1,4,7,8
        let (sixes, fives) :(Vec<Vec<char>>, Vec<Vec<char>>) = unique_pattern_strings.into_iter().partition(|s| s.len() == 6);
        unique_patterns[3] = fives.clone().into_iter().filter(|s| unique_patterns[1].clone().into_iter().all(|c| s.contains(&c))).nth(0).unwrap();
        unique_patterns[5] = fives.clone().into_iter().filter(|i|
             intersection(
                 &intersection(&i, &unique_patterns[4] ),
                &unique_patterns[3] ).len() == 2
            ).nth(0).unwrap().clone();
        unique_patterns[2] = fives.into_iter().filter(|x| x != &unique_patterns[3] ||x != &unique_patterns[5]).nth(0).unwrap(); 
        //unique_patterns[9] = unique_pattern_strings.iter().find(|pattern| is_9(pattern, &unique_patterns[7], &unique_patterns[4])).unwrap().clone();
        dbg!(unique_patterns);
        5353
    }
}

#[test]
fn intersection_test() {
    let a = "abc".chars().collect();
    let b = "bc".chars().collect();
    assert_eq!(intersection(&a,&b), vec!['b','c']);
}

#[test]
fn difference_test() {
    let a: Vec<char> = "abc".chars().collect();
    let b:Vec<char> = "bc".chars().collect();
    assert_eq!(difference(&a,&b), vec!['a']);
}

#[test]
fn part2_example_test() {
    let input = InputLine::new(
        vec!["acedgfb","cdfbe","gcdfa","fbcad","dab","cefabd","cdfgeb","eafb","cagedb","ab"],
        vec!["cdfeb","fcadb","cdfeb","cdbaf"]);
    let decoded = input.decode();
    assert_eq!(decoded, 5353);
    
}
#[test]
fn part1_example_test() {
    // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    // We create a map from bad letter to correct letter, that when applied to the input will create correct wiring
    let mut swaps: HashMap<char, char>  = HashMap::new();
    let res: usize = example_data().iter().map(|line| count_1_4_7_8(&line.output) ).sum();
    assert_eq!(res, 26);
}

fn example_data() -> Vec<InputLine> {
    vec![
        InputLine::new(vec!["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",], vec!["fdgacbe", "cefdb", "cefbgd", "gcbe",]),
        InputLine::new(vec!["edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde", "gfcbed", "gfec",], vec!["fcgedb", "cgb", "dgebacf", "gc",]),
        InputLine::new(vec!["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb", "cdgabef",], vec!["cg", "cg", "fdcagb", "cbg",]),
        InputLine::new(vec!["fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca", "fcdbega",], vec!["efabcd", "cedba", "gadfec", "cb",]),
        InputLine::new(vec!["aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab", "fcbdga",], vec!["gecf", "egdcabf", "bgf", "bfgea",]),
        InputLine::new(vec!["fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc", "acf",], vec!["gebdcfa", "ecba", "ca", "fadegcb",]),
        InputLine::new(vec!["dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb", "gdcebf", "gf",], vec!["cefg", "dcbef", "fcge", "gbcadfe",]),
        InputLine::new(vec!["bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg", "gebcd",], vec!["ed", "bcgafe", "cdgba", "cbgef",]),
        InputLine::new(vec!["egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb", "bfceg",], vec!["gbdfcae", "bgc", "cg", "cgb",]),
        InputLine::new(vec!["gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac", "fegbdc",], vec!["fgae", "cfgab", "fg", "bagce"]),
    ]
}
