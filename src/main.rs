use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_result(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
}

fn divide_mass(mass: u64) -> u64 {
    if mass / 3 < 2 {
        return 0;
    }

    (mass / 3) - 2
}

fn aoc_01() {
    let file_path = "input/01";
    let reader = get_result(file_path);
    let reader2 = get_result(file_path);
    let mut sum = 0;
    let mut sum2 = 0;

    for line in reader.lines() {
        let module_mass: u64 = line.unwrap().parse().unwrap();
        let mut div = divide_mass(module_mass);

        sum = sum + div;
    }

    println!("sum part1: {}", sum);

    for line in reader2.lines() {
        let module_mass: u64 = line.unwrap().parse().unwrap();
        let mut div = divide_mass(module_mass);

        sum2 = sum2 + div;

        loop {
            div = divide_mass(div);

            if div == 0 {
                break;
            };

            sum2 = sum2 + div;
        }
    }
    println!("sum part2: {}", sum2);
}

fn main() {
    aoc_01();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_three_and_minus_two() {
        assert_eq!(divide_mass(1969), 654);
    }
}
