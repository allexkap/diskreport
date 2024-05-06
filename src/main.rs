use std::env;
use std::error::Error;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    match walker(root_path) {
        Ok(data) => {
            let size = data.iter().map(|e| e.size).sum::<u64>();
            println!("{} Mb", size / 1048576);
        }
        Err(error) => eprintln!("{}: {}", root_path.display(), error),
    }
}

#[derive(Debug)]
struct Entry {
    name: String,
    size: u64,
    dir: Option<Vec<Entry>>,
}

fn walker(root_path: &Path) -> Result<Vec<Entry>, Box<dyn Error>> {
    match root_path.read_dir() {
        Ok(dir) => {
            let mut content = Vec::new();
            for entry in dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let Ok(name) = path
                            .file_name()
                            .unwrap_or(path.as_os_str())
                            .to_owned()
                            .into_string()
                        else {
                            eprintln!("Invalid Unicode data: {}", path.display());
                            continue;
                        };
                        if path.is_dir() {
                            match walker(&path) {
                                Ok(inner_dir) => content.push(Entry {
                                    name,
                                    size: inner_dir.iter().map(|e| e.size).sum(),
                                    dir: Some(inner_dir),
                                }),
                                Err(error) => eprintln!("{}: {}", error, path.display()),
                            }
                        } else {
                            match path.metadata() {
                                Ok(meta) => content.push(Entry {
                                    name,
                                    size: meta.len(),
                                    dir: None,
                                }),
                                Err(error) => eprintln!("{}: {}", error, path.display()),
                            }
                        }
                    }
                    Err(error) => eprintln!("{}: {}", error, root_path.display()),
                }
            }
            Ok(content)
        }
        Err(err) => Err(err.into()),
    }
}
