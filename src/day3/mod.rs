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
    let mut total = 0;
    
    for line in input.lines() {
        let numbers = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut digits = numbers[..12].to_owned();
        let mut displacement = 0;
        
        for i in 1..numbers.len() {
            for j in 0..digits.len() {
                let slice_end = i + digits.len() - j;
                if numbers[i] > digits[j] && i >= j + displacement && slice_end - 1 < numbers.len() {
                    digits[j..].copy_from_slice(&numbers[i..slice_end]);
                    displacement = i - j;
                    break;
                }
            }
        }
        
        for i in 0..digits.len() {
            total += digits[i] as u64 * 10_u64.pow((digits.len() - 1 - i) as u32);
        }
    }

    total
}