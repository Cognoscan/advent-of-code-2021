use owo_colors::OwoColorize;

const PRINT: bool = true;

fn print_map(map: &[Vec<u8>]) {
    map.iter().for_each(|row| {
        row.iter().copied().for_each(|v| match v {
            0 => print!("{}", "0".truecolor(10,10,10)),
            1..=9 => print!("{}",v.truecolor(28*v,28*v,0)),
            _ => print!("{}", "X".truecolor(255,255,0)),
        });
        println!();
    });
}

fn main() {
    //let filename = "./example.txt";
    let filename = "./input.txt";
    let mut map: Vec<Vec<u8>> = std::fs::read_to_string(filename).unwrap()
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    if PRINT { print_map(&map); }

    let height = map.len();
    let width = map[0].len();
    let mut flash_count = 0;
    let mut count = 0;
    let mut map_new = map.clone();
    loop {
        map.iter_mut().flatten().for_each(|v| *v += 1);
        map_new.iter_mut().flatten().for_each(|v| *v += 1);
        if PRINT {
            print!("\x1b[{}A\x1b[{}D", map.len(), map[0].len());
            print_map(&map);
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        let mut flashed = true;
        let mut flashes = 0;
        while flashed {
            flashed = false;
            for y in 0..height {
                for x in 0..width {
                    if map[y][x] > 9 {
                        flashed = true;
                        flashes += 1;
                        map_new[y][x] = 0;
                        if y > 0 {
                            if x > 0 && map_new[y-1][x-1] != 0 { map_new[y-1][x-1] += 1; }
                            if map_new[y-1][x] != 0 { map_new[y-1][x] += 1; }
                            if x < width-1 && map_new[y-1][x+1] != 0 { map_new[y-1][x+1] += 1; }
                        }
                        if x > 0 && map_new[y][x-1] != 0 { map_new[y][x-1] += 1; }
                        if x < width-1 && map_new[y][x+1] != 0 { map_new[y][x+1] += 1; }
                        if y < height-1 {
                            if x > 0 && map_new[y+1][x-1] != 0 { map_new[y+1][x-1] += 1; }
                            if map_new[y+1][x] != 0 { map_new[y+1][x] += 1; }
                            if x < width-1 && map_new[y+1][x+1] != 0 { map_new[y+1][x+1] += 1; }
                        }
                    }
                }
            }
            map.iter_mut().flatten().zip(map_new.iter().flatten()).for_each(|(v,v_new)| *v = *v_new);
            if PRINT && flashed {
                print!("\x1b[{}A\x1b[{}D", map.len(), map[0].len());
                print_map(&map);
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
        count += 1;
        if count <= 100 { flash_count += flashes; }
        if flashes == (width * height) { break; }
    }

    println!();
    println!("Total Number of flashes from iterations 1-100: {}", flash_count);
    println!("Number of cycles to fully synchronize: {}", count);

}
