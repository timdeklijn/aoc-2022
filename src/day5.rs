/// Bit anoyhing that the answer to this day is not a number but a string. This
/// is why the returned value is not the actual anser. The answer is simply
/// printed in the solution functions.
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::aoc_trait::AocDay;

pub struct Day {}

/// Command is a type where the three values needed to move crates are saved
/// into.
#[derive(Debug)]
struct Command {
    amount: i64,
    start: i64,
    end: i64,
}

/// FromStr parses a &str into a Command. It will return an error if this
/// failes
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

/// From the input string, create a vector of Commands, a hashmap that
/// represents the stacks and a Vec with the stack numbers.
fn parse_input(s: &str) -> (Vec<Command>, HashMap<i64, Vec<char>>, Vec<i64>) {
    let mut crates = Vec::new();
    let mut nums = "";
    let mut commands = Vec::new();
    let mut find_commands = false;

    // Parse input
    for l in s.lines() {
        // skip empty lines
        if l == "" {
            continue;
        }

        // Get the stack numbers
        if l.split("  ").collect::<Vec<&str>>()[0] == " 1" {
            nums = l;
            find_commands = true;
            continue;
        }

        // either save the stack strings, or the commands
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

    // Add creates from bottom to top to the stacks
    crates.reverse();
    for s in &crates {
        // Use a Vec of chars to make indexing easy
        let s: Vec<char> = s.chars().collect();
        for i in nums.clone() {
            // index is like 1,5,9,...
            let c = s[(1 + ((i - 1) * 4)) as usize];
            // A create may not be there, if there is no crate in this stack
            // create a new vec and push the create, else push the create
            // on the stack.
            if c != ' ' {
                stacks.entry(i).or_insert_with(|| vec![]).push(c);
            }
        }
    }
    (commands, stacks, nums)
}

/// Retrieve the answer (top crate from each stack)
fn get_answer<'a>(nums: &'a Vec<i64>, stacks: &mut HashMap<i64, Vec<char>>) -> String {
    let mut answer = String::new();
    // Pop the top create from each stack in the sequence of the nums. Add the
    // create to the answer.
    for num in nums {
        let c = match stacks.get_mut(&num) {
            Some(x) => x.pop().unwrap(),
            None => unreachable!("stack should have boxes"),
        };
        answer += &c.to_string();
    }
    answer
}

impl AocDay for Day {
    fn num(&self) -> usize {
        5
    }

    fn part_1(&self, s: String) -> i64 {
        // Get the stack numbers, the commands and the stacks itself from the
        // input.
        let (commands, mut stacks, nums) = parse_input(&s);

        // Apply the commands to the stacks
        for command in commands {
            for _ in 0..command.amount as usize {
                // Pop top create
                let c = match stacks.get_mut(&command.start) {
                    Some(x) => x.pop().unwrap(),
                    None => unreachable!("stack should have boxes"),
                };

                // Add to new stack
                match stacks.get_mut(&command.end) {
                    Some(x) => x.push(c),
                    None => unreachable!("stack should have boxes"),
                };
            }
        }

        let answer = get_answer(&nums, &mut stacks);
        println!("ANSWER: {}", answer);
        1
    }

    fn part_2(&self, s: String) -> i64 {
        // Get the stack numbers, the commands and the stacks itself from the
        // input.
        let (commands, mut stacks, nums) = parse_input(&s);

        // Apply the commands to the stacks
        for command in commands {
            let mut c = match stacks.get_mut(&command.start) {
                // get the command.amount top creates
                Some(x) => {
                    let index = x.len() - command.amount as usize;
                    x.split_off(index)
                }
                None => unreachable!("stack should have boxes"),
            };

            // Add retrieved crates to the new stack
            match stacks.get_mut(&command.end) {
                Some(x) => x.append(&mut c),
                None => unreachable!("stack should have boxes"),
            };
        }

        let answer = get_answer(&nums, &mut stacks);
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
