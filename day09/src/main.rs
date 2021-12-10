fn main() {
    // Load data
    //let mut map: Vec<Vec<u16>> = std::fs::read_to_string("./example.txt").unwrap()
    let mut map: Vec<Vec<u16>> = std::fs::read_to_string("./input.txt").unwrap()
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u16).collect())
        .collect();

    // Find low points
    let mut low_points = Vec::new();
    let height = map.len();
    let width = map[0].len();
    for y in 0..height {
        for x in 0..width {
            let v = map[y][x];
            if x > 0 && map[y][x-1] <= v { continue; }
            if x < width-1 && map[y][x+1] <= v { continue; }
            if y > 0 && map[y-1][x] <= v { continue; }
            if y < height-1 && map[y+1][x] <= v { continue; }
            low_points.push((x,y,v));
        }
    }
    println!("Low points: {}", low_points.len());
    println!("Risk level sum: {}", low_points.iter().map(|x| x.2 as usize).sum::<usize>() + low_points.len());

    // Ok, let's get nutso here. I'm going to build a second map, and ping-pong between map sets 
    // until I have a fully-filled map. This would map (ha) pretty easily to a GPU kernel as well.
    // This will flood-fill by checking neighbor squares and "populating the current square with 
    // the value if it has been filled already.
    //
    // As a GPU kernel, this would be like any nonlinear filter being convolved across an image. 

    // Start by marking each low point with a unique value
    for (i, (x,y,_)) in low_points.iter().copied().enumerate() {
        map[y][x] = (i as u16) | 0x8000;
    }
    let mut map_new = map.clone();
    let mut iter = 0;
    let mut changed = 1; // We could use a bool, but I wanted to make sure it was changing values as it went.
    while changed > 0 {
        iter+=1;
        changed = 0;
        // Mark current square with the value of a filled neighbor
        for y in 0..height {
            for x in 0..width {
                if map[y][x] >= 9 { continue; }
                if x > 0 && ((map[y][x-1] & 0x8000) == 0x8000) { map_new[y][x] = map[y][x-1]; changed += 1; continue; }
                if x < width-1 && ((map[y][x+1] & 0x8000) == 0x8000) { map_new[y][x] = map[y][x+1]; changed += 1; continue; }
                if y > 0 && ((map[y-1][x] & 0x8000) == 0x8000) { map_new[y][x] = map[y-1][x]; changed += 1; continue; }
                if y < height-1 && ((map[y+1][x] & 0x8000) == 0x8000) { map_new[y][x] = map[y+1][x]; changed += 1; continue; }
            }
        }
        map.iter_mut().flatten().zip(map_new.iter().flatten()).for_each(|(v,v_new)| *v = *v_new);
        print!("{} ", changed);
    }
    println!();
    println!("Looped {} times", iter);

    let mut basins: Vec<usize> = (0..low_points.len()).map(|i| {
        map.iter().flatten().fold(0, |acc, v| acc + if *v == ((i as u16) | 0x8000) { 1 } else { 0 })
    }).collect();
    basins.sort_unstable();

    let len = basins.len();
    let prod = basins[len-1] * basins[len-2] * basins[len-3];
    println!("Product of 3 largest basins: {}", prod);

}
