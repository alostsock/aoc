use crate::Solution;

#[derive(Default)]
pub struct Day7 {}

impl Solution for Day7 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let tree = parse_into_tree(include_str!("data/day7"));
        sum_small_dirs(&tree)
    }

    fn part_2(&self) -> Self::Result {
        let tree = parse_into_tree(include_str!("data/day7"));
        find_deletable_dir(&tree)
    }
}

fn sum_small_dirs(tree: &Tree) -> usize {
    tree.find_dirs(0, &mut |size| size <= 100_000).iter().sum()
}

fn find_deletable_dir(tree: &Tree) -> usize {
    let total = 70_000_000;
    let used = tree.node(0).size;

    let target = 30_000_000;
    let currently_free = total - used;
    let min_deletable_size = target - currently_free;

    let mut deletable_sizes = tree.find_dirs(0, &mut |size| size >= min_deletable_size);
    deletable_sizes.sort_unstable();
    *deletable_sizes.first().unwrap()
}

#[derive(Debug)]
struct Node {
    parent: Option<usize>,
    children: Vec<usize>,
    size: usize,
    is_dir: bool,
}

impl Node {
    fn new_dir(parent: Option<usize>) -> Self {
        Node {
            parent,
            children: vec![],
            size: 0,
            is_dir: true,
        }
    }

    fn new_file(parent: Option<usize>, size: usize) -> Self {
        Node {
            parent,
            children: vec![],
            size,
            is_dir: false,
        }
    }
}

#[derive(Default, Debug)]
struct Tree(Vec<Node>);

impl Tree {
    fn new() -> Self {
        Self::default()
    }

    fn node(&self, i: usize) -> &Node {
        self.0.get(i).unwrap()
    }

    fn node_mut(&mut self, i: usize) -> &mut Node {
        self.0.get_mut(i).unwrap()
    }

    /// Add a new directory or file
    fn insert(&mut self, node: Node) -> usize {
        let id = self.0.len();
        if let Some(parent) = node.parent {
            self.node_mut(parent).children.push(id);
        }
        if !node.is_dir {
            self.update_parent_sizes(node.parent, node.size);
        }
        self.0.push(node);
        id
    }

    fn update_parent_sizes(&mut self, first_parent: Option<usize>, size: usize) {
        let mut next_parent = first_parent;
        while let Some(index) = next_parent {
            self.node_mut(index).size += size;
            next_parent = self.node(index).parent;
        }
    }

    /// Recursively finds file sizes that meet a given size condition
    fn find_dirs<F: FnMut(usize) -> bool>(
        &self,
        index: usize,
        size_condition: &mut F,
    ) -> Vec<usize> {
        let mut large_dirs = vec![];

        let node = self.node(index);

        if node.is_dir && size_condition(node.size) {
            large_dirs.push(node.size);
        }

        large_dirs.extend(
            node.children
                .iter()
                .flat_map(|i| self.find_dirs(*i, size_condition)),
        );

        large_dirs
    }
}

fn parse_into_tree(s: &str) -> Tree {
    let mut tree = Tree::new();

    // current working directory
    let mut cwd: Option<usize> = None;

    for line in s.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        match &*parts {
            ["$", "cd", ".."] => {
                cwd = tree.node(cwd.unwrap()).parent;
            }
            ["$", "cd", _name] => {
                let new_dir = tree.insert(Node::new_dir(cwd));
                cwd = Some(new_dir);
            }
            [size, _name] if size.chars().next().unwrap().is_numeric() => {
                tree.insert(Node::new_file(cwd, size.parse().unwrap()));
            }
            _ => continue,
        };
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
$ cd /
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
";

    #[test]
    fn parsing_works() {
        dbg!(parse_into_tree(TEST_INPUT));
    }

    #[test]
    fn find_dirs_small_works() {
        let tree = parse_into_tree(TEST_INPUT);
        assert_eq!(sum_small_dirs(&tree), 95437);
    }

    #[test]
    fn find_dirs_large_works() {
        let tree = parse_into_tree(TEST_INPUT);
        assert_eq!(find_deletable_dir(&tree), 24_933_642);
    }

    #[test]
    fn part_1() {
        assert_eq!(Day7::new().part_1(), 1_783_610);
    }

    #[test]
    fn part_2() {
        assert_eq!(Day7::new().part_2(), 4_370_655);
    }
}
