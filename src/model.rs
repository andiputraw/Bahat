
use std::collections::HashMap;
use std::{fs, io};
use std::path::{PathBuf,Path};
use std::fs::DirEntry;

type FilePath = String;
type FileName = String;
type Point = i32;


#[derive(Debug)]
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

    pub fn search(&self, query : String) -> QueryResult {
        let mut find = QueryResult::new();

        for (path, name) in &self.files {
            let point = Files::find_pattern(
                &name.chars().collect::<Vec<_>>(),
                &query.chars().collect::<Vec<_>>(),
            );

            find.result.insert(path.to_string(), point);
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

                if prev_char == '_' || prev_char == '-' || prev_char.is_whitespace() {
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

    fn insert(&mut self, dir_entry: DirEntry) {
        self.files.insert(
            dir_entry.path().to_string_lossy().to_string(),
            dir_entry.file_name().to_string_lossy().to_string(),
        );
    }

    


}

pub struct QueryResult {
    pub result : HashMap<FilePath,Point>
}

impl QueryResult {
    pub fn new() -> Self{
        QueryResult {result: HashMap::new()}
    }
    pub fn rank(self, quantity : usize  ) -> Vec<(String,i32)>{
        let mut ranking : Vec<(String,i32)> = self.result.into_iter().collect(); 
        ranking.sort_by(|pre,curr| curr.1.cmp(&pre.1));

        let top : Vec<(String,i32)> = ranking.into_iter().take(quantity).collect();
        return top ;
    }   
}