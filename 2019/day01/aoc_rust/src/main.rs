use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("../resources/input.txt")?;
    let results: i32 = input.lines().map(|line| {
        let num: i32 = line.parse().unwrap();
        num / 3 - 2
    }).sum();
    println!("First: {:?}", results);
    Ok(())
}
