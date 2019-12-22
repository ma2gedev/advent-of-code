use std::fs;
use intcode_computer::IntcodeComputer;

fn main() -> std::io::Result<()> {
    let input: Vec<i64> = fs::read_to_string("../resources/input.txt")?
        .trim().split(',').map(|op| op.parse().unwrap()).collect();

    // first
    let mut intcode = IntcodeComputer::new(&input, 0, 0);
    let mut inputs = vec![];
    let mut outputs = vec![];
    let state = intcode.execute(&mut inputs, &mut outputs);

    let bytes: Vec<u8> = outputs.iter().map(|&i| i as u8).collect();
    let line_len = bytes.iter().position(|&b| b == 10u8).unwrap() + 1;
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
    let convert = bytes.iter().map(|&u| u as char).collect::<String>();
    println!("{}", convert);
    println!("first: {}", sum);
    Ok(())
}
