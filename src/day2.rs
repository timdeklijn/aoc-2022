use crate::aoc_trait::AocDay;

/// RockPaperScissors moves
enum RPS {
    Rock,     // 1
    Paper,    // 2
    Scissors, // 3
}

/// Parse elf input
fn elf(s: &str) -> RPS {
    match s {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => unreachable!(),
    }
}

/// Parse player input
fn player(s: &str) -> RPS {
    match s {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => unreachable!(),
    }
}

/// Find player move based on elf move (part 2)
fn player_2(p: &str, elf: &RPS) -> RPS {
    match p {
        "X" => match elf {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        },
        "Y" => match elf {
            RPS::Rock => RPS::Rock,
            RPS::Paper => RPS::Paper,
            RPS::Scissors => RPS::Scissors,
        },
        "Z" => match elf {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
        _ => unreachable!(),
    }
}

/// Determine outcome of a single game
fn game(e: &RPS, p: &RPS) -> i64 {
    match e {
        RPS::Rock => match p {
            RPS::Rock => 4,
            RPS::Paper => 8,
            RPS::Scissors => 3,
        },
        RPS::Paper => match p {
            RPS::Rock => 1,
            RPS::Paper => 5,
            RPS::Scissors => 9,
        },
        RPS::Scissors => match p {
            RPS::Rock => 7,
            RPS::Paper => 2,
            RPS::Scissors => 6,
        },
    }
}

/// Play game according to part 1
fn play_1(l: &str) -> i64 {
    let v: Vec<&str> = l.split(" ").collect();
    game(&elf(v[0]), &player(v[1]))
}

/// Play game according to part 2
fn play_2(l: &str) -> i64 {
    let v: Vec<&str> = l.split(" ").collect();
    let elf = elf(v[0]);
    let player = player_2(v[1], &elf);
    game(&elf, &player)
}

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        2
    }

    fn part_1(&self, s: String) -> i64 {
        s.lines().map(|l| play_1(l)).sum()
    }

    fn part_2(&self, s: String) -> i64 {
        s.lines().map(|l| play_2(l)).sum()
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"A Y
B X
C Z"#
            .to_string();
        let expected: i64 = 15;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"A Y
B X
C Z"#
            .to_string();
        let expected: i64 = 12;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
