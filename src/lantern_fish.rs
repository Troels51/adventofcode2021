fn lantern_fish_day(fish: &Vec<u64>) -> Vec<u64> {
    let spawning_fish = fish[0];
    let mut new_population = fish[1..9].to_vec();

    new_population[6] += spawning_fish;
    new_population.push(spawning_fish);
    new_population
}
fn fish_counting(fish_population: Vec<u64>) -> Vec<u64>{
    let mut fish_counting = vec![0;10];

    for fish in fish_population {
        fish_counting[fish as usize] += 1;
    }
    fish_counting
}
fn model_fish() {
    let fish_population = fish_population_dataset();
    // The count of number of fish with that index counter
    let lantern_fish :Vec<u64> = fish_counting(fish_population);
    let result = iterate(lantern_fish, |fish| lantern_fish_day(fish)).nth(256);
    let sum : u64 = result.unwrap().into_iter().sum();
    dbg!(sum);
}
#[test]
fn example_fish() {
    // The count of number of fish with that index counter
    let lantern_fish :Vec<u64> = vec![0,1,1,2,1,0,0,0,0,0];
    let result = iterate(lantern_fish, |fish| lantern_fish_day(fish)).nth(80);
    let sum : u64 = result.unwrap().into_iter().sum();
    assert_eq!(sum, 5934)
}
fn fish_population_dataset() -> Vec<u64> {
    vec![1,1,1,2,1,1,2,1,1,1,5,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1,4,1,1,1,1,3,1,1,3,1,1,1,4,1,5,1,3,1,1,1,1,1,5,1,1,1,1,1,5,5,2,5,1,1,2,1,1,1,1,3,4,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,5,4,1,1,1,1,1,5,1,2,4,1,1,1,1,1,3,3,2,1,1,4,1,1,5,5,1,1,1,1,1,2,5,1,4,1,1,1,1,1,1,2,1,1,5,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,3,1,1,3,1,3,1,4,1,5,4,1,1,2,1,1,5,1,1,1,1,1,5,1,1,1,1,1,1,1,1,1,4,1,1,4,1,1,1,1,1,1,1,5,4,1,2,1,1,1,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1,1,4,1,1,1,2,1,4,1,1,1,1,1,1,1,1,1,4,2,1,2,1,1,4,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,3,2,1,4,1,5,1,1,1,4,5,1,1,1,1,1,1,5,1,1,5,1,2,1,1,2,4,1,1,2,1,5,5,3]
}