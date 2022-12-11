use std::{num::ParseIntError, str::FromStr};

use crate::aoc_trait::AocDay;

/// Keep track of what part of the puzzle is being solved
enum Part {
    One,
    Two,
}

/// Container for the monkey operation
#[derive(Debug)]
struct Op {
    op: String,
    num: Option<i64>,
}

impl Op {
    /// Do the operation. If self.num is None use val as value for the op.
    fn do_it(&self, val: &i64) -> i64 {
        let n = match self.num {
            Some(n) => n,
            None => *val,
        };
        match self.op.as_str() {
            "*" => n * val,
            "+" => n + val,
            _ => unreachable!("Not an op"),
        }
    }
}

impl FromStr for Op {
    type Err = ParseIntError;

    /// Parse an 'op' block. This can either contain a number or the string 'old'.
    /// Depending on this return an option.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<&str> = s.split(" ").collect();
        let op = spl[4].to_string();
        let num = match spl[5] {
            "old" => None,
            _ => Some(spl[5].parse::<i64>().unwrap()),
        };
        Ok(Op { op, num })
    }
}

/// Container for all monkey values.
#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    test: i64,
    if_true: usize,
    if_false: usize,
    inspected: i64,
}

impl Monkey {
    /// Calc the new worry score based on the monkey's `op`. Depending on the part
    /// this value is modified or not.
    fn calc_worry(&self, val: &i64, part: &Part) -> i64 {
        let new_val = self.op.do_it(val);
        match part {
            Part::One => new_val / 3,
            Part::Two => new_val,
        }
    }

    // Check if the worry is divisible by the test number and return the asociated
    // monkey index.
    fn run_test(&self, val: &i64) -> usize {
        let rem = val % self.test;
        if rem == 0 {
            return self.if_true;
        }
        self.if_false
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    /// Parse a monkey block.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<&str> = s.lines().collect();

        let items = l[1]
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|x| x.trim().parse::<i64>().expect("item is a number"))
            .collect::<Vec<i64>>();

        let op = Op::from_str(l[2].split_once(":").unwrap().1).expect("op should create");

        let test = l[3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<i64>()
            .expect("test number");

        let if_true = l[4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("if true");

        let if_false = l[5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("if false");

        Ok(Monkey {
            items,
            op,
            test,
            if_true,
            if_false,
            inspected: 0,
        })
    }
}

/// Container for the full list of monkeys.
#[derive(Debug)]
struct Monkeys {
    inner: Vec<Monkey>,
}

impl Monkeys {
    /// Run a single round of monkeys inspecting items.
    fn round(&mut self, part: &Part) {
        // Chinese remainder theorem:
        let mm: i64 = self.inner.iter().map(|m| m.test).product();

        for m in 0..self.inner.len() {
            for i in 0..self.inner[m].items.len() {
                // Calculate worry and run the test to see to which monkey the item
                // will be passed
                let worry = self.inner[m].calc_worry(&self.inner[m].items[i], part);
                let index = self.inner[m].run_test(&worry);

                // Reduce the worry value for part two
                match part {
                    Part::One => {
                        self.inner[index].items.push(worry);
                    }
                    Part::Two => {
                        // Use this special modifyer to reduce the worry number withou
                        // modifying its 'modulo' properties
                        let new_worry = worry % mm;
                        self.inner[index].items.push(new_worry);
                    }
                }
            }
            // Update inspection number and clear monkey's item list.
            self.inner[m].inspected += self.inner[m].items.len() as i64;
            self.inner[m].items.clear();
        }
    }
}
impl FromStr for Monkeys {
    type Err = ParseIntError;

    /// Parse the full input
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .split("\n\n")
            .map(|m| Monkey::from_str(m).expect("monkey should exists"))
            .collect::<Vec<Monkey>>();
        Ok(Monkeys { inner })
    }
}

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        11
    }

    fn part_1(&self, s: String) -> i64 {
        let mut monkeys = Monkeys::from_str(&s).expect("monkeys can be parsed");
        let part = Part::One;
        for _ in 0..20 {
            monkeys.round(&part);
        }
        //
        // Fet the two highest inspection numbers
        let mut answer_vec = Vec::new();
        for m in monkeys.inner {
            answer_vec.push(m.inspected);
        }
        answer_vec.sort_by(|a, b| b.cmp(a));
        answer_vec[0] * answer_vec[1]
    }

    fn part_2(&self, s: String) -> i64 {
        let mut monkeys = Monkeys::from_str(&s).expect("monkeys can be parsed");
        let part = Part::Two;
        for _ in 0..10000 {
            monkeys.round(&part);
        }
        let mut answer_vec = Vec::new();
        for m in monkeys.inner {
            answer_vec.push(m.inspected);
        }

        answer_vec.sort_by(|a, b| b.cmp(a));
        answer_vec[0] * answer_vec[1]
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
            "#
        .to_string();
        let expected: i64 = 10605;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
            "#
        .to_string();
        let expected: i64 = 2713310158;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
