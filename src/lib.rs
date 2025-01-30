use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let action = config.action.clone();
    let filepath = config.filepath.clone();
    let flag = config.flag.clone();

    if action.contains("-delfile") {
        delete_file(filepath);
    } else if action.contains("-deldir") {
        delete_directory(filepath, flag);
    } else if action.contains("-makedir") {
        create_directory(filepath);
    } else if action.contains("-makefile") {
        create_file(filepath);
    }

    Ok(())
}


fn delete_file(filepath:String) {
    let delete = fs::remove_file(filepath);
}

fn delete_directory(filepath:String, flag: String) {
    if flag.contains("all"){
        let delete = fs::remove_dir_all(filepath);
    } else {
        let delete = fs::remove_dir(filepath);
    }
}

fn create_file(filepath:String) {
    let create = fs::create_dir_all(filepath);
}

fn create_directory(filepath:String) {
    let create = fs::create_dir(filepath);
}



pub struct Config {
    action: String,
    filepath: String,
    flag: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("Not Enough arguments");
        }

        let action = args[1].clone();
        let filepath = args[2].clone();
        let flag = args[3].clone();

        Ok(Config { action, filepath, flag })
    }
}
