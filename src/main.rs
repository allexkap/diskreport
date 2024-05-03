use std::env;
use std::path::Path;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    println!(
        "Total of '{}': {:.2} bytes",
        root_path.display(),
        walker(root_path, true)
    )
}

fn walker(root_path: &Path, thread_mode: bool) -> u64 {
    let mut total = 0;
    let mut threads = if thread_mode { Some(Vec::new()) } else { None };
    match root_path.read_dir() {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        total += if path.is_dir() {
                            if thread_mode {
                                let th = thread::spawn(move || walker(&path, false));
                                threads.as_mut().unwrap().push(th);
                                0
                            } else {
                                walker(&path, false)
                            }
                        } else {
                            match path.metadata() {
                                Ok(data) => data.len(),
                                Err(error) => {
                                    println!("{}: {}", path.display(), error);
                                    0
                                }
                            }
                        };
                    }
                    Err(error) => println!("{:?}", error),
                }
            }
        }
        Err(error) => println!("{:?}", error),
    }
    if thread_mode {
        for thread in threads.unwrap() {
            total += thread.join().unwrap();
        }
    }
    total
}
