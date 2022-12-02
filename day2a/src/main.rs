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
        let their_move = fields.next().unwrap().chars().next().unwrap() as u32 - 'A' as u32 + 1;
        let my_move = fields.next().unwrap().chars().next().unwrap() as u32 - 'X' as u32 + 1;
        let diff = (their_move+3).saturating_sub(my_move) % 3;
        // 2 == I win, 1 == they win, 0 == draw
        let round = my_move + match diff {
            0 => 3,
            1 => 0,
            2 => 6,
            3_u32..=u32::MAX => todo!()
        };
        score = score + round;
        // println!("them {their_move} me: {my_move} diff: {diff} round: {round}")
    }
    println!("{score}");
}