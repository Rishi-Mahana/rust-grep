use minigrep::{search, search_insensitive};
use std::env;
use std::fs;
use std::io;
use std::process;
use std::error::Error;
fn main (){
    let args: Vec<String>= env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}", config);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
#[derive(Debug)]
struct Config{
    query:String,
    filepath:String,
    insensitive:bool,
}
impl Config{
    fn build(args:Vec<String>)->Result<Config, &'static str>{
        if args.len()<3{
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        let insensitive=env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, filepath, insensitive})
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    if config.insensitive==false{
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }
    else{
        for line in search_insensitive(&config.query, &contents){
            println!("{line}");
        }
    }

    Ok(())
}