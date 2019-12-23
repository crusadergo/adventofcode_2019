use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn aoc_01_p1() {
    let file_path = "input/01";
    let reader = get_result(file_path);
    let mut sum = 0;

    for line in reader.lines() {
        let module_mass: i32 = line.unwrap().parse().unwrap();
        let mut div = divide_mass(module_mass);

        sum = sum + div;
    }

    println!("AOC_01_part_1: {}", sum);
}

pub fn aoc_01_p2() {
    let file_path = "input/01";
    let reader = get_result(file_path);
    let mut sum = 0;

    for line in reader.lines() {
        let module_mass: i32 = line.unwrap().parse().unwrap();
        let mut div = divide_mass(module_mass);

        sum = sum + div;

        loop {
            div = divide_mass(div);

            if div == 0 {
                break;
            };

            sum = sum + div;
        }
    }
    println!("AOC_01_part_2: {}", sum);
}

fn get_result(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
}

fn divide_mass(mass: i32) -> i32 {
    if mass / 3 < 2 {
        return 0;
    }

    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_three_and_minus_two() {
        assert_eq!(divide_mass(1969), 654);
    }
}
