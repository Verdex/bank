
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

    let blarg = fs::read_dir(".").expect("couldn't open current directory");
    for x in blarg {
        let x = x.unwrap();
        let w = x.path();
        if w.is_dir() && let Some(name) = w.file_name() && name == ".bank" {
            println!("{}", w.display());
        
        }
    }
}
