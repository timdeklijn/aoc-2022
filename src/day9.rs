use std::collections::HashSet;

use crate::aoc_trait::AocDay;

pub struct Day {}

/// Based on distance between head and el, modify el.
fn determine_move(mut el: (i64, i64), head: (i64, i64)) -> (i64, i64) {
    let delta = (head.0 - el.0, head.1 - el.1);
    match delta {
        (0, 2) => el.1 += 1,
        (0, -2) => el.1 -= 1,
        (2, 0) => el.0 += 1,
        (-2, 0) => el.0 -= 1,
        (1, 2) => {
            el.0 += 1;
            el.1 += 1;
        }
        (-1, 2) => {
            el.0 -= 1;
            el.1 += 1;
        }
        (1, -2) => {
            el.0 += 1;
            el.1 -= 1;
        }
        (-1, -2) => {
            el.0 -= 1;
            el.1 -= 1;
        }
        (2, 1) => {
            el.0 += 1;
            el.1 += 1
        }
        (2, -1) => {
            el.0 += 1;
            el.1 -= 1;
        }
        (-2, 1) => {
            el.0 -= 1;
            el.1 += 1
        }
        (-2, -1) => {
            el.0 -= 1;
            el.1 -= 1;
        }
        (2, 2) => {
            el.0 += 1;
            el.1 += 1;
        }
        (-2, -2) => {
            el.0 -= 1;
            el.1 -= 1;
        }
        (2, -2) => {
            el.0 += 1;
            el.1 -= 1;
        }
        (-2, 2) => {
            el.0 -= 1;
            el.1 += 1;
        }
        _ => (),
    }
    el
}

impl AocDay for Day {
    fn num(&self) -> usize {
        9
    }

    fn part_1(&self, s: String) -> i64 {
        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut coords = HashSet::new();
        coords.insert(tail);

        let _ = s
            .lines()
            .map(|l| {
                let (dir, amount) = l.split_once(" ").expect("input can be parsed");
                let amount = amount.trim().parse::<i64>().expect("is integer");

                for _ in 0..amount {
                    match dir {
                        "U" => head.1 += 1,
                        "D" => head.1 -= 1,
                        "L" => head.0 -= 1,
                        "R" => head.0 += 1,
                        _ => unreachable!("This is not a direction"),
                    }

                    tail = determine_move(tail, head);
                    coords.insert(tail);
                }

                amount
            })
            .collect::<Vec<i64>>();
        coords.len() as i64
    }

    fn part_2(&self, s: String) -> i64 {
        let mut rope: Vec<(i64, i64)> = (0..10).map(|_| (0, 0)).collect();
        let mut coords = HashSet::new();
        coords.insert(rope[9]);

        let _ = s
            .lines()
            .map(|l| {
                let (dir, amount) = l.split_once(" ").expect("input can be parsed");
                let amount = amount.trim().parse::<i64>().expect("is integer");

                // Move the head
                for _ in 0..amount {
                    match dir {
                        "U" => rope[0].1 += 1,
                        "D" => rope[0].1 -= 1,
                        "L" => rope[0].0 -= 1,
                        "R" => rope[0].0 += 1,
                        _ => unreachable!("This is not a direction"),
                    }

                    // move the rest
                    for i in 1..=9 {
                        rope[i] = determine_move(rope[i], rope[i - 1])
                    }

                    coords.insert(rope[9]);
                }

                amount
            })
            .collect::<Vec<i64>>();

        coords.len() as i64
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#
            .to_string();
        let expected: i64 = 13;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#
            .to_string();
        let expected: i64 = 36;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
