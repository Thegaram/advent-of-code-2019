use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn base_fuel(mass: u32) -> u32 {
    match mass / 3 {
        0 | 1 => 0,
        x => x - 2,
    }
}

fn fuel_components(mut mass: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(move || match base_fuel(mass) {
        0 => None,
        fuel => {
            mass = fuel;
            Some(fuel)
        }
    })
}

fn part_1(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .map(base_fuel)
        .sum()
}

fn part_2(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .flat_map(fuel_components)
        .sum()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    println!("part-1 = {}", part_1(filename));
    println!("part-2 = {}", part_2(filename));

    Ok(())
}
