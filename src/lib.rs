use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let action = config.action.clone();
    let filepath = config.filepath.clone();

    match action.as_str() {
        "-rmfile" => delete_file(&filepath),
        "-rmdir" => delete_directory(&filepath),
        "-mkdir" => create_directory(&filepath),
        "-mkfile" => create_file(&filepath),
        _ => {
            panic!("Error! No argument recognized");
        }
    };

    Ok(())
}

fn delete_file(filepath: &String) {
    fs::remove_file(filepath).expect("Error");
}

fn delete_directory(filepath: &String) {
    fs::remove_dir_all(filepath).expect("error");
}

fn create_file(filepath: &String) {
    fs::File::create_new(filepath).expect("Error");
}

fn create_directory(filepath: &String) {
    fs::create_dir(filepath).expect("Error");
}

pub struct Config {
    action: String,
    filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2{
            return Err("Not Enough arguments");
        }

        let action = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { action, filepath })
    }
}
