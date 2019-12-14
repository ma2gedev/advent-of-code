use std::fs;
use std::collections::HashMap;
use intcode_computer::{execute, IntcodeState};

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();
    let mut ops = input.to_vec();
    let mut inputs = vec![1];
    let mut outputs = vec![];
    let mut ex_memory = HashMap::new();
    let (_pc, intcode_state, _rb) = execute(&mut ops, &mut inputs, &mut outputs, 0, 0, &mut ex_memory);
    let _next_input = *outputs.last().unwrap();
    println!("first: {:?}, {:?}", outputs, intcode_state);

    ops = input.to_vec();
    inputs = vec![2];
    outputs = vec![];
    ex_memory = HashMap::new();
    let (_pc, intcode_state, _rb) = execute(&mut ops, &mut inputs, &mut outputs, 0, 0, &mut ex_memory);
    let _next_input = *outputs.last().unwrap();
    println!("second: {:?}, {:?}", outputs, intcode_state);
    Ok(())
}
