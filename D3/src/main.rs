use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut diagnostic = input.lines();
    let mut bits = [0,0,0,0,0,0,0,0,0,0,0,0];
    
    while let Some(c) = diagnostic.next() {
        let code = isize::from_str_radix(c, 2).unwrap();
        for i in 0..bits.len() {
            bits[i] += if code & (1 << i) > 0 { 1 } else { -1};
        }
    }

    

    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, b) in bits.iter().enumerate() {
        if b > &0 { 
            gamma += 2_i32.pow(i as u32);
        }
        else { epsilon += 2_i32.pow(i as u32); }
    }

    println!("Gamma {}, Epsilon {}", gamma, epsilon);
}

fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}