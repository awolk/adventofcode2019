use std::collections::HashMap;

struct Graph<'a> {
    children: HashMap<&'a str, Vec<&'a str>>,
    parent: HashMap<&'a str, &'a str>,
}

impl<'a> Graph<'a> {
    fn from_str(input: &'a str) -> Result<Graph, &'static str> {
        let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut parent: HashMap<&str, &str> = HashMap::new();
        for line in input.lines() {
            if let [center, child] = line.split(')').collect::<Vec<_>>().as_slice() {
                children.entry(center).or_insert(Vec::new()).push(child);
                parent.insert(child, center);
            } else {
                return Err("invalid line");
            }
        }

        Ok(Graph { children, parent })
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

    fn orbital_transfers(&self, a: &str, b: &str) -> Option<u32> {
        let mut a_hierarchy = HashMap::new();

        // record all of a's ancestors
        let mut a_ancestor = self.parent.get(a);
        let mut distance = 0; // a is already in orbit of a's parent
        while let Some(object) = a_ancestor {
            a_hierarchy.insert(object, distance);

            a_ancestor = self.parent.get(object);
            distance += 1;
        }

        // go through b's ancestors until we find a common ancestor
        let mut b_ancestor = self.parent.get(b);
        distance = 0; // b is already in orbit of b's parent
        while let Some(object) = b_ancestor {
            if let Some(&common_ancestor_dist_to_a) = a_hierarchy.get(object) {
                // found a common ancestor, so return the distances
                return Some(common_ancestor_dist_to_a + distance);
            }

            b_ancestor = self.parent.get(object);
            distance += 1;
        }

        None
    }
}

fn main() {
    let input = include_str!("input.txt");
    let graph = Graph::from_str(input).expect("failed to parse graph");
    println!("Part 1 total orbits: {}", graph.total_orbits());
    println!(
        "Part 2 orbital transfers: {}",
        graph
            .orbital_transfers("SAN", "YOU")
            .expect("failed to find path")
    );
}
