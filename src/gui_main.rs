mod core;

use eframe::egui;
use poll_promise::Promise;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "File Splitter & Joiner",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(FileSplitterApp::default()))
        }),
    )
}

#[derive(Default)]
struct FileSplitterApp {
    active_tab: Tab,
    split_state: SplitState,
    join_state: JoinState,
}

#[derive(Default, PartialEq)]
enum Tab {
    #[default]
    Split,
    Join,
}

#[derive(Default)]
struct SplitState {
    input_file: Option<PathBuf>,
    part_size: u64,
    part_size_text: String,
    progress: Arc<Mutex<Option<core::ProgressInfo>>>,
    operation: Option<Promise<Result<Vec<PathBuf>, String>>>,
    result: Option<Result<Vec<PathBuf>, String>>,
    start_time: Option<Instant>,
}

#[derive(Default)]
struct JoinState {
    first_part: Option<PathBuf>,
    output_file: Option<PathBuf>,
    progress: Arc<Mutex<Option<core::ProgressInfo>>>,
    operation: Option<Promise<Result<PathBuf, String>>>,
    result: Option<Result<PathBuf, String>>,
    start_time: Option<Instant>,
}

impl FileSplitterApp {
    fn render_split_tab(&mut self, ui: &mut egui::Ui) {
        let is_processing = self.split_state.operation.is_some();

        // File Selection Card
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(245, 247, 250))
            .rounding(10.0)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.heading("📁 Select File to Split");
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    let button = egui::Button::new("  📂 Choose File  ")
                        .min_size(egui::vec2(150.0, 40.0));
                    if ui.add_enabled(!is_processing, button).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.split_state.input_file = Some(path);
                        }
                    }

                    ui.add_space(15.0);

                    if let Some(file) = &self.split_state.input_file {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(
                                    file.file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string(),
                                )
                                .strong()
                                .size(14.0),
                            );
                            if let Ok(metadata) = std::fs::metadata(file) {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Size: {}",
                                        core::format_bytes(metadata.len())
                                    ))
                                    .size(12.0)
                                    .color(egui::Color32::DARK_GRAY),
                                );
                            }
                        });
                    } else {
                        ui.label(
                            egui::RichText::new("No file selected")
                                .color(egui::Color32::GRAY)
                                .italics(),
                        );
                    }
                });

                ui.add_space(5.0);
            });

        ui.add_space(20.0);

        // Part Size Card
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(245, 247, 250))
            .rounding(10.0)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.heading("📏 Part Size");
                ui.add_space(15.0);

                // Manual input
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Custom size (bytes):").size(14.0));
                    ui.add_space(10.0);
                    let text_edit = egui::TextEdit::singleline(&mut self.split_state.part_size_text)
                        .desired_width(200.0)
                        .font(egui::TextStyle::Monospace);
                    ui.add_enabled(!is_processing, text_edit);
                });

                ui.add_space(15.0);

                // Quick size buttons
                ui.label(egui::RichText::new("Quick Select:").size(14.0).strong());
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let sizes = [
                        ("10 MB", 10 * 1024 * 1024),
                        ("100 MB", 100 * 1024 * 1024),
                        ("500 MB", 500 * 1024 * 1024),
                        ("1 GB", 1024 * 1024 * 1024),
                    ];

                    for (label, size) in sizes {
                        let button = egui::Button::new(egui::RichText::new(label).size(14.0))
                            .min_size(egui::vec2(100.0, 35.0));
                        if ui.add_enabled(!is_processing, button).clicked() {
                            self.split_state.part_size = size;
                            self.split_state.part_size_text = size.to_string();
                        }
                    }
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let sizes = [
                        ("2 GB", 2 * 1024 * 1024 * 1024),
                        ("4 GB", 4 * 1024 * 1024 * 1024),
                        ("8 GB", 8 * 1024 * 1024 * 1024),
                        ("Custom", 0),
                    ];

                    for (label, size) in sizes {
                        if size == 0 {
                            continue;
                        }
                        let button = egui::Button::new(egui::RichText::new(label).size(14.0))
                            .min_size(egui::vec2(100.0, 35.0));
                        if ui.add_enabled(!is_processing, button).clicked() {
                            self.split_state.part_size = size;
                            self.split_state.part_size_text = size.to_string();
                        }
                    }
                });

                // Parse part size
                if let Ok(size) = self.split_state.part_size_text.parse::<u64>() {
                    self.split_state.part_size = size;
                    if size > 0 {
                        ui.add_space(15.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Selected: {}",
                                core::format_bytes(size)
                            ))
                            .size(14.0)
                            .color(egui::Color32::from_rgb(0, 128, 0)),
                        );
                    }
                }

                ui.add_space(5.0);
            });

        ui.add_space(20.0);

        // Action Button
        let can_process = self.split_state.input_file.is_some()
            && self.split_state.part_size > 0
            && self.split_state.operation.is_none();

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2.0 - 100.0);
            let button = egui::Button::new(egui::RichText::new("  🚀 SPLIT FILE  ").size(16.0).strong())
                .fill(egui::Color32::from_rgb(59, 130, 246))
                .min_size(egui::vec2(200.0, 50.0));
            if ui.add_enabled(can_process, button).clicked() {
                self.start_split();
            }
        });

        ui.add_space(20.0);

        // Progress Section
        self.render_split_progress(ui);

        // Result Section
        self.render_split_result(ui);
    }

    fn render_split_progress(&mut self, ui: &mut egui::Ui) {
        let mut operation_done = false;
        let mut operation_result = None;

        {
            if let Some(operation) = &self.split_state.operation {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(240, 249, 255))
                    .rounding(10.0)
                    .inner_margin(20.0)
                    .show(ui, |ui| {
                        ui.add_space(5.0);

                        if let Some(progress) = self.split_state.progress.lock().unwrap().as_ref() {
                            let progress_fraction = progress.percentage() / 100.0;

                            // Status message
                            ui.label(
                                egui::RichText::new(&progress.message)
                                    .size(16.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(59, 130, 246)),
                            );

                            ui.add_space(15.0);

                            // Progress bar
                            let progress_bar = egui::ProgressBar::new(progress_fraction)
                                .text(format!("{:.1}%", progress.percentage()))
                                .animate(true)
                                .desired_height(30.0);
                            ui.add(progress_bar);

                            ui.add_space(15.0);

                            // Stats in columns
                            ui.columns(2, |columns| {
                                columns[0].vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new("Progress")
                                            .size(12.0)
                                            .color(egui::Color32::GRAY),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{} / {}",
                                            core::format_bytes(progress.current_bytes),
                                            core::format_bytes(progress.total_bytes)
                                        ))
                                        .size(14.0)
                                        .strong(),
                                    );
                                });

                                columns[1].vertical(|ui| {
                                    if let Some(start_time) = self.split_state.start_time {
                                        let elapsed = start_time.elapsed().as_secs_f64();
                                        if progress.current_bytes > 0 && elapsed > 0.0 {
                                            let bytes_per_sec = progress.current_bytes as f64 / elapsed;
                                            let remaining_bytes =
                                                progress.total_bytes - progress.current_bytes;
                                            let eta_secs = remaining_bytes as f64 / bytes_per_sec;

                                            ui.label(
                                                egui::RichText::new("Estimated Time")
                                                    .size(12.0)
                                                    .color(egui::Color32::GRAY),
                                            );
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{} remaining",
                                                    format_duration(eta_secs as u64)
                                                ))
                                                .size(14.0)
                                                .strong(),
                                            );
                                        }
                                    }
                                });
                            });
                        }

                        ui.add_space(5.0);
                    });

                if let Some(result) = operation.ready() {
                    operation_done = true;
                    operation_result = Some(result.clone());
                }

                ui.add_space(20.0);
            }
        }

        if operation_done {
            self.split_state.operation = None;
            self.split_state.result = operation_result;
        }
    }

    fn render_split_result(&mut self, ui: &mut egui::Ui) {
        if let Some(result) = &self.split_state.result.clone() {
            egui::Frame::none()
                .fill(match result {
                    Ok(_) => egui::Color32::from_rgb(240, 253, 244),
                    Err(_) => egui::Color32::from_rgb(254, 242, 242),
                })
                .rounding(10.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    match result {
                        Ok(parts) => {
                            ui.label(
                                egui::RichText::new("✅ Split Complete!")
                                    .size(18.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(34, 197, 94)),
                            );

                            ui.add_space(15.0);

                            ui.label(
                                egui::RichText::new(format!("Created {} part files:", parts.len()))
                                    .size(14.0),
                            );

                            ui.add_space(10.0);

                            egui::ScrollArea::vertical()
                                .max_height(200.0)
                                .show(ui, |ui| {
                                    for (i, part) in parts.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(format!("{}.", i + 1))
                                                    .color(egui::Color32::GRAY)
                                                    .monospace(),
                                            );
                                            ui.label(
                                                egui::RichText::new(
                                                    part.file_name()
                                                        .unwrap_or_default()
                                                        .to_string_lossy()
                                                        .to_string(),
                                                )
                                                .monospace()
                                                .size(12.0),
                                            );
                                        });
                                    }
                                });

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 100.0);
                                let button = egui::Button::new(
                                    egui::RichText::new("  🔄 Split Another File  ").size(14.0),
                                )
                                .min_size(egui::vec2(200.0, 40.0));
                                if ui.add(button).clicked() {
                                    self.split_state = SplitState::default();
                                }
                            });
                        }
                        Err(e) => {
                            ui.label(
                                egui::RichText::new("❌ Error Occurred")
                                    .size(18.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(239, 68, 68)),
                            );

                            ui.add_space(15.0);

                            ui.label(egui::RichText::new(e).size(13.0).color(egui::Color32::DARK_RED));

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 80.0);
                                let button = egui::Button::new(
                                    egui::RichText::new("  🔄 Try Again  ").size(14.0),
                                )
                                .min_size(egui::vec2(160.0, 40.0));
                                if ui.add(button).clicked() {
                                    self.split_state.result = None;
                                }
                            });
                        }
                    }

                    ui.add_space(5.0);
                });
        }
    }

    fn render_join_tab(&mut self, ui: &mut egui::Ui) {
        let is_processing = self.join_state.operation.is_some();

        // First Part Selection Card
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(245, 247, 250))
            .rounding(10.0)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.heading("📁 Select First Part File");
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    let button = egui::Button::new("  📂 Choose First Part (.part001)  ")
                        .min_size(egui::vec2(250.0, 40.0));
                    if ui.add_enabled(!is_processing, button).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.join_state.first_part = Some(path);
                        }
                    }
                });

                ui.add_space(15.0);

                if let Some(file) = &self.join_state.first_part {
                    ui.label(
                        egui::RichText::new(format!("Selected: {}", file.display()))
                            .size(13.0)
                            .strong(),
                    );

                    ui.add_space(10.0);

                    if let Ok(parts) = core::find_all_parts(file.as_path()) {
                        let total_size: u64 = parts
                            .iter()
                            .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
                            .sum();

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("✓ Found {} parts", parts.len()))
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(34, 197, 94)),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "Total: {}",
                                    core::format_bytes(total_size)
                                ))
                                .size(14.0)
                                .color(egui::Color32::DARK_GRAY),
                            );
                        });
                    }
                } else {
                    ui.label(
                        egui::RichText::new("No part file selected")
                            .color(egui::Color32::GRAY)
                            .italics(),
                    );
                }

                ui.add_space(5.0);
            });

        ui.add_space(20.0);

        // Output File Selection Card
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(245, 247, 250))
            .rounding(10.0)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.heading("💾 Output Location");
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    let button = egui::Button::new("  💾 Choose Save Location  ")
                        .min_size(egui::vec2(200.0, 40.0));
                    if ui.add_enabled(!is_processing, button).clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.join_state.output_file = Some(path);
                        }
                    }
                });

                ui.add_space(15.0);

                if let Some(file) = &self.join_state.output_file {
                    ui.label(
                        egui::RichText::new(format!("Save as: {}", file.display()))
                            .size(13.0)
                            .strong(),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("No output location selected")
                            .color(egui::Color32::GRAY)
                            .italics(),
                    );
                }

                ui.add_space(5.0);
            });

        ui.add_space(20.0);

        // Action Button
        let can_process = self.join_state.first_part.is_some()
            && self.join_state.output_file.is_some()
            && self.join_state.operation.is_none();

        ui.horizontal(|ui| {
            ui.add_space(ui.available_width() / 2.0 - 100.0);
            let button = egui::Button::new(egui::RichText::new("  🔗 JOIN FILES  ").size(16.0).strong())
                .fill(egui::Color32::from_rgb(139, 92, 246))
                .min_size(egui::vec2(200.0, 50.0));
            if ui.add_enabled(can_process, button).clicked() {
                self.start_join();
            }
        });

        ui.add_space(20.0);

        // Progress Section
        self.render_join_progress(ui);

        // Result Section
        self.render_join_result(ui);
    }

    fn render_join_progress(&mut self, ui: &mut egui::Ui) {
        let mut operation_done = false;
        let mut operation_result = None;

        {
            if let Some(operation) = &self.join_state.operation {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(245, 243, 255))
                    .rounding(10.0)
                    .inner_margin(20.0)
                    .show(ui, |ui| {
                        ui.add_space(5.0);

                        if let Some(progress) = self.join_state.progress.lock().unwrap().as_ref() {
                            let progress_fraction = progress.percentage() / 100.0;

                            ui.label(
                                egui::RichText::new(&progress.message)
                                    .size(16.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(139, 92, 246)),
                            );

                            ui.add_space(15.0);

                            let progress_bar = egui::ProgressBar::new(progress_fraction)
                                .text(format!("{:.1}%", progress.percentage()))
                                .animate(true)
                                .desired_height(30.0);
                            ui.add(progress_bar);

                            ui.add_space(15.0);

                            ui.columns(2, |columns| {
                                columns[0].vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new("Progress")
                                            .size(12.0)
                                            .color(egui::Color32::GRAY),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{} / {}",
                                            core::format_bytes(progress.current_bytes),
                                            core::format_bytes(progress.total_bytes)
                                        ))
                                        .size(14.0)
                                        .strong(),
                                    );
                                });

                                columns[1].vertical(|ui| {
                                    if let Some(start_time) = self.join_state.start_time {
                                        let elapsed = start_time.elapsed().as_secs_f64();
                                        if progress.current_bytes > 0 && elapsed > 0.0 {
                                            let bytes_per_sec = progress.current_bytes as f64 / elapsed;
                                            let remaining_bytes =
                                                progress.total_bytes - progress.current_bytes;
                                            let eta_secs = remaining_bytes as f64 / bytes_per_sec;

                                            ui.label(
                                                egui::RichText::new("Estimated Time")
                                                    .size(12.0)
                                                    .color(egui::Color32::GRAY),
                                            );
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{} remaining",
                                                    format_duration(eta_secs as u64)
                                                ))
                                                .size(14.0)
                                                .strong(),
                                            );
                                        }
                                    }
                                });
                            });
                        }

                        ui.add_space(5.0);
                    });

                if let Some(result) = operation.ready() {
                    operation_done = true;
                    operation_result = Some(result.clone());
                }

                ui.add_space(20.0);
            }
        }

        if operation_done {
            self.join_state.operation = None;
            self.join_state.result = operation_result;
        }
    }

    fn render_join_result(&mut self, ui: &mut egui::Ui) {
        if let Some(result) = &self.join_state.result.clone() {
            egui::Frame::none()
                .fill(match result {
                    Ok(_) => egui::Color32::from_rgb(240, 253, 244),
                    Err(_) => egui::Color32::from_rgb(254, 242, 242),
                })
                .rounding(10.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    match result {
                        Ok(output) => {
                            ui.label(
                                egui::RichText::new("✅ Join Complete!")
                                    .size(18.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(34, 197, 94)),
                            );

                            ui.add_space(15.0);

                            ui.label(
                                egui::RichText::new("Output file created:")
                                    .size(14.0)
                                    .color(egui::Color32::DARK_GRAY),
                            );
                            ui.label(
                                egui::RichText::new(output.display().to_string())
                                    .size(13.0)
                                    .monospace()
                                    .strong(),
                            );

                            if let Ok(metadata) = std::fs::metadata(output) {
                                ui.add_space(10.0);
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Size: {}",
                                        core::format_bytes(metadata.len())
                                    ))
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(34, 197, 94)),
                                );
                            }

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 100.0);
                                let button = egui::Button::new(
                                    egui::RichText::new("  🔄 Join Another File  ").size(14.0),
                                )
                                .min_size(egui::vec2(200.0, 40.0));
                                if ui.add(button).clicked() {
                                    self.join_state = JoinState::default();
                                }
                            });
                        }
                        Err(e) => {
                            ui.label(
                                egui::RichText::new("❌ Error Occurred")
                                    .size(18.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(239, 68, 68)),
                            );

                            ui.add_space(15.0);

                            ui.label(egui::RichText::new(e).size(13.0).color(egui::Color32::DARK_RED));

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 80.0);
                                let button = egui::Button::new(
                                    egui::RichText::new("  🔄 Try Again  ").size(14.0),
                                )
                                .min_size(egui::vec2(160.0, 40.0));
                                if ui.add(button).clicked() {
                                    self.join_state.result = None;
                                }
                            });
                        }
                    }

                    ui.add_space(5.0);
                });
        }
    }

    fn start_split(&mut self) {
        let input = self.split_state.input_file.clone().unwrap();
        let size = self.split_state.part_size;
        let progress = self.split_state.progress.clone();
        self.split_state.start_time = Some(Instant::now());

        let promise = Promise::spawn_thread("split", move || {
            core::split_file(&input, size, |info| {
                *progress.lock().unwrap() = Some(info);
            })
            .map_err(|e| e.to_string())
        });

        self.split_state.operation = Some(promise);
        self.split_state.result = None;
    }

    fn start_join(&mut self) {
        let first_part = self.join_state.first_part.clone().unwrap();
        let output = self.join_state.output_file.clone().unwrap();
        let progress = self.join_state.progress.clone();
        self.join_state.start_time = Some(Instant::now());

        let promise = Promise::spawn_thread("join", move || {
            core::join_files(&first_part, &output, |info| {
                *progress.lock().unwrap() = Some(info);
            })
            .map_err(|e| e.to_string())
        });

        self.join_state.operation = Some(promise);
        self.join_state.result = None;
    }
}

impl eframe::App for FileSplitterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.split_state.operation.is_some() || self.join_state.operation.is_some() {
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("📦 File Splitter & Joiner")
                        .size(24.0)
                        .strong()
                        .color(egui::Color32::from_rgb(30, 41, 59)),
                );
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("Split large files or join parts back together")
                        .size(13.0)
                        .color(egui::Color32::GRAY),
                );
            });

            ui.add_space(20.0);

            ui.separator();

            ui.add_space(20.0);

            // Tab selector
            ui.horizontal(|ui| {
                ui.add_space(20.0);

                let split_selected = self.active_tab == Tab::Split;
                let join_selected = self.active_tab == Tab::Join;

                let split_button = egui::Button::new(
                    egui::RichText::new("  ✂️  Split  ")
                        .size(15.0)
                        .strong(),
                )
                .fill(if split_selected {
                    egui::Color32::from_rgb(59, 130, 246)
                } else {
                    egui::Color32::from_rgb(226, 232, 240)
                })
                .min_size(egui::vec2(120.0, 40.0));

                if ui.add(split_button).clicked() {
                    self.active_tab = Tab::Split;
                }

                ui.add_space(10.0);

                let join_button = egui::Button::new(
                    egui::RichText::new("  🔗 Join  ")
                        .size(15.0)
                        .strong(),
                )
                .fill(if join_selected {
                    egui::Color32::from_rgb(139, 92, 246)
                } else {
                    egui::Color32::from_rgb(226, 232, 240)
                })
                .min_size(egui::vec2(120.0, 40.0));

                if ui.add(join_button).clicked() {
                    self.active_tab = Tab::Join;
                }
            });

            ui.add_space(25.0);

            // Content area with padding
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        ui.set_max_width(ui.available_width() - 40.0);

                        match self.active_tab {
                            Tab::Split => self.render_split_tab(ui),
                            Tab::Join => self.render_join_tab(ui),
                        }

                        ui.add_space(20.0);
                    });
                });
            });
        });
    }
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}
