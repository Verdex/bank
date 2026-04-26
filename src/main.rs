
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

    let bank_dir = find_bank().expect("couldn't find .bank directory");

    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1 {

    }

}

fn find_next_repo(mut bank : PathBuf) -> io::Result<PathBuf> {
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
        .max_by_key(|(p, t)| *t);

    match result { 
        Some(x) => {
            let m = fs::metadata(&x.0)?;
            if m.len() <= MAX_REPO_SIZE {
                Ok(x.0)
            }
            else {
                bank.push(format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).expect("time name failure").as_secs()));
                Ok(bank)
            }
        },
        None => { 
            bank.push(format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).expect("time name failure").as_secs()));
            Ok(bank)
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
