use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::anyhow;

/// Maps a contiguous range of IDs in space A to a contiguous range of IDs in space B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MappingChunk {
    /// The first ID in space A
    source_start: i64,

    /// The last ID in space A
    source_end: i64,

    /// The constant offset to add to the source ID to get the destination ID
    offset: i64,
}

impl MappingChunk {
    fn dest_range(&self) -> RangeInclusive<i64> {
        (self.source_start + self.offset)..=(self.source_end + self.offset)
    }
}

impl FromStr for MappingChunk {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Expect a line like "50 98 2"
        // The numbers are: dest_start, source_start, and range_len
        let mut parts = s.split_whitespace();
        let dest_start: i64 = parts
            .next()
            .ok_or_else(|| anyhow!("expected dest_start"))?
            .parse()?;
        let source_start: i64 = parts
            .next()
            .ok_or_else(|| anyhow!("expected source_start"))?
            .parse()?;
        let range_len: i64 = parts
            .next()
            .ok_or_else(|| anyhow!("expected range_len"))?
            .parse()?;

        Ok(MappingChunk {
            source_start,
            source_end: source_start + range_len - 1,
            offset: dest_start - source_start,
        })
    }
}

#[derive(Debug)]
struct Mapping {
    /// Set of non-overlapping chunks, sorted by source_start
    ///
    /// The range covered is contiguous, ie interspersed with chunks with a zero
    /// offset
    chunks: Vec<MappingChunk>,
}

impl Mapping {
    fn query_point(&self, source_id: i64) -> i64 {
        for chunk in &self.chunks {
            if source_id >= chunk.source_start && source_id <= chunk.source_end {
                return source_id + chunk.offset;
            }
        }

        source_id
    }

    /// Generate a set of mapping chunks that cover the given range of source
    /// IDs exactly
    fn query_range(
        &self,
        source_range: RangeInclusive<i64>,
    ) -> impl Iterator<Item = MappingChunk> + '_ {
        // Find the first chunk idx that doesn't end before the start of the source_range
        let chunk_idx = self
            .chunks
            .iter()
            .position(|chunk| chunk.source_end >= *source_range.start())
            .unwrap_or(self.chunks.len());

        RangeQueryIter {
            mapping: self,
            source_start: *source_range.start(),
            source_end: *source_range.end(),
            chunk_idx,
        }
    }
}

#[derive(Debug)]
struct RangeQueryIter<'a> {
    mapping: &'a Mapping,

    // The remaining piece of the source range to cover
    source_start: i64,
    source_end: i64,

    // The next chunk to try intersecting
    chunk_idx: usize,
}

impl<'a> Iterator for RangeQueryIter<'a> {
    type Item = MappingChunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source_start > self.source_end {
            return None;
        }

        if self.chunk_idx >= self.mapping.chunks.len() {
            let chunk = MappingChunk {
                source_start: self.source_start,
                source_end: self.source_end,
                offset: 0,
            };

            self.source_start = self.source_end + 1;
            return Some(chunk);
        }

        let map_chunk = &self.mapping.chunks[self.chunk_idx];

        let source_start = self.source_start;
        let source_end;
        let offset;
        if map_chunk.source_start > self.source_start {
            // The prefix before the next mapping chunk
            source_end = map_chunk.source_start - 1;
            offset = 0;
        } else {
            // The bit of the next mapping chunk covered by the requested source range
            source_end = map_chunk.source_end.min(self.source_end);
            offset = map_chunk.offset;
            self.chunk_idx += 1;
        };

        self.source_start = source_end + 1;

        Some(MappingChunk {
            source_start,
            source_end,
            offset,
        })
    }
}

pub struct Input {
    source_ids: Vec<i64>,

    // Set of mappings arranged a -> b, b -> c, etc..
    mappings: Vec<Mapping>,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

pub fn parse(input: &str) -> Input {
    // Parses a string like:
    //
    // seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    let mut lines = input.lines().filter(|line| !line.is_empty());

    let source_ids = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut mappings = Vec::new();

    for line in lines {
        if line.ends_with("map:") {
            mappings.push(Mapping { chunks: Vec::new() });
        } else {
            let chunk = line.parse().unwrap();
            mappings.last_mut().unwrap().chunks.push(chunk);
        }
    }

    // Ensure all the mappings are correctly sorted
    for mapping in &mut mappings {
        mapping.chunks.sort_by_key(|chunk| chunk.source_start);
    }

    Input {
        source_ids,
        mappings,
    }
}

pub fn solve_part_1(input: &Input) -> i64 {
    let mut min = i64::MAX;

    for id in &input.source_ids {
        let mut id = *id;
        for mapping in &input.mappings {
            id = mapping.query_point(id);
        }

        min = min.min(id);
    }

    min
}

pub fn solve_part_2(input: &Input) -> i64 {
    fn min_dest(source_range: RangeInclusive<i64>, mappings: &[Mapping]) -> i64 {
        match mappings {
            [] => *source_range.start(),
            [first, rest @ ..] => {
                let mut min = i64::MAX;
                for chunk in first.query_range(source_range) {
                    let this_min = min_dest(chunk.dest_range(), rest);
                    min = min.min(this_min);
                }
                min
            }
        }
    }

    let starts = input.source_ids.iter().copied().step_by(2);
    let lens = input.source_ids.iter().copied().skip(1).step_by(2);
    let mut min = i64::MAX;
    for (start, len) in starts.zip(lens) {
        let source_range = start..=(start + len - 1);
        min = min.min(min_dest(source_range, &input.mappings));
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mapping_query_range() {
        let mapping = Mapping {
            chunks: vec![
                MappingChunk {
                    source_start: 100,
                    source_end: 199,
                    offset: 50,
                },
                MappingChunk {
                    source_start: 300,
                    source_end: 399,
                    offset: -50,
                },
            ],
        };

        let query = mapping.query_range(0..=349).collect::<Vec<_>>();
        assert_eq!(
            query,
            vec![
                MappingChunk {
                    source_start: 0,
                    source_end: 99,
                    offset: 0,
                },
                MappingChunk {
                    source_start: 100,
                    source_end: 199,
                    offset: 50,
                },
                MappingChunk {
                    source_start: 200,
                    source_end: 299,
                    offset: 0,
                },
                MappingChunk {
                    source_start: 300,
                    source_end: 349,
                    offset: -50,
                },
            ]
        )
    }

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE_INPUT);
        let ans = solve_part_1(&input);
        assert_eq!(ans, 35)
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE_INPUT);
        let ans = solve_part_2(&input);
        assert_eq!(ans, 46)
    }
}
