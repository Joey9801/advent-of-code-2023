use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub struct NodeAndCost<Node> {
    pub cost: i64,
    pub node: Node,
}

impl<Node: Debug> Debug for NodeAndCost<Node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeAndCost")
            .field("cost", &self.cost)
            .field("node", &self.node)
            .finish()
    }
}

/// Wrapper around NodeAndCost that only considers the cost when comparing
struct CostOrder<Node>(NodeAndCost<Node>);

impl<Node> PartialEq for CostOrder<Node> {
    fn eq(&self, other: &Self) -> bool {
        self.0.cost == other.0.cost
    }
}

impl<Node> Eq for CostOrder<Node> {}

impl<Node> PartialOrd for CostOrder<Node> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.cost.partial_cmp(&other.0.cost)
    }
}

impl<Node> Ord for CostOrder<Node> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cost.cmp(&other.0.cost)
    }
}

pub struct Path<Node> {
    pub cost: i64,
    pub nodes: Vec<Node>,
}

pub fn dijkstra<Node, NodeIter>(
    start: Node,
    is_end: impl Fn(&Node) -> bool,
    next_nodes: impl Fn(&Node) -> NodeIter,
) -> Option<Path<Node>>
where
    Node: Clone + Eq + Hash + Debug,
    NodeIter: Iterator<Item = NodeAndCost<Node>>,
{
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    // Maps a node to the node that came before it in the optimal path
    let mut optimal_edges: HashMap<Node, Node> = HashMap::new();
    let mut end = None;

    // Wrap in a reverse as Rust's standard BinaryHeap is a max heap
    queue.push(Reverse(CostOrder(NodeAndCost {
        node: (start.clone(), start),
        cost: 0,
    })));

    while let Some(Reverse(CostOrder(NodeAndCost {
        node: (prev_node, node),
        cost: path_cost,
    }))) = queue.pop()
    {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node.clone());

        if node != prev_node {
            optimal_edges.insert(node.clone(), prev_node);
        }

        if is_end(&node) {
            end = Some(NodeAndCost {
                node,
                cost: path_cost,
            });
            break;
        }

        for NodeAndCost {
            node: next_node,
            cost: edge_cost,
        } in next_nodes(&node)
        {
            if visited.contains(&next_node) {
                continue;
            }

            queue.push(Reverse(CostOrder(NodeAndCost {
                node: (node.clone(), next_node),
                cost: path_cost + edge_cost,
            })));
        }
    }

    end.map(|end| {
        let mut path = vec![end.node];
        while let Some(prev_node) = optimal_edges.get(&path.last().unwrap()) {
            path.push(prev_node.clone());
        }
        path.reverse();

        Path {
            cost: end.cost,
            nodes: path,
        }
    })
}
