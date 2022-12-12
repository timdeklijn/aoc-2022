use std::collections::{HashMap, VecDeque};

use crate::aoc_trait::AocDay;

pub struct Day {}

enum Part {
    One,
    Two,
}

struct Visited {
    len: usize,
    exits: i32,
}

type Coord = (usize, usize);

fn solve(map: &Vec<Vec<u8>>, start: Coord, end: Coord, part: Part) -> i64 {
    let mut len = usize::MAX; // will return
    let mut path: Vec<Coord> = vec![start]; // path will contain the steps
    let mut visited: HashMap<Coord, Visited> = HashMap::new(); // record of visits
    visited.insert(start, Visited { len: 0, exits: 0 }); // insert start

    while !path.is_empty() {
        // take the last item of path
        let current: Coord = *path.last().unwrap();
        let (x, y) = current;

        // mark last item as visited.
        let v = visited.get_mut(&current).unwrap();
        v.exits += 1;

        // Get next coord, match exits and check boundaries
        let next: Coord = match v.exits {
            1 if x + 1 < map[y].len() => (x + 1, y),
            2 if x > 0 => (x - 1, y),
            3 if y + 1 < map.len() => (x, y + 1),
            4 if y > 0 => (x, y - 1),
            5.. => {
                path.pop().unwrap();
                continue;
            }
            _ => continue,
        };

        // Either go up or down
        let allowed = match part {
            Part::One => map[next.1][next.0] <= map[current.1][current.0] + 1,
            Part::Two => map[next.1][next.0] >= map[current.1][current.0] - 1,
        };

        // Make sure path is shorter then what we went to
        let path_len = path.len() < visited.get(&next).map(|v| v.len).unwrap_or(usize::MAX);

        // Follow the path
        if allowed && path_len {
            visited.insert(
                next,
                Visited {
                    len: path.len(),
                    exits: 0,
                },
            );
            // Either on the end coordinate or on a bottom value
            let is_end = match part {
                Part::One => next == end,
                Part::Two => map[next.1][next.0] == 0,
            };

            // Return or continue looking
            if is_end {
                len = std::cmp::min(len, path.len());
            } else {
                path.push(next);
            }
        }
    }
    len as i64
}

impl AocDay for Day {
    fn num(&self) -> usize {
        12
    }

    fn part_1(&self, s: String) -> i64 {
        let mut start: Coord = (0, 0);
        let mut end: Coord = (0, 0);
        let map: Vec<Vec<u8>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            0
                        }
                        'E' => {
                            end = (x, y);
                            b'z' - b'a'
                        }
                        _ => c as u8 - b'a',
                    })
                    .collect()
            })
            .collect();

        solve(&map, start, end, Part::One)
    }

    fn part_2(&self, s: String) -> i64 {
        let mut start: Coord = (0, 0);
        let mut end: Coord = (0, 0);
        let map: Vec<Vec<u8>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            0
                        }
                        'E' => {
                            end = (x, y);
                            b'z' - b'a'
                        }
                        _ => c as u8 - b'a',
                    })
                    .collect()
            })
            .collect();

        solve(&map, end, start, Part::Two)
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#
            .to_string();
        let expected: i64 = 31;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#
            .to_string();
        let expected: i64 = 29;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
