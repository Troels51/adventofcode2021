
fn to_vec(input: u32) -> Vec<u32> {
    let mut result = vec![0; 32];
    for i in 0..32 {
        result[i] = (input >> i) & 1;
    }
    result
}
pub fn diagnostic(input: &[u32] ) -> (u32, u32) {
    let accumulator : Vec<u32> = vec![0; 32];
    let gamma : Vec<u32> = input.into_iter().fold(accumulator, |acc, x| acc.into_iter().zip(to_vec(*x).into_iter().map(|(a,b)| a + b)).collect());
    (0,0)
    //(gamma, !gamma) 
}
#[test]
fn to_vec_test() {
    let test_data = 0b1;
    let mut result = vec![0; 32];
    result[0] = 1;
    assert_eq!(to_vec(test_data), result);
    result[2] = 1;
    assert_eq!(to_vec(0b101), result);

}

#[test]
fn diagnostic_test() {
    let example_data = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
    let (gamma, epsilon) = diagnostic(example_data.as_ref());
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);


}