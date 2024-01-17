#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use mme_shader_graph::NodeGraphExample;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::Visuals;

    eframe::run_native(
        "MME Shader Graph",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::<NodeGraphExample>::default()
        }),
    )
    .expect("Failed to run native example");
}
