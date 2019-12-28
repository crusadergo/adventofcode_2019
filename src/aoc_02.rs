use crate::file;
use crate::computer;
use std::io::prelude::*;

pub fn aoc_02_p1() {
    let path = "input/02";
    let reader = file::reader(path);
    let mut vec: Vec<isize> = vec![];

    for line in reader.lines() {
        for number in line.unwrap().split(",") {
            let ez: isize = number.parse().unwrap();

            vec.push(ez);
        }
    }

    vec[1] = 12;
    vec[2] = 2;

    computer::calculate_instruction(&mut vec);

    println!("AOC_02_part_1: {:?}", vec[0]);
}

pub fn aoc_02_p2() {
    let path = "input/02";
    let reader = file::reader(path);
    let mut vec: Vec<isize> = vec![];
    let goal = 19_690_720;

    for line in reader.lines() {
        for number in line.unwrap().split(",") {
            let ez: isize = number.parse().unwrap();

            vec.push(ez);
        }
    }

    for noun in 0..100 {
        for verb in 0..100 {
            vec[1] = noun;
            vec[2] = verb;

            let mut instructions = vec.clone();

            computer::calculate_instruction(&mut instructions);

            if instructions[0] == goal {
                let result = 100 * noun + verb;

                println!("AOC_02_part_2: {:?}", result);
            }

        }
    }
}
