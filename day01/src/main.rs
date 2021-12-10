fn main() {

    // Assumptions:
    // - The file exists
    // - The data is valid (one u32 in ASCII per line of text)
    // - The first value in the data is not zero

    // Read & parse the file
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let data: Vec<u32> = f.lines()
        .map(|line| line.parse::<u32>().unwrap_or_default())
        .collect();

    // Part 1
    let (_, increases) = data
        .iter()
        .fold((0,0), |(prev, sum), &cur| (cur, if cur > prev { sum+1 } else { sum }));
    println!("Part 1: Total # of increases = {}", increases-1);

    // Part 2
    let (_, increases) = data
        .windows(3)
        .fold((0,0), |(prev, sum), cur| {
            let cur_sum = cur.iter().sum();
            (cur_sum, if cur_sum > prev { sum+1 } else { sum })
        });
    println!("Part 2: Total # of increases = {}", increases-1);
}
