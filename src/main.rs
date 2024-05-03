use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    let total = walker(root_path);
    println!("Total of '{}': {:.2} Mb", root_path.display(), total as f64 / f64::powf(2.0, 20.0));
}

fn walker(root_path: &Path) -> u64 {
    let mut total = 0;
    for entry in root_path.read_dir().unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            total += walker(&path);
        }
        else {
            println!("{:?} {:?}", path, path.metadata().unwrap().len());
            total += path.metadata().unwrap().len();
        }
    }
    total
}
