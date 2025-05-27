use bevy::{color::palettes::basic::*, prelude::*};

#[derive(Component)]
struct MyMusic;

#[derive(Component)]
struct ActiveNoteDo;
#[derive(Component)]
struct ActiveNoteRe;
#[derive(Component)]
struct ActiveNoteMi;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, set_initial_volume) 
        .add_systems(Update, button_system) // button stuff
        .add_systems(Update, volume) // audio stuff
        .run();
}

fn set_initial_volume(
    music_controller: Query<&AudioSink, With<MyMusic>>,
) {
    for sink in music_controller.iter() {
        sink.set_volume(0.0); // Set the volume to 0
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.96, 0.94, 0.90);  // Milky white color
const HOVERED_BUTTON: Color = Color::srgb(0.96, 0.96, 0.86); // Beige color
const PRESSED_BUTTON: Color = Color::srgb(0.85, 0.80, 0.65); // Darker beige color

fn spawn_piano_key(
    parent: &mut ChildBuilder,
    name: impl Into<String>,
    style: Node,
    color: Color,
) {
    parent.spawn((
        Button,
        style.clone(),
        BorderColor(Color::BLACK),
        BackgroundColor(color),
        Name::new(name.into()),
    ));
}

fn setup(mut commands: Commands) {

    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            // Keyboard container: width = 3 white keys * 80px = 240px
            parent.spawn(Node {
                width: Val::Px(240.0),
                height: Val::Px(260.0),
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|keyboard| {
                let key_style = Node {
                    width: Val::Px(80.0),
                    height: Val::Px(260.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                };

                // White keys (spawn first!) 
                spawn_piano_key(keyboard, "do", key_style.clone(), NORMAL_BUTTON);
                spawn_piano_key(keyboard, "re", key_style.clone(), NORMAL_BUTTON);
                spawn_piano_key(keyboard, "mi", key_style, NORMAL_BUTTON);

                // Black keys (spawn after, so they're on top)
                // First black key (between "do" and "re")
                keyboard.spawn(Node {
                    width: Val::Px(50.0),
                    height: Val::Px(140.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(55.0),
                    bottom: Val::Px(120.0),
                    ..default()
                })
                .insert(BackgroundColor(Color::BLACK));

                // Second black key (between "re" and "mi")
                keyboard.spawn(Node {
                    width: Val::Px(50.0),
                    height: Val::Px(140.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(135.0),
                    bottom: Val::Px(120.0),
                    ..default()
                })
                .insert(BackgroundColor(Color::BLACK));
            });
        });
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Name, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
    active_note_query: Query<(Entity, &AudioSink), With<ActiveNoteDo>>,
    active_note_re_query: Query<(Entity, &AudioSink), With<ActiveNoteRe>>,
    active_note_mi_query: Query<(Entity, &AudioSink), With<ActiveNoteMi>>,
) {
    for (name, interaction, mut color, mut border_color) in &mut interaction_query {
        match (name.as_str(), *interaction) {
            ("do", Interaction::Pressed) => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                if active_note_query.iter().next().is_none() {
                    let sound: Handle<AudioSource> = asset_server.load("note_do.ogg");
                    commands.spawn((
                        bevy::audio::AudioPlayer::new(sound),
                        PlaybackSettings::DESPAWN,
                        ActiveNoteDo,
                    ));
                }
            }
            ("do", Interaction::Hovered) | ("do", Interaction::None) => {
                *color = if *interaction == Interaction::Hovered {
                    HOVERED_BUTTON.into()
                } else {
                    NORMAL_BUTTON.into()
                };
                border_color.0 = if *interaction == Interaction::Hovered {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                for (_entity, sink) in active_note_query.iter() {
                    sink.stop();
                }
            }
            ("re", Interaction::Pressed) => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                if active_note_re_query.iter().next().is_none() {
                    let sound: Handle<AudioSource> = asset_server.load("note_re.ogg");
                    commands.spawn((
                        bevy::audio::AudioPlayer::new(sound),
                        PlaybackSettings::DESPAWN,
                        ActiveNoteRe,
                    ));
                }
            }
            ("re", Interaction::Hovered) | ("re", Interaction::None) => {
                *color = if *interaction == Interaction::Hovered {
                    HOVERED_BUTTON.into()
                } else {
                    NORMAL_BUTTON.into()
                };
                border_color.0 = if *interaction == Interaction::Hovered {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                for (_entity, sink) in active_note_re_query.iter() {
                    sink.stop();
                }
            }
            ("mi", Interaction::Pressed) => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                if active_note_mi_query.iter().next().is_none() {
                    let sound: Handle<AudioSource> = asset_server.load("note_mi.ogg");
                    commands.spawn((
                        bevy::audio::AudioPlayer::new(sound),
                        PlaybackSettings::DESPAWN,
                        ActiveNoteMi,
                    ));
                }
            }
            ("mi", Interaction::Hovered) | ("mi", Interaction::None) => {
                *color = if *interaction == Interaction::Hovered {
                    HOVERED_BUTTON.into()
                } else {
                    NORMAL_BUTTON.into()
                };
                border_color.0 = if *interaction == Interaction::Hovered {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                for (_entity, sink) in active_note_mi_query.iter() {
                    sink.stop();
                }
            }
            _ => {}
        }
    }
}

fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MyMusic>>,
) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::Equal) {
            if sink.volume() < 4.9 {
                sink.set_volume(sink.volume() + 0.1);
            }
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            if sink.volume() > 0.0 {
                sink.set_volume(sink.volume() - 0.1);
            }
        }
    }

    if let Ok(sink) = music_controller.get_single() {
        println!("Volume: {:.1}", sink.volume()); // print the volume rounded to 1 decimal place
    }
}