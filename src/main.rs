mod day4;

fn main() {
    let test_input = std::fs::read_to_string("src/day4/test.txt").unwrap();
    let puzzle_input = std::fs::read_to_string("src/day4/input.txt").unwrap();
    
    println!(
        "{:<28}puzzle\npart 1: {:<20}part 1: {}\npart 2: {:<20}part 2: {}",
        "test",
        day4::part_1(&test_input),
        day4::part_1(&puzzle_input),
        day4::part_2(&test_input),
        day4::part_2(&puzzle_input)
    );
}
