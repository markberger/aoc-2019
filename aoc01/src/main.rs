use std::cmp;
use std::error;
use std::io;
use std::io::prelude::*;

fn parse_input() -> Result<Vec<i32>, Box<error::Error>> {
    let reader = io::stdin();
    let mut output: Vec<i32> = Vec::new();

    for line in reader.lock().lines() {
        let i: i32 = line?.parse()?;
        output.push(i)
    }

    return Ok(output)
}

fn fuel_needed(mass: i32) -> i32 {
    return cmp::max(mass / 3 - 2, 0);
}

fn part_one(v: &Vec<i32>) {
    let fuel: i32 = v.iter().map(|x| fuel_needed(*x)).sum();
    println!("Fuel req: {}", fuel);
}

fn part_two(v: &Vec<i32>) {
    let total_fuel: i32 = v.iter().map(|x| {
        let mut curr = fuel_needed(*x);
        let mut total = curr;
        while curr > 0 {
            curr = fuel_needed(curr);
            total += curr;
        }
        total
    }).sum();

    println!("Fuel req: {}", total_fuel);
}

fn main() {
    let input = parse_input().unwrap();
    part_one(&input);
    part_two(&input);
}
