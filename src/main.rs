use eframe::egui;
use std::env;
use std::path::Path;
use std::rc::Rc;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let root_path = Path::new(&args[1]);

    let report = match ReportEntry::build(root_path) {
        Ok(data) => {
            println!("{}: {} Mb", data.name, data.size / u64::pow(2, 20));
            data
        }
        Err(error) => {
            eprintln!("{}: {}", root_path.display(), error);
            return;
        }
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Disk Report",
        options,
        Box::new(|_cc| {
            Box::new(DiskReportApp {
                dir_root: report.clone(),
                selected_dir: report,
            })
        }),
    )
    .unwrap();
}

struct DiskReportApp {
    dir_root: ReportEntry,
    selected_dir: ReportEntry,
}

impl DiskReportApp {
    fn _add_collapsing(&mut self, ui: &mut egui::Ui, entry: &ReportEntry) {
        if entry.dir_entries.is_some() {
            let col_resp = ui.collapsing(&entry.name, |ui| {
                entry
                    .dir_entries
                    .as_ref()
                    .unwrap()
                    .iter()
                    .for_each(|d| self._add_collapsing(ui, &d))
            });
            if col_resp.header_response.clicked() {
                self.selected_dir = entry.clone();
            }
        } else {
            ui.label(&entry.name);
        }
    }
}

impl eframe::App for DiskReportApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.selected_dir.name);
            self._add_collapsing(ui, &self.dir_root.clone());
        });
    }
}

#[derive(Debug, Clone)]
struct ReportEntry {
    name: String,
    size: u64,
    dir_entries: Option<Rc<Vec<ReportEntry>>>,
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
            dir_entries: Some(Rc::new(data)),
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
                dir_entries: Some(Rc::new(data)),
            }
        } else {
            ReportEntry {
                name,
                size: meta.len(),
                dir_entries: None,
            }
        })
    }
    content.sort_by(|a, b| {
        b.dir_entries
            .is_some()
            .cmp(&a.dir_entries.is_some())
            .then(b.size.cmp(&a.size))
    });
    content
}
