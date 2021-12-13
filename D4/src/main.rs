use std::{fs, collections::HashMap};

fn main() {
    // Gather input
    let input = read_input("input.txt");

    // Convert to integers
    let mut lines = input.lines();
    let draws = lines.next().unwrap_or("nope").split(",").map(|c| c.parse().unwrap()).collect::<Vec<u32>>();
    println!("{:?}", draws);

    let mut boards: Vec<Board> = Vec::new();
    let mut bucket : Vec<u32> = Vec::with_capacity(25);

    // Populate boards
    while let Some(line) = lines.next() {
        if line.is_empty() && !bucket.is_empty() {
            boards.push(Board::new(&bucket));
            bucket.clear();
        } else {
            let mut split_line = line.split_whitespace();
            while let Some(word) = split_line.next() {
                let num = word.parse().unwrap_or(0);
                bucket.push(num);
            }
        }
    }
    boards.push(Board::new(&bucket));
    bucket.clear();

    let mut draws_iter = draws.iter();

    // Add hits to all boards
    while let Some(draw) = draws_iter.next() {
        boards.iter_mut().for_each(|b| {
            if !b.has_won && b.mark(*draw) { 
                println!("This board {:?} wins!", b.hits);
            };
        });
    }
}

fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}

struct Board {
    hits : Vec<u32>,
    map : HashMap<u32, usize>,
    win_condition_horizontal: [u32; 5],
    win_condition_vertical: [u32; 5],
    total_board_sum: u32,
    hit_board_sum: u32,
    has_won: bool,
}

impl Board {
    pub fn new(positions: &Vec<u32>) -> Self {
        Self {
            hits: Vec::new(),
            map: HashMap::<u32, usize>::from_iter(positions.into_iter().enumerate().map(|(i, c)| (*c as u32, i as usize))),
            win_condition_horizontal: [0; 5],
            win_condition_vertical: [0; 5],
            total_board_sum: positions.iter().sum(),
            hit_board_sum: 0,
            has_won: false,
        }
    }

    pub fn mark(&mut self, h: u32) -> bool {
        let lookup = self.map.get(&h);
        let i = match lookup {
            Some(p) => { 
                self.hits.push(*p as u32);
                self.hit_board_sum += h;
                *p as i32
            },
            None => -1,
        };

        if i < 0 { return false }; 
        
        let x = i % 5;
        self.win_condition_horizontal[x as usize] += 1;

        let y = i / 5;
        self.win_condition_vertical[y as usize] += 1;

        if self.win_condition_vertical[y as usize] > 4 || self.win_condition_horizontal[x as usize] > 4 {
            println!("Victory value: {}", (self.total_board_sum - self.hit_board_sum) * h);
        }

        self.has_won = self.win_condition_vertical[y as usize] > 4 || self.win_condition_horizontal[x as usize] > 4;
        return self.has_won;
    }
}