use std::fs;
use std::collections::HashMap;
use intcode_computer::{execute, IntcodeState};

#[derive(Copy, Clone, Debug)]
enum Dir {
    UP, LEFT, DOWN, RIGHT,
}
fn get_cur_value(map: &Vec<(i32, i32)>, cur: (i32, i32, Dir), size: usize) -> (i32, i32) {
    //println!("{:?}, {:?}, {:?}", cur.0, cur.1, cur.2);
    let x = cur.0 as usize;
    let y = size * cur.1 as usize;
    map[x + y]
}
fn paint(map: &mut Vec<(i32, i32)>, cur: (i32, i32, Dir), size: usize, color: i32) {
    let x = cur.0 as usize;
    let y = size * cur.1 as usize;
    let (_, count) = map[x + y];
    map[x + y] = (color, count + 1);
}

fn move_on_next(cur: (i32, i32, Dir), dir: i32) -> (i32, i32, Dir) {
    // dir 0:left, 1:right
    let mut x = cur.0;
    let mut y = cur.1;
    let d;
    match cur.2 {
        Dir::UP => {
            if dir == 0 {
                x -= 1;
                d = Dir::LEFT;
            } else {
                x += 1;
                d = Dir::RIGHT;
            }
        },
        Dir::LEFT => {
            if dir == 0 {
                y += 1;
                d = Dir::DOWN;
            } else {
                y -= 1;
                d = Dir::UP;
            }
        },
        Dir::DOWN => {
            if dir == 0 {
                x += 1;
                d = Dir::RIGHT;
            } else {
                x -= 1;
                d = Dir::LEFT;
            }
        },
        Dir::RIGHT => {
            if dir == 0 {
                y -= 1;
                d = Dir::UP;
            } else {
                y += 1;
                d = Dir::DOWN;
            }
        },
    }
    (x, y, d)
}

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let size = 101;
    let mut map = vec![(0, 0); size * size]; // (101 x 101 field
    let mut current = (51, 51, Dir::UP); // current position and direction
    let mut ops = input.to_vec();
    let mut ex_memory = HashMap::new();
    let mut pc = 0;
    loop {
        let mut outputs = vec![];
        // decide input by current positions color
        let (cur_color, cur_count) = get_cur_value(&map, current, size);
        let input = if cur_color == 0 { 0 } else { 1 };
        // execute with input
        let (tmp_pc, intcode_state, rb) =
            execute(&mut ops, &mut vec![input], &mut outputs, pc, 0, &mut ex_memory);
        pc = tmp_pc;
        // get latest output 
        let (color, left_or_right) = (outputs[0], outputs[1]);
        // paint with output color and count up
        paint(&mut map, current, size, color as i32);
        // and then move with output dir left or right
        current = move_on_next(current, left_or_right as i32);
        // break loop if intcode computer halts
        // println!("pc: {:?}, state: {:?}, rb: {:?}", pc, intcode_state, rb);
        if intcode_state == IntcodeState::Halt {
            break;
        }
    }
    debug_print_map(&map, size);
    let panel_count = map.iter().filter(|(_, count)| {
        *count > 0
    }).count();
    println!("first: {:?}", panel_count);

    // second
    let size = 105;
    let mut map = vec![(1, 0); size * size]; // (105 x 105 field
    let mut current = (53, 53, Dir::UP); // current position and direction
    let mut ops = input.to_vec();
    let mut ex_memory = HashMap::new();
    let mut pc = 0;
    let mut rb = 0;
    let mut exec_count = 0;
    let mut x_max = current.0;
    let mut x_min = current.0;
    let mut y_max = current.1;
    let mut y_min = current.1;
    loop {
        let mut outputs = vec![];
        // decide input by current positions color
        let (cur_color, cur_count) = get_cur_value(&map, current, size);
        let input = if cur_color == 0 { 0 } else { 1 };
        // execute with input
        let (tmp_pc, intcode_state, tmp_rb) =
            execute(&mut ops, &mut vec![input], &mut outputs, pc, rb, &mut ex_memory);
        pc = tmp_pc;
        rb = tmp_rb;
        // get latest output 
        let (color, left_or_right) = (outputs[0], outputs[1]);
        // paint with output color and count up
        paint(&mut map, current, size, color as i32);
        // and then move with output dir left or right
        current = move_on_next(current, left_or_right as i32);
        // break loop if intcode computer halts
        exec_count += 1;
        if x_max < current.0 { x_max = current.0 };
        if x_min > current.0 { x_min = current.0 };
        if y_max < current.1 { y_max = current.1 };
        if y_min > current.1 { y_min = current.1 };
        // println!("pc: {:?}, state: {:?}, rb: {:?}", pc, intcode_state, rb);
        if intcode_state == IntcodeState::Halt {
            break;
        }
    }
    println!("total: {:?}, xmin: {:?}, xmax: {:?}, ymin: {:?}, ymax: {:?}", exec_count, x_min, x_max, y_min, y_max);
    debug_print_map(&map, size);
    let panel_count = map.iter().filter(|(_, count)| {
        *count > 0
    }).count();
    println!("second: {:?}", panel_count);

    Ok(())
}

fn debug_print_map(map: &Vec<(i32, i32)>, size: usize) {
    println!("map:");
    for i in 0..size {
        let base = i * size;
        let colors: Vec<String> = map[base..(base + size)].iter().map(|(color, _count)| {
            let str = if *color == 0 { "." } else { "#" };
            str.to_string()
        }).collect();
        println!("{:?}", colors.join(""));
    }
}