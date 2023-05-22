fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", &file[0..10]);
}
