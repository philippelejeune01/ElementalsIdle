use crate::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use num::bigint::*;
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(1., 0., 0.);

pub mod elemental_energy;
pub mod upgrades;

#[derive(Component)]
pub struct ElementalEnergyButton;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
    let elementals = generate_elemental_list();
    let upgrades = upgrades::generate_upgrade_list();
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
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..default()
                        })
                        .insert(elemental_energy::ElementalEnergyText);

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
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                },
                            ),
                            ..default()
                        })
                        .insert(elemental_energy::ElementalEnergyPerSecondText);
                });
            //da button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        padding: UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::Px(10.),
                            bottom: Val::Px(10.),
                        },
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "Create Elemental Energy",
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
            //Right side box
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        height: Val::Percent(100.0),

                        border: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(2.0),
                            bottom: Val::Px(2.0),
                        },

                        margin: UiRect {
                            right: Val::Px(10.),
                            ..default()
                        },

                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //upgrade buttons
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_wrap: FlexWrap::NoWrap,
                                height: Val::Percent(10.),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for i in upgrades {
                                parent.spawn(ButtonBundle { ..default() }).with_children(
                                    |parent| {
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
                                                format!("{}", i.label),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 20.0,
                                                    color: Color::srgb(0.9, 0.9, 0.9),
                                                },
                                            ),
                                            ..default()
                                        });
                                    },
                                );
                            }
                        });

                    //scrolling list
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,

                                align_items: AlignItems::Stretch,
                                align_content: AlignContent::FlexStart,
                                margin: UiRect {
                                    bottom: Val::Px(10.),
                                    ..default()
                                },
                                width: Val::Percent(100.),
                                height: Val::Percent(90.),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ScrollingList::default())
                        .with_children(|parent| {
                            for ele in elementals {
                                parent
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            flex_grow: 1.0,
                                            flex_basis: Val::Auto,
                                            display: Display::Flex,

                                            margin: UiRect {
                                                top: Val::Px(10.),
                                                ..default()
                                            },
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Px(32.0),
                                                    height: Val::Px(32.0),
                                                    flex_basis: Val::Px(32.),
                                                    margin: UiRect::top(Val::VMin(5.)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            UiImage::new(asset_server.load("icons/fire.png")),
                                        ));
                                        parent.spawn(NodeBundle {
                                            style: Style{
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            ..default()
                                        }).with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                style: Style {
                                                    flex_grow: 1.,
                                                    margin: UiRect {
                                                        left: Val::Px(10.),
                                                        right: Val::Px(10.),
                                                        top: Val::Px(10.),
                                                        bottom: Val::Px(10.),
                                                    },
                                                    ..default()
                                                },
                                                text: Text::from_section(
                                                    ele.get_label(),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::srgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                ..default()
                                            });
                                            parent.spawn(TextBundle {
                                                style: Style {
                                                    flex_grow: 1.,
                                                    margin: UiRect {
                                                        left: Val::Px(10.),
                                                        right: Val::Px(10.),
                                                        top: Val::Px(10.),
                                                        bottom: Val::Px(10.),
                                                    },
                                                    ..default()
                                                },
                                                text: Text::from_section(
                                                    format!("{}",ele.get_cost().to_str_radix(10)),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::srgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                ..default()
                                            });
                                            parent.spawn(TextBundle {
                                                style: Style {
                                                    flex_grow: 1.,
                                                    margin: UiRect {
                                                        left: Val::Px(10.),
                                                        right: Val::Px(10.),
                                                        top: Val::Px(10.),
                                                        bottom: Val::Px(10.),
                                                    },
                                                    ..default()
                                                },
                                                text: Text::from_section(
                                                    "0",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::srgb(0.9, 0.9, 0.9),
                                                    },
                                                ),
                                                ..default()
                                            }).insert(SummonEPSText);
                                        });
                                        
                                    })
                                    .insert(ele);
                            }
                        });
                });
        });
}

fn generate_elemental_list() -> Vec<Summon> {
    vec![
        Summon::FireElemental(Elemental {
            label: "Fire Elemental".to_string(),
            energy_per_second: BigInt::from(10),
            total_energy_per_second: BigInt::from(0),
            quantity: BigInt::from(0),
            cost: BigInt::from(25),
        }),
        Summon::AirElemental(Elemental {
            label: "Air Elemental".to_string(),
            energy_per_second: BigInt::from(20),
            total_energy_per_second: BigInt::from(0),
            quantity: BigInt::from(0),
            cost: BigInt::from(50),
        }),
        Summon::ElectricityElemental(Elemental {
            label: "Electricity Elemental".to_string(),
            energy_per_second: BigInt::from(30),
            total_energy_per_second: BigInt::from(0),
            quantity: BigInt::from(0),
            cost: BigInt::from(100),
        }),
    ]
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn mouse_scroll(
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

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            With<ElementalEnergyButton>,
        ),
    >,
    text_query: Query<&mut Text, With<ElementalEnergyButton>>,
    mut elemental_energy: ResMut<elemental_energy::ElementalEnergy>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Press".to_string();
                elemental_energy.amount += 1;
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
