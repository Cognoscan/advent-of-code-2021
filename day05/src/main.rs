#[derive(Clone,Copy,Debug)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Clone,Copy,Debug)]
struct Line {
    p0: Point,
    p1: Point
}

fn main() {

    let lines: Vec<Line> = std::fs::read_to_string("./input.txt").unwrap()
        .lines()
        .map(|line| {
            let mut split = line
                .split("->")
                .map(|s| {
                    let mut split = s.split(',').map(|s| s.trim().parse::<u32>().unwrap());
                    Point { x: split.next().unwrap(), y: split.next().unwrap() }
                });
            Line { p0: split.next().unwrap(), p1: split.next().unwrap() }
        })
        .collect();

    let vh_lines: Vec<Line> = lines
        .iter()
        .filter_map(|l| {
            if l.p0.x == l.p1.x {
                Some(if l.p0.y > l.p1.y {
                    Line { p0: l.p1, p1: l.p0 }
                }
                else {
                    Line { p0: l.p0, p1: l.p1 }
                })
            }
            else if l.p0.y == l.p1.y {
                Some(if l.p0.x > l.p1.x {
                    Line { p0: l.p1, p1: l.p0 }
                }
                else {
                    Line { p0: l.p0, p1: l.p1 }
                })
            }
            else {
                None
            }
        })
        .collect();

    let (x_max, y_max) = vh_lines
        .iter()
        .fold((0,0), |acc, l| (acc.0.max(l.p0.x).max(l.p1.x), acc.1.max(l.p0.y).max(l.p1.y)));
    let width = x_max as usize + 1;
    let height = y_max as usize + 1;

    let mut map = vec![vec![0; height]; width];

    for line in vh_lines.iter() {
        if line.p0.x == line.p1.x {
            let x = line.p0.x;
            for y in line.p0.y ..= line.p1.y {
                map[x as usize][y as usize] += 1;
            }
        }
        else {
            let y = line.p0.y;
            for x in line.p0.x ..= line.p1.x {
                map[x as usize][y as usize] += 1;
            }
        }
    }
    let points: u32 = map.iter().flatten().map(|x| if *x > 1 { 1 } else { 0 }).sum();

    println!("Part 1: Number of intersection points = {}", points);

    let (x_max, y_max) = lines
        .iter()
        .fold((0,0), |acc, l| (acc.0.max(l.p0.x).max(l.p1.x), acc.1.max(l.p0.y).max(l.p1.y)));
    let width = x_max as usize + 1;
    let height = y_max as usize + 1;
    let mut map = vec![vec![0; height]; width];

    for line in lines.iter() {
        if line.p0.x == line.p1.x {
            if line.p1.y > line.p0.y {
                for y in line.p0.y ..= line.p1.y {
                    map[line.p0.x as usize][y as usize] += 1;
                }
            }
            else {
                for y in line.p1.y ..= line.p0.y {
                    map[line.p0.x as usize][y as usize] += 1;
                }
            }
        }
        else if line.p0.y == line.p1.y {
            if line.p1.x > line.p0.x {
                for x in line.p0.x ..= line.p1.x {
                    map[x as usize][line.p0.y as usize] += 1;
                }
            }
            else {
                for x in line.p1.x ..= line.p0.x {
                    map[x as usize][line.p0.y as usize] += 1;
                }
            }
        }
        else if line.p1.x > line.p0.x {
            if line.p1.y > line.p0.y {
                for (x,y) in (line.p0.x ..= line.p1.x).zip(line.p0.y ..= line.p1.y) {
                    map[x as usize][y as usize] += 1;
                }
            }
            else {
                for (x,y) in (line.p0.x ..= line.p1.x).zip((line.p1.y ..= line.p0.y).rev()) {
                    map[x as usize][y as usize] += 1;
                }
            }
        }
        else if line.p1.y > line.p0.y {
            for (x,y) in (line.p1.x ..= line.p0.x).rev().zip(line.p0.y ..= line.p1.y) {
                map[x as usize][y as usize] += 1;
            }
        }
        else {
            for (x,y) in (line.p1.x ..= line.p0.x).rev().zip((line.p1.y ..= line.p0.y).rev()) {
                map[x as usize][y as usize] += 1;
            }
        }
        
    }

    let points: u32 = map.iter().flatten().map(|x| if *x > 1 { 1 } else { 0 }).sum();
    println!("Part 2: Number of intersection points = {}", points);

}
