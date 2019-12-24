use std::fs;
use intcode_computer::IntcodeComputer;

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();
    let x_max = 50;
    let y_max = 50;

    // first
    let mut map: Vec<i64> = vec![0; x_max * y_max];
    for x in 0..x_max {
        for y in 0..y_max {
            let mut outputs = vec![];
            // need to initialize each input
            let mut intcode = IntcodeComputer::new(&input, 0, 0);
            let _state = intcode.execute(&mut vec![x as i64, y as i64], &mut outputs);
            map[x % x_max + y * x_max] = *outputs.last().unwrap();
        }
    }
    let tmp: Vec<&i64> = map.iter().filter(|&i| *i == 1).collect();

    println!("first: {:?}", tmp.len());
    Ok(())
}
