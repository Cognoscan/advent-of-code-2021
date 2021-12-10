fn main() {
    // Parse the data
    let data: String = std::fs::read_to_string("./input.txt").unwrap();
    let bit_len = data
        .lines()
        .fold(0, |acc,line| line.trim().len().max(acc));
    let data: Vec<u64> = data
        .lines()
        .map(|line| u64::from_str_radix(line, 2).unwrap())
        .collect();

    // Part 1
    let mut ones_count = vec![0; bit_len];
    for v in data.iter() {
        for (i, count) in ones_count.iter_mut().enumerate() {
            *count += (v >> i) & 1;
        }
    }
    let mut gamma = 0;
    for (i, &count) in ones_count.iter().enumerate() {
        if 2*count == (data.len() as u64) {
            panic!("Puzzle doesn't specify what to do if the count is exactly split (is for bit {})",i);
        }
        if 2*count >= (data.len() as u64) {
            gamma |= 1<<i;
        }
    }
    let epsilon = !gamma & (u64::MAX >> (64-bit_len));
    println!("gamma={}, epsilon={}, result={}", gamma, epsilon, gamma*epsilon);

    // Part 2
    // Oxygen
    let mut finder = data.clone();
    let mut iter = bit_len-1;
    while finder.len() > 1 {
        let count = finder.iter().fold(0, |acc, v| acc + ((v>>iter) & 1));
        let common_bit = u64::from(2*count >= (finder.len() as u64));
        finder.retain(|v| ((v>>iter) & 1) == common_bit);
        if iter == 0 {
            if finder.len() > 1 {
                panic!("Can't find the oxygen generator rating");
            }
        }
        else {
            iter -= 1;
        }
    }
    let oxygen = finder.get(0).expect("Oxygen generator rating fetch got 0 results");
    // CO2
    let mut finder = data;
    let mut iter = bit_len-1;
    while finder.len() > 1 {
        let count = finder.iter().fold(0, |acc, v| acc + ((v>>iter) & 1));
        let common_bit = u64::from(2*count >= (finder.len() as u64));
        finder.retain(|v| ((v>>iter) & 1) != common_bit);
        if iter == 0 {
            if finder.len() > 1 {
                panic!("Can't find the oxygen generator rating");
            }
        }
        else {
            iter -= 1;
        }
    }
    let co2 = finder.get(0).expect("CO2 scrubber rating fetch got 0 results");

    println!("oxygen = {}, CO2 = {}, result = {}", oxygen, co2, oxygen * co2);
}
