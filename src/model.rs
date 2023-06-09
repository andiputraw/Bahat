use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};
use super::utils;

type FilePath = String;
type FileName = String;
type Point = i32;

#[derive(Debug, Clone)]
pub struct Files {
    pub files: HashMap<FilePath, FileName>,
    pub path: PathBuf,
}

impl Files {
    pub fn build(path: &Path) -> io::Result<Self> {
        let mut files = Files {
            files: HashMap::new(),
            path: path.to_path_buf(),
        };

        files.insert_files(path)?;

        Ok(files)
    }

    pub fn new() -> Self {
        Files {
            files: HashMap::new(),
            path: PathBuf::new(),
        }
    }

    fn insert_files(&mut self, path: &Path) -> io::Result<()> {
        let config = utils::read_config_file().unwrap();

        let excluded = config.query.exclude.unwrap_or(vec![]);

        let directories = fs::read_dir(path)?;
        for dir in directories {
            let dir = dir?;
            let path = dir.path();

            if excluded.contains(&dir.file_name().to_string_lossy().to_string()){                

            }            
            else if path.is_dir() {
                self.insert_files(&path)?;
            } else {
                self.insert(dir);
            }
        }

        Ok(())
    }

    pub fn search(&self, query: String) -> QueryResult {
        let mut query_result = QueryResult::new();

        if query.contains("/") {
            for (path, _) in &self.files {
                let exploded_path: Vec<&str> = path.split("/").collect();
                let len = exploded_path.len();

                let folder = exploded_path[len - 2];
                let file = exploded_path[len - 1];
                let point = Files::find_folder_pattern(folder, file, &query);

                query_result.result.insert(path.to_string(), point);
            }
        } else {
            for (path, name) in &self.files {
                let point = Files::find_pattern(
                    &name.chars().collect::<Vec<_>>(),
                    &query.chars().collect::<Vec<_>>(),
                );

                query_result.result.insert(path.to_string(), point);
            }
        }

        return query_result;
    }

    fn find_pattern(name: &[char], pattern: &[char]) -> i32 {
        let mut name_chars = name;
        let mut pattern_chars = pattern;
        let mut point = 0;
        let mut previously_matched = false;
        let mut prev_char: char = '|';
        let mut position = 0;

        loop {
            if name_chars.is_empty() || pattern_chars.is_empty() {
                break;
            }
            if name_chars[0].to_ascii_lowercase() == pattern_chars[0].to_ascii_lowercase() {
                point += 0;
                if previously_matched {
                    point += 5;
                }
                if name_chars[0].is_uppercase() {
                    point += 10;
                }

                if prev_char == '_' || prev_char == '-' || prev_char.is_whitespace() {
                    point += 10;
                }

                prev_char = *name_chars.get(0).unwrap_or(&'|');
                pattern_chars = &pattern_chars[1..];
                position += 1;
                previously_matched = true;
            } else {
                if position <= 3 {
                    point -= 3;
                }
                prev_char = *name_chars.get(0).unwrap_or(&'|');
                point -= 1;
                position += 1;
                previously_matched = false;
            }
            name_chars = &name_chars[1..];
        }

        return point;
    }

    fn find_folder_pattern(folder: &str, file: &str, pattern: &str) -> i32 {
        let exploded_pattern: Vec<&str> = pattern.split("/").collect();

        let len = exploded_pattern.len();
        let folder_query = exploded_pattern[len - 2];
        let file_query = exploded_pattern[len - 1];

        let folder_point = Files::find_pattern(
            &folder.chars().collect::<Vec<_>>(),
            &folder_query.chars().collect::<Vec<_>>(),
        );
        let file_point = Files::find_pattern(
            &file.chars().collect::<Vec<_>>(),
            &file_query.chars().collect::<Vec<_>>(),
        );

        folder_point * 2 + file_point
    }

    fn insert(&mut self, dir_entry: DirEntry) {
        self.files.insert(
            dir_entry.path().to_string_lossy().to_string(),
            dir_entry.file_name().to_string_lossy().to_string(),
        );
    }
}

pub struct QueryResult {
    pub result: HashMap<FilePath, Point>,
}

impl QueryResult {
    pub fn new() -> Self {
        QueryResult {
            result: HashMap::new(),
        }
    }
    pub fn rank(self, quantity: usize) -> Vec<(String, i32)> {
        let mut ranking: Vec<(String, i32)> = self.result.into_iter().collect();
        ranking.sort_by(|pre, curr| curr.1.cmp(&pre.1));

        let top: Vec<(String, i32)> = ranking.into_iter().take(quantity).collect();

        return top;
    }
}
