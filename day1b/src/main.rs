use std::{env, process};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: day1a input_file.txt");
        process::exit(1);
    }
    let filename: &String = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut elf = 1;
    let mut calories = 0;
    // let mut max_elf = 0;
    let mut cals: Vec<i32> = Vec::new();
    for (_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line == "" {
            cals.push(calories);
            calories = 0;
            elf = elf + 1;
            continue;
        }
        let food = line.parse::<i32>().unwrap();
        calories = calories + food;
    }
    cals.push(calories);
    cals.sort_by(|a, b| b.cmp(a));
    let sum = cals[0] + cals[1] + cals[2];
    println!("{sum}");
}