//! Demo app for egui
#![allow(clippy::missing_errors_doc)]

mod apps;
mod backend_panel;
mod frame_history;
mod wrap_app;
mod database;
mod about;

pub use wrap_app::WrapApp;

/// Time of day as seconds since midnight. Used for clock in demo app.
pub(crate) fn seconds_since_midnight() -> f64 {
    use chrono::Timelike;
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// Create a [`Hyperlink`](egui::Hyperlink) to this pocket source code file on github.
#[macro_export]
macro_rules! pocket_github_link_file {
    () => {
        $crate::pocket_github_link_file!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file!(
            "https://github.com/ktmeaton/pocket/blob/main/",
            egui::RichText::new($label).small()
        )
    };
}

/// Create a [`Hyperlink`](egui::Hyperlink) to this pocket source code file and line on github.
#[macro_export]
macro_rules! pocket_github_link_file_line {
    () => {
        $crate::pocket_github_link_file_line!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file_line!(
            "https://github.com/ktmeaton/pocket/blob/main/",
            egui::RichText::new($label).small()
        )
    };
}