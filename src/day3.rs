use std::collections::HashMap;

use crate::aoc_trait::AocDay;

pub struct Day {}

/// Convert the input character to an integer. Depending on upper/lower case
/// add a correction to that integer.
fn calc_priority(c: &char) -> i64 {
    if c.is_uppercase() {
        return ((*c as u32) - 38) as i64;
    }
    ((*c as u32) - 96) as i64
}

/// Split the input string in half and get the character that appears in both
/// halves. Then calculate the priority score of the character. Return that
/// score.
fn part_1(l: &str) -> i64 {
    let (a, b) = l.split_at(l.len() / 2);
    for c in a.chars() {
        if b.contains(c) {
            return calc_priority(&c);
        }
    }
    0
}

impl AocDay for Day {
    fn num(&self) -> usize {
        3
    }

    /// Calculate score for each line in the input and sum the scores.
    fn part_1(&self, s: String) -> i64 {
        s.split("\n").map(|l| part_1(l)).sum()
    }

    /// Get the characters that is present in three consecutive lines in the
    /// input. Calculate the score for that character. Sum all the scores.
    fn part_2(&self, s: String) -> i64 {
        let mut sum = 0; // keep count of priority
        let mut hm = HashMap::new(); // use to count occurences over chunks

        // Chunk input in 3 line parts. `chunks` only works on `Vec`.
        for c in s.split("\n").collect::<Vec<&str>>().chunks(3) {
            for l in c {
                // Only a Vec<char> can be sorted and deduped
                let mut tmp = l.chars().collect::<Vec<char>>();
                tmp.sort();
                tmp.dedup();

                // Update hashmap with counts
                for cc in tmp {
                    *hm.entry(cc).or_insert(0) += 1;
                }
            }

            // Check which key has a value of `3` and calculate the priority
            // of the key. Add to the total sum.
            for (k, v) in hm.iter() {
                if v == &3 {
                    sum += calc_priority(k)
                }
            }

            // Empty the hashmap to use again for the next chunk.
            hm.clear()
        }
        sum
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            .to_string();
        let expected: i64 = 157;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            .to_string();
        let expected: i64 = 70;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
