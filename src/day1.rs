use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        1
    }

    fn part_1(&self, s: String) -> i64 {
        s.split("\n\n")
            .map(|x| x.lines().map(|y| y.parse::<i64>().unwrap()).sum())
            .max()
            .unwrap()
    }

    fn part_2(&self, s: String) -> i64 {
        let mut answer = s
            .split("\n\n")
            .map(|x| x.lines().map(|y| y.parse::<i64>().unwrap()).sum())
            .collect::<Vec<i64>>();
        answer.sort_by(|a, b| b.cmp(a));
        answer.iter().take(3).sum::<i64>()
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"4000

5000
6000

7000
8000
9000

10000"#
            .to_string();
        let expected: i64 = 24000;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"4000

5000
6000

7000
8000
9000

10000"#
            .to_string();
        let expected: i64 = 45000;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
