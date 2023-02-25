#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod audio;
mod plugin_host;
mod plugins_container;
pub use app::TemplateApp;
