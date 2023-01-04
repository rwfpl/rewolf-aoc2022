use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Entry {
    File {
        size: usize,
    },
    Dir {
        id: usize,
        entries: HashMap<String, usize>,
        parent: usize,
    },
}

impl Entry {
    fn dir_entry(id: usize, parent: usize) -> Entry {
        Entry::Dir {
            id,
            entries: HashMap::new(),
            parent,
        }
    }

    fn file_entry(size: usize) -> Entry {
        Entry::File { size }
    }
}

struct Disk {
    disk: Vec<Entry>,
}

impl Disk {
    fn new() -> Self {
        Disk { disk: Vec::new() }
    }

    fn add_directory_entry(&mut self, current: usize, dir_name: String) -> usize {
        let next_id = self.disk.len();
        if let Entry::Dir {
            id,
            entries,
            parent: _,
        } = &mut self.disk[current]
        {
            match entries.get(&dir_name) {
                Some(c) => *c,
                _ => {
                    let d = Entry::dir_entry(next_id, *id);
                    entries.insert(dir_name, next_id);
                    self.disk.push(d);
                    next_id
                }
            }
        } else {
            panic!("Can't add entry to the file.");
        }
    }

    fn add_file_entry(&mut self, current: usize, file_name: String, file_size: usize) {
        let next_id = self.disk.len();
        if let Entry::Dir {
            id: _,
            entries,
            parent: _,
        } = &mut self.disk[current]
        {
            let d = Entry::file_entry(file_size);
            entries.insert(file_name, next_id);
            self.disk.push(d);
        } else {
            panic!("Can't add entry to the file.");
        }
    }

    fn get_dir_size<F>(&self, current: usize, dir_size_callback: &mut F) -> usize
    where
        F: FnMut(usize),
    {
        match &self.disk[current] {
            Entry::Dir {
                id: _,
                entries,
                parent: _,
            } => {
                let dir_size = entries
                    .values()
                    .map(|e_id| self.get_dir_size(*e_id, dir_size_callback))
                    .sum::<usize>();
                dir_size_callback(dir_size);
                dir_size
            }
            Entry::File { size } => *size,
        }
    }
}

fn solution() -> (usize, usize) {
    let file_re = Regex::new(r"^(?P<file_size>\d*)\s(?P<file_name>.*)$").unwrap();
    let mut disk = Disk::new();
    disk.disk.push(Entry::dir_entry(0, 0));
    let mut current = 0;

    fs::read_to_string("src/inputs/aoc_7.input")
        .unwrap()
        .lines()
        .for_each(|line| match line {
            "$ cd /" => {
                current = 0;
            }
            "$ cd .." => {
                current = match disk.disk[current] {
                    Entry::Dir {
                        id: _,
                        entries: _,
                        parent,
                    } => parent,
                    _ => 0,
                }
            }
            "$ ls" => {}
            x if x.starts_with("dir ") => {
                disk.add_directory_entry(current, x.get(4..).unwrap().to_string());
            }
            x if x.starts_with("$ cd ") => {
                current = disk.add_directory_entry(current, x.get(5..).unwrap().to_string())
            }
            _ => {
                let fd = file_re.captures(line).unwrap();
                let file_size = fd
                    .name("file_size")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let file_name = fd.name("file_name").unwrap().as_str();
                disk.add_file_entry(current, file_name.to_string(), file_size);
            }
        });

    let mut p1 = 0;
    let total_size = disk.get_dir_size(0, &mut |dir_size| {
        if dir_size <= 100000 {
            p1 += dir_size;
        }
    });

    let unused_space = 70000000 - total_size;
    let must_free = 30000000 - unused_space;
    let mut p2 = total_size;
    disk.get_dir_size(0, &mut |dir_size| {
        if dir_size >= must_free && dir_size < p2 {
            p2 = dir_size
        }
    });

    (p1, p2)
}

pub fn run() {
    let (p1, p2) = solution();
    println!("day7 p1: {p1}");
    println!("day7 p2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution(), (1432936, 272298));
    }
}
