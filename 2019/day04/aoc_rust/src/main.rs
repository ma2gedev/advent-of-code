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

fn candidate2(val: i32) -> bool {
    let mut rest:i32 = val;
    let mut prev:i32 = i32::max_value();
    let mut adjacents:[i32; 10] = [0; 10];

    while rest > 0 {
        let current = rest % 10;
        rest = rest / 10;
        if current > prev {
            return false;
        } else if current == prev {
            adjacents[current as usize] += 1;
        }
        prev = current;
    }
    // TODO: want to shorten
    let tmp:Vec<&i32> = adjacents.iter().filter(|&x| *x == 1).collect();
    tmp.len() > 0
}

fn main() -> std::io::Result<()> {
    let input:Vec<i32> = fs::read_to_string("../resources/input.txt")?
        .trim().split("-").map(|num| num.parse().unwrap()).collect();
    let range = input[0]..=input[1];

    let values:Vec<i32> = range.clone().filter(|i| {
        candidate(*i)
    }).collect();

    // first
    println!("First {:?}", values.len());

    let values2:Vec<i32> = range.filter(|i| {
        candidate2(*i)
    }).collect();

    // second
    println!("Second {:?}", values2.len());
    Ok(())
}

#[test]
fn test_candidate() {
    assert!(candidate(122345));
    assert!(candidate(111111));
    assert!(!candidate(223450));
    assert!(!candidate(123789));
}

#[test]
fn test_candidate2() {
    assert!(candidate2(112233));
    assert!(!candidate2(123444));
    assert!(candidate2(111122));
}