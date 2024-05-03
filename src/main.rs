use std::env;
use std::path::Path;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    match walker(root_path) {
        Ok(total) => println!("Total of '{}': {:.2} bytes", root_path.display(), total),
        Err(error) => println!("Error: {}", error),
    }
}

fn walker(root_path: &Path) -> Result<u64, Box<dyn Error>>  {
    let mut total = 0;
    for entry in root_path.read_dir()? {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                total += if path.is_dir() {
                    walker(&path).unwrap_or_else(|e| {
                        println!("{}: {}", path.display(), e);
                        0
                    })
                } else {
                    match path.metadata() {
                        Ok(data) => data.len(),
                        Err(error) => {
                            println!("{}: {}", path.display(), error);
                            0
                        }
                    }
                };
            },
            Err(error) => println!("{:?}", error),
        }
    }
    Ok(total)
}
