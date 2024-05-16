use std::{fmt::Error, path::PathBuf};
use structopt::StructOpt;
use std::fs::read_to_string;
use std::path::Path;
use std::env;
use serde_json::{Value, from_str};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "just a test")]
struct opts {
    alias: String,
    
    #[structopt(short, long, parse(from_os_str))]
    dirpath: Option<PathBuf>,
}


fn readfile(filename: &str) -> Result<Value, Error> {
    let contents = read_to_string(filename).expect("cant read");

    let json: Value = from_str(contents.as_str()).expect("JSON file incorrectly formatted");

    println!("JSON OUTPUT\n{}", json.to_string());

    Ok(json)
}

fn newalias(filename: &str, dirpath: &str) {
    // struct dirpa(String);
    // let newalias = alias("a".to_string());
}


fn main(){
    let opts = opts::from_args();
    let mut updateflag = false;
    // read the whole text file and return a list

    println!("{:?}", opts);

    if opts.dirpath.is_some() {
        // create new path or edit existing one, then return
        println!("directory provided! {:?}", opts.dirpath);
        // determine if path is objective or not

        
        return;
    }

    let json = readfile("foo.json").unwrap();

    // no new dirpath, travel to given directory using alias
    let path = Path::new(json[&opts.alias].as_str().unwrap());
    println!("given path: {:?}", path);
    if path.try_exists().unwrap() {
        println!("path exists!");
    } else {
        println!("path doesnt exist!");
    }

    // ensure path is valid
    // if no, return with error


    // path if dirpath is provided
    // if !opts.dirpath.is_empty() {
    //     if dict.get(&opts.alias).is_some() && !opts.force {
    //         println!("alias already exists, use force to overwrite.");
    //         return;
    //     }
    //     // TODO: verify path is valid
    //     dict.insert(opts.alias, opts.dirpath);
    //     println!("doesnt exist, adding now");
    //     updateflag = true;
    // }




    // TODO: make the alias variable optional
        // if included create new alias
        // if not included try and go to that alias
    // TODO: create command that lists all existing aliases
    // if updateflag {
    //     let mut output: String = "".to_string();
    //     for val in dict {
    //         output = format!("{}{:?}: {:?}\n", output, val.0, val.1);
    //     }
    //     println!("{output}");
    // }
    
    // write("foo.txt", path);
}
