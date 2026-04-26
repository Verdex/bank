
use std::env;
use std::io::{self, Write};
use std::fs::{self, OpenOptions, File};
use std::path::PathBuf;
use std::time::{ SystemTime, UNIX_EPOCH };

const MAX_REPO_SIZE : u64 = 1_048_576; 

fn main() {
    let input = io::read_to_string(io::stdin()).expect("reading stream failed");
    print!("{}", input); 

    let bank_dir = find_bank().expect("couldn't find .bank directory");
    let mut repo = find_next_repo(bank_dir).expect("failure finding next repo");

    writeln!(repo, "==================================================").expect("failure writing divider");
    writeln!(repo, "{}", epoch()).expect("failure writing time");
    
    for arg in env::args().skip(1) {
        writeln!(repo, "{}", arg).expect("failure writing meta");
    }

    writeln!(repo, "{}", input).expect("failure writing data");
}

fn find_next_repo(mut bank : PathBuf) -> io::Result<File> {
    fn to_time(x:PathBuf) -> (PathBuf, u64) {
        let time = match fs::metadata(&x) {
            Ok(m) => match m.created() {
                Ok(t) => t.duration_since(UNIX_EPOCH).expect("time failure").as_secs(),
                Err(_) => 0,
            },
            Err(_) => 0,
        };
        (x, time)
    }

    let dir = fs::read_dir(&bank)?;
    let result = dir
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| x.is_file())
        .map(to_time)
        .max_by_key(|(_, t)| *t);

    match result { 
        Some(x) => {
            let m = fs::metadata(&x.0)?;
            if m.len() <= MAX_REPO_SIZE {
                OpenOptions::new().write(true).append(true).open(x.0)
            }
            else {
                bank.push(epoch());
                OpenOptions::new().write(true).create_new(true).open(bank)
            }
        },
        None => { 
            bank.push(epoch());
            OpenOptions::new().write(true).create_new(true).open(bank)
        }
    }
}

fn find_bank() -> io::Result<PathBuf> {
    let mut target = PathBuf::new();
    target.push(".");

    loop {
        let dir = fs::read_dir(&target)?;
        for f in dir {
            let f = f?.path();
            if f.is_dir() && let Some(name) = f.file_name() && name == ".bank" {
                return Ok(f);
            }
        }
        target.push("..");
    }
}

fn epoch() -> String {
    format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).expect("time failure").as_secs())
}
