use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        1
    }

    fn part_1(&self, s: String) -> i64 {
        let mut max = 0;
        let mut sub_total = 0;

        for l in s.lines() {
            // Line is either empty or has a number.
            match l {
                "" => {
                    if sub_total > max {
                        max = sub_total
                    }
                    sub_total = 0
                }
                _ => {
                    let n: i64 = l.parse().unwrap();
                    sub_total += n;
                }
            }
        }

        // Do not forget the last element
        if sub_total > max {
            max = sub_total
        }

        max
    }

    fn part_2(&self, s: String) -> i64 {
        let mut totals: Vec<i64> = Vec::new();
        let mut sub_total = 0;

        for l in s.lines() {
            match l {
                "" => {
                    totals.push(sub_total);
                    sub_total = 0;
                }
                _ => {
                    let n: i64 = l.parse().unwrap();
                    sub_total += n;
                }
            }
        }
        // Do not forget the last element
        totals.push(sub_total);

        // Sort+Reverse the vec, then sum the first 3 elements.
        totals.sort();
        totals.reverse();
        totals[0..3].iter().sum::<i64>()
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
