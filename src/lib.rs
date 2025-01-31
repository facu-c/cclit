use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let action = config.action.clone();
    let filepath = config.filepath.clone();
    let flag = config.flag.clone();

    match action.as_str() {
        "-rmfile" => delete_file(&filepath),
        "-rmdir" => delete_directory(&filepath, flag),
        "-mkdir" => create_directory(&filepath),
        "-mkfile" => create_file(&filepath),
        _ => {
            panic!("Error! No argument recognized");
        }
    };

    Ok(())
}

fn delete_file(filepath: &String) {
    fs::remove_file(filepath);
}

fn delete_directory(filepath: &String, flag: String) {
    if flag.contains("all"){
        fs::remove_dir_all(filepath);
    } else {
        fs::remove_dir(filepath);
    }

    match flag.as_str() {
        "all" => fs::remove_dir_all(filepath),
        _ => { 
            fs::remove_dir(filepath)
        }
    };
}

fn create_file(filepath: &String) {
    fs::File::create_new(filepath);
}

fn create_directory(filepath: &String) {
    fs::create_dir(filepath);
}

pub struct Config {
    action: String,
    filepath: String,
    flag: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2{
            return Err("Not Enough arguments");
        }

        let action = args[1].clone();
        let filepath = args[2].clone();
        let flag = args[3].clone();

        Ok(Config { action, filepath, flag })
    }
}
