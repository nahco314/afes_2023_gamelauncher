use bevy::ecs::schedule::ShouldRun;

use super::*;
use crate::core::{Games, SelectedIndex};

pub fn update_title_text(
    mut title_text: Query<(&mut Text,), With<GameTitleText>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
    asset_server: Res<AssetServer>,
) {
    title_text.single_mut().0.sections = vec![TextSection {
        value: games.0[selected_idx.0 as usize].title.clone(),
        style: create_text_style(GAME_TITLE_TEXT_SIZE, &asset_server),
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
        style: create_text_style(DESCRIPTION_TEXT_SIZE, &asset_server),
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
        style: create_text_style(AUTHOR_NAME_TEXT_SIZE, &asset_server),
    }]
}

pub fn update_screenshot(
    mut background_screen_shot: Query<(&mut UiImage,), With<GameScreenShot>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    background_screen_shot.single_mut().0 .0 = games.0[selected_idx.0 as usize].screenshot.clone();
}

pub fn handle_play_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    let Ok((interaction, mut color)) = q.get_single_mut() else { return; /* Changed<Interaction> didn't hit */ };
    if *interaction == Interaction::Hovered {
        *color = BUTTON_COLOR_HOVER;
    } else {
        *color = BUTTON_COLOR_NORMAL;
        if *interaction == Interaction::Clicked {
            crate::core::run_game(&selected_idx, &games);
        }
    }
}

pub fn game_cards_ui(
    mut q: Query<(&Interaction, &mut BackgroundColor, &GameIndex), With<GameIndex>>,
    selected_idx: Res<SelectedIndex>,
) {
    for (interaction, mut color, idx) in q.iter_mut() {
        *color = if selected_idx.0 == idx.0 {
            GAME_CARD_COLOR_SELECTED
        } else if *interaction == Interaction::Hovered {
            GAME_CARD_COLOR_HOVER
        } else {
            GAME_CARD_COLOR_NORMAL
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
    let mut style = q.single_mut().0;
    let window = window.get_primary().unwrap();
    let screenshothandle = games.0[selected_idx.0 as usize].screenshot.clone();
    let Some(screenshot) = assets.get(&screenshothandle) else { return; };
    //aspect ratio = x / y
    let screenshot_ratio = {
        let size = screenshot.size();
        size.x / size.y
    };

    let window_ratio = (window.width() - GAMES_LAVEL_WIDTH) / window.height();

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

pub fn select_by_keybord(
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

pub fn select_by_cursor(
    query: Query<(&Interaction, &GameIndex), Changed<Interaction>>,
    mut selected_idx: ResMut<SelectedIndex>,
) {
    for (interaction, idx) in query.iter() {
        if *interaction == Interaction::Clicked {
            selected_idx.0 = idx.0;
        }
    }
}

pub fn run_by_keybord_sys(
    key_input: Res<Input<KeyCode>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    if key_input.just_pressed(KeyCode::Return) {
        crate::core::run_game(&selected_idx, &games);
    }
}

pub fn selected_idx_changed(selected_idx: Res<SelectedIndex>) -> ShouldRun {
    if selected_idx.is_changed() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
