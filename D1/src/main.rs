use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = read_input("input.txt");
    let measurements: Vec<&str> = input.split("\r\n").collect();

    let mut increase_count = 0;
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut current_window = 0;
    let window_length = 3;

    for i in 0..measurements.len() {
        let current_measurement  = measurements[i].parse::<i32>().unwrap();
        let last_window = current_window;
        window.push_front(current_measurement);
        current_window += current_measurement;
        if window.len() > window_length { 
            let end = window.pop_back().unwrap();
            current_window -= end;
        }

        if i >= window_length {
            increase_count += if last_window < current_window { 1 } else { 0 };
        }
    } 
    println!("Result: {}", increase_count);
}

fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}