fn main() {
    let raw = std::fs::read_to_string("./input.txt").unwrap();
    //let raw = std::fs::read_to_string("./example.txt").unwrap();

    let mut incomplete = Vec::new();
    let mut stack = Vec::with_capacity(128);
    let score = raw.lines().fold(0, |acc, s| {
        stack.clear();
        let mut score = 0;
        for c in s.chars() {
            if matches!(c, '[' | '{' | '(' | '<') {
                stack.push(c);
            }
            else {
                let open = stack.pop().unwrap();
                match (open, c) {
                    ('(',')') | ('[',']') | ('<','>') | ('{','}') => (),
                    (_, ')') => { score = 3; break; },
                    (_, ']') => { score = 57; break; },
                    (_, '}') => { score = 1197; break; },
                    (_, '>') => { score = 25137; break; },
                    _ => { panic!("Unexpected character in parsing engine"); },
                }
            }
        }
        if score == 0 && !stack.is_empty() {
            incomplete.push(s);
        }
        acc + score
    });
    println!("Error score: {}", score);

    let mut scores: Vec<u64> = incomplete.iter().map(|s| {
        stack.clear();
        for c in s.chars() {
            if matches!(c, '[' | '{' | '(' | '<') {
                stack.push(c);
            }
            else {
                let open = stack.pop().unwrap();
                if !matches!((open, c), ('(',')') | ('[',']') | ('<','>') | ('{','}')) {
                    println!("Got {} and {}", open, c);
                    panic!("This is the wrong type of line to end!!");
                }
            }
        }
        stack.iter().rev().fold(0, |acc, c| match c {
            '(' => 5 * acc + 1,
            '[' => 5 * acc + 2,
            '{' => 5 * acc + 3,
            '<' => 5 * acc + 4,
            _ => panic!("hey I got a {}, unexpected. I will die now", c),
        })
    })
    .collect();
    scores.sort_unstable();

    println!("Median score of incomplete lines: {}", scores[scores.len()/2]);



}
