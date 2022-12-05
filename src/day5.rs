use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::aoc_trait::AocDay;

pub struct Day {}

#[derive(Debug)]
struct Command {
    amount: i64,
    start: i64,
    end: i64,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        Ok(Self {
            amount: split[1].parse().unwrap(),
            start: split[3].parse().unwrap(),
            end: split[5].parse().unwrap(),
        })
    }
}

impl AocDay for Day {
    fn num(&self) -> usize {
        5
    }

    fn part_1(&self, s: String) -> i64 {
        let mut crates = Vec::new();
        let mut nums = "";
        let mut commands = Vec::new();
        let mut find_commands = false;

        // Parse input
        for l in s.lines() {
            if l == "" {
                continue;
            }
            if l.split("  ").collect::<Vec<&str>>()[0] == " 1" {
                nums = l;
                find_commands = true;
                continue;
            }

            if !find_commands {
                crates.push(l);
            } else {
                commands.push(l);
            }
        }

        // Convert nums into i64s
        let nums: Vec<i64> = nums
            .split("  ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        // Convert commands into Commands
        let commands: Vec<Command> = commands
            .iter()
            .map(|x| Command::from_str(x).unwrap())
            .collect();

        // Fill a hasmap with the crates stacked
        let mut stacks = HashMap::new();

        crates.reverse();
        for s in &crates {
            let s: Vec<char> = s.chars().collect();
            for i in &nums {
                let index = (1 + ((i - 1) * 4)) as usize;
                let c = s[index];
                if c != ' ' {
                    stacks.entry(i).or_insert_with(|| vec![]).push(c);
                }
            }
        }

        // Apply the commands to the stacks
        for command in commands {
            for _ in 0..command.amount as usize {
                let c = match stacks.get_mut(&command.start) {
                    Some(x) => x.pop().unwrap(),
                    None => unreachable!("stack should have boxes"),
                };

                match stacks.get_mut(&command.end) {
                    Some(x) => x.push(c),
                    None => unreachable!("stack should have boxes"),
                };
            }
        }

        // Retrieve the answer (top crate from each stack)
        let mut answer = String::new();
        for num in &nums {
            let c = match stacks.get_mut(&num) {
                Some(x) => x.pop().unwrap(),
                None => unreachable!("stack should have boxes"),
            };
            answer += &c.to_string();
        }

        // Print answer here because it is not an int to return
        println!("ANSWER: {}", answer);
        1
    }

    fn part_2(&self, s: String) -> i64 {
        let mut crates = Vec::new();
        let mut nums = "";
        let mut commands = Vec::new();
        let mut find_commands = false;

        // Parse input
        for l in s.lines() {
            if l == "" {
                continue;
            }
            if l.split("  ").collect::<Vec<&str>>()[0] == " 1" {
                nums = l;
                find_commands = true;
                continue;
            }

            if !find_commands {
                crates.push(l);
            } else {
                commands.push(l);
            }
        }

        // Convert nums into i64s
        let nums: Vec<i64> = nums
            .split("  ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        // Convert commands into Commands
        let commands: Vec<Command> = commands
            .iter()
            .map(|x| Command::from_str(x).unwrap())
            .collect();

        // Fill a hasmap with the crates stacked
        let mut stacks = HashMap::new();

        crates.reverse();
        for s in &crates {
            let s: Vec<char> = s.chars().collect();
            for i in &nums {
                let index = (1 + ((i - 1) * 4)) as usize;
                let c = s[index];
                if c != ' ' {
                    stacks.entry(i).or_insert_with(|| vec![]).push(c);
                }
            }
        }

        // Apply the commands to the stacks
        for command in commands {
            let mut c = match stacks.get_mut(&command.start) {
                Some(x) => {
                    let index = x.len() - command.amount as usize;
                    x.split_off(index)
                }
                None => unreachable!("stack should have boxes"),
            };

            match stacks.get_mut(&command.end) {
                Some(x) => x.append(&mut c),
                None => unreachable!("stack should have boxes"),
            };
        }

        // Retrieve the answer (top crate from each stack)
        let mut answer = String::new();
        for num in &nums {
            let c = match stacks.get_mut(&num) {
                Some(x) => x.pop().unwrap(),
                None => unreachable!("stack should have boxes"),
            };
            answer += &c.to_string();
        }

        // Print answer here because it is not an int to return
        println!("ANSWER: {}", answer);
        1
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .to_string();
        let expected: i64 = 1;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .to_string();
        let expected: i64 = 1;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
