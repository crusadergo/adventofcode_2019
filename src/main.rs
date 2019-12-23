mod aoc_01;
mod aoc_02;
mod file;

fn main() {
    let separator = "=========================================================================";
    println!("{}", separator);
    aoc_01::aoc_01_p1();
    aoc_01::aoc_01_p2();
    println!("{}", separator);
    aoc_02::compute();
    println!("{}", separator);
}
