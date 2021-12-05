use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn part1() -> std::io::Result<()> {
    let mut cnt = 0;
    let mut prev = 0;
    let reader = BufReader::new(File::open("input1.txt")?);
    for line in reader.lines() {
        let curr = line?.parse::<i32>().unwrap();
        if prev < curr {
            cnt += 1;
        }
        prev = curr;
    }
    println!("1.1: {}", cnt - 1);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut prev = 0;
    let mut curr;
    let mut cnt = 0;
    let file = File::open("input1.txt").expect("File not found");
    let reader = BufReader::new(file);
    let vec: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().expect("Could not parse line"))
        .collect();
    for window in vec.windows(3) {
        curr = window.iter().copied().sum();
        if prev < curr {
            cnt += 1;
        }
        prev = curr;
    }
    println!("1.2: {}", cnt - 1);
    Ok(())
}
