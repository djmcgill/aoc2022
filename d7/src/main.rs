use scanf::sscanf;
use std::{iter::Peekable, time::Instant};

#[derive(Debug)]
enum FileEntry {
    Directory(DirectoryEntry),
    File(u64), // no need for file names here
}
#[derive(Debug, Default)]
struct DirectoryEntry {
    contents: Vec<FileEntry>,
    size: u64,
}

fn main() {
    let start = Instant::now();
    let mut file_system = DirectoryEntry::default();
    parse_line(&mut REAL.lines().peekable(), &mut file_system);
    file_system.size = sum_contents(&file_system);

    let p1 = sum_under_10k(&file_system);
    let p1_time = Instant::now();
    let needed_space = file_system.size - 40_000_000;
    let p2 = find_smallest_dir_over_n(&file_system, needed_space);
    let p2_time = Instant::now();
    println!("{} {}", p1, p2);
    // p1: 296.8µs
    // p2: 6µs
    println!("p1: {:?}", p1_time - start);
    println!("p2: {:?}", p2_time - p1_time);
}

fn sum_contents(f: &DirectoryEntry) -> u64 {
    f.contents
        .iter()
        .map(|entry| match entry {
            FileEntry::File(size) => *size,
            FileEntry::Directory(dir) => dir.size,
        })
        .sum()
}

fn visit_dirs(current_dir_entry: &DirectoryEntry, f: &mut impl FnMut(&DirectoryEntry)) {
    for entry in &current_dir_entry.contents {
        if let FileEntry::Directory(dir) = entry {
            f(dir);
            visit_dirs(dir, f);
        }
    }
}

fn sum_under_10k(current_dir_entry: &DirectoryEntry) -> u64 {
    let mut size = 0;
    visit_dirs(current_dir_entry, &mut |dir| {
        if dir.size <= 100_000 {
            size += dir.size;
        }
    });
    size
}

fn find_smallest_dir_over_n(current_dir_entry: &DirectoryEntry, n: u64) -> u64 {
    let mut sizes = Vec::new();
    visit_dirs(current_dir_entry, &mut |dir| {
        if dir.size >= n {
            sizes.push(dir.size);
        }
    });
    sizes.into_iter().min().unwrap()
}

fn parse_line<'a>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
    dir_entry: &mut DirectoryEntry,
) {
    if let Some(&line) = lines.peek() {
        if line == "$ ls" || line == "$ cd .." || line.starts_with("dir") {
            // no op
        } else if line.starts_with("$ cd ") {
            // we don't actually care about the name lol
            cd_foo(lines, dir_entry);
        } else {
            // we found a file
            let mut size: u64 = 0;
            let mut _name = "".to_string();
            sscanf!(line, "{} {}", size, _name).unwrap();
            dir_entry.contents.push(FileEntry::File(size));
        };
    }
}

// TODO: calculate the answers in-situ rather than populating DirectoryEntries
fn cd_foo<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>, dir_entry: &mut DirectoryEntry) {
    let _ = lines.next(); // ditch "cd foo"
    dir_entry
        .contents
        .push(FileEntry::Directory(DirectoryEntry::default()));
    // this match is gross but what can you do
    let new_dir = if let FileEntry::Directory(dir) = dir_entry.contents.last_mut().unwrap() {
        dir
    } else {
        unreachable!()
    };

    // I feel like this could probably be a normal take_while, intentionally ditching the "cd .."
    while lines.peek().filter(|&&line| line != "$ cd ..").is_some() {
        parse_line(lines, new_dir);
        let _ = lines.next(); // ditch whatever line this is
    }

    new_dir.size = sum_contents(new_dir);
}

const REAL: &str = include_str!("real.txt");

const TEST: &str = r#"$ cd /
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
7214296 k"#;
