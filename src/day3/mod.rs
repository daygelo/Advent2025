pub fn part_1(input: &str) -> u32 {
    let mut total = 0;
    
    for line in input.lines() {
        let numbers = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut tens_digit = numbers[0];
        let mut ones_digit = numbers[1];
        
        for i in 1..numbers.len() {
            if numbers[i] > ones_digit {
                ones_digit = numbers[i];
            }

            if numbers[i] > tens_digit && (i + 1) < numbers.len() {
                tens_digit = numbers[i];
                ones_digit = numbers[i + 1];
            }
        }

        total += tens_digit * 10 + ones_digit;
    }

    total
}

pub fn part_2(input: &str) -> u64 {
    const LENGTH: usize = 12;
    let mut total = 0;
    
    for line in input.lines() {
        let numbers = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut digits = numbers[..LENGTH].to_owned();
        let mut last_displacement = 0;
        
        for i in 1..numbers.len() {
            for j in 0..LENGTH {
                let numbers_end = i + LENGTH - j;
                let displacement = i as i32 - j as i32;

                if numbers[i] > digits[j] && i > j && numbers_end - 1 < numbers.len() && displacement >= last_displacement {
                    digits[j..].copy_from_slice(&numbers[i..numbers_end]);
                    last_displacement = displacement;
                    break;
                }
            }
        }
        
        for i in 0..digits.len() {
            total += digits[i] as u64 * 10_u64.pow(11 - i as u32);
        }
    }

    total
}