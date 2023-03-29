use std::collections::HashMap;
use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

type Name = String;
type FilePath = String;
type Point = i32;

#[derive(Debug)]
struct Files {
    files: HashMap<FilePath, Name>,
    path: PathBuf,
}

impl Files {
    fn build(path: &Path) -> io::Result<Self> {
        let mut files = Files {
            files: HashMap::new(),
            path: path.to_path_buf(),
        };

        files.insert_files(path)?;

        Ok(files)
    }

    fn insert_files(&mut self, path: &Path) -> io::Result<()> {
        let directories = fs::read_dir(path)?;
        for dir in directories {
            let dir = dir?;
            let path = dir.path();

            if path.is_dir() {
                self.insert_files(&path)?;
            } else {
                self.insert(dir);
            }
        }

        Ok(())
    }

    fn insert(&mut self, dir_entry: DirEntry) {
        self.files.insert(
            dir_entry.path().to_string_lossy().to_string(),
            dir_entry.file_name().to_string_lossy().to_string(),
        );
    }
}

#[derive(Debug)]
struct Find {
    point: HashMap<FilePath, Point>,
}

impl Find {
    fn search(input: &Files, pattern: &String) -> Self {
        let mut find = Find {
            point: HashMap::new(),
        };

        for (path, name) in &input.files {
            let point = Find::find_pattern(
                &name.chars().collect::<Vec<_>>(),
                &pattern.chars().collect::<Vec<_>>(),
            );

            find.point.insert(path.to_string(), point);
        }

        return find;
    }
    fn find_pattern(name: &[char], pattern: &[char]) -> i32 {
        let mut name_chars = name;
        let mut pattern_chars = pattern;
        let mut point = 0;
        let mut prev_char: char = '|';

        loop {
            if name_chars.is_empty() || pattern_chars.is_empty() {
                break;
            }
            if name_chars[0].to_ascii_lowercase() == pattern_chars[0].to_ascii_lowercase() {
                point += 0;
                if prev_char == pattern_chars[0] {
                    point += 5;
                }
                if name_chars[0].is_uppercase() {
                    point += 10;
                }

                if prev_char == '_' || prev_char.is_whitespace() {
                    point += 10;
                }
                pattern_chars = &pattern_chars[1..];
            } else {
                name_chars = &name_chars[1..];
                point -= 1;
            }

            prev_char = *name_chars.get(0).unwrap_or(&'|');
        }

        return point;
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: Not enough arguments")
    }

    let path = Path::new(args.get(1).unwrap());
    let query = args.get(2).unwrap();
    let files =
        Files::build(path).unwrap_or_else(|error| panic!("Cannot Read Directory : {error}"));

    let mut rank: Vec<_> = Find::search(&files, query).point.into_iter().collect();
    rank.sort_by(|a, b| b.1.cmp(&a.1));
    let top: Vec<_> = rank.into_iter().take(10).collect();
    for i in top {
        println!("{:?}", i);
    }
}
