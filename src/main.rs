mod day2;

fn main() {
    let input = std::fs::read_to_string("src/day2/input.txt").unwrap();
    println!("{} {}", day2::part_1(&input), day2::part_2(&input));
}
