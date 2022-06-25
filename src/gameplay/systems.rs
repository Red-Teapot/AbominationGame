use bevy::prelude::*;
use crate::gameplay::components::*;
use crate::gameplay::resources::Wallet;

pub fn core_spinner(mut query: Query<&mut Transform, With<CoreSpinner>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(5.375 * time.delta_seconds()));
    }
}

pub fn wallet_display(mut query: Query<(&mut Text, &WalletDisplay)>, wallet: Res<Wallet>) {
    for (mut text, wallet_display) in query.iter_mut() {
        use Species::*;

        let value = match wallet_display.0 {
            Red => wallet.red_squares,
            Green => wallet.green_triangles,
            Blue => wallet.blue_circles,
        };

        text.sections[0].value = value.to_string();
    }
}