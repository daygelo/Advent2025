pub fn part_1(input: &str) -> u64 {
    let mut total = 0;

    for range in input.split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        
        for x in start..=end {
            let x_str = x.to_string();
            let (part1, part2) = x_str.split_at(x_str.len() / 2);
            
            if part1 == part2 {
                total += x;
            }
        }
    }

    total
}

pub fn part_2(input: &str) -> u64 {
    let mut total = 0;

    for range in input.split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        
        for x in start..=end {
            let x_str = x.to_string();
            
            for multiple in 1..=(x_str.len() / 2) {
                if x_str.len() % multiple != 0 {
                    continue;
                }

                let mut i = 1;
                let is_valid = loop {
                    if i >= x_str.len() / multiple {
                        break true;
                    }

                    if &x_str[(multiple * i)..(multiple * (i + 1))] != &x_str[0..multiple] {
                        break false;
                    }

                    i += 1;
                };

                if is_valid {
                    total += x;
                    break;
                }
            }
        }
    }

    total
}