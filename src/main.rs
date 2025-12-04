mod day3;

fn main() {
    let input = std::fs::read_to_string("src/day3/input.txt").unwrap();
    println!("{} {}", day3::part_1(&input), day3::part_2(&input));
}
