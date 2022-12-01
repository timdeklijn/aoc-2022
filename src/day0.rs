use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        0
    }

    fn part_1(&self, s: String) -> i64 {
        let mut count = 0;
        let lines: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();
        for (i, el) in lines.clone().into_iter().enumerate() {
            if i == 0 {
                continue;
            }
            if lines[i - 1] < el {
                count += 1
            }
        }
        count
    }

    fn part_2(&self, s: String) -> i64 {
        let mut count = 0; // update when condition is met
        let mut prev = 0; // used to store previous sum

        // Create a vec of i64s from the input data
        let lines: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();

        for i in 0..lines.len() - 2 {
            if prev == 0 {
                prev = lines[i] + lines[i + 1] + lines[i + 2];
                continue;
            }
            let cur = lines[i] + lines[i + 1] + lines[i + 2];
            if prev < cur {
                count += 1
            }
            prev = cur;
        }
        count
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"199
200
208
210
200
207
240
269
260
263"#
            .to_string();
        let expected: i64 = 7;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"199
200
208
210
200
207
240
269
260
263"#
            .to_string();
        let expected: i64 = 5;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
