use super::model::Files;
use std::env::Args;
use std::path::Path;

//TODO : Use other means to handle error

pub fn search_files(mut args: Args) -> Result<(), ()> {
    let dir_path = args.next().expect("Please provide directory path");

    let pattern = args.next().expect("Query not provided");

    let files = match Files::build(Path::new(&dir_path)) {
        Ok(files) => files,
        Err(err) => panic!("Error : {err}"),
    };

    let find_result: Vec<_> = files.search(pattern).rank(10);

    for top in find_result {
        println!("{:#?}", top);
    }

    Ok(())
}
