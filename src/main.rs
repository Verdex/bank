
// search upward for .bank directory indicate failure if it's not there
// get time
// take arg 2 if it exists 
// are there any files in .bank? no then make one and dump the info into it
// if files are here, then find the youngest one, if it's sufficiently small then append to it
// finished
//
// ===========================
// arg 2
// date
// stdin

use std::io;
use std::fs;
use std::path::Path;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("reading stream failed");
    print!("{}", input); // TODO println?

    find_bank();
}

fn find_bank() {

    let dir = fs::read_dir(".").expect("couldn't open current directory");
    for f in dir {
        let f = f.unwrap();
        let f = f.path();
        if f.is_dir() && let Some(name) = f.file_name() && name == ".bank" {
            println!("{}", f.display());
        
        }
    }
}
