use std::{env};



pub mod serve;
pub mod search;
pub mod model;



fn show_command(){
    println!("Usage : [program] [command] [sub-command]");
    println!("                   serve  [Directory]   [Address?]");
    println!("                   search [Directory]   [Query]");
}

fn entry() -> Result<(),()>{
    let mut args = env::args();
    let _ = args.next().expect("Program should be provided");

    let command = args.next().ok_or_else(|| {
        show_command();
        println!("Error : Command not provided");
    }).unwrap();

    match command.as_str() {
        "serve" => serve::serve_server(&mut args),
        "search" => search::search_files(&mut args)?,
        _ => {
            println!("Undefined command {command}");
            show_command();
        }
    }

    Ok(())
}


fn main() {
    //TODO Handle the error properly
    if let Ok(()) = entry() {
        
    }
}
