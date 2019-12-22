use std::collections::HashMap;

fn operations(ops: i64) -> (i64, i64, i64, i64) {
    let op = ops % 100;
    let mut mode = ops / 100;
    let mode1 = mode % 10;
    mode = mode / 10;
    let mode2 = mode % 10;
    mode = mode / 10;
    let mode3 = mode % 10;

    (mode3, mode2, mode1, op)
}

fn from_input(inputs: &mut Vec<i64>) -> Option<i64> {
    if inputs.len() > 0 {
        Some(inputs.remove(0))
    } else {
        None
        // from stdio for manual test
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        // Some(input.trim().parse::<i64>().unwrap())
    }
}

fn output_to_memory(outputs: &mut Vec<i64>, value: i64) -> () {
    outputs.push(value)
}

fn read_value(ops: &Vec<i64>, extra_memory: &mut HashMap<i64, i64>, index: usize, relative_base: i64, mode: i64) -> i64 {
    match mode {
        0 => {
            let i = read_memory(ops, extra_memory, index) as usize;
            read_memory(ops, extra_memory, i)
        },
        1 => ops[index],
        2 => {
            let i = (relative_base + read_memory(ops, extra_memory, index)) as usize;
            read_memory(ops, extra_memory, i)
        },
        _ => panic!("do not reach"),
    }
}

fn output_to(ops: &Vec<i64>, extra_memory: &mut HashMap<i64, i64>, index: usize, relative_base: i64, mode: i64) -> usize {
    if mode == 2 {
        (relative_base + read_memory(ops, extra_memory, index)) as usize
    } else {
        read_memory(ops, extra_memory, index) as usize
    }
}

fn read_memory(ops: &Vec<i64>, extra_memory: &mut HashMap<i64, i64>, index: usize) -> i64 {
    if index < ops.len() {
        ops[index]
    } else {
        match extra_memory.get(&(index as i64)) {
            Some(o) => *o,
            None => 0,
        }
    }
}

fn write_memory(ops: &mut Vec<i64>, extra_memory: &mut HashMap<i64, i64>, index: usize, value: i64) {
    if index < ops.len() {
        ops[index] = value;
    } else {
        extra_memory.insert(index as i64, value);
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum IntcodeState {
    Init,
    Halt,
    Suspend,
}

#[derive(Debug)]
pub struct IntcodeComputer {
    ops: Vec<i64>,
    pc: usize,
    relative_base: i64,
    extra_memory: HashMap<i64, i64>,
    state: IntcodeState,
}

impl IntcodeComputer {
    pub fn new(ops: &Vec<i64>, pc: usize, relative_base: i64) -> IntcodeComputer {
        IntcodeComputer {
            ops: ops.to_vec(),
            pc: pc,
            relative_base: relative_base,
            extra_memory: HashMap::new(),
            state: IntcodeState::Init,
        }
    }

    pub fn execute(&mut self, inputs: &mut Vec<i64>, outputs: &mut Vec<i64>) -> IntcodeState {
        let (pc, intcode_state, rb) = execute(&mut self.ops, inputs, outputs, self.pc, self.relative_base, &mut self.extra_memory);
        self.pc = pc;
        self.relative_base = rb;
        self.state = intcode_state;
        intcode_state
    }
}

pub fn execute(ops: &mut Vec<i64>, inputs: &mut Vec<i64>, outputs: &mut Vec<i64>, pc: usize, relative_base: i64, extra_memory: &mut HashMap<i64, i64>) -> (usize, IntcodeState, i64) {
    let mut pc = pc;
    let mut relative_base = relative_base;
    let mut op = -1; // dummy
    let mut arg1 = 0;
    let mut arg2 = 0;
    let mut operation_step = 0;
    let mut done_operation = false;
    let mut mode1 = 0;
    let mut mode2 = 0;
    let mut mode3 = 0; // maybe unnecessary
    let mut _calculation_result = IntcodeState::Init;

    loop {
        if done_operation {
            operation_step = 0;
            arg1 = 0;
            arg2 = 0;
            done_operation = false;
        }
        match operation_step {
            0 => match operations(read_memory(ops, extra_memory, pc)) {
                (_, _, _, 99) => break _calculation_result = IntcodeState::Halt,
                (parameter_mode3, parameter_mode2, parameter_mode1, o) => {
                    mode1 = parameter_mode1;
                    mode2 = parameter_mode2;
                    mode3 = parameter_mode3;
                    op = o;
                },
            },
            1 => match op {
                1 | 2 | 5 | 6 | 7 | 8 => arg1 = read_value(ops, extra_memory, pc, relative_base, mode1),
                3 => {
                    let output = output_to(ops, extra_memory, pc, relative_base, mode1);
                    match from_input(inputs) {
                        Some(o) => write_memory(ops, extra_memory, output, o),
                        None => break _calculation_result = IntcodeState::Suspend,
                    }
                    done_operation = true;
                },
                4 => {
                    output_to_memory(outputs, read_value(ops, extra_memory, pc, relative_base, mode1));
                    done_operation = true;
                },
                9 => {
                    relative_base += read_value(ops, extra_memory, pc, relative_base, mode1);
                    done_operation = true;
                }
                _ => panic!("do not reach"),
            },
            2 => {
                arg2 = read_value(ops, extra_memory, pc, relative_base, mode2);
                match op {
                    5 => {
                        done_operation = true;
                        if arg1 != 0 {
                            pc = arg2 as usize;
                            continue;
                        }
                    },
                    6 => {
                        done_operation = true;
                        if arg1 == 0 {
                            pc = arg2 as usize;
                            continue;
                        }
                    },
                    _ => (), // do nothing
                }
            },
            3 => {
                let output = output_to(ops, extra_memory, pc, relative_base, mode3);
                let res = match op {
                    1 => { arg1 + arg2 },
                    2 => { arg1 * arg2 },
                    7 => { if arg1 < arg2 { 1 } else { 0 } },
                    8 => { if arg1 == arg2 { 1 } else { 0 } },
                    _ => panic!("do not reach"),
                };
                write_memory(ops, extra_memory, output, res);
                done_operation = true;
            },
            _ => panic!("do not reach")
        }
        operation_step += 1;
        pc += 1;
        // println!("pc: {:?}, relative_base: {:?}, op: {:?}, step: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, arg1: {:?}, arg2: {:?}", pc, relative_base, op, operation_step, mode1, mode2, mode3, arg1, arg2);
    }
    if _calculation_result == IntcodeState::Suspend {
        pc -= 1; // getting from operation code for next execution
    }
    (pc, _calculation_result, relative_base)
}

#[test]
fn test_operations() {
    assert_eq!(operations(3), (0, 0, 0, 3));
    assert_eq!(operations(10003), (1, 0, 0, 3));
    assert_eq!(operations(01104), (0, 1, 1, 4));
}

#[test]
fn test_execute() {
    let mut input: Vec<i64> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let mut output = vec![];
    let (_pc, _calculation_result, _relative_base) = execute(&mut input, &mut vec![0], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![0], output);
}

#[test]
fn test_execute_with_suspend() {
    let mut input: Vec<i64> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let mut output = vec![];
    let mut extra_memory = HashMap::new();
    // no input
    let (pc, calculation_result, relative_base) = execute(&mut input, &mut vec![], &mut output, 0, 0, &mut extra_memory);
    assert_eq!(IntcodeState::Suspend, calculation_result);
    // input zero
    let (_pc, calculation_result, _relative_base) = execute(&mut input, &mut vec![0], &mut output, pc, relative_base, &mut extra_memory);
    assert_eq!(vec![0], output);
    assert_eq!(IntcodeState::Halt, calculation_result);
}

#[test]
fn test_execute_with_suspend_struct() {
    let input: Vec<i64> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let mut output = vec![];
    let mut intcode_computer = IntcodeComputer::new(&input, 0, 0);
    // no input
    let calculation_result = intcode_computer.execute(&mut vec![], &mut output);
    assert_eq!(IntcodeState::Suspend, calculation_result);
    // input zero
    let calculation_result = intcode_computer.execute(&mut vec![0], &mut output);
    assert_eq!(vec![0], output);
    assert_eq!(IntcodeState::Halt, calculation_result);
}

#[test]
fn test_relative_mode() {
    let mut ops: Vec<i64> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    // do not use `ops.resize(10000, 0);`
    let mut output = vec![];

    execute(&mut ops, &mut vec![], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], output);
}

#[test]
fn test_should_output_16_digit_number() {
    let mut ops: Vec<i64> = vec![1102,34915192,34915192,7,4,7,99,0];
    let mut output = vec![];

    execute(&mut ops, &mut vec![], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![1219070632396864], output);
}

#[test]
fn test_should_output_a_large_number() {
    let mut ops: Vec<i64> = vec![104,1125899906842624,99];
    // ops.resize(1125899906842624, 0);
    // `resize` caught the following
    // intcode_computer-d7c77519a0090917(50756,0x7000100ef000) malloc: can't allocate region
    // *** mach_vm_map(size=9007199254740992) failed (error code=3)
    let mut output = vec![];

    execute(&mut ops, &mut vec![], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![1125899906842624], output);
}

#[test]
fn test_day05_01() {
    let mut ops: Vec<i64> = vec![1101,100,-1,4,0];
    let mut output = vec![];

    execute(&mut ops, &mut vec![], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![] as Vec<i64>, output);
}

#[test]
fn test_day05_5678() {
    let mut ops: Vec<i64> = vec![3,9,8,9,10,9,4,9,99,-1,8];
    let mut output = vec![];
    execute(&mut ops, &mut vec![8], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![1], output);

    ops = vec![3,9,8,9,10,9,4,9,99,-1,8];
    output = vec![];
    execute(&mut ops, &mut vec![9], &mut output, 0, 0, &mut HashMap::new());
    assert_eq!(vec![0], output);
}