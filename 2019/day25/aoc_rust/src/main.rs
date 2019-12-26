use std::fs;
use intcode_computer::{ IntcodeComputer, IntcodeState };

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    let mut intcode = IntcodeComputer::new(&input, 0, 0);
    let mut inputs = vec![];

    loop {
        let mut outputs = vec![];
        let state = intcode.execute(&mut inputs, &mut outputs);

        let bytes: Vec<u8> = outputs.iter().map(|&i| i as u8).collect();
        let convert = bytes.iter().map(|&u| u as char).collect::<String>();
        println!("{}", convert);
        // println!("Outputs: {:?}", outputs);
        if state == IntcodeState::Halt { break }

        // do automatically
        // parse docs
        // get item(without something bad) or direction
        // get next action(switching mode take items and open locked door)
        // enter operation
        inputs = _get_input_stdin_as_operation(state);
    }
    Ok(())
}

// for manual
fn _get_input_stdin_as_operation(state: IntcodeState) -> Vec<i64> {
    println!("Please input instruction state is: {:?}", state);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.bytes().map(|b| b as i64).collect::<Vec<i64>>()
}