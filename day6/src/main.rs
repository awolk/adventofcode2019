use std::collections::HashMap;

struct Graph<'a> {
    children: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn from_str(input: &'a str) -> Result<Graph, &'static str> {
        let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in input.lines() {
            if let [center, child] = line.split(')').collect::<Vec<_>>().as_slice() {
                children.entry(center).or_insert(Vec::new()).push(child);
            } else {
                return Err("invalid line");
            }
        }

        Ok(Graph { children })
    }

    fn total_orbits_with_base(&self, start: &str, base_depth: u32) -> u32 {
        if let Some(children) = self.children.get(start) {
            base_depth
                + children
                    .iter()
                    .map(|child| self.total_orbits_with_base(child, base_depth + 1))
                    .sum::<u32>()
        } else {
            base_depth
        }
    }

    fn total_orbits(&self) -> u32 {
        // calculate a sum of the depth of each node in the graph
        self.total_orbits_with_base("COM", 0)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let graph = Graph::from_str(input).expect("failed to parse graph");
    println!("Part 1 total orbits: {}", graph.total_orbits());
}
