use bevy::prelude::*;
use crate::game::prelude::*;

pub fn spawn_start_ui(
    commands: &mut Commands,
) {
    commands.spawn((
            Sprite::from_color(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(RIGHT_WALL - LEFT_WALL + 200.0, TOP_WALL - BOTTOM_WALL + 200.0)),
            Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
            StartUi,
        ));

    // title text
    commands.spawn((
        Text::new("Breakout Game\n"),
        TextFont {
            font_size: START_UI_TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(TITLE_TEXT_COLOR),
        StartUi,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(20.0),
            ..default()
        },
        children![(
            TextSpan::new("Press Space to Start"),
            TextFont {
                font_size: START_UI_TITLE_FONT_SIZE / 4.0,
                ..default()
            },
            TextColor(TITLE_TEXT_COLOR),
        )]
    ));
}