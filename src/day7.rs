use std::str::Lines;

use crate::aoc_trait::AocDay;

#[derive(Debug)]
struct Node {
    children: Option<Vec<usize>>,
    is_file: Option<bool>,
    parent: Option<usize>,
    size: Option<i64>,
}

#[derive(Debug)]
struct FileSystem {
    cur: usize,
    nodes: Vec<Node>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            cur: 0,
            nodes: vec![],
        }
    }

    fn next_id(&self) -> usize {
        self.nodes.len()
    }

    /// Add child node
    fn add_child(&mut self, size: i64, is_file: bool) {
        let id = self.next_id();
        let new_node = Node {
            parent: Some(self.cur),
            children: Some(vec![]),
            size: Some(size),
            is_file: Some(is_file),
        };

        // Save node into nodes
        self.nodes.push(new_node);
        // Add node to parent
        self.nodes[self.cur].children.as_mut().unwrap().push(id);

        if !is_file {
            self.cur = id;
        }
    }

    /// make cur the parent node
    fn set_cur_to_parent(&mut self) {
        if let Some(p) = self.nodes[self.cur].parent {
            self.cur = p;
        }
    }

    /// Calculate the size of the node under `cur`
    fn calc_size(&mut self) {
        if self.nodes[self.cur].is_file.unwrap() {
            return;
        }
        let mut sum = 0;
        if let Some(children) = &self.nodes[self.cur].children {
            for child_id in children {
                if let Some(size) = self.nodes[child_id.to_owned()].size {
                    if child_id != &self.cur {
                        sum += size;
                    }
                }
            }
        }
        self.nodes[self.cur].size = Some(sum);
    }
}

/// Recursive function calls itself for each line.
fn build_file_system<'a>(fs: &'a mut FileSystem, mut lines: Lines<'a>) -> &'a mut FileSystem {
    // If there is no new line we are done
    let new_line = match lines.next() {
        Some(l) => l,
        None => {
            for i in 0..fs.nodes.len() {
                fs.cur = i;
                fs.calc_size();
            }
            return fs;
        }
    };

    let spl: Vec<&str> = new_line.split(" ").collect();
    let fs = match spl[0] {
        // list dir - do nothing
        "dir" => build_file_system(fs, lines),
        // command: ls or cd
        "$" => match spl[1] {
            // Do nothing with ls.
            "ls" => build_file_system(fs, lines),
            // Move into or up a directory
            "cd" => match spl[2] {
                // Go up a directory and calculate the size
                ".." => {
                    fs.set_cur_to_parent();
                    build_file_system(fs, lines)
                }
                // Move into a directory, create a child and add to parent.
                _ => {
                    fs.add_child(0, false);
                    build_file_system(fs, lines)
                }
            },
            _ => unreachable!("there are only two commands"),
        },
        // This is a file, create a child and add to parent
        _ => {
            let size = spl[0].parse::<i64>().expect("this is a number");
            fs.add_child(size, true);
            build_file_system(fs, lines)
        }
    };

    for i in 0..fs.nodes.len() {
        fs.cur = i;
        fs.calc_size();
    }

    fs
}

pub struct Day {}

impl AocDay for Day {
    fn num(&self) -> usize {
        7
    }

    fn part_1(&self, s: String) -> i64 {
        let mut file_system = FileSystem::new();
        let lines = s.lines();
        let file_system = build_file_system(&mut file_system, lines);

        file_system
            .nodes
            .iter()
            .filter(|n| {
                let is_file = n.is_file.expect("is_file exists");
                let size = n.size.expect("size exists");
                !is_file && size <= 100000
            })
            .map(|x| x.size.expect("size exists"))
            .sum::<i64>()
    }

    fn part_2(&self, s: String) -> i64 {
        let mut file_system = FileSystem::new();
        let lines = s.lines();
        let file_system = build_file_system(&mut file_system, lines);
        let root_size = file_system.nodes[0].size.as_ref().unwrap();

        file_system
            .nodes
            .iter()
            .filter(|n| {
                let is_file = n.is_file.expect("is_file exists");
                let size = n.size.expect("size exists");
                let tmp = 70000000 - (root_size.to_owned() - size);
                !is_file && tmp > 30000000
            })
            .map(|x| x.size.unwrap())
            .min()
            .unwrap()
    }

    fn test_part_1(&self) -> (i64, bool) {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
            .to_string();
        let expected: i64 = 95437;
        let answer = self.part_1(input);
        (answer, expected == answer)
    }

    fn test_part_2(&self) -> (i64, bool) {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
            .to_string();
        let expected: i64 = 24933642;
        let answer = self.part_2(input);
        (answer, expected == answer)
    }
}
