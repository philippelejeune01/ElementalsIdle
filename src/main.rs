use bevy::{prelude::*, window::WindowPlugin};
use num::bigint::*;

mod inc_game_ui;
use crate::inc_game_ui::elemental_energy::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Elementals Idle".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, inc_game_ui::setup)
            .add_systems(
                Update,
                (
                    inc_game_ui::mouse_scroll,
                    energy_system,
                    inc_game_ui::button_system,
                    energy_per_second_system,
                    summons_system,
                ),
            )
            .insert_resource(ElementalEnergy {
                amount: BigInt::from(0),
                per_second: BigInt::from(0),
            })
            .insert_resource(EnergyTimer(Timer::from_seconds(1., TimerMode::Repeating)));
    }
}
