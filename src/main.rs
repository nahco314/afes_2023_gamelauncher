use bevy::prelude::*;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
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
        .add_startup_system(load_game_folder)
        .add_startup_system(ui_setup.after(load_game_folder))
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

fn ui_setup(mut cmd: Commands, asset_server: Res<AssetServer>,games:Res<Games>) {
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
                    for i in /*1..18*/ games.0.iter() {
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
                            p.spawn_bundle(
                                TextBundle::from_section(
                                    i.title.to_owned(),
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
                                }),
                            );
                        });
                    }
                });
            });
        });
    });
}
