use std::collections::HashSet;

use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        4
    }

    fn part_1(&self, s: String) -> i64 {
        s.lines()
            .filter(|l| {
                let a = l
                    .split(",")
                    .flat_map(|x| x.split("-").map(|y| y.parse().unwrap()))
                    .collect::<Vec<i64>>();

                if a[0] >= a[2] && a[1] <= a[3] || a[0] <= a[2] && a[1] >= a[3] {
                    return true;
                }
                false
            })
            .count() as i64
    }

    fn part_2(&self, s: String) -> i64 {
        s.lines()
            .filter(|l| {
                let nums = l
                    .split(",")
                    .flat_map(|x| x.split("-").map(|x| x.parse().unwrap()))
                    .collect::<Vec<i64>>();

                // HashSets from range
                let elf_1: HashSet<i64> =
                    HashSet::from_iter((nums[0]..=nums[1]).collect::<Vec<i64>>());
                let elf_2: HashSet<i64> =
                    HashSet::from_iter((nums[2]..=nums[3]).collect::<Vec<i64>>());

                elf_1.intersection(&elf_2).into_iter().count() >= 1
            })
            .count() as i64
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            .to_string();
        let expected: i64 = 2;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            .to_string();
        let expected: i64 = 4;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
