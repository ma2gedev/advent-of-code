use std::fs;
use intcode_computer::IntcodeComputer;

const SHIP_WIDTH: usize = 100;
const SHIP_HEIGHT: usize = 100;

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();
    let x_max = 50;
    let y_max = 50;

    // first
    let mut map: Vec<i64> = vec![0; x_max * y_max];
    for y in 0..y_max {
        for x in 0..x_max {
            let mut outputs = vec![];
            // need to initialize each input
            let mut intcode = IntcodeComputer::new(&input, 0, 0);
            let _state = intcode.execute(&mut vec![x as i64, y as i64], &mut outputs);
            map[x % x_max + y * x_max] = *outputs.last().unwrap();
        }
    }
    let tmp: Vec<&i64> = map.iter().filter(|&i| *i == 1).collect();

    println!("first: {:?}", tmp.len());

    let x_max = 1200;
    let y_max = 1200;

    // first
    let mut map: Vec<i64> = vec![0; x_max * y_max];
    let mut first_point: (i64, i64) = (-1, -1);
    let mut prev_x = 0;
    for y in 0..y_max {
        let mut first_x: i64 = -1;
        let mut last_x: i64 = -1;
        for x in 0..x_max {
            if (x as i64) < prev_x { continue }
            let mut outputs = vec![];
            // need to initialize each input
            let mut intcode = IntcodeComputer::new(&input, 0, 0);
            let _state = intcode.execute(&mut vec![x as i64, y as i64], &mut outputs);
            let point_value = *outputs.last().unwrap();
            let index = x % x_max + y * x_max;
            map[index] = point_value;
            if first_x == -1 && point_value == 1 {
                first_x = x as i64;
            } else if first_x != -1 && point_value == 0 {
                last_x = (x - 1) as i64;
                break;
            };
        }
        prev_x = first_x;
        //println!("process {}", y);
        let emitted_len = last_x - first_x + 1;
        if emitted_len >= (SHIP_WIDTH as i64) && first_point == (-1, -1) {
            //println!("emitted len: {}, first_x: {}, last_x: {}", emitted_len, first_x, last_x);
            first_point = (first_x, y as i64);
        }
        // debug
        // let s = ((y * x_max)..(y * x_max + x_max)).map(|i| {
        //     if map[i] == 0 {
        //         '.'
        //     } else {
        //         '#'
        //     }
        // }).collect::<String>();
        // println!("{}", s);
    }
    let tmp: Vec<&i64> = map.iter().filter(|&i| *i == 1).collect();
    // println!("{:?}", map);
    // println!("first_point: {:?}", first_point);

    'outer: for y in (first_point.1 as usize)..y_max {
        for x in (first_point.0 as usize)..x_max {
            let index = x % x_max + y * x_max;
            if map[index] == 0 { continue }
            if find_region(&map, x, y, x_max, y_max) {
                println!("second: {}", x * 10000 + y);
                break 'outer;
            }
        }
    }
    Ok(())
}

fn find_region(map: &Vec<i64>, start_x: usize, start_y: usize, x_max: usize, y_max: usize) -> bool {
    for x in start_x..(start_x + SHIP_WIDTH) {
        for y in start_y..(start_y + SHIP_HEIGHT) {
            let index = x % x_max + y * x_max;
            if index >= x_max * y_max { return false }
            if map[index] == 0 { return false }
        }
    }
    true
}
