
pub fn calculate_instruction(vec: &mut Vec<isize>) {
    let mut pos: usize = 0;

    while vec[pos] != 99 {
        let write_to = vec[pos + 3] as usize;
        let first_arg = vec[pos + 1] as usize;
        let second_arg = vec[pos + 2] as usize;

        if vec[pos] == 1 {
            vec[write_to] = vec[first_arg] + vec[second_arg];
        } else if vec[pos] == 2 {
            vec[write_to] = vec[first_arg] * vec[second_arg];
        } else {
            println!("Invalid opcode: {}", vec[pos]);
        }

        pos = pos + 4;
    }
}
