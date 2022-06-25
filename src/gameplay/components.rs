use bevy::prelude::*;
use crate::palette;

pub const DEFAULT_HEALTH: i32 = 22;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CoreSpinner;

#[derive(Component)]
pub struct Health (pub i32);

#[derive(Copy, Clone, PartialEq)]
pub enum Species {
    Red,
    Green,
    Blue,
}

#[derive(Component)]
pub struct WalletDisplay(pub Species);

#[derive(Component, Copy, Clone)]
pub struct Money {
    pub species: Species,
    pub amount: u32,
}

impl Money {
    pub fn new(species: Species, amount: u32) -> Money {
        Money {
            species,
            amount,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum Item {
    Connector,
    RedCannon,
    GreenCannon,
    BlueCannon,
    ConverterRedGreen,
    ConverterRedBlue,
    ConverterGreenRed,
    ConverterGreenBlue,
    ConverterBlueRed,
    ConverterBlueGreen,
}

#[derive(Component)]
pub struct BuyItemButton {
    pub cost: Money,
    pub item: Item,
}

#[derive(Component)]
pub struct ItemPlacementGhost;

#[derive(Component)]
pub struct Cannon {
    pub species: Species,
    pub cooldown: f32,
}

#[derive(Component)]
pub struct CannonBase(pub Entity);

#[derive(Component)]
pub struct Monster(pub Species);

#[derive(Component)]
pub struct Bullet {
    pub species: Species,
    pub velocity: Vec2,
}
