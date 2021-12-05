use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn part1() -> std::io::Result<()> {
    let mut x = 0;
    let mut y = 0;
    let reader = BufReader::new(File::open("input2.txt")?);
    for line in reader.lines() {
        let line = line.unwrap();
        let curr: Vec<&str> = line.split(" ").collect();
        let val: i32 = curr[1].parse().unwrap();
        match curr[0] {
            "forward" => x += val,
            "up" => y -= val,
            "down" => y += val,
            _ => panic!("Invalid direction in the input!"),
        }
    }
    println!("2.1 product: {}", x * y);
    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let reader = BufReader::new(File::open("input2.txt")?);
    for line in reader.lines() {
        let line = line.unwrap();
        let curr: Vec<&str> = line.split(" ").collect();
        let val: i32 = curr[1].parse().unwrap();
        match curr[0] {
            "forward" => {
                x += val;
                y += val * aim;
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => panic!("Invalid direction in the input!"),
        }
    }
    println!("2.2 product: {}", x * y);
    Ok(())
}
