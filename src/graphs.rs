use petgraph::graph::{NodeIndex, Graph};
use petgraph::Direction;


fn search_path(graph: Graph<&str, u32>, from: NodeIndex, to: NodeIndex) {
    for neighbor in graph.neighbors_directed(from, Direction::Outgoing)
        .into_iter() {
        println!("{}", graph[neighbor]);
    }
}
#[cfg(test)]
mod tests {
    use petgraph::graph::{NodeIndex, Graph};
    use petgraph::visit::Bfs;
    use queues::*;
    use petgraph::Direction;
    use petgraph::visit::Visitable;
    use crate::graphs::search_path;

    #[test]
    fn graph() {
        let mut q: Queue<isize> = queue![];
        let mut g: Graph<&str, u32> = Graph::new();
        let givat_zeev = g.add_node("givat_zeev");
        let jerusalem = g.add_node("jerusalem");
        let tel_aviv = g.add_node("tel_aviv");
        let givat_shmuel = g.add_node("givat_shmuel");
        g.add_edge(givat_zeev, jerusalem, 0);
        g.add_edge(givat_zeev, tel_aviv, 0);
        g.add_edge(jerusalem, tel_aviv, 0);
        g.add_edge(tel_aviv, givat_shmuel, 0);

        search_path(g, givat_zeev, givat_shmuel);
    }
}