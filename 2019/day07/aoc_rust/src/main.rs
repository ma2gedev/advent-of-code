use std::fs;
use intcode_computer::{execute, IntcodeState};
// use std::io;

fn main() -> std::io::Result<()> {
    let input: Vec<i32> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let combinations = combination(&vec![0, 1, 2, 3, 4]);
    let mut max_output = -1;
    for combi in combinations {
        let mut next_input = 0;
        for phase in combi {
            let mut ops = input.to_vec();
            let mut inputs = vec![phase, next_input];
            let mut outputs = vec![];
            let (_pc, _intcode_state) = execute(&mut ops, &mut inputs, &mut outputs, 0);
            next_input = *outputs.last().unwrap();
        }
        if max_output < next_input {
            max_output = next_input;
        }
    }
    println!("max_output: {:?}", max_output);

    // second
    let combinations = combination(&vec![5, 6, 7, 8, 9]);
    let mut max_output = -1;
    for combi in combinations {
        let mut first_loop = true;
        let mut opss = vec![
            input.to_vec(),
            input.to_vec(),
            input.to_vec(),
            input.to_vec(),
            input.to_vec(),
        ];
        let mut pcs = vec![0, 0, 0, 0, 0];
        let mut next_input = 0i32;
        let mut latest_intcode_state = IntcodeState::Init;
        while latest_intcode_state != IntcodeState::Halt {
            for (i, phase) in combi.iter().enumerate() {
                let mut inputs = if first_loop {
                    vec![*phase, next_input]
                } else {
                    vec![next_input]
                };
                let mut outputs = vec![];
                let (pc, intcode_state) = execute(&mut opss[i], &mut inputs, &mut outputs, pcs[i]);
                pcs[i] = pc;
                latest_intcode_state = intcode_state;
                next_input = *outputs.last().unwrap();
            }
            first_loop = false;
        }
        if max_output < next_input {
            max_output = next_input;
        }
    }
    println!("max_output: {:?}", max_output);

    Ok(())
}

fn combination(elements: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut combinations = vec![];
    let mut i = 0;
    if elements.len() == 1 {
        return vec![elements.clone()];
    }
    for element in elements {
        let mut tmp_elements = elements.clone();
        tmp_elements.remove(i);
        // println!("{:?}", tmp_elements);
        for mut right in combination(&tmp_elements) {
            right.insert(0, *element);
            combinations.push(right);
        }
        i += 1;
    }
    combinations
}

#[test]
fn test_combination() {
    let mut elements = vec![1, 2];
    let combination1 = combination(&elements);
    assert_eq!(combination1[0], vec![1, 2]);
    assert_eq!(combination1[1], vec![2, 1]);

    elements = vec![1, 2, 3];
    let combination2 = combination(&elements);
    assert_eq!(combination2[0], vec![1, 2, 3]);
    assert_eq!(combination2[1], vec![1, 3, 2]);
    assert_eq!(combination2[2], vec![2, 1, 3]);
    assert_eq!(combination2[3], vec![2, 3, 1]);
    assert_eq!(combination2[4], vec![3, 1, 2]);
    assert_eq!(combination2[5], vec![3, 2, 1]);
}
