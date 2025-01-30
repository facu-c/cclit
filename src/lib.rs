use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let action = config.action.clone();
    let filepath = config.filepath.clone();
    let flag = config.flag.clone();

    if action == "-df".to_string() {
        delete_file(filepath);
    } else if action == "-dd" {
        delete_directory(filepath);
    }

    Ok(())
}


fn delete_file(filepath:String) {
    let delete = fs::remove_file(filepath);
}

fn delete_directory(filepath:String) {
    let delete = fs::remove_dir(filepath);
}

pub struct Config {
    action: String,
    filepath: String,
    flag: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 4{
            return Err("Not Enough arguments");
        }

        let action = args[1].clone();
        let filepath = args[2].clone();
        let flag = args[3].clone();

        Ok(Config { action, filepath, flag })
    }
}
