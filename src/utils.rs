use super::model::Files;
use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct QueryConfig {
    pub exclude: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize)]
pub struct Shell {
    pub open_in_file: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub query: QueryConfig,
    pub shell: Shell,
}

pub fn read_config_file() -> Result<AppConfig, String> {
    let config_file = fs::File::open("./config.json").map_err(|e| e.to_string())?;
    let reader = BufReader::new(config_file);
    let value: AppConfig = serde_json::from_reader(reader).map_err(|e| e.to_string())?;

    Ok(value)
}

pub fn open_file(path: String) -> Result<(), ()> {
    let config = read_config_file().unwrap();
    let open_command = config.shell.open_in_file;

    let mut buff = path.split("/").collect::<Vec<_>>();
    buff.pop().unwrap();

    Command::new(open_command)
        .arg(buff.join("/"))
        .output()
        .map_err(|_| ())?;
    Ok(())
}

pub fn open_file_dialog() -> Result<PathBuf, ()> {
    let dir = FileDialog::new()
        .show_open_single_dir()
        .map_err(|_| ())?
        .expect("Please select a directory");

    Ok(dir)
}

pub fn get_directory(path: String) -> Result<Files, ()> {
    let files = Files::build(Path::new(&path)).map_err(|_| ())?;

    Ok(files)
}

pub fn read_file(path: String) -> Result<String, String> {
    let path = Path::new(&path);
    let mut content = String::new();
    let file = fs::File::open(path).map_err(|_| "Path doesn't exists")?;
    let mut reader = BufReader::new(file);

    reader
        .read_to_string(&mut content)
        .map_err(|_| "Error reading file")?;

    Ok(content)
}
