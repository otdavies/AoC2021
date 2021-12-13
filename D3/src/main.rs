use std::fs;

fn main() {
    // Gather input
    let input = read_input("input.txt");
    // Convert to integers
    let lines = input.lines();
    let numeric_diagnostic = lines.map(|x| { isize::from_str_radix(x, 2).unwrap() as u32}).collect::<Vec<u32>>();

    // Bit representation for calculating epsilon & gamma
    let mut bits = [0,0,0,0,0,0,0,0,0,0,0,0];

    // Fill bit array based on commonality of bits in each column, > 0 -> 1, < 0 -> 0
    for (i, c) in numeric_diagnostic.iter().enumerate() {
        for i in 0..bits.len() {
            bits[i] += if c & (1 << i) > 0 { 1 } else { -1};
        }
    };

    // Calculate epsilon & gamma
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, b) in bits.iter().enumerate() {
        if *b > 0 { gamma += 2_i32.pow(i as u32); }
        else { epsilon += 2_i32.pow(i as u32); }
    }

    // Calculate oxygen rating
    let mut oxygen_rating = numeric_diagnostic.clone();
    let mut offset: u32 = 0;
    while oxygen_rating.len() > 1 {
        let mut balance = 0;
        let left_side = (bits.len() as u32 - 1) - offset;

        oxygen_rating.iter().for_each(|c| { balance += if *c & (1 << left_side) > 0 { 1 } else { -1 }; });

        if balance >= 0 {
            oxygen_rating = oxygen_rating.into_iter().filter(|c| { *c & (1 << left_side) > 0 }).collect();
        }
        else { 
            oxygen_rating = oxygen_rating.into_iter().filter(|c| { *c & (1 << left_side) == 0 }).collect();
        } 
        offset += 1;
    };
    oxygen_rating.iter().for_each(|c|{println!("Oxygen Rating: {:05b} -> {}", c, c)});

    // Calculate oxygen rating, yes this is just copy-pasta. Not proud of it, but not worth the time to abstract
    let mut co2_rating = numeric_diagnostic.clone();
    let mut offset: u32 = 0;
    while co2_rating.len() > 1 {
        let mut balance = 0;
        let left_side = (bits.len() as u32 - 1) - offset;

        co2_rating.iter().for_each(|c| { balance += if *c & (1 << left_side) > 0 { 1 } else { -1 }; });
        
        if balance >= 0 {
            co2_rating = co2_rating.into_iter().filter(|c| { *c & (1 << left_side) == 0 }).collect();
        }
        else { 
            co2_rating = co2_rating.into_iter().filter(|c| { *c & (1 << left_side) > 0 }).collect();
        } 
        offset += 1;
    };
    co2_rating.iter().for_each(|c|{println!("Co2 Rating: {:05b} -> {}", c, c)});
    
    println!("Gamma {}, Epsilon {}", gamma, epsilon);
}

fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}