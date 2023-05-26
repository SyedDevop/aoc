use day_04::process_part2;

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", process_part2(&file));
}
