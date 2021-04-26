use petgraph::graph::{NodeIndex, Graph, Neighbors};
use petgraph::Direction;
use queues::{Queue, IsQueue};
use std::collections::{HashMap, HashSet};
use petgraph::visit::FilterNode;
use petgraph::data::Element::Node;
use queues::*;

fn handle_neighbors(queue: & mut Queue<NodeIndex>, visited: &HashSet<NodeIndex>,
                            neighbors: Neighbors<u32, u32>) {
    for neighbor in neighbors.into_iter() {
        if visited.contains(&neighbor) {
            continue
        }
        queue.add(neighbor.clone());
    }
}

fn search_path(graph: Graph<&str, u32>, from: NodeIndex, to: NodeIndex) {
    let mut q = queue![];
    let mut visited: HashSet<petgraph::prelude::NodeIndex> = HashSet::new();
    handle_neighbors(&mut q, &visited, graph.neighbors_directed(from, Direction::Outgoing));

    while q.size() > 0 {
        let node = q.remove().expect("expected queue to contain a NodeIndex");
        handle_neighbors(&mut q, &visited, graph.neighbors_directed(node, Direction::Outgoing));
        if node == to {
            println!("Found path! {:?}, visited: {:?}", graph[node], &visited);
        }
        visited.insert(node);

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph() {

        let mut g: Graph<&str, u32> = Graph::new();
        let givat_zeev = g.add_node("givat_zeev");
        let jerusalem = g.add_node("jerusalem");
        let tel_aviv = g.add_node("tel_aviv");
        let ramat_gan = g.add_node("ramat gan");
        let yahud = g.add_node("yahud");
        let givat_shmuel = g.add_node("givat_shmuel");
        g.add_edge(givat_zeev, jerusalem, 0);
        g.add_edge(givat_zeev, tel_aviv, 0);
        g.add_edge(jerusalem, tel_aviv, 0);
        g.add_edge(tel_aviv, yahud, 0);
        g.add_edge(tel_aviv, ramat_gan, 0);
        g.add_edge(ramat_gan, givat_shmuel, 0);
        search_path(g, givat_zeev, givat_shmuel);
    }
}