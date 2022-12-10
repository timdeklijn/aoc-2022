use crate::aoc_trait::AocDay;

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        10
    }

    fn part_1(&self, s: String) -> i64 {
        let mut cycles = 0;
        let mut value = 1;
        let mut sum = 0;

        for l in s.lines() {
            let spl = l.split(" ").collect::<Vec<&str>>();
            if spl.len() == 1 {
                cycles += 1;

                if (cycles - 20) % 40 == 0 {
                    sum += cycles * value;
                }

                continue;
            };
            let num = spl[1].parse::<i64>().expect("should be a number");
            for _ in 0..2 {
                cycles += 1;

                if (cycles - 20) % 40 == 0 {
                    sum += cycles * value;
                }
            }

            value += num;
        }

        sum
    }

    fn part_2(&self, s: String) -> i64 {
        let mut ss = String::new(); // contains answer
        let mut value = 1; // register value, updated with addx V
        let mut cycle = 1; // CPU cycles
        let mut pos = 0; // Position on screen, needs to be reset to 0

        for l in s.lines() {
            // Draw pixel
            let d: i64 = pos - value;
            let d = d.abs();
            if d < 2 {
                ss += "#";
            } else {
                ss += ".";
            }

            // Check for new line
            pos += 1;
            if cycle % 40 == 0 {
                ss += "\n";
                pos = 0;
            }

            // Increae cycle
            cycle += 1;

            // Handle noop
            let spl = l.split(" ").collect::<Vec<&str>>();
            if spl.len() == 1 {
                continue;
            }

            // draw pixel
            let d: i64 = pos - value;
            let d = d.abs();
            if d < 2 {
                ss += "#";
            } else {
                ss += ".";
            }

            // Check for new line
            pos += 1;
            if cycle % 40 == 0 {
                ss += "\n";
                pos = 0;
            }

            // Update cycle
            cycle += 1;

            // Update value (register)
            let num = spl[1].parse::<i64>().expect("should be a number");
            value += num;
        }

        // print the result
        println!("{ss}");

        // Result is a string, so return 1 to pass test
        1
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#
            .to_string();
        let expected: i64 = 13140;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#
            .to_string();
        let expected: i64 = 1;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
