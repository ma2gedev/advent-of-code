use std::fs;

fn main() -> std::io::Result<()> {
    let input: String = fs::read_to_string("../resources/input.txt")?;
    let input_str: &str = input.trim();
    let input_num: Vec<u32> = input_str.chars().map(|num_char| {
        num_char.to_digit(10).unwrap()
    }).collect();

    // first
    let width = 25;
    let height = 6;
    let layer_num = input_str.len() / (width * height);

    let (min_index, _zero_counts) = (0..layer_num).map(|i| {
        let first: usize = i * width * height;
        let last: usize = (i + 1) * width * height;
        (i, (first..last).filter(|&j| {
            input_num[j] == 0
        }).collect::<Vec<usize>>().len())
    }).min_by_key(|&(_i, value)| value).unwrap();

    let first = min_index * width * height;
    let last = (min_index + 1) * width * height;
    let one_num = (first..last).filter(|&i| input_num[i] == 1).count();
    let two_num = (first..last).filter(|&i| input_num[i] == 2).count();
    println!("First: {:?}", one_num * two_num);

    // second
    let mut image = vec![2; width * height];
    image.copy_from_slice(&input_num[0..(width * height)]);

    for i in 0..layer_num {
        let first: usize = i * width * height;
        let last: usize = (i + 1) * width * height;
        for j in first..last {
            match image[j - first] {
                2 => image[j - first] = input_num[j],
                _ => (), // do nothing
            }
        }
    }
    println!("Second:");
    for i in 0..height {
        let str: String = ((i * width)..((i + 1) * width))
          .map(|num| std::char::from_digit(image[num], 10).unwrap()).collect();
        println!("{:?}", str);
    }

    Ok(())
}
