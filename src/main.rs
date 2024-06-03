use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    match ReportEntry::build(root_path) {
        Ok(data) => {
            println!("{}: {} Mb", data.name, data.size / u64::pow(2, 20));
        }
        Err(error) => eprintln!("{}: {}", root_path.display(), error),
    }
}

#[derive(Debug)]
struct ReportEntry {
    name: String,
    size: u64,
    dir_entries: Option<Vec<ReportEntry>>,
}

impl ReportEntry {
    fn build(root_path: &Path) -> Result<ReportEntry, &'static str> {
        if !root_path.is_dir() {
            return Err("root path is not dir");
        }
        let data = walker(&root_path);
        Ok(ReportEntry {
            name: root_path.to_string_lossy().into_owned(),
            size: data.iter().map(|e| e.size).sum(),
            dir_entries: Some(data),
        })
    }
}

fn walker(dir_path: &Path) -> Vec<ReportEntry> {
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
            ReportEntry {
                name,
                size: data.iter().map(|e| e.size).sum(),
                dir_entries: Some(data),
            }
        } else {
            ReportEntry {
                name,
                size: meta.len(),
                dir_entries: None,
            }
        })
    }
    content.sort_by_key(|e| std::cmp::Reverse(e.size));
    content
}
