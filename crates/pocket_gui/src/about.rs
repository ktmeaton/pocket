#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct About {}

impl egui_demo_lib::Demo for About {
    fn name(&self) -> &'static str {
        "About pocket"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .resizable([true, false])
            .show(ctx, |ui| {
                use egui_demo_lib::View as _;
                self.ui(ui);
            });
    }
}

impl egui_demo_lib::View for About {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.heading("pocket");
        ui.label(format!(
            "pocket is an immediate mode GUI library written in Rust. pocket runs both on the web and natively on {}{}{}. \
            On the web it is compiled to WebAssembly and rendered with WebGL.{}",
            OS_APPLE, OS_LINUX, OS_WINDOWS,
            if cfg!(target_arch = "wasm32") {
                " Everything you see is rendered as textured triangles. There is no DOM, HTML, JS or CSS. Just Rust."
            } else {""}
        ));
        ui.label("pocket is designed to be easy to use, portable, and fast.");

        ui.add_space(12.0); // ui.separator();
        ui.heading("Links");
        links(ui);

        ui.add_space(12.0);

        ui.vertical_centered(|ui| {
            ui.add(crate::pocket_github_link_file!());
        });
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::GITHUB;
    ui.hyperlink_to(
        format!("{GITHUB} pocket on GitHub"),
        "https://github.com/ktmeaton/pocket",
    );
    ui.hyperlink_to("pocket documentation", "https://docs.rs/pocket");
}