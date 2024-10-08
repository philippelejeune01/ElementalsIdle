use crate::*;
use bevy::prelude::*;
use num::bigint::*;

pub trait Upgradable {
    fn apply(&self, upgrade: &Upgrade, ele: &Elemental) -> BigInt;
}
#[derive(Component)]
pub struct Upgrade {
    pub upgrade: Upgrades,
    pub label: String,
}
pub enum Upgrades {
    ElectricityUpgrade,
    AirUpgrade,
    FireUpgrade,
}

impl Upgradable for Upgrade {
    fn apply(&self, upgrade: &Upgrade, ele: &Elemental) -> BigInt {
        match upgrade.upgrade {
            Upgrades::FireUpgrade => return ele.energy_per_second.clone() * BigInt::from(2),
            Upgrades::AirUpgrade => return ele.energy_per_second.clone() * BigInt::from(2),
            Upgrades::ElectricityUpgrade => return ele.energy_per_second.clone() * BigInt::from(2),
        }
    }
}
impl Upgrades {}
pub fn generate_upgrade_list() -> Vec<Upgrade> {
    vec![
        Upgrade {
            upgrade: Upgrades::FireUpgrade,
            label: "better fire elemental".to_string(),
        },
        Upgrade {
            upgrade: Upgrades::AirUpgrade,
            label: "better air elemental".to_string(),
        },
        Upgrade {
            upgrade: Upgrades::ElectricityUpgrade,
            label: "better electricty elemental".to_string(),
        },
    ]
}
