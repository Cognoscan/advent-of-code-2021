fn main() {
    // Read the data in
    //let filename = "./example.txt";
    let filename = "./input.txt";
    let raw = std::fs::read_to_string(filename).unwrap();
    let (template, rules) = raw.split_once("\n\n").unwrap();

    // Parse ruleset into ([u8;2] -> u8)
    let rules: Vec<([u8;2],u8)> = rules.lines().map(|l| {
        let (key, val) = l.split_once(" -> ").unwrap();
        let key: [u8;2] = key.as_bytes().try_into().unwrap();
        (key, val.as_bytes()[0])
    }).collect();

    // Initialize pair list with starting data
    let mut list0 = vec![0u64; rules.len()]; 
    let mut list1 = vec![0u64; rules.len()]; 
    template.as_bytes().windows(2).for_each(|x| {
        list0[rules.iter().position(|(r,_)| r == x).unwrap()] = 1;
    });
    // Record the first & last to make the counts later on add up right
    let first = template.as_bytes().first().unwrap();
    let last = template.as_bytes().last().unwrap();

    // Create the ruleset for how one pair generates two new pairs
    let ruleset: Vec<(usize,usize)> = rules.iter().map(|(pair,out)| {
        (
            rules.iter().position(|(r,_)| *r == [pair[0],*out]).unwrap(),
            rules.iter().position(|(r,_)| *r == [*out,pair[1]]).unwrap(),
        )
    }).collect();
    
    // Iterate, generating new pairs from old pairs according to the ruleset
    for i in 0..40 {
        list1.iter_mut().for_each(|x| *x = 0);
        for (count, (dest0, dest1)) in list0.iter().zip(ruleset.iter()) {
            list1[*dest0] += count;
            list1[*dest1] += count;
        }
        std::mem::swap(&mut list0, &mut list1);

        // Print out the min/max at iterations 10 & 40
        if i == 9 || i == 39 {
            let mut counts = [0u64; 26];
            for (c, (pair,_)) in list0.iter().zip(rules.iter()) {
                counts[(pair[0]-b'A') as usize] += c;
                counts[(pair[1]-b'A') as usize] += c;
            }
            counts[(first-b'A') as usize] += 1;
            counts[(last-b'A') as usize] += 1;
            let (max, min) = counts.iter().copied().fold((0,u64::MAX), |(max,min), c| {
                (max.max(c), if c > 0 { min.min(c) } else { min })
            });
            println!("Max = {}, Min = {}, diff = {}", max/2, min/2, (max-min)/2);
        }
    }
}
