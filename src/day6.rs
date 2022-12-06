use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        6
    }

    /// Get last index of unique window (size 4) chars from input string
    fn part_1(&self, s: String) -> i64 {
        for (i, w) in s.chars().collect::<Vec<_>>().windows(4).enumerate() {
            if !(1..w.len()).any(|i| w[i..].contains(&w[i - 1])) {
                return i as i64 + 4;
            }
        }

        0
    }

    /// Get last index of unique window (size 14) chars from input string
    fn part_2(&self, s: String) -> i64 {
        for (i, w) in s.chars().collect::<Vec<_>>().windows(14).enumerate() {
            if !(1..w.len()).any(|i| w[i..].contains(&w[i - 1])) {
                return i as i64 + 14;
            }
        }

        0
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#.to_string();
        let expected: i64 = 11;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#.to_string();
        let expected: i64 = 29;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
