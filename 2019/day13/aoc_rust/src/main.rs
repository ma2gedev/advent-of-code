use std::fs;
use std::collections::HashMap;
use std::io::{Read, Result, stdin};
use intcode_computer::{ execute, IntcodeState };

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let mut ops = input.to_vec();
    let mut ex_memory = HashMap::new();
    let mut pc = 0;
    let mut rb = 0;
    let mut outputs = vec![];
    loop {
        let (tmp_pc, intcode_state, tmp_rb) =
            execute(&mut ops, &mut vec![], &mut outputs, pc, rb, &mut ex_memory);
        pc = tmp_pc;
        rb = tmp_rb;
        // println!("pc: {:?}, state: {:?}, rb: {:?}, outputs: {:?}", pc, intcode_state, rb, outputs);
        if intcode_state == IntcodeState::Halt {
            break;
        }
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut block_count = 0;
    for i in 0..(outputs.len() / 3) {
        let x = outputs[i * 3];
        let y = outputs[1 + i * 3];
        let t = outputs[2 + i * 3];
        if max_x < x { max_x = x};
        if max_y < y { max_y = y};
        if t == 2 { block_count += 1 };
    }
    max_x += 1;
    max_y += 1;
    println!("max x: {:?}, max y: {:?}, block count: {:?}", max_x, max_y, block_count);

    // second
    let mut ops = input.to_vec();
    ops[0] = 2;
    ex_memory = HashMap::new();
    pc = 0;
    rb = 0;
    let mut game_state = vec![0; (max_x * max_y + 1) as usize]; // last 1 is for score
    let mut inputs = vec![];
    loop {
        outputs = vec![];
        let (tmp_pc, intcode_state, tmp_rb) =
            execute(&mut ops, &mut inputs, &mut outputs, pc, rb, &mut ex_memory);
        pc = tmp_pc;
        rb = tmp_rb;
        // println!("pc: {:?}, state: {:?}, rb: {:?}, outputs len: {:?}", pc, intcode_state, rb, outputs.len());
        update_game_state(&mut game_state, &outputs, max_x as usize, max_y as usize);
        print_screen(&game_state, max_x as usize, max_y as usize, intcode_state == IntcodeState::Halt);
        if intcode_state == IntcodeState::Halt {
            println!("Game Over");
            break;
        }
        inputs = vec![];
        inputs.push(get_input());
    }

    Ok(())
}

fn update_game_state(game_state: &mut Vec<i64>, intcode_out: &Vec<i64>, max_x: usize, max_y: usize) {
    for i in 0..(intcode_out.len() / 3) {
        let x = intcode_out[i * 3];
        let y = intcode_out[1 + i * 3];
        let t = intcode_out[2 + i * 3];
        if x == -1 { // score
            game_state[max_x * max_y] = t;
        } else {
            game_state[y as usize * max_x + x as usize] = t;
        }
    }
}

fn print_object(v: i64) -> String {
    let str = match v {
        0 => " ", // empty
        1 => "#", // wall
        2 => "O", // block
        3 => "-", // paddle
        4 => "*", // ball
        _s => panic!("do not reach {:?}", _s),
    };
    str.to_string()
}

fn print_screen(game_state: &Vec<i64>, max_x: usize, max_y: usize, last: bool) {
    for y in 0..max_y {
        let base = max_x * y;
        let str: Vec<String> = game_state[base..(base + max_x)].iter().map(|v| {
            print_object(*v)
        }).collect();
        println!("{:?}", str.join(""));
    }
    println!("score: {:?}", game_state[max_x * max_y]);
    if !last { print!("\x1B[{:?}A", max_y + 2) }; // paint same region
}

fn get_input() -> i64 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let i = input.trim();
    match &i as &str {
        "h" => -1,
        "j" | "k" => 0,
        "l" => 1,
        _ => 0,
    }
}