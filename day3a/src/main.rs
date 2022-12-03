use std::{env, process};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: day3a input_file.txt");
        process::exit(1);
    };
    let filename: &String = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (_idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let len = line.chars().count();
        let mut count = [0; 53];
        for (i, c) in line.chars().enumerate() {
            let idx = priority(c);
            if i >= len / 2 {
                if count[idx as usize] != 0 {
                    // println!("idx: {idx} c: {c} line: {line}");
                    sum = sum + idx;
                    break;
                }
            } else {
                count[idx as usize] = 1;
            }
        }
    }
    println!("{sum}");
}

fn priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        return c as u32 - 'a' as u32  + 1;
    }
    if c >= 'A' && c <= 'Z' {
        return c as u32 - 'A' as u32 + 27;
    }
    panic!("bad character in priority")
}