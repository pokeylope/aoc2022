use std::collections::BTreeMap;

const FS_SIZE: u32 = 70_000_000;
const NEEDED_SIZE: u32 = 30_000_000;

#[derive(Debug, Default)]
struct Directory {
    entries: BTreeMap<String, usize>,
}

impl Directory {
    fn children<'a>(&'a self, tree: &'a DirectoryTree) -> impl Iterator<Item = (&String, &FsNode)> {
        self.entries.iter().map(|(name, idx)| (name, &tree.get(*idx).contents))
    }

    fn get_size(&self, tree: &DirectoryTree) -> u32 {
        self.children(tree).map(|(_, node)| node.size(tree)).sum()
    }
}

#[derive(Debug, Default)]
struct File {
    size: u32,
}

#[derive(Debug)]
enum FsNode {
    Directory(Directory),
    File(File),
}

impl FsNode {
    fn size(&self, tree: &DirectoryTree) -> u32 {
        match self {
            FsNode::Directory(dir) => dir.get_size(tree),
            FsNode::File(file) => file.size,
        }
    }
}

#[derive(Debug)]
struct DirEntry {
    idx: usize,
    parent_idx: Option<usize>,
    contents: FsNode,
}

#[derive(Debug)]
struct DirectoryTree {
    nodes: Vec<DirEntry>,
    root: usize,
}

impl DirectoryTree {
    fn new() -> Self {
        let mut nodes = vec![];
        let root_dir = FsNode::Directory(Default::default());
        let idx = nodes.len();
        let root_entry = DirEntry { idx, parent_idx: None, contents: root_dir };
        nodes.push(root_entry);
        Self { nodes, root: idx }
    }

    fn get(&self, idx: usize) -> &DirEntry {
        &self.nodes[idx]
    }

    fn get_mut(&mut self, idx: usize) -> &mut DirEntry {
        &mut self.nodes[idx]
    }

    fn get_root(&self) -> &Directory {
        let FsNode::Directory(parent) = &self.nodes[self.root].contents else { panic!() };
        parent
    }

    fn add_entry(&mut self, parent_idx: usize, name: String, entry: FsNode) -> usize {
        let idx = self.nodes.len();
        let child = DirEntry { idx, parent_idx: Some(parent_idx), contents: entry };
        self.nodes.push(child);
        let FsNode::Directory(parent) = &mut self.get_mut(parent_idx).contents else { panic!() };
        parent.entries.insert(name, idx);
        idx
    }

    fn mkdir(&mut self, parent_idx: usize, name: String) -> usize {
        let dir = FsNode::Directory(Default::default());
        self.add_entry(parent_idx, name, dir)
    }

    fn add_file(&mut self, parent_idx: usize, name: String, size: u32) {
        let file = FsNode::File(File { size });
        self.add_entry(parent_idx, name, file);
    }

    fn dirs(&self) -> impl Iterator<Item = &Directory> {
        self.nodes.iter().filter_map(|e| match &e.contents {
            FsNode::Directory(dir) => Some(dir),
            FsNode::File(_) => None,
        })
    }
}

fn parse(input: &str) -> DirectoryTree {
    let mut lines = input.lines();
    let mut tree = DirectoryTree::new();
    let mut curdir = tree.root;
    assert_eq!(lines.next(), Some("$ cd /"));
    for line in lines {
        let (a, b) = line.split_once(' ').unwrap();
        match a {
            "$" => {
                let mut cmdline = b.split_whitespace();
                let cmd = cmdline.next().unwrap();
                match cmd {
                    "cd" => {
                        let arg = cmdline.next().unwrap();
                        if arg == ".." {
                            curdir = tree.get(curdir).parent_idx.unwrap_or(curdir);
                        } else {
                            curdir = tree.mkdir(curdir, arg.to_string());
                        }
                    },
                    "ls" => (),
                    _ => panic!("Unexpected command: {cmd}"),
                }
            },
            "dir" => (),
            _ => {
                let size = a.parse().unwrap();
                let filename = b.to_string();
                tree.add_file(curdir, filename, size);
            },
        }
    }
    tree
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = parse(input);
    Some(tree.dirs().filter_map(|d| {
        let size = d.get_size(&tree);
        if size <= 100_000 {
            Some(size)
        } else {
            None
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = parse(input);
    let free = FS_SIZE - tree.get_root().get_size(&tree);
    let needed = NEEDED_SIZE - free;
    Some(tree.dirs().filter_map(|d| {
        let size = d.get_size(&tree);
        if size >= needed {
            Some(size)
        } else {
            None
        }
    }).min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
