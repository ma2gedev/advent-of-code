use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("../resources/input.txt")?;
    let first_result: i32 = input.lines().map(|line| {
        calculate(line.parse().unwrap())
    }).sum();

    let second_result: i32 = input.lines().map(|line| {
        let mut result = 0;
        let mut calc = calculate(line.parse().unwrap());
        while calc > 0 {
            result = result + calc;
            calc = calculate(calc);
        }
        result
    }).sum();

    println!("First: {:?}\nSecond: {:?}", first_result, second_result);
    Ok(())
}

fn calculate(num: i32) -> i32 {
    num / 3 - 2
}