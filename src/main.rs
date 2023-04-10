#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui_setup;
mod ui_sys;

use bevy::{prelude::*, window::WindowResizeConstraints, winit::WinitSettings};
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const SELECTED_GAME_TITLE_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.53, 0.65, 0.73));
const NORMAL_GAME_TITLE_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.20, 0.24, 0.26));
const GAME_TITLE_COLOR_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.2, 0.41, 0.52));
const GAMES_LAVEL_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.20, 0.4, 0.40));
const TEXT_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
const BUTTON_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.12, 0.76, 0.12));
const BUTTON_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.25, 0.82, 0.25));
const GAMES_LAVEL_WIDTH: f32 = 360.;
const GAME_DESC_TEXT_WIDTH: f32 = 900.;
const GAME_AUTHOR_TEXT_WIDTH: f32 = 650.;

struct Game {
    path: PathBuf,
    title: String,
    description: String,
    author: String,
    screenshot: Handle<Image>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GameManifest {
    title: String,
    description: String,
    author: String,
    game_exe_name: String,
}

#[derive(Resource)]
struct Games(Vec<Game>);
#[derive(Resource)]
struct SelectedIndex(u32);

#[derive(Component)]
struct GameIndex(u32);

#[derive(Component)]
struct GameTitleText;

#[derive(Component)]
struct GameDescriptionText;

#[derive(Component)]
struct GameAuthorText;

#[derive(Component)]
struct GameScreenShot;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1600.,
                height: 900.,
                resizable: true,
                title: "Afes 2022 Game Launcher".to_owned(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 1280.,
                    min_height: 680.,
                    max_height: f32::INFINITY,
                    max_width: f32::INFINITY,
                },
                scale_factor_override: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(Games(Vec::new()))
        .insert_resource(SelectedIndex(0))
        .add_startup_system(load_game_folder)
        .add_startup_system(ui_setup::setup.after(load_game_folder))
        .add_system(select_by_keybord)
        .add_system(select_by_cursor)
        .add_system(run_by_keybord_sys)
        .add_system(ui_sys::play_button_sys)
        .add_system(ui_sys::update_title_text)
        .add_system(ui_sys::update_desc_text)
        .add_system(ui_sys::update_author_text)
        .add_system(ui_sys::update_screenshot)
        .add_system(ui_sys::game_titles_ui_sys)
        .add_system(ui_sys::fit_screenshot)
        .run();
}

fn load_game_folder(mut games: ResMut<Games>, asset_server: Res<AssetServer>) {
    for d in fs::read_dir("assets/games").unwrap().filter_map(|e| e.ok()) {
        info!("{:?}", d.path());
        let game_manifest = get_game_manifest(d.path());
        info!("{:?}", game_manifest);
        games.0.push(Game {
            path: d.path().join(game_manifest.game_exe_name),
            title: game_manifest.title,
            description: game_manifest.description,
            author: game_manifest.author,
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

fn get_game_manifest<P: AsRef<Path>>(path: P) -> GameManifest {
    let manifest_file = path.as_ref().join("launcher_manifest.toml");
    let manifest_file_content = fs::read_to_string(manifest_file).unwrap();
    toml::from_str::<GameManifest>(&manifest_file_content).unwrap()
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
