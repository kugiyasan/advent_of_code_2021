use std::{fs::File, io::Read};

fn main() {
    let file_path = "input";
    let mut s = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let levels: Vec<i32> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();

    // let size = 2; // part 1
    let size = 4; // part 2

    let increase_count: usize = levels
        .windows(size)
        .filter(|x| x.first().unwrap() < x.last().unwrap())
        .count();

    println!("{increase_count}");
}
