use day_10::process_part1;

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", process_part1(&file));
}
