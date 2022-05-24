use std::str::FromStr;
use std::{fs::File, io::Read};

enum Direction {
    Down(u32),
    Forward(u32),
    Up(u32),
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let direction = s.next().unwrap();
        let x = s.next().unwrap().parse().unwrap();
        match direction {
            "down" => Ok(Direction::Down(x)),
            "forward" => Ok(Direction::Forward(x)),
            "up" => Ok(Direction::Up(x)),
            _ => Err(()),
        }
    }
}

fn main() {
    let file_path = "input";
    let mut s = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let directions: Vec<Direction> = s.trim().split('\n').map(|x| x.parse().unwrap()).collect();

    // part 1
    // let (horizontal_position, depth) =
    //     directions
    //         .iter()
    //         .fold((0, 0), |(pos, depth), direction| match direction {
    //             Direction::Down(x) => (pos, depth + x),
    //             Direction::Forward(x) => (pos + x, depth),
    //             Direction::Up(x) => (pos, depth - x),
    //         });

    // println!("{}", horizontal_position * depth);

    // part 2
    let (horizontal_position, depth, _aim) =
        directions
            .iter()
            .fold((0, 0, 0), |(pos, depth, aim), direction| match direction {
                Direction::Down(x) => (pos, depth, aim + x),
                Direction::Forward(x) => (pos + x, depth + aim * x, aim),
                Direction::Up(x) => (pos, depth, aim - x),
            });

    println!("{}", horizontal_position * depth);
}
