fn main() {
    // Parse the file
    //let filename = "example.txt";
    let filename = "input.txt";
    let raw = std::fs::read_to_string(filename).unwrap();
    let (dots, rules) = raw.split_once("\n\n").unwrap();
    let dots: Vec<(u16,u16)> = dots.lines().map(|l| {
        let (x,y) = l.split_once(',').unwrap();
        (x.parse().unwrap(),y.parse().unwrap())
    }).collect();
    let rules: Vec<(char,u16)> = rules.lines().map(|l| {
        let (fold, pos) = l.split_once('=').unwrap();
        (fold.chars().last().unwrap(), pos.parse().unwrap())
    }).collect();

    let mut width = dots.iter().map(|(x,_)| *x).max().unwrap() as usize + 1;
    let mut height = dots.iter().map(|(_,y)| *y).max().unwrap() as usize + 1;

    let mut paper = vec![vec![false; height]; width];
    dots.iter().for_each(|(x,y)| paper[*x as usize][*y as usize] = true);

    let mut first_fold = true;
    for (fold, pos) in rules.iter() {
        let pos = *pos as usize;
        match fold {
            'x' => {
                for x in pos..width {
                    for y in 0..height {
                        if paper[x][y] {
                            paper[x][y] = false;
                            paper[2*pos-x][y] = true;
                        }
                    }
                }
                width = pos;
            },
            'y' => {
                for x in 0..width {
                    for y in pos..height {
                        if paper[x][y] {
                            paper[x][y] = false;
                            paper[x][2*pos-y] = true;
                        }
                    }
                }
                height = pos;
            },
            _ => panic!("Fold instruction wasn't along x or y"),
        }
        if first_fold {
            first_fold = false;
            let visible_dots: u64 = paper.iter().flatten().fold(0, |acc,x| acc + u64::from(*x));
            println!("Total visible dots: {}", visible_dots);
        }
    }

    for y in 0..height {
        for x in 0..width {
            if paper[x][y] {
                print!("█");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}
