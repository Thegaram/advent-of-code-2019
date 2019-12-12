const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

// const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;

fn count(array: &[u32], target: u32) -> usize {
    array.iter().filter(|x| **x == target).count()
}

fn part_1(raw: &Vec<u32>) -> usize {
    raw.chunks((WIDTH * HEIGHT) as usize)
        .map(|layer| (count(&layer, 0), count(&layer, 1), count(&layer, 2)))
        .min_by_key(|(cnt_0, _, _)| cnt_0.clone())
        .map(|(_, cnt_1, cnt_2)| cnt_1 * cnt_2)
        .unwrap()
}

fn part_2(raw: &Vec<u32>) {
    let mut res = [TRANSPARENT; (WIDTH * HEIGHT) as usize];

    for layer in raw.chunks((WIDTH * HEIGHT) as usize) {
        for (ii, pixel) in layer.iter().enumerate() {
            if res[ii] == TRANSPARENT {
                res[ii] = *pixel;
            }
        }
    }

    println!("\npart-2:");
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            match res[(row * WIDTH + col) as usize] {
                WHITE => print!("#"),
                _ => print!(" "),
            }
        }
        print!("\n");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let raw: Vec<u32> = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("part-1 = {:?}", part_1(&raw));
    part_2(&raw);
}
