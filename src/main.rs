mod aoc_01;
mod aoc_02;
mod file;
mod computer;

fn main() {
    let separator = "=========================================================================";
    println!("{}", separator);
    aoc_01::aoc_01_p1();
    aoc_01::aoc_01_p2();
    println!("{}", separator);
    aoc_02::aoc_02_p1();
    aoc_02::aoc_02_p2();
    println!("{}", separator);
}
