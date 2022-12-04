use std::{env, process};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: day4a input_file.txt");
        process::exit(1);
    };
    let filename: &String = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for (_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        let mut a: [i32; 5] = [0; 5];
        for i in 1..5 {
            a[i] = cap.get(i).unwrap().as_str().parse::<i32>().unwrap();
        }
        // println!("{line} {} {} {} {}", a[1], a[2], a[3], a[4]);
        if contains(a[1], a[2], a[3], a[4]) || contains(a[3], a[4], a[1],a[2]) {
            sum+= 1;
        }
    }
    println!("{sum}");
}

fn contains(a: i32, b: i32, x: i32, y: i32) -> bool {
    return x >= a && y <= b;
}