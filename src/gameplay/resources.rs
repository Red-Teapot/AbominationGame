use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::gameplay::components::*;

#[derive(Inspectable, Default)]
pub struct Wallet {
    pub red_squares: u32,
    pub green_triangles: u32,
    pub blue_circles: u32,
}

impl Wallet {
    pub fn try_spend(&mut self, money: Money) -> bool {
        match money.species {
            Species::Red => if self.red_squares >= money.amount {
                self.red_squares -= money.amount;
                return true;
            },
            Species::Green => if self.green_triangles >= money.amount {
                self.green_triangles -= money.amount;
                return true;
            },
            Species::Blue => if self.blue_circles >= money.amount {
                self.blue_circles -= money.amount;
                return true;
            },
        }

        return false;
    }
}