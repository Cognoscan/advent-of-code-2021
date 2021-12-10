fn main() {
    
    // Assumptions:
    // - The file exists
    // - The data is valid

    // Parse the data
    enum Command {
        Forward,
        Down,
        Up,
    }
    let data: Vec<(Command, i32)> = std::fs::read_to_string("./input.txt").unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let cmd = match split.next().unwrap().chars().next().unwrap() {
                'f' => Command::Forward,
                'd' => Command::Down,
                'u' => Command::Up,
                _ => panic!("Unknown command issued to sub!"),
            };
            let units = split.next().unwrap().parse::<i32>().unwrap();
            (cmd, units)
        })
        .collect();

    // Part 1
    let (horizontal, depth) = data.iter()
        .fold((0,0), |(horizontal, depth), (cmd, units)|
            match cmd {
                Command::Forward => (horizontal+units, depth),
                Command::Up => (horizontal, depth-units),
                Command::Down => (horizontal, depth+units),
            }
        );
    println!("Horizontal: {}, Depth: {}, Product of them = {}",
        horizontal, depth, horizontal * depth);

    // Part 2
    let (horizontal, depth, _) = data.iter()
        .fold((0,0,0), |(horizontal, depth, aim), (cmd, units)|
            match cmd {
                Command::Forward => (horizontal+units, depth + aim*units, aim),
                Command::Up => (horizontal, depth, aim-units),
                Command::Down => (horizontal, depth, aim+units),
            }
        );
    println!("Horizontal: {}, Depth: {}, Product of them = {}",
        horizontal, depth, (horizontal as u64) * (depth as u64));
}
