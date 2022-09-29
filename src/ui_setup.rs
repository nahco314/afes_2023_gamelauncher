use crate::{GameAuthorText, GameDescriptionText, GameIndex, GameTitleText, Games};
use bevy::{prelude::*, ui::FocusPolicy};

pub(crate) fn setup(mut cmd: Commands, asset_server: Res<AssetServer>, games: Res<Games>) {
    cmd.spawn_bundle(Camera2dBundle::default());
    cmd.spawn_bundle(
        //root node
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        },
    )
    .with_children(|p| {
        //games label
        p.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Px(crate::GAMES_LAVEL_WIDTH), Val::Percent(100.)),
                min_size: Size::new(Val::Px(crate::GAMES_LAVEL_WIDTH), Val::Percent(100.)),
                ..Default::default()
            },
            color: crate::GAMES_LAVEL_COLOR.into(),
            ..Default::default()
        })
        .with_children(|p| {
            //label title
            p.spawn_bundle(
                TextBundle::from_section(
                    "Games",
                    TextStyle {
                        font_size: 35.,
                        color: crate::TEXT_COLOR,
                        font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(35.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
            //game *cards* node
            p.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|p| {
                //game card node
                p.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        flex_grow: 1.0,
                        max_size: Size::new(Val::Undefined, Val::Undefined),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|p| {
                    for (idx, game) in games.0.iter().enumerate() {
                        //game card node
                        p.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Px(25.0)),
                                ..default()
                            },
                            color: crate::NORMAL_GAME_COLOR.into(),
                            ..default()
                        })
                        .with_children(|p| {
                            //game title
                            p.spawn_bundle({
                                let mut tmp = TextBundle::from_section(
                                    game.title.to_owned(),
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/NotoSansCJKjp-DemiLight.otf"),
                                        font_size: 25.,
                                        color: crate::TEXT_COLOR,
                                    },
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
                            });
                        })
                        .insert(GameIndex(idx as u32))
                        .insert(Interaction::default());
                    }
                });
            });
        });
    })
    .with_children(|p| {
        //game detail root node
        p.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|p| {
            //title
            p.spawn_bundle(
                TextBundle::from_section(
                    games.0[0].title.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                        font_size: 90.,
                        color: crate::TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    min_size: Size::new(Val::Undefined, Val::Px(90.)),
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(20.),
                        bottom : Val::Px(20.),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .insert(GameTitleText);
        })
        .with_children(|p| {
            p.spawn_bundle(
                //desc sroot node
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                },
            )
            .with_children(|p| {
                //desc text
                p.spawn_bundle(
                    TextBundle::from_section(
                        games.0[0].description.clone(),
                        TextStyle {
                            font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                            font_size: 50.,
                            color: crate::TEXT_COLOR,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::FlexStart,
                        margin: UiRect {
                            left: Val::Px(20.),
                            top: Val::Px(70.),
                            right: Val::Px(20.),
                            ..Default::default()
                        },
                        max_size: Size::new(Val::Px(crate::GAME_DESC_TEXT_WIDTH), Val::Auto),
                        ..Default::default()
                    }).with_text_alignment(TextAlignment::TOP_LEFT),
                )
                .insert(GameDescriptionText);
            })
            .with_children(|p| {
                p.spawn_bundle(
                    //author root node
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexStart,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    },
                )
                .with_children(|p| {
                    //author text
                    p.spawn_bundle(
                        TextBundle::from_section(
                            games.0[0].author.clone(),
                            TextStyle {
                                font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                                font_size: 40.,
                                color: crate::TEXT_COLOR,
                            },
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
                            max_size: Size::new(Val::Px(crate::GAME_AUTHOR_TEXT_WIDTH), Val::Auto),
                            ..Default::default()
                        }),
                    )
                    .insert(GameAuthorText);
                });
            });
        });
    })
    .with_children(|p| {
        //play button
        p.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(60.)),
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
            color: crate::BUTTON_COLOR.into(),
            ..Default::default()
        })
        .with_children(|p| {
            // "プレイ" text
            p.spawn_bundle(TextBundle::from_section(
                "プレイ",
                TextStyle {
                    font: asset_server.load("fonts/NotoSansCJKjp-DemiLight.otf"),
                    font_size: 40.,
                    color: crate::TEXT_COLOR,
                },
            ));
        });
    });
}
