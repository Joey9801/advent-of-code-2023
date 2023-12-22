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
    dir: Option<Dir>,
}

fn next_nodes<'a>(
    map: &'a Map2d<u8>,
    current_node: DijkstraNode,
    min_in_dir: u8,
    max_in_dir: u8,
) -> impl Iterator<Item = NodeAndCost<DijkstraNode>> + 'a {
    let dirs = match current_node.dir {
        Some(dir) => [dir.rotate_left(), dir.rotate_right()],
        None => [Dir::Right, Dir::Down],
    };

    let [a, b] = dirs.map(|dir| {
        let mut cost = (1..min_in_dir)
            .map(|count| {
                let pos = current_node.pos + dir.to_vec2() * count as i64;
                map.get(pos).unwrap_or_default() as i64
            })
            .sum::<i64>();

        (min_in_dir..=max_in_dir).map_while(move |count| {
            let pos = current_node.pos + dir.to_vec2() * count as i64;
            if let Some(tile) = map.get(pos) {
                cost += tile as i64;
                Some(NodeAndCost {
                    node: DijkstraNode {
                        pos,
                        dir: Some(dir),
                    },
                    cost,
                })
            } else {
                None
            }
        })
    });

    a.chain(b)
}

pub fn solve_part_1(input: &Map2d<u8>) -> i64 {
    graph::dijkstra(
        DijkstraNode {
            pos: Vec2::new(0, 0),
            dir: None,
        },
        |node| node.pos == input.size() - Vec2::new(1, 1),
        |node| next_nodes(input, node, 1, 3),
    )
    .unwrap()
    .cost
}

pub fn solve_part_2(input: &Map2d<u8>) -> i64 {
    graph::dijkstra(
        DijkstraNode {
            pos: Vec2::new(0, 0),
            dir: None,
        },
        |node| node.pos == input.size() - Vec2::new(1, 1),
        |node| next_nodes(input, node, 4, 10),
    )
    .unwrap()
    .cost
}
