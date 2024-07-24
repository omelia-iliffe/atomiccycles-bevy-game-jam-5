//! Displays the Cycle Count as text

use bevy::prelude::*;

use crate::{game::cycles::CycleCount, ui::palette::BUTTON_TEXT};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_cycle_ui);
    app.add_systems(Update, update_cycle_count_text);
}

#[derive(Event, Debug)]
pub struct SpawnCycleUi;

fn spawn_cycle_ui(_trigger: Trigger<SpawnCycleUi>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    // commands.trigger(SpawnPlayer);

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Cycles: ",
                TextStyle {
                    font_size: 40.0,
                    color: BUTTON_TEXT,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: BUTTON_TEXT,
                ..default()
            }),
        ]) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        CycleCountText,
    ));
}

#[derive(Component)]
pub struct CycleCountText;

fn update_cycle_count_text(
    count: Res<CycleCount>,
    mut query: Query<&mut Text, With<CycleCountText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", count.0);
    }
}
