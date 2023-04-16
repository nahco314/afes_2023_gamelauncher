use super::*;
use crate::core::Games;

const TEXT_Z_INDEX: i32 = 2;
const TEXT_BG_Z_INDEX: i32 = 1;
const TEXT_BG_COLOR: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.,
    green: 0.,
    blue: 0.,
    alpha: 0.55,
});

pub fn setup(mut cmd: Commands, asset_server: Res<AssetServer>, games: Res<Games>) {
    cmd.spawn(Camera2dBundle::default());
    cmd.spawn(root())
        .with_children(|p| {
            p.spawn(games_label::root()).with_children(|p| {
                p.spawn(games_label::title(&asset_server));
                p.spawn(games_label::cards::root()).with_children(|p| {
                    p.spawn(games_label::cards::list::root())
                        .with_children(|p| {
                            for (idx, game) in games.0.iter().enumerate() {
                                p.spawn(games_label::cards::list::card::root())
                                    .with_children(|p| {
                                        p.spawn(games_label::cards::list::card::title(
                                            &asset_server,
                                            game,
                                        ));
                                    })
                                    .insert(GameIndex(idx as u32))
                                    .insert(Interaction::default());
                            }
                        });
                });
            });
        })
        .with_children(|p| {
            p.spawn(game_detail::root())
                .with_children(|p| {
                    p.spawn(game_detail::screenshot(&games))
                        .insert(GameScreenShot);
                })
                .with_children(|p| {
                    p.spawn(game_detail::title(&games, &asset_server))
                        .with_children(|p| {
                            p.spawn(text_bg()).insert(TextBg);
                        })
                        .insert(GameTitleText);
                })
                .with_children(|p| {
                    p.spawn(game_detail::additional::root())
                        .with_children(|p| {
                            p.spawn(game_detail::additional::description(&games, &asset_server))
                                .with_children(|p| {
                                    p.spawn(text_bg()).insert(TextBg);
                                })
                                .insert(GameDescriptionText);
                        })
                        .with_children(|p| {
                            p.spawn(game_detail::additional::author::root())
                                .with_children(|p| {
                                    p.spawn(game_detail::additional::author::text(
                                        &games,
                                        &asset_server,
                                    ))
                                    .with_children(|p| {
                                        p.spawn(text_bg()).insert(TextBg);
                                    })
                                    .insert(GameAuthorText);
                                });
                        });
                });
        })
        .with_children(|p| {
            p.spawn(play_button::root_button()).with_children(|p| {
                p.spawn(play_button::text(&asset_server));
            });
        });
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    }
}

mod games_label {

    use crate::ui::*;

    pub fn root() -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Px(GAMES_LAVEL_WIDTH), Val::Percent(100.)),
                min_size: Size::new(Val::Px(GAMES_LAVEL_WIDTH), Val::Percent(100.)),
                ..Default::default()
            },
            background_color: GAMES_LAVEL_COLOR,
            ..Default::default()
        }
    }

    pub fn title(asset_server: &Res<AssetServer>) -> TextBundle {
        TextBundle::from_section(GAMES_LAVEL_TITLE, create_text_style(35., asset_server))
            .with_style(Style {
                size: Size::new(Val::Undefined, Val::Px(35.)),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                ..Default::default()
            })
    }

    pub mod cards {

        use bevy::prelude::*;

        pub fn root() -> NodeBundle {
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            }
        }

        pub mod list {

            use bevy::prelude::*;
            pub fn root() -> NodeBundle {
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        max_size: Size::new(Val::Undefined, Val::Undefined),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                }
            }

            pub mod card {
                use crate::core::Game;
                use crate::ui::*;
                use bevy::ui::FocusPolicy;

                pub fn root() -> NodeBundle {
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            flex_grow: 1.0,
                            max_size: Size::new(Val::Undefined, Val::Undefined),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::NONE),
                        ..Default::default()
                    }
                }

                pub fn title(asset_server: &Res<AssetServer>, game: &Game) -> TextBundle {
                    {
                        let mut tmp = TextBundle::from_section(
                            game.title.to_owned(),
                            create_text_style(25., asset_server),
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
                        });
                        tmp.focus_policy = FocusPolicy::Pass;
                        tmp
                    }
                }
            }
        }
    }
}

mod game_detail {
    use crate::core::Games;
    use crate::ui::{setup::TEXT_Z_INDEX, *};
    use bevy::ui::widget::ImageMode;

    pub fn root() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        }
    }

    pub fn screenshot(games: &Res<Games>) -> ImageBundle {
        ImageBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(100.)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            image_mode: ImageMode::KeepAspect,
            image: games.0[0].screenshot.clone().into(),
            ..Default::default()
        }
    }

    pub fn title(games: &Res<Games>, asset_server: &Res<AssetServer>) -> TextBundle {
        let mut tmp = TextBundle::from_section(
            games.0[0].title.clone(),
            create_text_style(GAME_TITLE_TEXT_SIZE, asset_server),
        )
        .with_style(Style {
            size: Size::new(Val::Auto, Val::Auto),
            align_self: AlignSelf::FlexStart,
            margin: UiRect {
                left: Val::Px(20.),
                top: Val::Px(20.),
                bottom: Val::Px(20.),
                ..Default::default()
            },
            ..Default::default()
        });
        tmp.z_index = ZIndex::Global(TEXT_Z_INDEX);
        tmp
    }

    pub mod additional {
        use crate::core::Games;
        use crate::ui::{setup::TEXT_Z_INDEX, *};

        pub fn root() -> NodeBundle {
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            }
        }

        pub fn description(games: &Res<Games>, asset_server: &Res<AssetServer>) -> TextBundle {
            let mut tmp = TextBundle::from_section(
                games.0[0].description.clone(),
                create_text_style(DESCRIPTION_TEXT_SIZE, asset_server),
            )
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                margin: UiRect {
                    left: Val::Px(20.),
                    top: Val::Px(70.),
                    right: Val::Px(20.),
                    ..Default::default()
                },
                max_size: Size::new(Val::Px(DESCRIPTION_WIDTH_MAX), Val::Undefined),
                ..Default::default()
            })
            .with_text_alignment(TextAlignment::TOP_LEFT);
            tmp.z_index = ZIndex::Global(TEXT_Z_INDEX);
            tmp
        }

        pub mod author {
            use crate::core::Games;
            use crate::ui::{setup::TEXT_Z_INDEX, *};

            pub fn root() -> NodeBundle {
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                }
            }

            pub fn text(games: &Res<Games>, asset_server: &Res<AssetServer>) -> TextBundle {
                let mut tmp = TextBundle::from_section(
                    games.0[0].author.clone(),
                    create_text_style(AUTHOR_NAME_TEXT_SIZE, asset_server),
                )
                .with_text_alignment(TextAlignment::CENTER_LEFT)
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(20.),
                        right: Val::Px(20.),
                        bottom: Val::Px(30.),
                    },
                    size: Size::new(Val::Auto, Val::Auto),
                    ..Default::default()
                });
                tmp.z_index = ZIndex::Global(TEXT_Z_INDEX);
                tmp
            }
        }
    }
}

pub mod play_button {
    use crate::ui::*;

    pub fn root_button() -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.), Val::Px(90.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(30.),
                    right: Val::Px(35.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BUTTON_COLOR_NORMAL,
            ..Default::default()
        }
    }
    pub fn text(asset_server: &Res<AssetServer>) -> TextBundle {
        let mut tmp = TextBundle::from_sections(vec![
            TextSection::new("プレイ", create_text_style(52., asset_server)),
            TextSection::new("\n(Enter)", create_text_style(28., asset_server)),
        ]);
        tmp.text.alignment = TextAlignment::CENTER;
        tmp
    }
}

fn text_bg() -> NodeBundle {
    NodeBundle {
        style: Style::default(),
        background_color: TEXT_BG_COLOR,
        z_index: ZIndex::Global(TEXT_BG_Z_INDEX),
        ..Default::default()
    }
}
