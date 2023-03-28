use std::collections::HashMap;
use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

type Name = String;

#[derive(Debug)]
struct Files {
    files: HashMap<String, Name>,
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
                self.insert_files(&path);
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

fn main() {

    let args : Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: Not enough arguments")
    }

    let path = Path::new(args.get(1).unwrap());
    let files =
        Files::build(path).unwrap_or_else(|error| panic!("Cannot Read Directory : {error}"));

    println!("{:?}",files);
}
