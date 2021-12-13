use std::collections::HashMap;

fn count_paths(map: &HashMap<&str,Vec<&str>>, mut hit_twice: bool) -> u64 {
    let mut path_count = 0;
    let mut path = Vec::new();
    let mut cur_cave = "start";
    let mut cur_idx = 0;
    loop {
        let cave = map.get(cur_cave).unwrap();
        if let Some(next_cave) = cave.get(cur_idx) {
            if *next_cave == "end" {
                path_count += 1;
                cur_idx += 1;
            }
            else if *next_cave == "start" {
                cur_idx += 1;
            }
            else if next_cave.chars().next().unwrap().is_ascii_lowercase() &&
                path.iter().any(|(c, _, _)| c == next_cave) {
                    if hit_twice {
                        cur_idx += 1;
                    }
                    else {
                        path.push((cur_cave, cur_idx+1, false));
                        hit_twice = true;
                        cur_cave = next_cave;
                        cur_idx = 0;
                    }
            }
            else {
                path.push((cur_cave, cur_idx+1, hit_twice));
                cur_cave = next_cave;
                cur_idx = 0;
            }
        }
        else if let Some((next_cave, next_idx, next_hit_twice)) = path.pop() {
            cur_cave = next_cave;
            cur_idx = next_idx;
            hit_twice = next_hit_twice;
        }
        else {
            break;
        }
    }
    path_count
}

fn main() {
    //let filename = "example.txt";
    let filename = "input.txt";
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let raw = std::fs::read_to_string(filename).unwrap();
    raw.lines()
        .for_each(|line| {
            let (p0, p1) = line.split_once('-').unwrap();
            map.entry(p0).or_default().push(p1);
            map.entry(p1).or_default().push(p0);
        });

    println!("Part 1: Total number of possible paths: {}", count_paths(&map, true));
    println!("Part 2: Total number of possible paths: {}", count_paths(&map, false));
}
