use bevy::{prelude::Color, ui::BackgroundColor};

pub mod bevy_system;
pub mod setup;

const SELECTED_GAME_TITLE_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.53, 0.65, 0.73));
const NORMAL_GAME_TITLE_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.20, 0.24, 0.26));
const GAME_TITLE_COLOR_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.2, 0.41, 0.52));
const GAMES_LAVEL_COLOR: BackgroundColor = BackgroundColor(Color::rgb(0.20, 0.4, 0.40));
const TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const BUTTON_COLOR_NORMAL: BackgroundColor = BackgroundColor(Color::rgb(0.12, 0.76, 0.12));
const BUTTON_COLOR_HOVER: BackgroundColor = BackgroundColor(Color::rgb(0.25, 0.82, 0.25));
const TEXT_BG_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.55,
});
const GAMES_LAVEL_WIDTH: f32 = 360.;
const GAME_DESC_WIDTH_MAX: f32 = 900.;
const GAME_AUTHOR_WIDTH_MAX: f32 = 650.;
