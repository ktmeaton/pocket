#[cfg(feature = "glow")]
use eframe::glow;

use crate::BackendPanel;
use crate::is_mobile;
use crate::view::About;
//use crate::Tree;

// ----------------------------------------------------------------------------
// Applications
// ----------------------------------------------------------------------------


#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Game {}

impl eframe::App for Game {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
    }
}

// ----------------------------------------------------------------------------
// View
// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
enum View {
    About,
    Game,
    BackendPanel,
}

impl View {
    #[cfg(target_arch = "wasm32")]
    fn all() -> Vec<Self> {
        vec![
            Self::About,
            Self::Game,
        ]
    }
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<View> for egui::WidgetText {
    fn from(value: View) -> Self {
        Self::RichText(egui::RichText::new(value.to_string()))
    }
}

impl Default for View {
    fn default() -> Self {
        Self::About
    }
}

// ----------------------------------------------------------------------------
// COMMAND
// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
#[must_use]
enum Command {
    Nothing,
    ResetEverything,
}


// ----------------------------------------------------------------------------
// STATE
// ----------------------------------------------------------------------------

/// The state that we persist (serialize).
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State { 
    about: About,
    game: Game,
    backend_panel: BackendPanel,
    selected_view: View
}

/// Interface to our applications
pub struct Interface { state: State }


impl Interface {

    fn backend_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Command {
        // The backend-panel can be toggled on/off.
        // We show a little animation when the user switches it.
        let is_open =
            self.state.backend_panel.open || ctx.memory(|mem| mem.everything_is_visible());

        let mut cmd = Command::Nothing;

        egui::SidePanel::right("backend_panel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Backend");
                });

                ui.separator();
                self.backend_panel_contents(ui, frame, &mut cmd);
            });

        cmd
    }

    fn backend_panel_contents(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        cmd: &mut Command,
    ) {
        self.state.backend_panel.ui(ui, frame);

        ui.separator();

        ui.horizontal(|ui| {
            if ui
                .button("Reset egui")
                .on_hover_text("Forget scroll, positions, sizes etc")
                .clicked()
            {
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close_menu();
            }

            if ui.button("Reset everything").clicked() {
                *cmd = Command::ResetEverything;
                ui.close_menu();
            }
        });
    }

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
        };

        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }

    fn run_cmd(&mut self, ctx: &egui::Context, cmd: Command) {
        match cmd {
            Command::Nothing => {}
            Command::ResetEverything => {
                self.state = Default::default();
                ctx.memory_mut(|mem| *mem = Default::default());
            }
        }
    }

    fn show_selected_view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_view = self.state.selected_view;
        for (_name, view, app) in self.view_iter_mut() {
            if view == selected_view || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
            }
        }
    }

    /// Interate over the possible views in the interface
    fn view_iter_mut(&mut self) -> impl Iterator<Item = (&str, View, &mut dyn eframe::App)> {
        vec![
            ( "❔About", View::About, &mut self.state.about as &mut dyn eframe::App),
            ( "🎮 Game", View::Game,  &mut self.state.game  as &mut dyn eframe::App),
        ].into_iter()
    }

    fn view_menu_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame, cmd: &mut Command) {

        //  Non-Backend Views
        let mut selected_view = self.state.selected_view;
        for (name, view, _app) in self.view_iter_mut() {
            if ui
                .selectable_label(selected_view == view, name)
                .clicked()
            {
                selected_view = view;
                if frame.is_web() {
                    ui.ctx()
                        .open_url(egui::OpenUrl::same_tab(format!("#{view}")));
                }
            }
        }

        // Backend menu button
        if is_mobile(ui.ctx()) {
            ui.menu_button("💻 Backend", |ui| {
                ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
                self.backend_panel_contents(ui, frame, cmd);
            });
        } else {
            ui.toggle_value(&mut self.state.backend_panel.open, "💻 Backend");
        }

        ui.separator();

        // Theme
        egui::widgets::global_theme_preference_switch(ui);
        ui.separator();

        self.state.selected_view = selected_view;

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::warn_if_debug_build(ui);
        });
    }
}

impl eframe::App for Interface {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        // Give the area behind the floating windows a different color, because it looks better:
        let color = egui::lerp(
            egui::Rgba::from(visuals.panel_fill)..=egui::Rgba::from(visuals.extreme_bg_color),
            0.5,
        );
        let color = egui::Color32::from(color);
        color.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Check the url for an initial view
        #[cfg(target_arch = "wasm32")]
        if let Some(view) = frame.info().web_info.location.hash.strip_prefix('#') {
            let view = View::all().into_iter().find(|v| v.to_string() == view);
            if let Some(v) = view {
                self.state.selected_view = v;
            }
        }

        // On native allow full screen
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
        }

        let mut cmd = Command::Nothing;

        egui::TopBottomPanel::top("wrap_app_top_bar")
            .frame(egui::Frame::none().inner_margin(4.0))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.visuals_mut().button_frame = false;
                    self.view_menu_contents(ui, frame, &mut cmd);
                });
            });

        // egui::Window::new("My Window")
        //     //.open(&mut self.state.about)
        //     .show(ctx, |ui| { ui.label("Hello World!");});


        self.state.backend_panel.update(ctx, frame);
        if !is_mobile(ctx) {
            cmd = self.backend_panel(ctx, frame);
        }
        self.show_selected_view(ctx, frame);
        self.state.backend_panel.end_of_frame(ctx);
        self.run_cmd(ctx, cmd);
    }
}