use bevy::prelude::*;
use crate::palette;

pub const DEFAULT_HEALTH: u8 = 22;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CoreSpinner;

#[derive(Component)]
pub struct Health (pub u8);

#[derive(Copy, Clone)]
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
pub struct Cannon(pub Species);

#[derive(Component)]
pub struct CannonBase(pub Entity);
