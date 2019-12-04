use std::error;
use std::io;
use std::io::prelude::*;


fn parse_input() -> Result<Vec<u32>, Box<error::Error>> {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buf)?;

    let input = buf.trim()
        .split(',')
        .map(|elem| elem.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(input)
}

fn eval_program(v: &mut Vec<u32>) {
    for i in (0..v.len()).step_by(4) {
        let opcode = v[i];
        match opcode {
            1 | 2 => {
                let idx1 = v[i+1] as usize;
                let idx2 = v[i+2] as usize;
                let dest = v[i+3] as usize;

                if opcode == 1 {
                    v[dest] = v[idx1] + v[idx2];
                } else {
                    v[dest] = v[idx1] * v[idx2];
                }
            }
            99 => return,
            _ => println!("Error"),
        }
    }
}

fn search_inputs(program: &Vec<u32>, i: u32, j: u32, v: u32) -> Result<u32, &'static str> {
    for noun in 0..i {
        for verb in 0..j {
            let mut curr = program.clone();
            curr[1] = noun;
            curr[2] = verb;
            eval_program(&mut curr);
            if curr[0] == v {
                return Ok(100 * noun + verb)
            }
        }
    }

    return Err("no result")
}

fn main() {
    let program = parse_input().unwrap();

    // Part 1
    let mut input = program.clone();
    input[1] = 12;
    input[2] = 2;
    eval_program(&mut input);
    println!("Part 1: {}", input[0]);

    // Part 2
    let r = search_inputs(&program, 100, 100, 19690720);
    println!("Part 2: {:?}", r);
}
