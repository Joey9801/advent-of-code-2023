use crate::util::{
    graph::{self, NodeAndCost},
    Dir, Map2d, Map2dExt, Vec2,
};

pub fn parse(input: &str) -> Map2d<u8> {
    Map2d::parse_grid(input, |c| c.to_digit(10).unwrap() as u8)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct DijkstraNode {
    pos: Vec2,

    // The direction that this node was entered from
    dir: Dir,

    /// The number of nodes traveled in the current direction, including this
    /// one
    count_in_dir: u8,
}

fn next_nodes(
    map: &Map2d<u8>,
    current_node: &DijkstraNode,
    min_in_dir: u8,
    max_in_dir: u8,
) -> impl Iterator<Item = NodeAndCost<DijkstraNode>> {
    let left = if current_node.count_in_dir < min_in_dir {
        None
    } else {
        let dir = current_node.dir.rotate_left();
        let pos = current_node.pos + dir;
        map.get(pos).map(|cost| NodeAndCost {
            node: DijkstraNode {
                pos,
                dir,
                count_in_dir: 1,
            },
            cost: cost as i64,
        })
    };

    let right = if current_node.count_in_dir < min_in_dir {
        None
    } else {
        let dir = current_node.dir.rotate_right();
        let pos = current_node.pos + dir;
        map.get(pos).map(|cost| NodeAndCost {
            node: DijkstraNode {
                pos,
                dir,
                count_in_dir: 1,
            },
            cost: cost as i64,
        })
    };

    let straight = if current_node.count_in_dir >= max_in_dir {
        None
    } else {
        let pos = current_node.pos + current_node.dir;
        map.get(pos).map(|cost| NodeAndCost {
            node: DijkstraNode {
                pos,
                dir: current_node.dir,
                count_in_dir: current_node.count_in_dir + 1,
            },
            cost: cost as i64,
        })
    };

    left.into_iter().chain(right).chain(straight)
}

pub fn solve_part_1(input: &Map2d<u8>) -> i64 {
    graph::dijkstra(
        DijkstraNode {
            pos: Vec2::new(0, 0),
            dir: Dir::Right,
            count_in_dir: 0,
        },
        |node| node.pos == input.size() - Vec2::new(1, 1),
        |node| next_nodes(input, node, 0, 3),
    )
    .unwrap()
    .cost
}

pub fn solve_part_2(input: &Map2d<u8>) -> i64 {
    graph::dijkstra(
        DijkstraNode {
            pos: Vec2::new(0, 0),
            dir: Dir::Right,
            count_in_dir: 0,
        },
        |node| node.pos == input.size() - Vec2::new(1, 1) && node.count_in_dir >= 4,
        |node| next_nodes(input, node, 4, 10),
    )
    .unwrap()
    .cost
}
