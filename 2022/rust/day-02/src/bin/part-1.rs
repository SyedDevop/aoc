use day_02::process_part1;

fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    // println!("{:?}", &file[0..10]);
    process_part1(&file);
}
