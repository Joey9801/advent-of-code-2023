use std::collections::HashMap;

use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(usize);

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    source: NodeId,
    sink: NodeId,
}

#[derive(Clone, Debug)]
pub struct Graph {
    name_to_id: HashMap<String, NodeId>,

    edges: Vec<Edge>,
}

impl AsRef<Graph> for Graph {
    fn as_ref(&self) -> &Graph {
        self
    }
}

pub fn parse(input: &str) -> Graph {
    // Input like:
    //
    // jqt: rhn xhk nvd
    // rsh: frs pzl lsr
    // xhk: hfx
    // cmg: qnr nvd lhk bvb
    // rhn: xhk bvb hfx
    // bvb: xhk hfx
    // pzl: lsr hfx nvd

    // First, allocate every node name a unique ID
    let mut name_to_id = HashMap::<String, NodeId>::new();
    for line in input.lines() {
        for name in line.split_whitespace() {
            let name = name.trim_end_matches(':');
            if !name_to_id.contains_key(name) {
                let id = NodeId(name_to_id.len());
                name_to_id.insert(name.to_string(), id);
            }
        }
    }

    // Then build up the edges
    let mut edges = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut nodes = line
            .split_whitespace()
            .map(|name| name_to_id.get(name.trim_end_matches(':')).unwrap());

        let source = nodes.next().unwrap();
        for sink in nodes {
            edges.push(Edge {
                source: *source,
                sink: *sink,
            })
        }
    }

    Graph {
        name_to_id,
        edges,
    }
}

/// A single trial of the Karger Algorithm
///
/// Returns the number of nodes on the left/right of the cut, and the number of
/// edges that cross the cut
fn karger_trial(g: &Graph) -> (usize, usize, usize) {
    let mut g = g.clone();
    let mut merged_nodes = (0..g.name_to_id.len())
        .map(|i| NodeId(i))
        .map(|id| (id, 1))
        .collect::<HashMap<_, _>>();
    let mut rng = SmallRng::from_entropy();

    // The next ID we'll use for new merged nodes
    let mut next_id = NodeId(g.name_to_id.len());

    while merged_nodes.len() > 2 {
        // Pick a random edge to contract
        let edge_idx = rng.gen_range(0..g.edges.len());
        let edge = g.edges.remove(edge_idx);

        // Remove any edges identical to the one we're contracting
        g.edges.retain(|e| !{
            (e.source == edge.source && e.sink == edge.sink)
                || (e.source == edge.sink && e.sink == edge.source)
        });

        // The edge previously connected two nodes together. We're going to
        // collapse the edge such that those two nodes are merged into one new
        // node
        let left_id = edge.source;
        let right_id = edge.sink;
        let merged_id = next_id;
        next_id = NodeId(next_id.0 + 1);

        // Record which nodes are in the merged node set
        let mut merged = merged_nodes.remove(&left_id).unwrap();
        merged += merged_nodes.remove(&right_id).unwrap();
        merged_nodes.insert(merged_id, merged);

        // Update any edges that reference the old left/right nodes to reference
        // the new merged node instead
        for edge in &mut g.edges {
            if edge.source == left_id {
                edge.source = merged_id;
            }
            if edge.sink == left_id {
                edge.sink = merged_id;
            }
            if edge.source == right_id {
                edge.source = merged_id;
            }
            if edge.sink == right_id {
                edge.sink = merged_id;
            }

            assert_ne!(edge.source, edge.sink);
        }
    }

    let left = g.edges[0].source;
    let right = g.edges[0].sink;

    (
        merged_nodes.remove(&left).unwrap(),
        merged_nodes.remove(&right).unwrap(),
        g.edges.len(),
    )
}

pub fn solve_part_1(graph: &Graph) -> usize {
    let (left, right) = loop {
        let (left, right, cut) = karger_trial(graph);
        if cut == 3 {
            break (left, right);
        }
    };

    left * right
}

pub fn solve_part_2(_graph: &Graph) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_parse() {
        let g = parse(EXAMPLE_INPUT);
        dbg!(g);
    }
}
