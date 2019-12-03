use std::fs;

fn main() -> std::io::Result<()> {
    let input: Vec<i32> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let mut ops = input.to_vec();
    replace_pos1and2(&mut ops, 12, 2);
    let first_result = execute(&mut ops);

    println!("First: {:?}", first_result);

    // second
    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut ops_day2 = input.to_vec();
            replace_pos1and2(&mut ops_day2, noun, verb);
            let second_result = execute(&mut ops_day2);
            // println!("noun: {:?}, verb: {:?}", noun, verb);
            if second_result == 19690720 {
                println!("Second: {:?}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
    Ok(())
}

fn replace_pos1and2(input: &mut Vec<i32>, pos1: i32, pos2: i32) -> () {
    input[1] = pos1;
    input[2] = pos2;
}

fn execute(ops: &mut Vec<i32>) -> i32 {
    let mut pc: i32 = 0;
    let mut op: i32 = -1; // dummy
    let mut arg1: i32 = 0;
    let mut arg2: i32 = 0;

    for i in 0..ops.len() {
        match pc % 4 {
            0 => match ops[i] {
                99 => break,
                o => op = o
            },
            1 => arg1 = ops[ops[i] as usize],
            2 => arg2 = ops[ops[i] as usize],
            3 => {
                let output = ops[i] as usize;
                ops[output] = if op == 1 {
                    arg1 + arg2
                } else {
                    arg1 * arg2
                }
            },
            _ => panic!("do not reach")
        }
        pc += 1;
        // println!("{:?}", ops[i]);
    }

    // println!("{:?}", ops);
    ops[0]
}
