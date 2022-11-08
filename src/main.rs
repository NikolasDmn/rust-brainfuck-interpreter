use std::{char::from_u32, fs, io::Read};
fn main() {
    let text = get_file();
    interpret(&text.chars().collect());
}
fn interpret(c: &Vec<char>) {
    let mut tape = [0; 30000];
    let mut loop_stack: Vec<usize> = vec![];
    let mut character_index = 0;
    let mut pointer = 0;
    while character_index < c.len() {
        match c[character_index] {
            '[' => loop_stack.push(character_index - 1),
            ']' => {
                if tape[pointer] != 0 {
                    character_index = loop_stack.pop().unwrap();
                }
            }
            '+' => tape[pointer] += 1,
            '-' => tape[pointer] -= 1,
            '>' => {
                pointer += 1;
            }
            '<' => pointer -= 1,
            '.' => print!("{}", from_u32(tape[pointer] as u32).unwrap()),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                tape[pointer] = input[0];
            }
            _ => {}
        }
        character_index += 1
    }
}

fn get_file() -> String {
    let text = fs::read_to_string("./main.bf").expect("Should have been able to read the file");
    return text;
}
