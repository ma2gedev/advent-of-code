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

    fn find_node_route(&self, name: String, current_depth: i32) -> Option<Vec<(i32, String)>> {
        if self.name == name {
            Some(vec![(current_depth, self.name.to_string())])
        } else {
            match self.nodes.iter().map(|node| {
                node.find_node_route(name.to_string(), current_depth + 1)
            }).find(|route| route.is_some()) {
                Some(Some(route)) => {
                    let mut routes = vec![(current_depth, self.name.to_string())];
                    route.iter().for_each(|(d, n)| routes.push((*d, n.to_string())));
                    Some(routes)
                },
                Some(None) => panic!("do not reach"), // how to remove this line?
                None => None,
            }
        }
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

    // first
    println!("{:?}", tree.total_depth(0));

    // second
    let route_to_you = tree.find_node_route("YOU".to_string(), 0).unwrap();
    let route_to_santa = tree.find_node_route("SAN".to_string(), 0).unwrap();
    let mut i = 0;
    let crossed_orbit;
    loop {
        if route_to_you[i].1 != route_to_santa[i].1 {
            crossed_orbit = route_to_you[i].0 - 1;
            break;
        }
        i += 1;
    }
    let transfer = (route_to_you[route_to_you.len() - 1].0 - crossed_orbit - 1)
        + (route_to_santa[route_to_santa.len() - 1].0 - crossed_orbit - 1);
    println!("{:?}", transfer);
    Ok(())
}
