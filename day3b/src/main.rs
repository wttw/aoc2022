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
    let mut count = [0; 53];
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let group_idx = line_num % 3;
        if group_idx == 0 {
            count.fill(0)
        }
        for c in line.chars() {
            let pri = priority(c);
            count[pri as usize] |= 1 << group_idx;
            if count[pri as usize] == 7 {
                // println!("{c}: {pri}");
                sum += pri;
                break;
            }
        }
        // println!("{line_num}: {line}");

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