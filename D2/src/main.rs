use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut directions = input.lines();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    while let Some(row) = directions.next() {
        let mut column = row.split_whitespace();
        let direction = column.next().unwrap();
        let amount = column.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => { 
                horizontal += amount;
                depth += aim * amount;
            },
            _ => println!("Parsing bug!")
        }
    }
    println!("Result {}", horizontal * depth);
}

fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}