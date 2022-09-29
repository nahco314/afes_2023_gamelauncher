use crate::{
    GameAuthorText, GameDescriptionText, GameScreenShot, GameTitleText, Games, SelectedIndex,
};
use bevy::prelude::*;

pub(crate) fn update_title_text(
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
            color: crate::TEXT_COLOR,
        },
    }]
}

pub(crate) fn update_desc_text(
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
            color: crate::TEXT_COLOR,
        },
    }]
}

pub(crate) fn update_author_text(
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
            color: crate::TEXT_COLOR,
        },
    }]
}

pub(crate) fn update_screenshot(
    mut image: Query<(&mut UiImage,), With<GameScreenShot>>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>
) {
    image.single_mut().0 .0 = games.0[selected_idx.0 as usize].screenshot.clone();
}

pub(crate) fn play_button_sys(
    q: Query<(&Interaction,), (Changed<Interaction>, With<Button>)>,
    selected_idx: Res<SelectedIndex>,
    games: Res<Games>,
) {
    if let Ok(q) = q.get_single() {
        if *q.0 == Interaction::Clicked {
            crate::run_game(&selected_idx, &games);
        }
    }
}
