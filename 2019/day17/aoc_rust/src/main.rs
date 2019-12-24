use std::fs;
use intcode_computer::IntcodeComputer;
use regex::Regex;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let mut intcode = IntcodeComputer::new(&input, 0, 0);
    let mut inputs = vec![];
    let mut outputs = vec![];
    let _state = intcode.execute(&mut inputs, &mut outputs);

    let bytes: Vec<u8> = outputs.iter().map(|&i| i as u8).collect();
    let line_len = bytes.iter().position(|&b| b == 10u8).unwrap() + 1;
    let line_max = bytes.len() / line_len;
    let mut intersections: Vec<usize> = vec![];
    for i in 0..(bytes.len() as usize) {
        if i < line_len { continue; }; // first line
        if i >= bytes.len() - line_len { continue; }; // last line
        if i % line_len == 0 || i % line_len == (line_len - 1) { continue; } // lines at both ends
        if bytes[i] != '#' as u8 { continue; };
        if bytes[i - 1] == '#' as u8 &&
           bytes[i + 1] == '#' as u8 &&
           bytes[i - line_len] == '#' as u8 &&
           bytes[i + line_len] == '#' as u8 {
            intersections.push(i);
        }
    }
    let sum: usize = intersections.iter().map(|&intersection| {
        let side_units = intersection % line_len;
        let units = intersection / line_len;
        side_units * units
    }).sum();
    // let convert = bytes.iter().map(|&u| u as char).collect::<String>();
    // println!("{}", convert);
    println!("first: {}", sum);

    // second
    // calculate route
    let mut robot_pos = bytes.iter().position(|&b| b == ('^' as u8)).unwrap();
    let mut operations: Vec<i64> = vec![];
    let mut current_dir = Direction::UP;
    loop {
        // get next direction (current_dir, map)
        let direction = get_next_direction(current_dir, robot_pos, &bytes, line_len, line_max);
        match direction {
            Some(dir) => {
                let operation = get_next_dir_operation(current_dir, dir);
                if operations.len() != 0 {
                    operations.push(',' as i64);
                }
                operations.push(operation);
                operations.push(',' as i64);
                current_dir = dir;
            },
            None => break,
        }

        // loop with increment count continuing to find '#'
        let mut count = 0;
        for i in 1..99 { // 99 is temporary should decide with line_len and line_max
            match get_next_pos(current_dir, robot_pos, &bytes, line_len, line_max) {
                Some(pos) => {
                    count = i;
                    robot_pos = pos;
                },
                None => break,
            }
        }
        // push ops with count
        count.to_string().chars().for_each(|b| {
            operations.push(b as i64);
        });
        // println!("robot_pos: x: {:?} y: {:?}", robot_pos % line_len, robot_pos / line_len);
    }
    let route = operations.iter().map(|&u| u as u8 as char).collect::<String>();
    // println!("operations len: {}", operations.len());
    // println!("{}", route);

    // extract routines
    let mut routines = get_routine(route);
    routines.push('n' as i64); // < y or n is needed
    routines.push(10);

    // input routine to intcode
    let mut modified_input = input.to_vec();
    modified_input[0] = 2;
    intcode = IntcodeComputer::new(&modified_input, 0, 0);
    outputs = vec![];
    let _state = intcode.execute(&mut routines, &mut outputs);
    println!("second: {}", outputs.last().unwrap());
    // let bytes: Vec<u8> = outputs.iter().map(|&i| i as u8).collect();
    // let convert = bytes.iter().map(|&u| u as char).collect::<String>();
    // println!("{}", convert);

    Ok(())
}


fn get_next_pos(dir: Direction, cur_pos: usize, map: &Vec<u8>, line_len: usize, line_max: usize) -> Option<usize> {
    let has_scaffold = has_scaffold(dir, cur_pos, map, line_len, line_max);
    if !has_scaffold { return None }

    match dir {
        Direction::UP => {
            Some(cur_pos - line_len)
        },
        Direction::DOWN => {
            Some(cur_pos + line_len)
        },
        Direction::LEFT => {
            Some(cur_pos - 1)
        },
        Direction::RIGHT => {
            Some(cur_pos + 1)
        },
    }
}

fn has_scaffold(dir: Direction, cur_index: usize, map: &Vec<u8>, line_len: usize, line_max: usize) -> bool {
    match dir {
        Direction::UP => {
            if cur_index / line_len == 0 {
                false
            } else {
                map[cur_index - line_len] == '#' as u8
            }
        },
        Direction::DOWN => {
            if cur_index / line_len == line_max - 1 {
                false
            } else {
                map[cur_index + line_len] == '#' as u8
            }
        },
        Direction::LEFT => {
            if cur_index % line_len == 0 {
                false
            } else {
                map[cur_index - 1] == '#' as u8
            }
        },
        Direction::RIGHT => {
            if cur_index % line_len == line_len - 1 {
                false
            } else {
                map[cur_index + 1] == '#' as u8
            }
        },
    }
}

fn get_next_dir_operation(cur_dir: Direction, next_dir: Direction) -> i64 {
    match cur_dir {
        Direction::UP => {
            if next_dir == Direction::LEFT {
                'L' as i64
            } else if next_dir == Direction::RIGHT {
                'R' as i64
            } else {
                panic!("do not reach")
            }
        },
        Direction::DOWN => {
            if next_dir == Direction::LEFT {
                'R' as i64
            } else if next_dir == Direction::RIGHT {
                'L' as i64
            } else {
                panic!("do not reach")
            }
        },
        Direction::LEFT => {
            if next_dir == Direction::UP {
                'R' as i64
            } else if next_dir == Direction::DOWN {
                'L' as i64
            } else {
                panic!("do not reach")
            }
        },
        Direction::RIGHT => {
            if next_dir == Direction::UP {
                'L' as i64
            } else if next_dir == Direction::DOWN {
                'R' as i64
            } else {
                panic!("do not reach")
            }
        },
    }
}

fn opposite_dir(dir: Direction) -> Direction {
    match dir {
        Direction::UP => Direction::DOWN,
        Direction::DOWN => Direction::UP,
        Direction::LEFT => Direction::RIGHT,
        Direction::RIGHT => Direction::LEFT,
    }
}

fn get_next_direction(dir: Direction, cur_index: usize, map: &Vec<u8>, line_len: usize, line_max: usize) -> Option<Direction> {
    let directions = vec![
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ];
    for d in directions {
        if d == dir || d == opposite_dir(dir) { continue; }
        if has_scaffold(d, cur_index, map, line_len, line_max) {
            return Some(d);
        }
    }
    None
}

fn get_routine(route: String) -> Vec<i64> {
    let routine_pat = Regex::new("^[RL][RL0-9,]*[0-9]$").unwrap();
    let main_pat = Regex::new("^[ABC,]*$").unwrap();

    for i in 4..=20 {
        let a;
        let replace_re_a;
        let route_with_a;
        let re_a = Regex::new(&format!("([RL0-9,]{{{}}})", i)).unwrap();
        match re_a.captures(&route).unwrap().get(0) {
            Some(re) => {
                let str_a = re.as_str();
                if routine_pat.is_match(str_a) {
                    a = str_a;
                    replace_re_a = Regex::new(a).unwrap();
                    route_with_a = replace_re_a.replace_all(&route, "A");
                } else {
                    continue;
                }
            },
            None => continue,
        }

        // println!("first loop: {}: {}", a, route_with_a);
        for j in 4..=20 {
            let b;
            let replace_re_b;
            let route_with_b;
            let re_b = Regex::new(&format!("^(A,)+([RL0-9,]{{{}}})", j)).unwrap();
            match re_b.captures(&route_with_a) {
                Some(re_cap) => {
                    match re_cap.get(2) {
                        Some(re) => {
                            let str_b = re.as_str();
                            if routine_pat.is_match(str_b) {
                                b = str_b;
                                replace_re_b = Regex::new(b).unwrap();
                                route_with_b = replace_re_b.replace_all(&route_with_a, "B");
                            } else {
                                continue;
                            }
                        },
                        None => continue,
                    }
                },
                None => continue,
            }
            // println!("second loop: {}: {}", a, route_with_b);

            for k in 4..=20 {
                let c;
                let replace_re_c;
                let route_with_c;
                let re_c = Regex::new(&format!("^(A,|B,)+([RL0-9,]{{{}}})", k)).unwrap();
                match re_c.captures(&route_with_b) {
                    Some(re_cap) => {
                        match re_cap.get(2) {
                            Some(re) => {
                                let str_c = re.as_str();
                                if routine_pat.is_match(str_c) {
                                    c = str_c;
                                    replace_re_c = Regex::new(c).unwrap();
                                    route_with_c = replace_re_c.replace_all(&route_with_b, "C");
                                    if main_pat.is_match(&route_with_c) && route_with_c.len() <= 20 {
                                        let mut routine_combi: Vec<i64> = vec![];
                                        routine_combi.extend(route_with_c.chars().map(|c| c as i64));
                                        routine_combi.push(10);
                                        routine_combi.extend(a.chars().map(|c| c as i64));
                                        routine_combi.push(10);
                                        routine_combi.extend(b.chars().map(|c| c as i64));
                                        routine_combi.push(10);
                                        routine_combi.extend(c.chars().map(|c| c as i64));
                                        routine_combi.push(10);
                                        return routine_combi;
                                    }
                                } else {
                                    continue;
                                }
                            },
                            None => continue,
                        }
                    },
                    None => continue,
                }
                // println!("third loop: {}: {}", c, route_with_c);
            }
        }
    }
    panic!("do not reach");
}

#[test]
fn test_regex() {
    let sample = "R,2,L,3,R,10,R,2,R,2,L,3";
    let re = Regex::new(&format!("([RL0-9,]{{{}}})", 2)).unwrap();
    let caps = re.captures(sample).unwrap();
    assert_eq!("R,", caps.get(0).unwrap().as_str());
}

#[test]
fn test_regex_2() {
    let sample = "R,2";
    let sample_not1 = "R,";
    let sample_not2 = "R,2,L";
    let re_pat = Regex::new("^[RL][RL0-9,]*[0-9]$").unwrap();

    assert!(re_pat.is_match(&sample));
    assert!(!re_pat.is_match(&sample_not1));
    assert!(!re_pat.is_match(&sample_not2));
}

#[test]
fn test_regex_3() {
    let sample = "A,R,2";
    let sample1 = "A,A,R,2,A";
    let sample2 = "A,B,R,2";
    let re_pat = Regex::new("^(A,|B,)+([RL0-9,]{3})").unwrap();

    assert!(re_pat.is_match(&sample));
    assert!(re_pat.is_match(&sample1));
    assert!(re_pat.is_match(&sample2));
}
