mod interface;
#[doc(inline)]
pub use interface::Interface;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

mod backend_panel;
#[doc(inline)]
pub use backend_panel::BackendPanel;

mod frame_history;
pub use frame_history::FrameHistory;

pub mod view;

/// Detect narrow screens. This is used to show a simpler UI on mobile devices,
/// especially for the web demo at <https://egui.rs>.
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
