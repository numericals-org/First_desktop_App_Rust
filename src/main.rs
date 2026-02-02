use eframe::egui::CentralPanel;

#[derive(Default)]
struct App {

}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|{
            ui.heading("Hello From Application");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("Hello, world!");
    let options = eframe::NativeOptions{
        viewport: eframe::egui::ViewportBuilder::default().with_resizable(true).with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    return eframe::run_native("Jrss", options, Box::new(|_cc|Ok(Box::<App>::default())));
}
