use std::env;
use std::error::Error;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    match Entry::build(root_path) {
        Ok(data) => {
            println!("{} {}", data.name, data.size);
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

impl Entry {
    fn build(root_path: &Path) -> Result<Entry, Box<dyn Error>> {
        let data = walker(&root_path);
        Ok(Entry {
            name: root_path.to_string_lossy().into_owned(),
            size: data.iter().map(|e| e.size).sum::<u64>(),
            dir: Some(data),
        })
    }
}

fn walker(dir_path: &Path) -> Vec<Entry> {
    let mut content = Vec::new();
    let Ok(dir) = dir_path.read_dir() else {
        return content;
    };
    for entry in dir {
        let Ok(entry) = entry else { continue };
        let Ok(meta) = entry.metadata() else { continue };

        let name = entry.file_name().to_string_lossy().into_owned();
        content.push(if meta.is_dir() {
            let data = walker(entry.path().as_path());
            Entry {
                name,
                size: data.iter().map(|e| e.size).sum::<u64>(),
                dir: Some(data),
            }
        } else {
            Entry {
                name,
                size: meta.len(),
                dir: None,
            }
        })
    }
    content
}
