use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Default)]
pub struct Wallet {
    pub red_squares: u32,
    pub green_triangles: u32,
    pub blue_circles: u32,
}
