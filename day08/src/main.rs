
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Digit {
    d: u8,
}

impl Digit {
    fn new() -> Self {
        Self { d: 0 }
    }

    fn print(&self) {
        if (self.d & 0x01) > 0 { print!("a"); }
        if (self.d & 0x02) > 0 { print!("b"); }
        if (self.d & 0x04) > 0 { print!("c"); }
        if (self.d & 0x08) > 0 { print!("d"); }
        if (self.d & 0x10) > 0 { print!("e"); }
        if (self.d & 0x20) > 0 { print!("f"); }
        if (self.d & 0x40) > 0 { print!("g"); }
    }

    fn read(s: &str) -> Self {
        let mut d = 0;
        for c in s.trim().chars() {
            match c {
                'a' => d |= 0x01,
                'b' => d |= 0x02,
                'c' => d |= 0x04,
                'd' => d |= 0x08,
                'e' => d |= 0x10,
                'f' => d |= 0x20,
                'g' => d |= 0x40,
                _ => (),
            }
        }
        Self { d }
    }

    fn is_1478(&self) -> bool {
        matches! (self.d.count_ones(), 2 | 4 | 3 | 7)
    }
}

struct DigitMap {
    map: [Digit; 10]
}

impl DigitMap {
    fn build(raw: &[Digit; 10]) -> Self {
        let mut map = [Digit::new(); 10];
        let count_ones = raw.map(|d| d.d.count_ones());
        // Get initial hits we need to fill out the rest of the map
        map[1] = raw[count_ones.iter().position(|c| *c == 2).unwrap()];
        map[4] = raw[count_ones.iter().position(|c| *c == 4).unwrap()];
        map[7] = raw[count_ones.iter().position(|c| *c == 3).unwrap()];
        map[8] = raw[count_ones.iter().position(|c| *c == 7).unwrap()];
        // Fill out the rest
        for (&count, raw) in count_ones.iter().zip(raw.iter().copied()) {
            if count == 5 {
                if (map[7].d & raw.d).count_ones() == 3 {
                    map[3] = raw;
                }
                else if (map[4].d & raw.d).count_ones() == 3 {
                    map[5] = raw;
                }
                else {
                    map[2] = raw;
                }
            }
            else if count == 6 {
                if (map[4].d & raw.d).count_ones() == 4 {
                    map[9] = raw;
                }
                else if (map[1].d & raw.d).count_ones() == 2 {
                    map[0] = raw;
                }
                else {
                    map[6] = raw;
                }
            }
        }
        DigitMap { map }
    }

    fn map(&self, d: Digit) -> u8 {
        self.map.iter().position(|c| *c == d).unwrap() as u8
    }
}

fn main() {
    // We'll map a-g to 0-6, and create an ordered set.
   
    //let raw: Vec<([Digit; 10], [Digit; 4])>  = std::fs::read_to_string("./example.txt")
    let raw: Vec<([Digit; 10], [Digit; 4])>  = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (digits0, digits1) = line.split_once('|').unwrap();
            let mut seen_digits = [Digit::new(); 10];
            let mut out_digits = [Digit::new(); 4];
            digits0.trim().split(' ').zip(seen_digits.iter_mut()).for_each(|(s,d)| *d = Digit::read(s));
            digits1.trim().split(' ').zip(out_digits.iter_mut()).for_each(|(s,d)| *d = Digit::read(s));
            (seen_digits, out_digits)
        })
        .collect();

    let num_1478 = raw.iter().fold(0, |acc, (_, digits)| acc + digits.iter().fold(0, |acc, d| acc + u32::from(d.is_1478())));
    println!("Number of 1/4/7/8 digits in output: {}", num_1478);

    println!("Decoding the screen...");
    let mut v_sum = 0;
    for (map, out) in raw.iter() {
        let map = DigitMap::build(map);
        let val = out.iter().fold(0, |acc, d| 10*acc + (map.map(*d) as u32));
        println!("{}", val);
        v_sum += val;
    }

    println!("Total = {}", v_sum);
}
