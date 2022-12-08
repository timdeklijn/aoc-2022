use std::{num::ParseIntError, str::FromStr};

use crate::aoc_trait::AocDay;

pub struct Day {}

#[derive(Debug)]
struct Tree {
    height: usize,
    visible: bool,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

#[derive(Debug)]
struct Forrest {
    trees: Vec<Vec<Tree>>,
}

impl Forrest {
    // Remove corners from width + height. Add 4 corners afterwards
    fn outside(&mut self) {
        let w = self.trees.len();
        let h = self.trees[0].len();
        for y in 0..w {
            for x in 0..h {
                if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
                    self.trees[y][x].visible = true
                }
            }
        }
    }

    /// Find visible trees that are not on the edge
    fn inside(&mut self) {
        let w = self.trees.len();
        let h = self.trees[0].len();

        // only loop over indices that are NOT on the outside
        for y in 1..w - 1 {
            for x in 1..h - 1 {
                let height = self.trees[y][x].height;

                let a = (0..y)
                    .filter(|i| self.trees[i.to_owned()][x].height >= height)
                    .count()
                    == 0;

                let b = ((y + 1)..h)
                    .filter(|i| self.trees[i.to_owned()][x].height >= height)
                    .count()
                    == 0;

                let c = (0..x)
                    .filter(|i| self.trees[y][i.to_owned()].height >= height)
                    .count()
                    == 0;

                let d = ((x + 1)..w)
                    .filter(|i| self.trees[y][i.to_owned()].height >= height)
                    .count()
                    == 0;

                if a || b || c || d {
                    self.trees[y][x].visible = true
                }
            }
        }
    }

    /// ugly function that sort of works
    fn calc_score(&self, x: usize, y: usize) -> i64 {
        let w = self.trees.len();
        let h = self.trees[0].len();
        let height = self.trees[y][x].height;
        let mut sum = 0;

        for i in (0..y).rev() {
            sum += 1;
            if self.trees[i][x].height >= height {
                break;
            }
        }
        let a = sum;
        sum = 0;

        for i in (y + 1)..h {
            sum += 1;
            if self.trees[i][x].height >= height {
                break;
            }
        }
        let b = sum;
        sum = 0;

        for i in (0..x).rev() {
            sum += 1;
            if self.trees[y][i].height >= height {
                break;
            }
        }
        let c = sum;
        sum = 0;

        for i in (x + 1)..w {
            sum += 1;
            if self.trees[y][i].height >= height {
                break;
            }
        }
        let d = sum;

        a * b * c * d
    }

    /// calc all scores and calculate the max
    fn max_score(&self) -> i64 {
        let w = self.trees.len();
        let h = self.trees[0].len();
        let mut score = 0;

        for y in 0..w {
            for x in 0..h {
                let tmp = self.calc_score(x, y);
                if tmp > score {
                    score = tmp
                }
            }
        }

        score
    }

    /// Count all trees that have visible = true
    fn count_visible(&self) -> i64 {
        self.trees.iter().flatten().filter(|t| t.visible).count() as i64
    }

    /// print all trees after parsing (sanity check)
    fn show(&self) {
        let mut ss = String::new();
        for y in 0..self.trees.len() {
            let mut row = String::new();
            for x in 0..self.trees[0].len() {
                row += &self.trees[y][x].height.to_string();
            }
            row += "\n";
            ss += &row;
        }
        println!("{}", ss);
    }

    /// print visible trees as '1' (sanity check)
    fn show_visible(&self) {
        let mut ss = String::new();
        for y in 0..self.trees.len() {
            let mut row = String::new();
            for x in 0..self.trees[0].len() {
                let new = if self.trees[y][x].visible { "1" } else { "0" };
                row += new;
            }
            row += "\n";
            ss += &row;
        }
        println!("{}", ss);
    }
}

impl FromStr for Forrest {
    type Err = ParseIntError;

    /// Input -> Forrest
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<Tree>> = s
            .lines()
            .map(|x| {
                x.chars()
                    .map(|c| Tree::new(c.to_digit(10).unwrap() as usize))
                    .collect::<Vec<Tree>>()
            })
            .collect();

        Ok(Forrest { trees })
    }
}

impl AocDay for Day {
    fn num(&self) -> usize {
        8
    }

    fn part_1(&self, s: String) -> i64 {
        let mut forrest = Forrest::from_str(&s).unwrap();
        forrest.outside();
        forrest.inside();
        forrest.show();
        forrest.show_visible();
        forrest.count_visible()
    }

    fn part_2(&self, s: String) -> i64 {
        let forrest = Forrest::from_str(&s).unwrap();
        forrest.show();
        forrest.max_score()
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"30373
25512
65332
33549
35390"#
            .to_string();
        let expected: i64 = 21;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"30373
25512
65332
33549
35390"#
            .to_string();
        let expected: i64 = 8;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
