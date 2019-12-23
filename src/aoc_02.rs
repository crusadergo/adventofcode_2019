use crate::file;
use std::io::prelude::*;

pub fn compute() {
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

    let mut pos: usize = 0;

    while vec[pos] != 99 {
        if vec[pos] == 1 {
            let write_to = vec[pos + 3] as usize;
            vec[write_to] = vec[vec[pos + 1] as usize] + vec[vec[pos + 2] as usize];
    
            pos = pos + 4;
        } else if vec[pos] == 2 {
            let write_to = vec[pos + 3] as usize;
            vec[write_to] = vec[vec[pos + 1] as usize] * vec[vec[pos + 2] as usize];

            pos = pos + 4;
        } else {
            println!("ERROR!!!");
        }
    }

    println!("{:?}", vec);
}
