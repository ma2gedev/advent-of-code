use std::fs;

fn candidate(val: i32) -> bool {
    let mut rest:i32 = val;
    let mut prev:i32 = i32::max_value();
    let mut adjacent:bool = false;

    while rest > 0 {
        let current = rest % 10;
        rest = rest / 10;
        if current > prev {
            return false;
        } else if current == prev {
            adjacent = true;
        }
        prev = current;
    }
    adjacent
}

fn main() -> std::io::Result<()> {
    let input:Vec<i32> = fs::read_to_string("../resources/input.txt")?
        .trim().split("-").map(|num| num.parse().unwrap()).collect();
    let range = input[0]..=input[1];
    //let range = 123..=144;

    let values:Vec<i32> = range.filter(|i| {
        candidate(*i)
    }).collect();

    println!("{:?}", values.len());

    Ok(())
}