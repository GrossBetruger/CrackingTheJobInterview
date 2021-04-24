

// Create an undirected graph with `i32` nodes and edges with `()` associated data.

#[cfg(test)]
mod tests {
    use petgraph::graph::{NodeIndex, Graph};
    use petgraph::visit::Bfs;
    use queues::*;
    use petgraph::Direction;
    use petgraph::visit::Visitable;

    #[test]
    fn graph() {
        let mut q: Queue<isize> = queue![];
        let mut g = Graph::new();
        let givat_zeev = g.add_node("givat_zeev");
        let jerusalem = g.add_node("jerusalem");
        let tel_aviv = g.add_node("tel_aviv");
        let givat_shmuel = g.add_node("givat_shmuel");
        g.add_edge(givat_zeev, jerusalem, 0);
        g.add_edge(givat_zeev, tel_aviv, 0);
        g.add_edge(jerusalem, tel_aviv, 0);
        g.add_edge(tel_aviv, givat_shmuel, 0);


        for neighbor in g.neighbors_directed(givat_zeev, Direction::Outgoing).into_iter() {
            println!("{:?}", g[neighbor]);
        }
    }
}