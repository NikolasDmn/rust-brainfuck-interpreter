use std::{char::from_u32, env, fs, io::Read};
fn main() {
    //executable needs argument of the file path of the .bf file
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = &args[1];
        let text = get_file(&path);
        interpret(&text.chars().collect());
    } else {
        println!("File path was not provided");
    }
}
// interpret takes as an argument a character vector of all characters in a .bf file
fn interpret(c: &Vec<char>) {
    //initialize the tape, loop_stack and the two pointers/indexes
    let mut tape = [0; 30000];
    //loop_stack stores the index of the opening bracket for a loop
    //so that it can be easily brought back at the closing bracket
    let mut loop_stack: Vec<usize> = vec![];
    let mut character_index = 0;
    let mut pointer = 0;
    while character_index < c.len() {
        //match the character to its corresponding command
        match c[character_index] {
            //store character_index-1 as we increment the index by one at the end of the while loop
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
                    .expect("Failed to read input");
                tape[pointer] = input[0];
            }
            _ => {}
        }
        character_index += 1
    }
}

fn get_file(path: &str) -> String {
    let text = fs::read_to_string(path).expect("Error reading file, check if it exists");
    return text;
}
