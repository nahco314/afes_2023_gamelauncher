use bevy::prelude::*;
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use bevy::utils::HashMap;

pub struct Game {
    pub path: PathBuf,
    pub title: String,
    pub description: String,
    pub author: String,
    pub screenshot: Handle<Image>,
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
pub struct Games(pub Vec<Game>);
#[derive(Resource)]
pub struct SelectedIndex(pub u32);


pub fn load_game_folder(mut games: ResMut<Games>, asset_server: Res<AssetServer>) {
    let mut game_dir_map: HashMap<String, DirEntry> = HashMap::new();
    let mut root_dirs = [PathBuf::from("assets/games")].into_iter()
        .chain((0..).map(|i| PathBuf::from(format!("assets/games_{}", i))));

    for root in root_dirs {
        if !root.exists() {
            info!("stop in {:?}", root);
            break;
        }

        for d in fs::read_dir(root).unwrap().filter_map(|e| e.ok()) {
            let game_name = d.path().file_name().unwrap().to_os_string().into_string().unwrap();

            info!("found: {:?}", game_name);

            game_dir_map.insert(game_name, d);
        }
    }

    for (_game_name, d) in game_dir_map {
        let game_manifest = load_game_manifest(d.path());
        info!("{:?}", d);
        info!("{:?}", game_manifest);

        games.0.push(Game {
            path: d.path().join(game_manifest.game_exe_name),
            title: game_manifest.title,
            description: game_manifest.description,
            author: game_manifest.author,
            screenshot: asset_server.load(
                d.path()
                    .iter()
                    .skip(1) //skip "assets/"
                    .collect::<PathBuf>()
                    .join("screenshot.png"),
            ),
        });
    }
}

fn load_game_manifest<P: AsRef<Path>>(path: P) -> GameManifest {
    let manifest_file = path.as_ref().join("launcher_manifest.toml");
    let manifest_file_content = fs::read_to_string(manifest_file).unwrap();
    toml::from_str::<GameManifest>(&manifest_file_content).unwrap()
}

pub fn run_game(selected_idx: &SelectedIndex, games: &Games) {
    let abs_path = env::current_dir()
        .unwrap()
        .join(&games.0[selected_idx.0 as usize].path);
    let mut game_cmd = Command::new(&abs_path);
    game_cmd.current_dir(abs_path.parent().unwrap());
    game_cmd.spawn().expect("failed to run game");
}
