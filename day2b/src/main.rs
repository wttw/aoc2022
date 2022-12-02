use std::{env, process};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: day2a input_file.txt");
        process::exit(1);
    };
    let filename: &String = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for (_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut fields = line.split_whitespace();
        let their_move = fields.next().unwrap().chars().next().unwrap() as u32 - 'A' as u32;
        let goal = fields.next().unwrap();
        let round = match goal {
            "X" => {
                // lose, my move is theirs - 1
                (their_move + 2) % 3 + 1
            },
            "Y" => {
                // draw, my move is theirs
                3 + their_move + 1
            },
            "Z" => {
                // win, my move is theirs + 1
                6 + (their_move + 1) % 3 + 1
            },
            _ => 0
        };
        // println!("round: {round}");
        score = score + round;
    }
    println!("{score}");
}