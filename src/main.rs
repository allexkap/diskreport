use eframe::egui;
use std::env;
use std::f32::consts::PI;
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
                dir_root: Rc::clone(&report),
                selected_dir: report,
            })
        }),
    )
    .unwrap();
}

struct DiskReportApp {
    dir_root: Rc<ReportEntry>,
    selected_dir: Rc<ReportEntry>,
}

impl DiskReportApp {
    fn add_collapsing(&mut self, ui: &mut egui::Ui, entry: &Rc<ReportEntry>, is_root: bool) {
        if entry.dir_entries.is_some() {
            let col_resp = egui::CollapsingHeader::new(&entry.name)
                .default_open(is_root)
                .show(ui, |ui| {
                    entry
                        .dir_entries
                        .as_ref()
                        .unwrap()
                        .iter()
                        .for_each(|d| self.add_collapsing(ui, d, false))
                });
            if col_resp.header_response.clicked() {
                self.selected_dir = Rc::clone(entry);
            }
        } else {
            ui.label(&entry.name);
        }
    }
    fn draw_pie(&self, painter: egui::Painter) {
        let clip_rect = painter.clip_rect();
        let center = clip_rect.center();
        let radius = clip_rect.size().min_elem() / 2.0;
        let w = vec![10, 3, 4, 12, 5];
        let arcs = gen_points(w, &center, radius);
        for points in arcs {
            let arc = egui::epaint::QuadraticBezierShape::from_points_stroke(
                points,
                false,
                egui::Color32::WHITE,
                egui::Stroke::new(1.0, egui::Color32::RED),
            );
            painter.add(arc);
        }
    }
}

fn gen_points(mut weights: Vec<u64>, center: &egui::Pos2, radius: f32) -> Vec<[egui::Pos2; 3]> {
    let total = weights.iter().sum::<u64>() as f32;
    weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut angle = 0.0;
    weights
        .into_iter()
        .map(|w| {
            let sector = w as f32 / total * 2.0 * PI;
            let points: [egui::Pos2; 3] = [angle, angle + sector / 2.0, angle + sector]
                .iter()
                .map(|a| egui::Pos2::new(a.sin() * radius + center.x, a.cos() * radius + center.y))
                .collect::<Vec<egui::Pos2>>()
                .try_into()
                .unwrap();
            angle += sector;
            points
        })
        .collect()
}

impl eframe::App for DiskReportApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let frame_size = ui.available_size();

            ui.horizontal(|ui| {
                egui::Resize::default()
                    .min_height(frame_size.y)
                    .max_height(frame_size.y)
                    .max_width(frame_size.x)
                    .default_width(frame_size.x / 3.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                self.add_collapsing(ui, &self.dir_root.clone(), true);
                            });
                    });
                ui.vertical_centered(|ui| {
                    ui.heading(&self.selected_dir.name);
                    let painter = ui.painter_at(ui.available_rect_before_wrap());
                    self.draw_pie(painter);
                });
            });
        });
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

#[derive(Debug)]
struct ReportEntry {
    name: String,
    size: u64,
    dir_entries: Option<Vec<Rc<ReportEntry>>>,
}

impl ReportEntry {
    fn build(root_path: &Path) -> Result<Rc<ReportEntry>, &'static str> {
        if !root_path.is_dir() {
            return Err("root path is not dir");
        }
        let data = walker(&root_path);
        Ok(Rc::new(ReportEntry {
            name: root_path.to_string_lossy().into_owned(),
            size: data.iter().map(|e| e.size).sum(),
            dir_entries: Some(data),
        }))
    }
}

fn walker(dir_path: &Path) -> Vec<Rc<ReportEntry>> {
    let mut content = Vec::new();
    let Ok(dir) = dir_path.read_dir() else {
        return content;
    };
    for entry in dir {
        let Ok(entry) = entry else { continue };
        let Ok(meta) = entry.metadata() else { continue };

        let name = entry.file_name().to_string_lossy().into_owned();
        let report_entry = if meta.is_dir() {
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
        };
        content.push(Rc::new(report_entry));
    }
    content.sort_by(|a, b| {
        b.dir_entries
            .is_some()
            .cmp(&a.dir_entries.is_some())
            .then(b.size.cmp(&a.size))
    });
    content
}
