use crate::{
    GameAuthorText, GameDescriptionText, GameIndex, GameScreenShot, GameTitleText, Games,
    SelectedIndex, TextBg,
};
use bevy::prelude::*;

pub fn update_title_text(
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
            color: super::TEXT_COLOR,
        },
    }]
}

pub fn update_desc_text(
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
            color: super::TEXT_COLOR,
        },
    }]
}

pub fn update_author_text(
    mut author_text: Query<(&mut Text,), With<GameAuthorText>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
    asset_server: Res<AssetServer>,
) {
    author_text.single_mut().0.sections = vec![TextSection {
        value: games.0[selected_idx.0 as usize].author.clone(),
        style: TextStyle {
            font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
            font_size: 40.,
            color: super::TEXT_COLOR,
        },
    }]
}

pub fn update_screenshot(
    mut image: Query<(&mut UiImage,), With<GameScreenShot>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    image.single_mut().0 .0 = games.0[selected_idx.0 as usize].screenshot.clone();
}

pub fn play_button_sys(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    if let Ok((itr, mut col)) = q.get_single_mut() {
        if *itr == Interaction::Hovered {
            *col = super::BUTTON_HOVER;
        } else {
            *col = super::BUTTON_COLOR;
            if *itr == Interaction::Clicked {
                crate::run_game(&selected_idx, &games);
            }
        }
    }
}

pub fn game_titles_ui_sys(
    mut q: Query<(&Interaction, &mut BackgroundColor, &GameIndex), With<GameIndex>>,
    selected_idx: Res<SelectedIndex>,
) {
    for (itr, mut col, idx) in q.iter_mut() {
        *col = if selected_idx.0 == idx.0 {
            super::SELECTED_GAME_TITLE_COLOR
        } else if *itr == Interaction::Hovered {
            super::GAME_TITLE_COLOR_HOVER
        } else {
            super::NORMAL_GAME_TITLE_COLOR
        }
    }
}

pub fn fit_screenshot(
    mut q: Query<(&mut Style,), With<GameScreenShot>>,
    window: Res<Windows>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
    assets: Res<Assets<Image>>,
) {
    let Ok((mut style,)) = q.get_single_mut() else { return; };
    let window = window.get_primary().unwrap();
    let screenshothandle = games.0[selected_idx.0 as usize].screenshot.clone();
    let Some(screenshot) = assets.get(&screenshothandle) else { return; };
    //aspect ratio = x / y
    let screenshot_ratio = {
        let size = screenshot.size();
        size.x / size.y
    };

    let window_ratio = { (window.width() - super::GAMES_LAVEL_WIDTH) / window.height() };

    style.size = if window_ratio > screenshot_ratio {
        Size::new(Val::Percent(100.), Val::Auto)
    } else {
        Size::new(Val::Auto, Val::Percent(100.))
    };
}

pub fn update_text_bg(
    mut q: Query<(&mut Style, &Parent), With<TextBg>>,
    calc_sizes: Query<(&CalculatedSize,), With<Text>>,
) {
    for (mut style, parent) in q.iter_mut() {
        let (parent_style,) = calc_sizes.get(parent.get()).unwrap();
        style.size = parent_style.size;
    }
}
