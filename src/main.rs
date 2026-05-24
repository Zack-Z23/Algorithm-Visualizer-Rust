use eframe::egui;
use rand::Rng;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;


#[derive(PartialEq)]
enum Algorithm {
    Bubble,
    Insertion,
    Bogo,
}


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Algorithm Visualizer",
        options,
        Box::new(|_| Ok(Box::new(App::new()))),
    );
}

struct App {
    arr: Vec<i32>,
    step: usize,
    sorting: bool,
    steps_per_frame: usize,
    algorithm: Algorithm,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let arr = (0..150).map(|_| rng.gen_range(1..1000)).collect();
        Self {
            arr,
            step: 0,
            sorting: false,
            steps_per_frame: 50,
            algorithm: Algorithm::Bubble
        }
    }

    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.arr = (0..150).map(|_| rng.gen_range(1..1000)).collect();
        self.step = 0;
        self.sorting = false;
    }

    fn bubble_sort_step(&mut self) -> bool {
        let n = self.arr.len();
        if self.step >= n * (n - 1) {
            return false;
        }
        let i = self.step / n;
        let j = self.step % n;
        if j + 1 < n && self.arr[j] > self.arr[j + 1] {
            self.arr.swap(j, j + 1);
        }
        self.step += 1;
        true
    }
    fn insertion_sort_step(&mut self) -> bool {
        let n = self.arr.len();
        if self.step >= n {
            return false;
        }

        let i = self.step;
        let key = self.arr[i];
        let mut j = i as isize - 1;

        while j >= 0 && self.arr[j as usize] > key {
            self.arr[(j + 1) as usize] = self.arr[j as usize];
            j -= 1;
        }
        self.arr[(j + 1) as usize] = key;
        self.step += 1;

        true
    }
     fn bogo_sort_step(&mut self) -> bool {
        if Self::is_sorted(&self.arr) {
            return false; 
        }

        let mut rng = thread_rng();
        self.arr.shuffle(&mut rng); 
        true 
    }

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Sorting Visualizer");

            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.reset();
                }
                if ui.button(if self.sorting { "Pause" } else { " Start" }).clicked() {
                    self.sorting = !self.sorting;
                }
                ui.add(egui::Slider::new(&mut self.steps_per_frame, 1..=10).text("Steps/Frame"));
            });

            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                egui::ComboBox::from_label("")
                        .selected_text(match self.algorithm {
                            Algorithm::Bubble => "Bubble Sort",
                                Algorithm::Insertion => "Insertion Sort",
                                Algorithm::Bogo => "Bogo Sort",
             })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.algorithm, Algorithm::Bubble, "Bubble Sort");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Insertion, "Insertion Sort");
                        ui.selectable_value(&mut self.algorithm, Algorithm::Bogo, "Bogo Sort")

                        });
            });


            ui.separator();

         
            let max = *self.arr.iter().max().unwrap_or(&1) as f32;
            let available_width = ui.available_width();
            let bar_width = available_width / self.arr.len() as f32;

            let painter = ui.painter();
            let rect = ui.max_rect();
            let base_y = rect.bottom();

            for (i, &val) in self.arr.iter().enumerate() {
                let height = (val as f32 / max) * rect.height() * 0.9;
                let x = rect.left() + i as f32 * bar_width;
                let color = egui::Color32::from_rgb(100, 150, 250);
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(x, base_y - height),
                        egui::vec2(bar_width - 2.0, height),
                    ),
                    0.0,
                    color,
                );
            }
        });

 
        if self.sorting {
            for _ in 0..self.steps_per_frame {
             let running = match self.algorithm {
                    Algorithm::Bubble => self.bubble_sort_step(),
                    Algorithm::Insertion => self.insertion_sort_step(),
                    Algorithm::Bogo => self.bogo_sort_step(),
            };
                if !running {
                    self.sorting = false;
                    break;
            }

        }
            ctx.request_repaint();
        }
    }
}