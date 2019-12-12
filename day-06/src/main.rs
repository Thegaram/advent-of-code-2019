use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Graph = HashMap<String, String>;

fn read_graph(filename: &str) -> Graph {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let mut components = line.splitn(2, ')');
            let center = components.next().unwrap().to_owned();
            let object = components.next().unwrap().to_owned();
            (object, center)
        })
        .collect()
}

fn num_orbits(graph: &Graph, mut object: String) -> u32 {
    let mut num = 0;

    while let Some(center) = graph.get(&object) {
        num += 1;
        object = center.to_string();
    }

    num
}

fn ancestors<'a>(graph: &'a Graph, mut object: String) -> impl Iterator<Item = String> + 'a {
    std::iter::from_fn(move || match graph.get(&object) {
        None => None,
        Some(center) => {
            object = center.to_string();
            Some(center.to_string())
        }
    })
}

fn part_1(graph: &Graph) -> u32 {
    graph
        .keys()
        .map(|object| num_orbits(&graph, object.to_string()))
        .sum()
}

fn part_2(graph: &Graph) -> usize {
    let my_orbits: Vec<_> = ancestors(&graph, "YOU".to_string()).collect();
    let santas_orbits: Vec<_> = ancestors(&graph, "SAN".to_string()).collect();

    let mut ii = my_orbits.len() - 1;
    let mut jj = santas_orbits.len() - 1;

    while my_orbits[ii] == santas_orbits[jj] {
        ii -= 1;
        jj -= 1;
    }

    ii + 1 + jj + 1
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let graph = read_graph(&args[1]);

    println!("part-1 = {}", part_1(&graph));
    println!("part-2 = {}", part_2(&graph));
}
