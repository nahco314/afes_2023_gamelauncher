use bevy::{prelude::*, text::TextStyle, ui::BackgroundColor};

pub mod bevy_system;
pub mod setup;

const GAME_CARD_COLOR_SELECTED: BackgroundColor = BackgroundColor(Color::rgb(0.53, 0.65, 0.73));
const GAME_CARD_COLOR_NORMAL: BackgroundColor = BackgroundColor(Color::rgb(0.20, 0.24, 0.26));
const GAME_CARD_COLOR_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.2, 0.41, 0.52));
const GAMES_LAVEL_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.20, 0.4, 0.40));
const BUTTON_COLOR_NORMAL: BackgroundColor = BackgroundColor(Color::rgb(0.12, 0.76, 0.12));
const BUTTON_COLOR_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.25, 0.82, 0.25));
const GAMES_LAVEL_TITLE: &str = "マウスか上下方向キーで選択";

const GAMES_LAVEL_WIDTH: f32 = 330.;
const DESCRIPTION_WIDTH_MIN: f32 = 600.;

const AUTHOR_NAME_TEXT_SIZE: f32 = 40.;
const GAME_TITLE_FONT_SIZE_RATIO: f32 = 0.065;
const DESCRIPTION_FONT_SIZE_RATIO: f32 = 0.045;

#[derive(Component)]
pub struct GameIndex(pub u32);

#[derive(Component)]
pub struct GameTitleText;

#[derive(Component)]
pub struct GameDescriptionText;

#[derive(Component)]
pub struct GameAuthorText;

#[derive(Component)]
pub struct GameScreenShot;

#[derive(Component)]
pub struct TextBg;

fn create_text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
        font_size,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
