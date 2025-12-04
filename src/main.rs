mod day3;

fn main() {
    let test_input = std::fs::read_to_string("src/day3/test.txt").unwrap();
    let puzzle_input = std::fs::read_to_string("src/day3/input.txt").unwrap();
    
    println!(
        "{:<28}puzzle\npart 1: {:<20}part 1: {}\npart 2: {:<20}part 2: {}",
        "test",
        day3::part_1(&test_input),
        day3::part_1(&puzzle_input),
        day3::part_2(&test_input),
        day3::part_2(&puzzle_input)
    );
}
