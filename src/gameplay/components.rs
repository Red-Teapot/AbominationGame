use bevy::prelude::*;
use crate::palette;

pub const DEFAULT_HEALTH: u8 = 22;

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
