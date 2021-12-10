fn main() {

    let mut data: Vec<i32> = std::fs::read_to_string("./input.txt").unwrap()
    //let data: Vec<i32> = std::fs::read_to_string("./example.txt").unwrap()
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    data.sort_unstable();
    let median = if data.len() % 2 == 1 {
        data[data.len()/2]
    }
    else {
        (data[data.len()/2-1] + data[data.len()/2]) / 2
    };
    let mean = data.iter().sum::<i32>() / data.len() as i32;

    let fuel_median = data.iter().fold(0, |acc,&x| acc + (x-median).abs());
    let fuel_mean = data.iter().fold(0, |acc,&x| {
        let diff = (x-mean).abs();
        acc + (diff * (diff+1) / 2)
    });

    println!("Part 1: median = {}, fuel = {}", median, fuel_median);
    println!("Part 2: mean = {}, fuel = {}", mean, fuel_mean);

}
