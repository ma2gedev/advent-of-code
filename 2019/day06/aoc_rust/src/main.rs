use std::fs;

#[derive(Default, Debug)]
struct Tree {
    name: String,
    nodes: Vec<Tree>,
}

impl Tree {
    fn add_nodes_from_source(&mut self, source: &Vec<(String, String)>) {
        let next_node_names: Vec<String> = source.iter().filter(|(left, _)| {
            *left == self.name
        }).map(|(_, right)| right.to_string()).collect();

        for next_node_name in next_node_names {
            let mut next_tree = Tree {
                name: next_node_name,
                nodes: vec![],
            };
            next_tree.add_nodes_from_source(source);
            self.nodes.push(next_tree);
        }
    }

    fn total_depth(&self, current_depth: i32) -> i32 {
        current_depth + self.nodes.iter().map(|node| node.total_depth(current_depth + 1)).sum::<i32>()
    }
}

fn main() -> std::io::Result<()> {
    let input:Vec<(String, String)> = fs::read_to_string("../resources/input.txt")?
        .lines().map(|line| {
            let orbits: Vec<&str> = line.split(")").collect();
            (orbits[0].to_string(), orbits[1].to_string())
        }).collect();

    let mut tree = Tree {
        name: "COM".to_string(),
        nodes: vec![],
    };
    tree.add_nodes_from_source(&input);

    println!("{:?}", tree.total_depth(0));
    Ok(())
}
