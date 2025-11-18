use std::arch::global_asm;
use std::collections::HashMap;
use rand::Rng;
use rand::seq::SliceRandom;  // brings .choose() into scope
use rand::prelude::IndexedRandom;

#[derive(Debug)]
#[derive(Clone)]
struct Graph {
    adjacency_matrix: Vec<Vec<usize>>, 
    node_names: Vec<String>,
    name_to_index: HashMap<String, usize>,
    nodes: usize,
    edges: usize,
}

impl Graph {
    /// Create an empty graph (no nodes)
    fn new() -> Self {
        Graph {
            adjacency_matrix: Vec::new(),
            nodes: 0,
            node_names: Vec::new(),
            name_to_index: HashMap::new(),
            edges: 0,
        }
    }

    /// Add a new node with a name
    fn add_node(&mut self, name: &str) {
        let index = self.nodes;
        self.nodes += 1;

        self.node_names.push(name.to_string());
        self.name_to_index.insert(name.to_string(), index);

        // Grow existing rows
        for row in &mut self.adjacency_matrix {
            row.push(0);
        }

        // Add new row
        self.adjacency_matrix.push(vec![0; self.nodes]);
    }
    fn delete_node(&mut self, name: &str) {
        // Lookup the index
        let Some(&i) = self.name_to_index.get(name) else {
            return; // Node does not exist
        };


        // 1. Remove row i
        self.adjacency_matrix.remove(i);

        // 2. Remove column i from every row
        for row in &mut self.adjacency_matrix {
            row.remove(i);
        }

        // 3. Remove name and index mapping
        self.node_names.remove(i);
        self.name_to_index.remove(name);

        // 4. Fix name_to_index mappings (shift down indices > i)
        for (_name, idx) in self.name_to_index.iter_mut() {
            if *idx > i {
                *idx -= 1;
            }
        }

        // 5. Update node count
        self.nodes -= 1;

        // 6. Recompute edge count
        self.recompute_edges();
    }


    /// Add one edge (supports multigraph)
    fn add_edge(&mut self, from: &str, to: &str) {
        if let (Some(&i), Some(&j)) = (
            self.name_to_index.get(from),
            self.name_to_index.get(to),
        ) {
            self.adjacency_matrix[i][j] += 1;
            self.adjacency_matrix[j][i] += 1;
            self.edges += 1; // only count if valid
        }
    }

    /// Delete one edge
    fn delete_edge(&mut self, from: &str, to: &str) {
        if let (Some(&i), Some(&j)) = (
            self.name_to_index.get(from),
            self.name_to_index.get(to),
        ) {
            if self.adjacency_matrix[i][j] > 0 {
                self.adjacency_matrix[i][j] -= 1;
                self.edges -= 1;
            }
        }
    }
    fn recompute_edges(&mut self) {
        let mut e = 0;
        for i in 0..self.nodes {
            for j in i+1..self.nodes {
                e += self.adjacency_matrix[i][j];
            }
        }
        self.edges = e / 2;
    }
    fn contract_edge(&mut self, from : &str, to : &str){
        if let (Some(&i), Some(&j)) = (
            self.name_to_index.get(from),
            self.name_to_index.get(to),
        ) {
            self.add_node(&(from.to_string() + "_" + to));
            for x in 0..self.nodes - 1{
                self.adjacency_matrix[x][self.nodes - 1] = self.adjacency_matrix[x][i] + self.adjacency_matrix[x][j];
                self.adjacency_matrix[self.nodes - 1][x] = self.adjacency_matrix[x][self.nodes - 1];
            }
            self.delete_node(from);
            self.delete_node(to);
        }
    }
    fn random_edge(&self) -> Option<(&str, &str)> {
        let mut edges: Vec<(&str, &str)> = Vec::new();

        // Enumerate all edges
        for n1  in &self.node_names {
            for n2  in &self.node_names { // i..j for undirected graph
                if let (Some(&i), Some(&j)) = (
                    self.name_to_index.get(n1),
                    self.name_to_index.get(n2),
                ) {
                    let count = self.adjacency_matrix[i][j];
                    if count > 0 {
                        // Add the edge multiple times if there are multiple edges
                        for _ in 0..count {
                            edges.push((&self.node_names[i], &self.node_names[j]));
                        }
                    }
                }
            }
        }

        if edges.is_empty() {
            None
        } else {
            let mut rng = rand::rng();
            Some(*edges.choose(&mut rng).unwrap())
        }
    }


}
struct Krager {
    graph: Graph, 

}

impl Krager{
    fn new(g : Graph) -> Self {
        Krager {
            graph : g,
        }
    }
    fn krager_iteration(&mut self) -> Option<(String, String)>{
        let mut rng = rand::rng();

        let mut g1 = self.graph.clone();
        while (g1.nodes > 2 && g1.edges > 1){
            if let Some((from, to)) = g1.random_edge() {
                let from = from.to_string();
                let to = to.to_string();
                g1.contract_edge(from.as_str(), to.as_str());
            }
        }
        return Some((
            g1.node_names.get(0).unwrap().to_string(),
            g1.node_names.get(1).unwrap().to_string()
        ));
    }
}
fn main() {
    let mut g = Graph::new();

    g.add_node("A");
    g.add_node("B");
    g.add_node("C");

    g.add_edge("A", "B");
    g.add_edge("A", "B"); // multigraph: now A→B has 2 edges
    g.add_edge("B", "C");
    let mut k = Krager::new(g);

    println!("{:?}", k.krager_iteration());

}
