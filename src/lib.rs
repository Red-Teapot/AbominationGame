#[cfg(target_arch = "wasm32")]
mod web_main;

mod palette;
mod loading;
mod assets;
mod game;
mod gameplay;
mod lose;

pub use game::*;
