use std::fs;

fn main() -> std::io::Result<()> {
    let input: String = fs::read_to_string("../resources/input.txt")?;
    let input_str: &str = input.trim();
    let input_num: Vec<i32> = input_str.chars().map(|num_char| {
        num_char.to_string().parse().unwrap()
    }).collect();

    // first
    let width = 25;
    let height = 6;
    let layer_num = input_str.len() / (width * height);
    let mut layers: Vec<Vec<Vec<i32>>> = vec![];

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

    // let mut i = 0; // for
    // while i < layer_num {
    //     let base = i * width * height;
    //     let mut layer = vec![];
    //     let mut h = 0;
    //     while h < height {
    //         let height_base = h * width;
    //         let mut width_line: Vec<i32> = vec![];
    //         let mut w = 0;
    //         while w < width {
    //             width_line.push(input_num[base + height_base + w]);
    //             w += 1;
    //         }
    //         layer.push(width_line);
    //         h += 1;
    //     }
    //     layers.push(layer);
    //     i += 1;
    // }

    Ok(())
}
