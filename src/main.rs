#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod ui;

use bevy::{prelude::*, window::WindowResizeConstraints, winit::WinitSettings};
use ui::bevy_system::*;

fn main() {
    App::new()
        .insert_resource(WinitSettings::game())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1600.,
                height: 900.,
                resizable: true,
                title: "Game Launcher".to_owned(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 960.,
                    min_height: 540.,
                    max_height: f32::INFINITY,
                    max_width: f32::INFINITY,
                },
                scale_factor_override: Some(1.0),
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(core::Games(Vec::new()))
        .insert_resource(core::SelectedIndex(0))
        .add_startup_system(core::load_game_folder)
        .add_startup_system(ui::setup::setup.after(core::load_game_folder))
        .add_system(select_by_keybord)
        .add_system(select_by_cursor)
        .add_system(run_by_keybord_sys)
        .add_system(handle_play_button)
        .add_system(
            update_title_text
                .with_run_criteria(selected_idx_changed)
                .before(adjust_title_size),
        )
        .add_system(
            update_desc_text
                .with_run_criteria(selected_idx_changed)
                .before(adjust_description_size),
        )
        .add_system(update_author_text.with_run_criteria(selected_idx_changed))
        .add_system(update_screenshot.with_run_criteria(selected_idx_changed))
        .add_system(game_cards_ui)
        .add_system(fit_screenshot)
        .add_system(update_text_bg)
        .add_system(adjust_title_size)
        .add_system(adjust_description_size)
        .run();
}
