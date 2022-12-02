# Advent of Code 2022

Try it in rust for a change. Wrote a small program that takes a struct with a
specific trait. This will be run based on command line input:

``` bash
cargo run 1 1
```

Will run part one of day 1. The struct looks like:

``` rust
use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        1
    }

    fn part_1(&self, s: String) -> i64 {
      todo!()
    }

    fn part_2(&self, s: String) -> i64 {
      todo!()
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#""#
            .to_string();
        let expected: i64 = 1;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#""#
            .to_string();
        let expected: i64 = 1;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
```

The `num` function simply returns the day number. `part_1` and `part_2` contain
the solutions of the specific parts and the tests are written in `test_part_1`
and `test_part_2`.

## Adding a new day

- add `mod dayX` to the top of `main.rs`
- add `"X" => Box::new(dayX::Day {}),` to the `main` function.
- create a file called `src/dayX`
- Add the template from above to the file and start working on a solution.
- Once the test passes add the input for that day to: `data/day_X.txt`
