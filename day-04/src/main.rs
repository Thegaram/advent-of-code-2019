const MIN: u32 = 235741;
const MAX: u32 = 706948;

fn is_increasing(num: &u32) -> bool {
    let mut num = match *num {
        x if x < 10 => return true,
        x => x,
    };

    let mut last = num % 10;
    num /= 10;

    while num > 0 {
        if last < num % 10 { return false; }
        last = num % 10;
        num /= 10;
    }

    true
}

fn groups(mut n: u32) -> impl Iterator<Item = (u32, u32)> {
    std::iter::from_fn(move || match n {
        0 => None,
        digit @ 1..=9 => {
            n = 0;
            return Some((digit, 1));
        }
        _ => {
            let digit = n % 10;
            n /= 10;

            let mut len = 1;

            while n > 0 && n % 10 == digit {
                len += 1;
                n /= 10;
            }

            Some((digit, len))
        }
    })
}

fn main() {
    let part1 = (MIN..MAX)
        .filter(is_increasing)
        .filter(|num| groups(*num).any(|(_, l)| l >= 2))
        .count();

    println!("part-1 = {}", part1);

    let part2 = (MIN..MAX)
        .filter(is_increasing)
        .filter(|num| groups(*num).any(|(_, l)| l == 2))
        .count();

    println!("part-2 = {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groups() {
        assert_eq!(groups(0).collect::<Vec<_>>(), vec![]);
        assert_eq!(groups(1).collect::<Vec<_>>(), vec![(1, 1)]);
        assert_eq!(groups(10).collect::<Vec<_>>(), vec![(0, 1), (1, 1)]);
        assert_eq!(groups(11).collect::<Vec<_>>(), vec![(1, 2)]);
        assert_eq!(groups(112333).collect::<Vec<_>>(), vec![(3, 3), (2, 1), (1, 2)]);
    }
}
