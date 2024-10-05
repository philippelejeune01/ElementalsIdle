use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui,
    window::WindowPlugin,
    winit::WinitSettings,
};
use num::bigint::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
#[derive(Resource)]
struct ElementalEnergyPerSecond(BigInt);
#[derive(Resource)]
struct ElementalEnergy(BigInt);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "something other than \"Bevy App\"".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(HelloPlugin)
        .run();
}

#[derive(Component)]
struct ElementalEnergyButton;

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            With<ElementalEnergyButton>,
        ),
    >,
    mut text_query: Query<&mut Text, With<ElementalEnergyButton>>,
    mut amount: ResMut<ElementalEnergy>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Press".to_string();
                amount.0 += 1;
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component)]
struct ElementalEnergyText;
#[derive(Resource)]
struct EnergyTimer(Timer);
fn energy_system(
    mut energy: ResMut<ElementalEnergy>,
    energy_per_second: Res<ElementalEnergyPerSecond>,
    mut energy_text: Query<&mut Text, With<ElementalEnergyText>>,
    time: Res<Time>,
    mut timer: ResMut<EnergyTimer>,
) {
    for mut text in energy_text.iter_mut() {
        text.sections[0].value = format!("{}\n elemental energy", energy.0.to_str_radix(10));
    }
    if timer.0.tick(time.delta()).just_finished() {
        energy.0 += energy_per_second.0.clone();
    }
}

#[derive(Component)]
struct ElementalEnergyPerSecondText;
fn energy_per_second_system(
    energy_per_second: Res<ElementalEnergyPerSecond>,
    mut energy_per_second_text: Query<&mut Text, With<ElementalEnergyPerSecondText>>,
) {
    for mut text in energy_per_second_text.iter_mut() {
        text.sections[0].value = format!(
            "{}\n elemental energy per second",
            energy_per_second.0.to_str_radix(10)
        )
    }
}

struct Elemental {
    energy_per_second: BigInt,
    total_energy_per_second: BigInt,
    quantity: BigInt,
}

#[derive(Component)]
enum Summon {
    FireElemental(Elemental),
    ElectricityElemental(Elemental),
    AirElemental(Elemental),
}
impl Summon {
    /*fn upgrade(
        &self,
        mut energy: ResMut<ElementalEnergy>,
        mut energy_per_second: ResMut<ElementalEnergyPerSecond>,
    ) -> f32 {
        match &self {
            Summon::FireElemental(mut ele) => {
                ele.quantity += 1;
                let increase = 10. + f32::powf(10., 1.1);
                increase
            }
            Summon::AirElemental(mut ele) => {
                ele.quantity += 1;
                let increase = 10. + f32::powf(100., 1.2);
                increase
            }
            Summon::ElectricityElemental(mut ele) => {
                ele.quantity += 1;
                let increase = 10. + f32::powf(1000., 1.3);
                increase
            }
        }
    }*/
}

fn summons_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &Summon),
        (Changed<Interaction>, With<Button>, With<Summon>),
    >,
    mut text_query: Query<&mut Text, (With<Summon>)>,
    mut energy_per_second: ResMut<ElementalEnergyPerSecond>,
    mut amount: ResMut<ElementalEnergy>,
) {
    for (interaction, mut color, children, summon) in interaction_query.iter_mut() {
        //let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                println!("Summon button pressed for an elemental");
                println!(
                    "increased eps by",
                    //summon.upgrade(amount, energy_per_second)
                );
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    //root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..default()
            },

            ..default()
        })
        .with_children(|parent| {
            //energy display
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::RowReverse,
                        justify_content: JustifyContent::Center,
                        flex_wrap: FlexWrap::WrapReverse,
                        position_type: PositionType::Absolute,
                        left: Val::Percent(0.),
                        bottom: Val::Percent(75.),
                        width: Val::Percent(70.),
                        height: Val::Percent(25.),
                        max_width: Val::Percent(70.),
                        max_height: Val::Auto,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            text: Text::from_section(
                                "0\n energy",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..default()
                        })
                        .insert(ElementalEnergyText);

                    parent
                        .spawn(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            text: Text::from_section(
                                "0\n energy per second",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..default()
                        })
                        .insert(ElementalEnergyPerSecondText);
                });
            //da button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                })
                .insert(ElementalEnergyButton);
            //Summons box
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(95.0),
                        border: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(2.0),
                            bottom: Val::Px(2.0),
                        },
                        align_items: AlignItems::Center,
                        //align_self: AlignSelf::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //scrolling list
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                flex_grow: 1.0,
                                max_width: Val::Percent(100.),
                                max_height: Val::Percent(100.),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ScrollingList::default())
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Auto,
                                        height: Val::Px(65.0),
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Auto,
                                            bottom: Val::Auto,
                                        },
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        style: Style {
                                            margin: UiRect {
                                                left: Val::Px(10.),
                                                right: Val::Px(10.),
                                                top: Val::Px(10.),
                                                bottom: Val::Px(10.),
                                            },
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            "Fire Elemental",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.0,
                                                color: Color::srgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(Summon::FireElemental(Elemental {
                                    energy_per_second: BigInt::from(10),
                                    total_energy_per_second: BigInt::from(0),
                                    quantity: BigInt::from(0),
                                }));

                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Auto,
                                        height: Val::Px(65.0),
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Auto,
                                            bottom: Val::Auto,
                                        },
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        style: Style {
                                            margin: UiRect {
                                                left: Val::Px(10.),
                                                right: Val::Px(10.),
                                                top: Val::Px(10.),
                                                bottom: Val::Px(10.),
                                            },
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            "Air Elemental",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.,
                                                color: Color::srgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(Summon::AirElemental(Elemental {
                                    energy_per_second: BigInt::new(Sign::Plus, vec![1, 0, 0]),
                                    total_energy_per_second: BigInt::new(Sign::Plus, vec![1, 0, 0]),
                                    quantity: BigInt::new(Sign::Plus, vec![0]),
                                }));

                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Auto,
                                        height: Val::Px(65.0),
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            top: Val::Auto,
                                            bottom: Val::Auto,
                                        },
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        style: Style {
                                            margin: UiRect {
                                                left: Val::Px(10.),
                                                right: Val::Px(10.),
                                                top: Val::Px(10.),
                                                bottom: Val::Px(10.),
                                            },
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            "Electricity Elemental",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(Summon::ElectricityElemental(Elemental {
                                    energy_per_second: BigInt::new(Sign::Plus, vec![1, 0, 0]),
                                    total_energy_per_second: BigInt::new(Sign::Plus, vec![1, 0, 0]),
                                    quantity: BigInt::new(Sign::Plus, vec![0]),
                                }));
                        });
                });
        });
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    mouse_scroll,
                    energy_system,
                    button_system,
                    energy_per_second_system,
                    summons_system,
                ),
            )
            .insert_resource(ElementalEnergy(BigInt::from(0)))
            .insert_resource(ElementalEnergyPerSecond(BigInt::from(0)))
            .insert_resource(EnergyTimer(Timer::from_seconds(1., TimerMode::Repeating)));
    }
}
