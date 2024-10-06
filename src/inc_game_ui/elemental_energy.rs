use bevy::prelude::*;
use num::bigint::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(1., 0., 0.);

#[derive(Resource)]
pub struct ElementalEnergy {
    pub per_second: BigInt,
    pub amount: BigInt,
}

pub struct Elemental {
    pub energy_per_second: BigInt,
    pub total_energy_per_second: BigInt,
    pub quantity: BigInt,
    pub cost: BigInt,
}

#[derive(Component)]
pub enum Summon {
    FireElemental(Elemental),
    ElectricityElemental(Elemental),
    AirElemental(Elemental),
}
impl Summon {
    fn upgrade(&mut self) -> BigInt {
        match self {
            Summon::FireElemental(ele) => {
                let increase = 10.;
                let bigint_increase = increase.to_bigint();
                match bigint_increase {
                    Some(x) => {
                        ele.quantity += 1;
                        let new_eps = x;
                        ele.total_energy_per_second = new_eps;
                        return ele.energy_per_second.clone();
                    }
                    None => panic!(),
                }
            }
            Summon::AirElemental(ele) => {
                let increase = 20.;
                let bigint_increase = increase.to_bigint();
                match bigint_increase {
                    Some(x) => {
                        ele.quantity += 1;
                        let new_eps = x;
                        println!("Reached air elemental {}", new_eps);
                        ele.total_energy_per_second = new_eps;

                        return ele.energy_per_second.clone();
                    }
                    None => panic!(),
                }
            }
            Summon::ElectricityElemental(ele) => {
                let increase = 30.;
                let bigint_increase = increase.to_bigint();
                match bigint_increase {
                    Some(x) => {
                        ele.quantity += 1;
                        let new_eps = x;
                        println!("Reached electricity elemental: {}", new_eps);
                        ele.total_energy_per_second = new_eps;

                        return ele.energy_per_second.clone();
                    }
                    None => panic!(),
                }
            }
        }
    }
    fn get_energy_per_second(&self) -> BigInt {
        match self {
            &Summon::FireElemental(ref ele) => return ele.energy_per_second.clone(),
            &Summon::AirElemental(ref ele) => return ele.energy_per_second.clone(),
            &Summon::ElectricityElemental(ref ele) => return ele.energy_per_second.clone(),
        }
    }
    fn get_total_energy_per_second(&self) -> BigInt {
        match self {
            &Summon::FireElemental(ref ele) => return ele.total_energy_per_second.clone(),
            &Summon::AirElemental(ref ele) => return ele.total_energy_per_second.clone(),
            &Summon::ElectricityElemental(ref ele) => return ele.total_energy_per_second.clone(),
        }
    }
    fn get_cost(&self) -> BigInt {
        match self {
            &Summon::FireElemental(ref ele) => return ele.cost.clone(),
            &Summon::AirElemental(ref ele) => return ele.cost.clone(),
            &Summon::ElectricityElemental(ref ele) => return ele.cost.clone(),
        }
    }
}

pub fn summons_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &mut Summon),
        (Changed<Interaction>, With<Button>, With<Summon>),
    >,
    text_query: Query<&mut Text, With<Summon>>,
    mut energy: ResMut<ElementalEnergy>,
) {
    for (interaction, mut color, children, mut summon) in interaction_query.iter_mut() {
        //let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let eps_increase = summon.upgrade();
                println!("Summon button pressed for an elemental");
                println!("increased eps by {}", eps_increase);

                if energy.amount >= summon.get_cost() {
                    energy.amount -= summon.get_cost();
                    energy.per_second += eps_increase;
                } else {
                    println!("Not enough energy to summon this elemental")
                }

                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
#[derive(Component)]
pub struct ElementalEnergyPerSecondText;
pub fn energy_per_second_system(
    energy: Res<ElementalEnergy>,
    mut energy_per_second_text: Query<&mut Text, With<ElementalEnergyPerSecondText>>,
) {
    for mut text in energy_per_second_text.iter_mut() {
        text.sections[0].value = format!(
            "{}\n elemental energy per second",
            energy.per_second.to_str_radix(10)
        )
    }
}

#[derive(Component)]
pub struct ElementalEnergyText;
#[derive(Resource)]
pub struct EnergyTimer(pub Timer);
pub fn energy_system(
    mut energy: ResMut<ElementalEnergy>,
    mut energy_text: Query<&mut Text, With<ElementalEnergyText>>,
    time: Res<Time>,
    mut timer: ResMut<EnergyTimer>,
) {
    for mut text in energy_text.iter_mut() {
        text.sections[0].value = format!("{}\n elemental energy", energy.amount.to_str_radix(10));
    }
    if timer.0.tick(time.delta()).just_finished() {
        let per_second = energy.per_second.clone();
        energy.amount += per_second;
    }
}
