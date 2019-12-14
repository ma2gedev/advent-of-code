use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("../resources/input.txt")?;
    println!("Hello, world!");
    Ok(())
}
