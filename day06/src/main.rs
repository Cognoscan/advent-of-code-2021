fn main() {
    let mut age_sets = [0u64; 10];
    std::fs::read_to_string("./input.txt").unwrap()
        .trim()
        .split(',')
        .for_each(|s| age_sets[s.parse::<usize>().unwrap()] += 1);
    println!("Starting generation set: {:?}", age_sets);

    for i in 0..256 {
        let fish = age_sets[i % 10];
        age_sets[i % 10] = 0;
        age_sets[(i+7) % 10] += fish;
        age_sets[(i+9) % 10] += fish;
        if i == 79 {
            println!("After 80 days: {:?}", age_sets);
            println!("Total = {}", age_sets.iter().sum::<u64>());
        }
    }
    println!("After 256 days: {:?}", age_sets);
    println!("Total = {}", age_sets.iter().sum::<u64>());
}
