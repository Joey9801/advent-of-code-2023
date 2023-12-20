use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MapNodeId(usize);

#[derive(Clone, Copy, Debug)]
struct NodeLinks {
    left: MapNodeId,
    right: MapNodeId,
}

#[derive(Clone, Debug)]
struct Map {
    name_to_id: HashMap<String, MapNodeId>,
    node_links: Vec<NodeLinks>,
}

impl Map {
    fn get_node(&self, name: &str) -> Option<MapNodeId> {
        self.name_to_id.get(name).copied()
    }

    fn filter_nodes<'a>(
        &'a self,
        predicate: impl Fn(&str) -> bool + 'a,
    ) -> impl Iterator<Item = MapNodeId> + 'a {
        self.name_to_id
            .iter()
            .filter(move |(name, _)| predicate(name))
            .map(|(_, id)| *id)
    }

    fn next_node(&self, node: MapNodeId, dir: Dir) -> MapNodeId {
        match dir {
            Dir::Left => self.node_links[node.0].left,
            Dir::Right => self.node_links[node.0].right,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    instructions: Vec<Dir>,
    map: Map,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    // Input like:
    //
    // RLLLRL
    //
    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)

    let mut lines = input.lines();

    // First parse the instructions
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<_>>();

    // Skip the blank line
    lines.next();

    // Then make a stringy version of the map
    let nodes = lines
        .map(|line| {
            let node_name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (node_name, (left, right))
        })
        .collect::<HashMap<_, _>>();

    // Sanity check that every node name seen goes somewhere
    let all_destinations = nodes
        .values()
        .flat_map(|(left, right)| [left, right])
        .collect::<HashSet<_>>();
    assert!(all_destinations
        .iter()
        .all(|name| nodes.contains_key(*name)));

    // Fix the ordering of the node names, it doesn't matter what that ordering is
    let node_names = nodes.keys().map(|s| s).collect::<Vec<_>>();

    let name_to_id = node_names
        .iter()
        .enumerate()
        .map(|(id, name)| (name.to_string(), MapNodeId(id)))
        .collect::<HashMap<_, _>>();

    // Build the ID-based map in the same order, so it can be stored in a simple vec
    let node_links = node_names
        .iter()
        .map(|name| {
            let (left, right) = nodes.get(*name).unwrap();
            let left = name_to_id.get(*left).unwrap();
            let right = name_to_id.get(*right).unwrap();

            NodeLinks {
                left: *left,
                right: *right,
            }
        })
        .collect::<Vec<_>>();

    Input {
        instructions,
        map: Map {
            name_to_id,
            node_links,
        },
    }
}

pub fn solve_part_1(input: &Input) -> u64 {
    let mut steps = 0;
    let mut node = input.map.get_node("AAA").unwrap();
    let zzz = input.map.get_node("ZZZ").unwrap();

    while node != zzz {
        for dir in &input.instructions {
            node = input.map.next_node(node, *dir);
            steps += 1;
        }
    }

    steps
}

pub fn solve_part_2(input: &Input) -> i64 {
    let source_nodes = input.map.filter_nodes(|name| name.ends_with('A'));
    let sink_nodes = input
        .map
        .filter_nodes(|name| name.ends_with('Z'))
        .collect::<HashSet<_>>();

    // Assume that each source node only ever reaches a single one of the sink
    // nodes after an integer number of applications of the instructions. If
    // that holds, then each source node will have some number of preamble steps
    // before visiting a sink node for the first time, then visits a sink node
    // on a regular clock.

    let mut preambles = Vec::new();
    let mut periods = Vec::new();

    for source_node in source_nodes {
        let mut steps = 0;
        let mut node = source_node;
        let mut first_sink_node = None;

        loop {
            for dir in &input.instructions {
                node = input.map.next_node(node, *dir);
                steps += 1;
            }

            if sink_nodes.contains(&node) {
                match &mut first_sink_node {
                    None => {
                        first_sink_node = Some(node);
                        preambles.push(steps);
                    }
                    Some(first_sink_node) => {
                        assert!(node == *first_sink_node);
                        periods.push(steps - preambles.last().unwrap());
                        break;
                    }
                }
            }
        }
    }

    // It turns out (at least in my input) that the preambles are all the same
    // as the periods, such that the answer is just the plain lcm of the periods
    // rather than anything clever with phase offsets
    debug_assert!(preambles.iter().zip(periods.iter()).all(|(a, b)| a == b));

    // The common period
    crate::util::lcm_iter(periods.iter().copied())
}
