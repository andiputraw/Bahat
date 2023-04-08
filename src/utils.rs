use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::BufReader;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct QueryConfig {
    exclude: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize)]
pub struct Shell {
    open_in_file: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    query: QueryConfig,
    shell: Shell,
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
