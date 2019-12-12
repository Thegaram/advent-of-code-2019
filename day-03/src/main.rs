use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Direction { Up, Right, Down, Left }

fn parse_direction(raw: &str) -> Direction {
    match raw {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        x => panic!("Unexpected direction: {}", x),
    }
}

type Wire = Vec<(Direction, i32)>;
type DrawnWire = HashMap<(i32, i32), u32>;

fn parse_wire(raw: &str) -> Wire {
    raw.split(",")
        .map(|s| s.trim())
        .map(|s| s.split_at(1))
        .map(|(dir, n)| (parse_direction(dir), i32::from_str_radix(n, 10).unwrap()))
        .collect()
}

fn draw(wire: &Wire) -> DrawnWire {
    let mut result = HashMap::default();

    let mut coord = (0, 0);
    let mut steps = 0;

    for (dir, dist) in wire {
        match dir {
            Direction::Up => {
                for _ in 0..*dist {
                    coord.1 += 1;
                    steps += 1;
                    result.entry((coord.0, coord.1)).or_insert(steps);
                }
            }
            Direction::Down => {
                for _ in 0..*dist {
                    coord.1 -= 1;
                    steps += 1;
                    result.entry((coord.0, coord.1)).or_insert(steps);
                }
            }
            Direction::Right => {
                for _ in 0..*dist {
                    coord.0 += 1;
                    steps += 1;
                    result.entry((coord.0, coord.1)).or_insert(steps);
                }
            }
            Direction::Left => {
                for _ in 0..*dist {
                    coord.0 -= 1;
                    steps += 1;
                    result.entry((coord.0, coord.1)).or_insert(steps);
                }
            }
        };

        
    }

    result
}

fn manhattan(c0: (i32, i32), c1: (i32, i32)) -> u32 {
    let dx = (c0.0 - c1.0).abs() as u32;
    let dy = (c0.1 - c1.1).abs() as u32;
    dx + dy
}

fn closest_dist(wire1: &DrawnWire, wire2: &DrawnWire) -> u32 {
    let coords1: HashSet<_> = wire1.keys().collect();
    let coords2: HashSet<_> = wire2.keys().collect();

    coords1.intersection(&coords2)
        .map(|c| manhattan((0, 0), **c))
        .min()
        .unwrap()
}

fn closest_steps(wire1: &DrawnWire, wire2: &DrawnWire) -> u32 {
    let coords1: HashSet<_> = wire1.keys().collect();
    let coords2: HashSet<_> = wire2.keys().collect();

    coords1.intersection(&coords2)
        .map(|c| wire1[c] + wire2[c])
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let wires: Vec<Wire> = reader
        .lines()
        .into_iter()
        .map(|line| parse_wire(&line.unwrap()))
        .collect();

    let wire1 = draw(&wires[0]);
    let wire2 = draw(&wires[1]);

    let res = closest_dist(&wire1, &wire2);
    println!("part-1 = {:?}", res);

    let res = closest_steps(&wire1, &wire2);
    println!("part-2 = {:?}", res);

    Ok(())
}
