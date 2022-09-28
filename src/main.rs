use bevy::{prelude::*, ui::FocusPolicy};
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

struct Game {
    path: PathBuf,
    title: String,
    description: String,
    author: String,
    screenshot: Handle<Image>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GameSummary {
    title: String,
    description: String,
    author: String,
}

struct Games(Vec<Game>);
struct SelectedIndex(u32);

#[derive(Component)]
struct GameIndex(u32);

#[derive(Component)]
struct GameTitleText;

#[derive(Component)]
struct GameDescriptionText;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1600.,
            height: 900.,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(Games(Vec::new()))
        .insert_resource(SelectedIndex(0))
        .add_startup_system(load_game_folder)
        .add_startup_system(ui_setup.after(load_game_folder))
        .add_system(ui_selected_color_system)
        .add_system(select_by_keybord)
        .add_system(select_by_cursor)
        .add_system(run_by_keybord_sys)
        .add_system(play_button_sys)
        .add_system(update_title_text)
        .add_system(update_desc_text)
        .run();
}

fn load_game_folder(mut games: ResMut<Games>, asset_server: Res<AssetServer>) {
    for d in fs::read_dir("assets/games").unwrap().filter_map(|e| e.ok()) {
        info!("{:?}", d.path());
        let game_summary = get_game_summary(d.path());
        info!("{:?}", game_summary);
        games.0.push(Game {
            path: d.path().join("game.exe"),
            title: game_summary.title,
            description: game_summary.description,
            author: game_summary.author,
            screenshot: asset_server.load(
                d.path()
                    .iter()
                    .skip(1)
                    .collect::<PathBuf>() //skip "assets/"
                    .join("screenshot.png"),
            ),
        });
    }
}

fn get_game_summary<P: AsRef<Path>>(path: P) -> GameSummary {
    let summary_file = path.as_ref().join("launcher_summary.toml");
    let summary_file_content = fs::read_to_string(summary_file).unwrap();
    toml::from_str::<GameSummary>(&summary_file_content).unwrap()
}

fn ui_setup(mut cmd: Commands, asset_server: Res<AssetServer>, games: Res<Games>) {
    cmd.spawn_bundle(Camera2dBundle::default());
    cmd.spawn_bundle(
        //root node
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        },
    )
    .with_children(|p| {
        //games label
        p.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Px(320.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgb(0.28, 0.38, 0.57).into(),
            ..Default::default()
        })
        .with_children(|p| {
            //label title
            p.spawn_bundle(
                TextBundle::from_section(
                    "Games",
                    TextStyle {
                        font_size: 35.,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(35.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
            //game *cards* node
            p.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|p| {
                //game card node
                p.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 1.0,
                        max_size: Size::new(Val::Undefined, Val::Undefined),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|p| {
                    for (idx, game) in games.0.iter().enumerate() {
                        //game card node
                        p.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Px(25.0)),
                                ..default()
                            },
                            color: Color::rgb(0.54, 0.73, 0.8).into(),
                            ..default()
                        })
                        .with_children(|p| {
                            //game title
                            p.spawn_bundle({
                                let mut tmp = TextBundle::from_section(
                                    game.title.to_owned(),
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/NotoSansCJKjp-DemiLight.otf"),
                                        font_size: 25.,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    flex_shrink: 0.,
                                    size: Size::new(Val::Undefined, Val::Px(25.)),
                                    margin: UiRect {
                                        left: Val::Px(5.0),
                                        right: Val::Auto,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                                tmp.focus_policy = FocusPolicy::Pass;
                                tmp
                            });
                        })
                        .insert(GameIndex(idx as u32))
                        .insert(Interaction::default());
                    }
                });
            });
        });
    })
    .with_children(|p| {
        //game detail root node
        p.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|p| {
            //title
            p.spawn_bundle(
                TextBundle::from_section(
                    games.0[0].title.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                        font_size: 100.,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(20.),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .insert(GameTitleText);
        })
        .with_children(|p| {
            //description
            p.spawn_bundle(
                TextBundle::from_section(
                    games.0[0].description.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(20.),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .insert(GameDescriptionText);
        });
    })
    .with_children(|p| {
        //play button
        p.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(60.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(30.),
                    right: Val::Px(35.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::LIME_GREEN.into(),
            ..Default::default()
        })
        .with_children(|p| {
            // "プレイ" text
            p.spawn_bundle(TextBundle::from_section(
                "プレイ",
                TextStyle {
                    font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                    font_size: 40.,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

fn ui_selected_color_system(
    selected_idx: Res<SelectedIndex>,
    mut ui_games: Query<(&mut UiColor, &GameIndex)>,
) {
    for (mut color, idx) in ui_games.iter_mut() {
        *color = if idx.0 == selected_idx.0 {
            Color::rgb(0.44, 0.63, 0.7).into()
        } else {
            Color::rgb(0.54, 0.73, 0.8).into()
        }
    }
}

fn select_by_keybord(
    key_input: Res<Input<KeyCode>>,
    mut selected_idx: ResMut<SelectedIndex>,
    games: Res<Games>,
) {
    if key_input.just_pressed(KeyCode::Up) && selected_idx.0 != 0 {
        selected_idx.0 -= 1;
    }

    if key_input.just_pressed(KeyCode::Down) && selected_idx.0 != games.0.len() as u32 - 1 {
        selected_idx.0 += 1;
    }
}

fn select_by_cursor(
    query: Query<(&Interaction, &GameIndex), Changed<Interaction>>,
    mut selected_idx: ResMut<SelectedIndex>,
) {
    for e in query.iter() {
        if *e.0 == Interaction::Clicked {
            selected_idx.0 = e.1 .0;
        }
    }
}

fn run_by_keybord_sys(
    key_input: Res<Input<KeyCode>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    if key_input.just_pressed(KeyCode::Return) {
        run_game(&selected_idx, &games);
    }
}

fn run_game(selected_idx: &SelectedIndex, games: &Games) {
    let abs_path = env::current_dir()
        .unwrap()
        .join(&games.0[selected_idx.0 as usize].path);
    let mut game_cmd = Command::new(&abs_path);
    game_cmd.current_dir(abs_path.parent().unwrap());
    game_cmd.spawn().expect("failed run game");
}

fn play_button_sys(
    q: Query<(&Interaction,), (Changed<Interaction>, With<Button>)>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    if let Ok(q) = q.get_single() {
        if *q.0 == Interaction::Clicked {
            run_game(&selected_idx, &games);
        }
    }
}

fn update_title_text(
    mut title_text: Query<(&mut Text,), With<GameTitleText>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
    asset_server: Res<AssetServer>,
) {
    title_text.single_mut().0.sections = vec![TextSection {
        value: games.0[selected_idx.0 as usize].title.clone(),
        style: TextStyle {
            font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
            font_size: 100.,
            color: Color::WHITE,
        },
    }]
}

fn update_desc_text(
    mut desc_text: Query<(&mut Text,), With<GameDescriptionText>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
    asset_server: Res<AssetServer>,
) {
    desc_text.single_mut().0.sections = vec![TextSection {
        value: games.0[selected_idx.0 as usize].description.clone(),
        style: TextStyle {
            font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
            font_size: 50.,
            color: Color::WHITE,
        },
    }]
}
