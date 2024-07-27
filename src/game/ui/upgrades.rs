use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::game::upgrades::{Upgrade, Upgrades};
use crate::ui::{interaction::InteractionPalette, palette::*};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_upgrades_ui).add_systems(
        Update,
        (add_new_upgrades, update_upgrade_text, mouse_scroll).chain(),
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
pub(crate) struct GlobalUpgradeIndex(pub(crate) usize);

#[derive(Component)]
pub(crate) struct UpgradeEntity {
    pub entity: Entity,
    pub index: usize,
}

#[derive(Component)]
struct UpgradeText;

#[derive(Bundle)]
struct UpgradeButtonBundle {
    button_bundle: ButtonBundle,
    interaction_palette: InteractionPalette,
}

impl UpgradeButtonBundle {
    pub fn new() -> Self {
        Self {
            button_bundle: ButtonBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    margin: UiRect {
                        bottom: Val::Percent(2.),
                        ..default()
                    },
                    padding: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        bottom: Val::Px(10.),
                    },
                    border: UiRect {
                        left: Val::Px(1.),
                        right: Val::Px(1.),
                        top: Val::Px(1.),
                        bottom: Val::Px(1.),
                    },
                    width: Val::Percent(100.),
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

impl UpgradeTextBundle {
    pub fn new(upgrade: &Upgrade) -> Self {
        Self {
            text_bundle: TextBundle::from_sections([
                TextSection::new(format!("{}\n", upgrade.name()), TextStyle::default()),
                TextSection::new(
                    format!("{}\n", upgrade.description()),
                    TextStyle {
                        font_size: 18.,
                        ..default()
                    },
                ),
                TextSection::new(upgrade.cost(), TextStyle::default()),
            ]),
            upgrade_text: UpgradeText,
            label: Label,
            accessibility_node: AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        }
    }
}

fn add_upgrade_list(parent: &mut ChildBuilder, entity: Entity, name: &Name, upgrades: &Upgrades) {
    // Container
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Header
            parent.spawn(TextBundle::from_section(name, TextStyle::default()));
            for (index, u) in upgrades.0.iter().enumerate() {
                // upgrade
                parent
                    .spawn((UpgradeButtonBundle::new(), UpgradeEntity { entity, index }))
                    .with_children(|parent| {
                        parent.spawn(UpgradeTextBundle::new(u));
                    });
            }
        });
}

fn spawn_upgrades_ui(
    _trigger: Trigger<SpawnUpgradesUi>,
    mut commands: Commands,
    global_upgrades: Res<Upgrades>,
) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(300.),
                height: Val::Percent(95.),
                ..default()
            },
            ..default()
        })
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
                            // List items
                            for (index, u) in global_upgrades.0.iter().enumerate() {
                                parent
                                    .spawn((UpgradeButtonBundle::new(), GlobalUpgradeIndex(index)))
                                    .with_children(|parent| {
                                        parent.spawn(UpgradeTextBundle::new(u));
                                    });
                            }
                            // non global upgrades now spawn on Add<Upgrades>
                        });
                });
        });
}

fn add_new_upgrades(
    mut commands: Commands,
    upgrade_list: Query<(Entity, &UpgradeList)>,
    upgrades: Query<(Entity, &Name, &Upgrades), Added<Upgrades>>,
) {
    let Ok((parent, _)) = upgrade_list.get_single() else {
        return;
    };
    for (entity, name, upgrades) in &upgrades {
        commands.entity(parent).with_children(|parent| {
            add_upgrade_list(parent, entity, name, upgrades);
        });
    }
}

fn update_upgrade_text(
    query_upgrades: Query<&Upgrades, Changed<Upgrades>>,
    global_upgrades: Res<Upgrades>,
    mut query_text: Query<(&Parent, &mut Text, &UpgradeText)>,
    query_parent_global: Query<&GlobalUpgradeIndex>,
    query_parent: Query<&UpgradeEntity>,
) {
    for (parent, mut text, _) in query_text.iter_mut() {
        let upgrade = if let Ok(index) = query_parent_global.get(parent.get()) {
            if !global_upgrades.is_changed() {
                continue;
            }
            global_upgrades.0.get(index.0)
        } else if let Ok(index) = query_parent.get(parent.get()) {
            query_upgrades
                .get(index.entity)
                .ok()
                .and_then(|upgrades| upgrades.0.get(index.index))
        } else {
            continue;
        };

        let Some(upgrade) = upgrade else {
            continue;
        };
        text.sections[0].value = format!("{}\n", upgrade.name());
        text.sections[1].value = format!("{}\n", upgrade.description());
        text.sections[2].value = upgrade.cost();
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
