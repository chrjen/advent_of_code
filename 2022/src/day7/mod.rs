use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 7: No Space Left On Device",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

const TOTAL_FILESYSTEM_SIZE: usize = 70_000_000;
const TOTAL_UPDATE_SIZE: usize = 30_000_000;

/// Node represent a single file or directory in the file system.
#[derive(Debug)]
struct Node {
    name: String,
    is_dir: bool,
    size: usize,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_dir(name: String) -> Self {
        Node {
            name,
            is_dir: true,
            size: 0,
            parent: Weak::new(),
            children: Vec::new(),
        }
    }

    fn new_file(name: String, size: usize) -> Self {
        Node {
            name,
            is_dir: false,
            size,
            parent: Weak::new(),
            children: Vec::new(),
        }
    }
}

impl Node {
    /// Adds a new child Node to this Node. Also updates its own as well as parent sizes to reflect
    /// the new total size which should be current size + size of new child node.
    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        let size_increase = child.borrow().size;

        fn update_parent(parent: Option<Rc<RefCell<Node>>>, size_increase: usize) {
            if let Some(parent) = parent {
                parent.borrow_mut().size += size_increase;
                update_parent(parent.borrow().parent.upgrade(), size_increase);
            }
        }

        self.size += size_increase;
        update_parent(self.parent.upgrade(), size_increase);

        self.children.push(child)
    }

    /// Returns the child Node with the given name, or `None` if there are none with that name.
    fn lookup_child(&self, name: &str) -> Option<&Rc<RefCell<Node>>> {
        self.children.iter().find(|node| {
            let node = node.borrow();
            node.is_dir && node.name == name
        })
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = &input[input.find('$').unwrap() + 1..];

    let root = Rc::new(RefCell::new(Node::new_dir(String::from("/"))));
    let mut current = root.clone();

    // Loop going through all the commands in the input. `cmd` is an iterator for all words in
    // a given command with any whitespaces trimmed. First value should be the name of the program,
    // followed by zero or a single argument and lastly output from the command.
    for mut cmd in input.split_terminator('$').map(|v| v.split_whitespace()) {
        let Some(program) = cmd.next() else {
            panic!("no program name in command");
        };

        match program {
            "cd" => match cmd.next() {
                Some(dir) => match dir {
                    "/" => current = root.clone(),
                    ".." => {
                        if let Some(parent) = current.clone().borrow().parent.upgrade() {
                            current = parent
                        }
                    }
                    name => {
                        let node = current.borrow().lookup_child(name).cloned();

                        if let Some(node) = node {
                            current = node;
                        } else {
                            panic!("tried to cd into a non-existent folder '{}'", name)
                        }
                    }
                },
                None => panic!("program 'cd' missing directory argument"),
            },

            "ls" => {
                while let (Some(arg), Some(name)) = (cmd.next(), cmd.next()) {
                    match arg {
                        "dir" => {
                            let mut new_dir = Node::new_dir(name.to_owned());
                            new_dir.parent = Rc::downgrade(&current);
                            current
                                .borrow_mut()
                                .add_child(Rc::new(RefCell::new(new_dir)));
                        }
                        arg => {
                            let mut new_file =
                                Node::new_file(name.to_owned(), arg.parse().unwrap());
                            new_file.parent = Rc::downgrade(&current);
                            current
                                .borrow_mut()
                                .add_child(Rc::new(RefCell::new(new_file)));
                        }
                    }
                }
            }
            _ => panic!("unknown program"),
        }
    }

    // Part 1.
    fn part1(node: &Rc<RefCell<Node>>) -> usize {
        let node = node.borrow();

        let mut size = node.size;
        if !node.is_dir || size > 100_000 {
            size = 0;
        }

        size + node.children.iter().map(part1).sum::<usize>()
    }

    // Part 2.
    let space_to_delete = TOTAL_UPDATE_SIZE - (TOTAL_FILESYSTEM_SIZE - root.borrow().size);

    fn part2(node: &Rc<RefCell<Node>>, target: usize) -> usize {
        let node = node.borrow();

        let mut size = node.size;
        if !node.is_dir || size < target {
            size = usize::MAX;
        }

        let max_children = node.children.iter().map(|node| part2(node, target)).min();

        size.min(max_children.unwrap_or(usize::MAX))
    }

    (
        part1(&root).to_string(),
        part2(&root, space_to_delete).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        r"$ cd /
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
7214296 k
",
        "95437"
    );
    solution!(p1, p1_solution, "1582412");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "$ cd /
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
7214296 k",
        "24933642"
    );
    solution!(p2, p2_solution, "3696336");
}
