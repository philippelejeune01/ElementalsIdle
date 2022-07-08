use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    winit::WinitSettings, ui,
};
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

struct ElementalEnergy(u128);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ElementalEnergy(0))
        .add_plugin(HelloPlugin)
        .run();
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut amount: ResMut<ElementalEnergy>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                amount.0+=1;
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
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
fn energy_system(energy: Res<ElementalEnergy>, mut query: Query<&mut Text, With<ElementalEnergyText>>) {
    for mut text in query.iter_mut() {
       text.sections[0].value = format!("{:+e}", energy.0);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    //root node
    commands.spawn_bundle(NodeBundle{
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(ButtonBundle {
            style: Style{
                size : Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin : Rect::all(Val::Auto),
                justify_content : JustifyContent::Center,
                align_items : AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            //da button
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9,0.9,0.9),
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
        //Upgrades box
        parent.spawn_bundle(NodeBundle {
            style: Style{
                size: Size::new(Val::Percent(25.0), Val::Percent(95.0)),
                border: Rect::all(Val::Px(2.0)),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                overflow: Overflow::Hidden,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            //scrolling list
            parent.spawn_bundle(NodeBundle {
                style: Style{
                    flex_direction: FlexDirection::ColumnReverse,
                    flex_grow: 1.0,
                    max_size: Size::new(Val::Undefined, Val::Undefined),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .insert(ScrollingList::default())
            .with_children(|parent|{
                for i in 1..25 {
                    parent.spawn_bundle(TextBundle{
                        style: Style{
                            margin: Rect::all(Val::Px(10.0)),
                            ..default()
                        },
                        text: Text::with_section(
                            "Test1",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),

                            },
                            Default::default(),
                        ),
                        ..default()
                    });

                }
            });
        });
        parent.spawn_bundle(TextBundle{
            style: Style {
                ..default()
            },
            text: Text::with_section("0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
            ),
        ..default()
        })
        .insert(ElementalEnergyText);
    });
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(mouse_scroll)
        .add_system(energy_system)
        .add_system(button_system);
    }
}