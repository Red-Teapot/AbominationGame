use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct GameplayAssets {
    #[asset(path = "gameplay/tiles.png")]
    pub tiles: Handle<Image>,

    #[asset(path = "gameplay/core-spinner.png")]
    pub core_spinner: Handle<Image>,

    #[asset(path = "gameplay/connector.png")]
    pub connector: Handle<Image>,

    #[asset(path = "gameplay/converter-red-green.png")]
    pub converter_red_green: Handle<Image>,

    #[asset(path = "gameplay/converter-red-blue.png")]
    pub converter_red_blue: Handle<Image>,
    #[asset(path = "gameplay/converter-green-red.png")]
    pub converter_green_red: Handle<Image>,
    #[asset(path = "gameplay/converter-green-blue.png")]
    pub converter_green_blue: Handle<Image>,
    #[asset(path = "gameplay/converter-blue-red.png")]
    pub converter_blue_red: Handle<Image>,
    #[asset(path = "gameplay/converter-blue-green.png")]
    pub converter_blue_green: Handle<Image>,

    #[asset(path = "gameplay/cannon-red.png")]
    pub cannon_red: Handle<Image>,
    #[asset(path = "gameplay/cannon-green.png")]
    pub cannon_green: Handle<Image>,
    #[asset(path = "gameplay/cannon-blue.png")]
    pub cannon_blue: Handle<Image>,

    #[asset(path = "gameplay/bullet-red.png")]
    pub bullet_red: Handle<Image>,
    #[asset(path = "gameplay/bullet-green.png")]
    pub bullet_green: Handle<Image>,
    #[asset(path = "gameplay/bullet-blue.png")]
    pub bullet_blue: Handle<Image>,

    #[asset(path = "gameplay/monster-red.png")]
    pub monster_red: Handle<Image>,
    #[asset(path = "gameplay/monster-green.png")]
    pub monster_green: Handle<Image>,
    #[asset(path = "gameplay/monster-blue.png")]
    pub monster_blue: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct UIAssets {
    #[asset(path = "red-square.png")]
    pub red_square: Handle<Image>,
    #[asset(path = "green-triangle.png")]
    pub green_triangle: Handle<Image>,
    #[asset(path = "blue-circle.png")]
    pub blue_circle: Handle<Image>,

    #[asset(path = "red-square-small.png")]
    pub red_square_small: Handle<Image>,
    #[asset(path = "green-triangle-small.png")]
    pub green_triangle_small: Handle<Image>,
    #[asset(path = "blue-circle-small.png")]
    pub blue_circle_small: Handle<Image>,
}