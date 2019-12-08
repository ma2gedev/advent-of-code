use std::fs;
use std::io;

fn main() -> std::io::Result<()> {
    let input: Vec<i32> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let mut ops = input.to_vec();
    execute(&mut ops);

    // second
    Ok(())
}

fn operations(ops: i32) -> (i32, i32, i32, i32) {
    let op = ops % 100;
    let mut mode = ops / 100;
    let mode1 = mode % 10;
    mode = mode / 10;
    let mode2 = mode % 10;
    mode = mode / 10;
    let mode3 = mode % 10;

    (mode3, mode2, mode1, op)
}

fn from_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn output(value: i32) -> () {
    println!("{:?}", value)
}

fn read_value(ops: &Vec<i32>, index: usize, mode: i32) -> i32 {
    // println!("mode: {:?}", mode);
    match mode {
        0 => ops[ops[index] as usize],
        1 => ops[index],
        _ => panic!("do not reach"),
    }
}

fn execute(ops: &mut Vec<i32>) {
    let mut op: i32 = -1; // dummy
    let mut arg1: i32 = 0;
    let mut arg2: i32 = 0;
    let mut operation_step: i32 = 0;
    let mut done_operation = false;
    let mut mode1 = 0;
    let mut mode2 = 0;
    let mut _mode3 = 0; // maybe unnecessary

    for i in 0..ops.len() {
        if done_operation {
            operation_step = 0;
            done_operation = false;
        }
        match operation_step {
            0 => match operations(ops[i]) {
                (_, _, _, 99) => break,
                (parameter_mode3, parameter_mode2, parameter_mode1, o) => {
                    mode1 = parameter_mode1;
                    mode2 = parameter_mode2;
                    _mode3 = parameter_mode3;
                    op = o;
                },
            },
            1 => match op {
                1 | 2 => arg1 = read_value(ops, i, mode1),
                3 => {
                    let output = ops[i] as usize;
                    ops[output] = from_input();
                    done_operation = true;
                },
                4 => {
                    output(ops[ops[i] as usize]);
                    done_operation = true;
                },
                _ => panic!("do not reach"),
            },
            2 => arg2 = read_value(ops, i, mode2),
            3 => {
                let output = ops[i] as usize;
                ops[output] = if op == 1 {
                    arg1 + arg2
                } else {
                    arg1 * arg2
                };
                done_operation = true;
            },
            _ => panic!("do not reach")
        }
        operation_step += 1;
        //println!("op: {:?}, step: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, arg1: {:?}, arg2: {:?}", op, operation_step, mode1, mode2, mode3, arg1, arg2);
    }
}

#[test]
fn test_operations() {
    assert_eq!(operations(3), (0, 0, 0, 3));
    assert_eq!(operations(10003), (1, 0, 0, 3));
    assert_eq!(operations(01104), (0, 1, 1, 4));
}