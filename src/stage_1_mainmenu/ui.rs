use bevy::{prelude::*, ui::entity};
use iyes_loopless::{
    condition::IntoConditionalExclusiveSystem,
    prelude::{AppLooplessStateExt, ConditionSet},
    state::NextState,
};

use crate::shared::{
    constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    general::{button_color_system, despawn_system, on_button_interact},
    resources::{AppState, UiTextures},
};

use super::components::{GameplayButton, MainMenu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MainMenu, setup_mainmenu_system)
            // --- Stage change ---
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .run_if(on_button_interact::<GameplayButton>)
                    .with_system(start_gameplay_system)
                    .into(),
            )
            // --- Basic button color changer ---
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(button_color_system)
                    .into(),
            )
            // --- Ui cleanup ---
            .add_exit_system(AppState::MainMenu, despawn_system::<MainMenu>);
    }
}

fn setup_mainmenu_system(mut commands: Commands, ui_textures: Res<UiTextures>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.), Val::Px(65.)),
                        border: Rect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.6, 0.6, 0.6).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        margin: Rect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: NORMAL_BUTTON.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Start game",
                                            TextStyle {
                                                font: ui_textures.ui_font.clone(),
                                                font_size: 40.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..default()
                                    });
                                })
                                .insert(GameplayButton);
                        });
                });
        })
        .insert(MainMenu);
}

fn start_gameplay_system(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::Gameplay));
}
