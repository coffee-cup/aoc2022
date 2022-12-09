use std::{collections::HashMap, fs, io::BufRead, path::PathBuf};

#[derive(Clone, Debug)]
enum Item {
    Cd(String),
    Ls,
    Dir(String),
    File(u32, String),
}

#[derive(Clone, Debug)]
pub enum Entry {
    File { name: String, size: u32 },

    // Just store the children as lookup paths because mutable trees are a pain in Rust
    Dir { name: String, children: Vec<String> },
}

struct FileSystem {
    pub cwd: PathBuf,
    pub entries: HashMap<String, Entry>,
}

impl FileSystem {
    pub fn new() -> Self {
        let cwd = PathBuf::from("/");
        let root = Entry::Dir {
            name: "/".to_string(),
            children: vec![],
        };

        let mut entries = HashMap::new();
        entries.insert(cwd.display().to_string(), root);

        Self { cwd, entries }
    }

    pub fn load_cmds(&mut self, cmds: &Vec<Item>) {
        for cmd in cmds {
            match cmd {
                Item::Cd(path) => self.cd(path),
                Item::Ls => {}
                Item::Dir(path) => {
                    self.entries.insert(
                        self.cwd.join(path).display().to_string(),
                        Entry::Dir {
                            name: path.clone(),
                            children: Vec::new(),
                        },
                    );

                    let entry = self
                        .entries
                        .get_mut(&self.cwd.display().to_string())
                        .unwrap();

                    match entry {
                        Entry::Dir { children, .. } => {
                            children.push(path.clone());
                        }
                        Entry::File { .. } => unreachable!(),
                    }
                }
                Item::File(size, name) => {
                    self.entries.insert(
                        self.cwd.join(name).display().to_string(),
                        Entry::File {
                            name: name.clone(),
                            size: *size,
                        },
                    );

                    let entry = self
                        .entries
                        .get_mut(&self.cwd.display().to_string())
                        .unwrap();

                    match entry {
                        Entry::Dir { children, .. } => {
                            children.push(name.clone());
                        }
                        Entry::File { .. } => unreachable!(),
                    }
                }
            }
        }
    }

    fn cd(&mut self, path: &String) {
        if path == ".." {
            let parent = self.cwd.parent().unwrap();
            self.cwd = parent.to_path_buf();
        } else if path == "/" {
            self.cwd = PathBuf::from("/");
        } else {
            self.cwd = self.cwd.join(path);
        }
    }

    pub fn print_system(&self) {
        self.print_entry(PathBuf::from("/"), 0);
    }

    fn print_entry(&self, path: PathBuf, indent: usize) {
        let entry = self.entries.get(&path.display().to_string()).unwrap();
        match entry {
            Entry::File { name, size } => {
                println!(
                    "{:indent$}- {name} (file, size={size})",
                    "",
                    indent = indent * 2
                );
            }
            Entry::Dir { name, children } => {
                println!("{:indent$}- {name} (dir)", "", indent = indent * 2);

                for child in children {
                    self.print_entry(path.join(child), indent + 1);
                }
            }
        }
    }

    pub fn size(&self, path: PathBuf) -> u32 {
        let entry = self.entries.get(&path.display().to_string()).unwrap();
        match entry {
            Entry::File { size, .. } => *size,
            Entry::Dir { children, .. } => {
                let mut size = 0;
                for child in children {
                    size += self.size(path.join(child));
                }
                size
            }
        }
    }
}

fn parse_commands(lines: &Vec<String>) -> Vec<Item> {
    let items = lines
        .iter()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts[0] == "$" {
                if parts[1] == "cd" {
                    Item::Cd(parts[2].to_string())
                } else {
                    Item::Ls
                }
            } else if parts[0] == "dir" {
                Item::Dir(parts[1].to_string())
            } else {
                Item::File(parts[0].parse::<u32>().unwrap(), parts[1].to_string())
            }
        })
        .collect::<Vec<_>>();

    items
}

fn part1(filesystem: &FileSystem) -> u32 {
    let mut sum = 0;
    for (path, entry) in filesystem.entries.iter() {
        match entry {
            Entry::File { .. } => {}
            Entry::Dir { .. } => {
                let size = filesystem.size(PathBuf::from(path));
                if size < 100000 {
                    sum += size;
                }
            }
        }
    }

    sum
}

fn part2(filesystem: &FileSystem) -> u32 {
    let total = 70000000;
    let needed = 30000000;
    let used = filesystem.size(PathBuf::from("/"));
    let unused = total - used;

    let to_free = needed - unused;

    let mut smallest = total;

    for (path, entry) in filesystem.entries.iter() {
        match entry {
            Entry::File { .. } => {}
            Entry::Dir { .. } => {
                let size = filesystem.size(PathBuf::from(path));
                if size >= to_free && size < smallest {
                    smallest = size;
                }
            }
        }
    }

    smallest
}

fn main() {
    let lines = fs::read("input.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect::<Vec<_>>();

    let items = parse_commands(&lines);

    let mut filesystem = FileSystem::new();
    filesystem.load_cmds(&items);

    // filesystem.print_system();

    let part1 = part1(&filesystem);
    println!("Part 1: {}", part1);

    let part2 = part2(&filesystem);
    println!("Part 2: {}", part2);
}
