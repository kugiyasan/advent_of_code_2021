use std::{fs::File, io::Read};

fn main() {
    let file_path = "input";
    let mut s = String::new();
    File::open(file_path)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let bits: Vec<&str> = s.trim().split('\n').collect();
    let number_of_bits = bits[0].len();

    let most_common_bits = bits.iter().fold(vec![0; number_of_bits], |mut acc, b| {
        for (i, c) in b.chars().enumerate() {
            if c == '1' {
                acc[i] += 1;
            }
        }
        acc
    });

    let gamma_rate: String = most_common_bits
        .iter()
        .map(|&b| if b > bits.len() / 2 { '1' } else { '0' })
        .collect();

    let epsilon_rate: String = most_common_bits
        .iter()
        .map(|&b| if b > bits.len() / 2 { '0' } else { '1' })
        .collect();

    // part 2
    let oxygen_generator_rating: String = bits
        .iter()
        .filter(|b| b.starts_with(gamma_rate.chars().next().unwrap()))
        .collect();
    // let co2_scrubber_rating;

    let gamma_rate = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("power consumption = {}", gamma_rate * epsilon_rate);

    let oxygen_generator_rating = i32::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    println!(
        "life support rating = {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}
