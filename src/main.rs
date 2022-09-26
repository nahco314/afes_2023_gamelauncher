use bevy::{asset::AssetServerSettings, prelude::*};
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
        .add_plugins(DefaultPlugins)
        .insert_resource(Games(Vec::new()))
        .add_startup_system(load_game_folder)
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
                    .into_iter()
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
