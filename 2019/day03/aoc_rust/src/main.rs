use std::fs;

#[derive(Default, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Default, Debug)]
struct Path {
    direction: String,
    len: i32,
    point1: Point,
    point2: Point
}

impl Path {
    fn step_to_point(&self, point: &Point) -> i32 {
        match self.direction.as_ref() {
            "R" => point.x - self.point1.x,
            "L" => self.point1.x - point.x,
            "U" => point.y - self.point1.y,
            "D" => self.point1.y - point.y,
            _ => panic!("do not reach"),
        }
    }
    fn include_point(&self, point: &Point) -> bool {
        match self.direction.as_ref() {
            "R" | "L" => self.in_range_x(point.x) && self.point1.y == point.y,
            "U" | "D" => self.in_range_y(point.y) && self.point1.x == point.x,
            _ => panic!("do not reach"),
        }
    }
    fn in_range_x(&self, x: i32) -> bool {
        if self.point1.x < self.point2.x {
            self.point1.x < x && x < self.point2.x
        } else {
            self.point2.x < x && x < self.point1.x
        }
    }
    fn in_range_y(&self, y: i32) -> bool {
        if self.point1.y < self.point2.y {
            self.point1.y < y && y < self.point2.y
        } else {
            self.point2.y < y && y < self.point1.y
        }
    }
    fn is_crossed(&self, path: &Path) -> bool {
        match self.direction.as_ref() {
            "R" | "L" => match path.direction.as_ref() {
                "R" | "L" => false,
                "U" | "D" => self.in_range_x(path.point1.x) && path.in_range_y(self.point1.y),
                _ => panic!("do not reach"),
            },
            "U" | "D" => match path.direction.as_ref() {
                "R" | "L" => self.in_range_y(path.point1.y) && path.in_range_x(self.point1.x),
                "U" | "D" => false,
                _ => panic!("do not reach"),
            },
            _ => panic!("do not reach"),
        }
    }

    fn cross_point(&self, path: &Path) -> Point {
        match self.direction.as_ref() {
            "R" | "L" => Point {
                x: path.point1.x,
                y: self.point1.y,
            },
            "U" | "D" => Point {
                x: self.point1.x,
                y: path.point1.y,
            },
            _ => panic!("do not reach"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let wires: Vec<Vec<(String, i32)>> = fs::read_to_string("../resources/input.txt")?
        .lines().map(|line| {
            line.split(',').map(|op| {
                let direction = op[..1].to_string();
                let len: i32 = op[1..].parse().unwrap();
                (direction, len)
            }).collect()
        }).collect();

    let wire_paths:Vec<Vec<Path>> = wires.iter().map(|wire| {
        let (paths, _) = wire.iter().fold((vec![], Point {x: 0, y: 0}), |(mut paths, last_point), (direction, len)| {
            let next_point = match direction.as_ref() {
                "R" => Point { x: last_point.x + len, y: last_point.y },
                "L" => Point { x: last_point.x - len, y: last_point.y },
                "U" => Point { x: last_point.x, y: last_point.y + len },
                "D" => Point { x: last_point.x, y: last_point.y - len },
                _ => panic!("do not reach")
            };
            paths.push(Path {
                direction: direction.to_string(),
                len: *len,
                point1: last_point,
                point2: next_point,
            });
            (paths, next_point)
        });
        paths
    }).collect();

    let mut results:Vec<Point> = vec![];
    for path_1 in wire_paths[0].iter() {
        for path_2 in wire_paths[1].iter() {
            if path_1.is_crossed(path_2) {
                results.push(path_1.cross_point(path_2));
            }
        }
    }

    let min_distance = results.iter().map(|point| {
        point.x.abs() + point.y.abs()
    }).min();

    // first
    println!("First: {:?}", min_distance);

    let each_steps:Vec<i32> = results.iter().map(|cross_point| {
        wire_paths.iter().fold(0, |acc, wire_path| {
            let mut wire_steps = 0;
            for path in wire_path.iter() {
                if path.include_point(cross_point) {
                    wire_steps += path.step_to_point(cross_point);
                    break;
                } else {
                    wire_steps += path.len;
                }
            }
            acc + wire_steps
        })
    }).collect();
    let min_steps = each_steps.iter().min();

    // second
    println!("Second: {:?}", min_steps);

    Ok(())
}
