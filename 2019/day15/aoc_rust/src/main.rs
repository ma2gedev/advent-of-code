use std::fs;
use intcode_computer::IntcodeComputer;

#[derive(Default, Debug)]
struct Node {
    done: bool,
    found: bool,
    nodes: Vec<Node>,
    count: i32,
}

impl Node {
    fn find_min_count(&self) -> i32 {
        let mut min = i32::max_value();
        if self.found {
            min = self.count;
        }
        let min_v = self.nodes.iter().map(|n| n.find_min_count()).min().unwrap_or(i32::max_value());
        if min > min_v { min_v } else { min }
    }

    fn find_max_count(&self) -> i32 {
        let max = self.count;
        let max_v = self.nodes.iter().map(|n| n.find_max_count()).max().unwrap_or(0);
        if max > max_v { max } else { max_v }
    }

    fn create_root() -> Node {
        Node {
            done: false,
            found: false,
            nodes: vec![],
            count: 0,
        }
    }

    fn create_node(parent: &Node) -> Node {
        Node {
            done: false,
            found: false,
            nodes: vec![],
            count: parent.count + 1,
        }
    }

    fn change_intcode_initial_position(intcode: &mut IntcodeComputer, from: i64) -> bool{
        let mut outputs = vec![];
        let mut find = false;
        for direction in 1..=4 {
            if from != direction {
                intcode.execute(&mut vec![direction], &mut outputs);
                match outputs.last().unwrap() {
                    0 => (), // do nothing
                    1 => {
                        find = Node::change_intcode_initial_position(intcode, Node::OPOSIT_DIR[direction as usize]);
                        if find { break; }
                        // put back droid
                        intcode.execute(&mut vec![Node::OPOSIT_DIR[direction as usize]], &mut outputs);
                    },
                    2 => {
                        find = true;
                        break;
                    },
                    _ => panic!("do not reach"),
                }
            }
        }
        find
    }

    const OPOSIT_DIR: [i64; 5] = [0, 2, 1, 4, 3];

    // from: 0 is root, 1: north, 2: south, 3: west, 4: east
    fn find_target(&mut self, intcode: &mut IntcodeComputer, from: i64) {
        let mut outputs = vec![];
        for direction in 1..=4 {
            if from != direction {
                intcode.execute(&mut vec![direction], &mut outputs);
                match outputs.last().unwrap() {
                    0 => (), // do nothing
                    1 => {
                        let mut node = Node::create_node(&self);
                        node.find_target(intcode, Node::OPOSIT_DIR[direction as usize]);
                        self.nodes.push(node);
                        // put back droid
                        intcode.execute(&mut vec![Node::OPOSIT_DIR[direction as usize]], &mut outputs);
                    },
                    2 => {
                        let mut node = Node::create_node(&self);
                        node.found = true;
                        node.done = true;
                        self.nodes.push(node);
                        // put back droid
                        intcode.execute(&mut vec![Node::OPOSIT_DIR[direction as usize]], &mut outputs);
                    },
                    _ => panic!("do not reach"),
                }
            }
        }
        self.done = true;
    }
}

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let mut intcode = IntcodeComputer::new(&input, 0, 0);
    let mut root_node = Node::create_root();
    root_node.find_target(&mut intcode, 0);
    println!("first: {:?}", root_node.find_min_count());
    // println!("first: {:?}", root_node);

    // second
    intcode = IntcodeComputer::new(&input, 0, 0);
    Node::change_intcode_initial_position(&mut intcode, 0);
    root_node = Node::create_root();
    root_node.find_target(&mut intcode, 0);
    println!("second: {:?}", root_node.find_max_count());
    Ok(())
}
