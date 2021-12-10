use std::str::Lines;
use owo_colors::{OwoColorize, Style};


struct BingoCard {
    nums: [[u8; 5]; 5],
    set: [[bool; 5]; 5],
}

impl BingoCard {
    fn load(lines: &mut Lines) -> Self {
        let mut card = BingoCard {
            nums: [[0;5];5],
            set: [[false;5];5],
        };
        for i in 0..5 {
            for (j, num) in lines.next().unwrap().split_ascii_whitespace().enumerate() {
                card.nums[i][j] = num.parse().unwrap();
            }
        }
        card
    }

    fn mark(&mut self, num: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.nums[i][j] == num {
                    self.set[i][j] = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u16 {
        self.nums.iter().flatten()
            .zip(self.set.iter().flatten())
            .fold(0, |acc, (&num,&set)| if !set { acc + num as u16 } else { acc })
    }

    fn check(&self) -> bool {
        // Rows
        for i in 0..5 {
            let mut test = true;
            for j in 0..5 { test = test && self.set[i][j]; }
            if test { return true; }
        }
        // Columns
        for i in 0..5 {
            let mut test = true;
            for j in 0..5 { test = test && self.set[j][i]; }
            if test { return true; }
        }
        false
    }

}

fn main() {

    // Read the file
    let raw_data = std::fs::read_to_string("./input.txt").unwrap();
    let mut lines = raw_data.lines();

    // Read the bingo calls
    let seq: Vec<u8> = lines.next().unwrap().split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    // Read the bingo cards
    let mut cards = Vec::new();
    while lines.next().is_some() {
        cards.push(Box::new(BingoCard::load(&mut lines)));
    }

    // find first & last
    let mut found_first = false;
    for num in seq.iter().copied() {
        for card in cards.iter_mut() {
            card.mark(num);
        }
        if !found_first {
            for card in cards.iter() {
                if card.check() {
                    let sum = card.sum_unmarked();
                    println!("First winner: sum_unmarked = {}, number = {}, product = {}", sum, num, (sum as u32) * (num as u32));
                    found_first = true;
                    break;
                }
            }
        }
        else if cards.len() > 1 {
            cards.retain(|c| !c.check());
        }
        else if cards[0].check() {
            let sum = cards[0].sum_unmarked();
            println!("Last winner: sum_unmarked = {}, number = {}, product = {}", sum, num, (sum as u32) * (num as u32));
            break;
        }
    }

}
