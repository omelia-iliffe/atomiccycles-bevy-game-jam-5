use crate::game::movement::Revolve;
use crate::game::spawn::atom::{Electron, Ring};
use crate::game::upgrades::costs::{
    compute_cycle_cost, compute_electron_cost, compute_ring_cost, compute_speed_cost,
};
use crate::game::upgrades::BuyNextRing;
use crate::game::upgrades::{BuyElectron, CycleUpgrade, SpeedUpgrade, INITIAL_REVOLVE_SPEED};
use crate::screen::Screen;
use crate::ui::{interaction::InteractionPalette, palette::*};
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_upgrades_ui).add_systems(
        Update,
        (
            add_new_upgrades,
            update_cycle_upgrades,
            update_electron_upgrades,
            update_speed_upgrades,
            mouse_scroll,
        )
            .chain(),
    );

    #[cfg(feature = "dev")]
    {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            .add_systems(Update, toggle_overlay);
    }
}

#[derive(Event, Debug)]
pub struct SpawnUpgradesUi;

#[derive(Component)]
pub struct UpgradeList;

#[derive(Component)]
struct UpgradeText;

#[derive(Bundle)]
struct UpgradeButtonBundle {
    button_bundle: ButtonBundle,
    interaction_palette: InteractionPalette,
}

impl UpgradeButtonBundle {
    pub fn new(width: f32) -> Self {
        Self {
            button_bundle: ButtonBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(4.),
                        right: Val::Px(4.),
                        top: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    width: Val::Percent(width),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            interaction_palette: InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        }
    }
}

#[derive(Bundle)]
struct UpgradeTextBundle {
    text_bundle: TextBundle,
    upgrade_text: UpgradeText,
    label: Label,
    accessibility_node: AccessibilityNode,
}

fn spawn_upgrades_ui(_trigger: Trigger<SpawnUpgradesUi>, mut commands: Commands) {
    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(400.),
                    height: Val::Percent(95.),
                    ..default()
                },
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                TextBundle::from_section("Upgrades", TextStyle::default()),
                Label,
            ));
            // List with hidden overflow
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        height: Val::Percent(80.),
                        overflow: Overflow::clip_y(),
                        margin: UiRect {
                            left: Val::Percent(5.),
                            right: Val::Percent(5.),
                            top: Val::Percent(10.),
                            bottom: Val::Percent(10.),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Moving panel
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            UpgradeList,
                            ScrollingList::default(),
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        ))
                        .with_children(|parent| {
                            //     // List items
                            //     // TODO: Add ring one here
                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        width: Val::Percent(100.),
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    let cost = compute_ring_cost(0);
                                    parent
                                        .spawn((UpgradeButtonBundle::new(100.), BuyNextRing))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_sections([
                                                    TextSection::new(
                                                        "Buy Ring",
                                                        TextStyle {
                                                            font_size: 18.,
                                                            ..default()
                                                        },
                                                    ),
                                                    TextSection::new(
                                                        "\nCost: ",
                                                        TextStyle {
                                                            font_size: 18.,
                                                            ..default()
                                                        },
                                                    ),
                                                    TextSection::new(
                                                        cost.to_string(),
                                                        TextStyle {
                                                            font_size: 18.,
                                                            ..default()
                                                        },
                                                    ),
                                                ])
                                                .with_text_justify(JustifyText::Center),
                                                UpgradeText,
                                                Label,
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                        });
                                });
                        });
                });
        });
}

fn add_new_upgrades(
    mut commands: Commands,
    query_list: Query<Entity, With<UpgradeList>>,
    query_ring: Query<(Entity, &Ring), Added<Ring>>,
) {
    let Ok(parent) = query_list.get_single() else {
        return;
    };
    for (entity, ring) in &query_ring {
        commands.entity(parent).with_children(|parent| {
            let title = format!("Ring {}", ring.index + 1);
            parent
                .spawn((
                    Name::new(title.clone()),
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Header
                    parent.spawn(TextBundle {
                        text: Text::from_section(title, TextStyle::default()),
                        style: Style {
                            width: Val::Percent(100.),
                            ..default()
                        },
                        ..Default::default()
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Buy electron
                            let electron_cost = compute_electron_cost(ring.index, 0);
                            parent
                                .spawn((UpgradeButtonBundle::new(38.), BuyElectron(entity)))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_sections([
                                            TextSection::new(
                                                "Electrons\n",
                                                TextStyle {
                                                    font_size: 18.,

                                                    ..default()
                                                },
                                            ),
                                            TextSection::new("2", TextStyle::default()),
                                            TextSection::new(
                                                "\nCost: ",
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                            TextSection::new(
                                                electron_cost.to_string(),
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                        ])
                                        .with_text_justify(JustifyText::Center),
                                        UpgradeText,
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                            // SPEED upgrade
                            let speed_cost = compute_speed_cost(INITIAL_REVOLVE_SPEED);
                            parent
                                .spawn((UpgradeButtonBundle::new(30.), SpeedUpgrade(entity)))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_sections([
                                            TextSection::new(
                                                "Speed\n",
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                            TextSection::new("2", TextStyle::default()),
                                            TextSection::new(
                                                "\nCost: ",
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                            TextSection::new(
                                                format!("{:.2}", speed_cost),
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                        ])
                                        .with_text_justify(JustifyText::Center),
                                        UpgradeText,
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                            // CYCLE
                            let cycle_cost = compute_cycle_cost(None);
                            parent
                                .spawn((UpgradeButtonBundle::new(30.), CycleUpgrade(entity)))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_sections([
                                            TextSection::new(
                                                "Cycles\n",
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                            TextSection::new("".to_string(), TextStyle::default()),
                                            TextSection::new(
                                                "s",
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                            TextSection::new(
                                                "\nCost: ",
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                            TextSection::new(
                                                cycle_cost.to_string(),
                                                TextStyle {
                                                    font_size: 18.,
                                                    ..default()
                                                },
                                            ),
                                        ])
                                        .with_text_justify(JustifyText::Center),
                                        UpgradeText,
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(2.),
                                    height: Val::Percent(100.),
                                    ..default()
                                },
                                background_color: BackgroundColor(BUTTON_HOVERED_BACKGROUND),
                                ..default()
                            });
                        });
                });
        });
    }
}

fn update_speed_upgrades(
    query_ring: Query<(Entity, &Revolve), Changed<Revolve>>,
    query_upgrade: Query<(&SpeedUpgrade, &Children)>,
    mut query_upgrade_text: Query<&mut Text, With<UpgradeText>>,
) {
    for (entity, revolve) in &query_ring {
        let cost = compute_speed_cost(revolve.speed());

        let Some(upgrade_entity) = query_upgrade
            .iter()
            .find(|(upgrade, _)| upgrade.0 == entity)
            .map(|(_, children)| children[0])
        else {
            continue;
        };

        let Ok(mut text) = query_upgrade_text.get_mut(upgrade_entity) else {
            continue;
        };

        text.sections[1].value = format!("{:.2}", revolve.speed());
        text.sections[3].value = format!("{}", cost);
        log::info!("Speed: {}, Cost: {}", revolve.speed(), cost);
    }
}

fn update_cycle_upgrades(
    query_ring: Query<(Entity, &Ring), Changed<Ring>>,
    query_upgrade: Query<(&CycleUpgrade, &Children)>,
    mut query_upgrade_text: Query<&mut Text, With<UpgradeText>>,
) {
    for (entity, ring) in &query_ring {
        let duration = ring.cycle_timer.as_ref().map(|t| t.duration());
        let cost = compute_cycle_cost(duration);

        let Some(upgrade_entity) = query_upgrade
            .iter()
            .find(|(upgrade, _)| upgrade.0 == entity)
            .map(|(_, children)| children[0])
        else {
            continue;
        };

        let Ok(mut text) = query_upgrade_text.get_mut(upgrade_entity) else {
            continue;
        };

        text.sections[1].value = match duration {
            Some(duration) => {
                format!("{:.2}", duration.as_secs_f32())
            }
            None => "".to_string(),
        };

        text.sections[4].value = format!("{}", cost);
    }
}

fn update_electron_upgrades(
    query_added_electron: Query<&Parent, Added<Electron>>,
    query_electrons: Query<Entity, With<Electron>>,
    query_ring: Query<(&Ring, Option<&Children>)>,
    query_upgrade: Query<(&BuyElectron, &Children)>,
    mut query_upgrade_text: Query<&mut Text, With<UpgradeText>>,
) {
    for added_electron_parent in &query_added_electron {
        let Ok((ring, children)) = query_ring.get(added_electron_parent.get()) else {
            continue;
        };

        let electron_count = {
            children
                .map(|children| {
                    children
                        .iter()
                        .filter_map(|child| query_electrons.get(*child).ok())
                        .count()
                })
                .unwrap_or_default()
        };

        let cost = compute_electron_cost(ring.index, electron_count);

        let Some(upgrade_entity) = query_upgrade
            .iter()
            .find(|(upgrade, _)| upgrade.0 == added_electron_parent.get())
            .map(|(_, children)| children[0])
        else {
            continue;
        };

        let Ok(mut text) = query_upgrade_text.get_mut(upgrade_entity) else {
            continue;
        };

        text.sections[1].value = format!("{}", electron_count);
        text.sections[3].value = format!("{}", cost);
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

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

#[cfg(feature = "dev")]
// The system that will enable/disable the debug outlines around the nodes
fn toggle_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut options: ResMut<bevy::dev_tools::ui_debug_overlay::UiDebugOptions>,
) {
    info_once!("The debug outlines are enabled, press Space to turn them on/off");
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug_overlay if disabled and disable if enabled
        options.toggle();
    }
}
