use bevy_app::{Plugin, Startup, Update};
use bevy_asset::AssetServer;
use bevy_color::Color;
use bevy_diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy_ecs::{
    change_detection::DetectChanges,
    component::Component,
    query::With,
    system::{Commands, Query, Res, Resource},
};
use bevy_text::{Text, TextSection, TextStyle};
use bevy_ui::node_bundles::TextBundle;

#[derive(Default)]
pub struct FpsOverlayPlugin {
    pub config: FpsOverlayConfig,
}

impl Plugin for FpsOverlayPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
        app.insert_resource(self.config.clone())
            .add_systems(Startup, setup)
            .add_systems(Update, (customize_text, update_text));
    }
}

#[derive(Resource, Clone)]
pub struct FpsOverlayConfig {
    pub font_path: Option<String>,
    pub font_size: f32,
    pub font_color: Color,
}

impl Default for FpsOverlayConfig {
    fn default() -> Self {
        FpsOverlayConfig {
            font_path: None,
            font_size: 32.0,
            font_color: Color::WHITE,
        }
    }
}

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    overlay_config: Res<FpsOverlayConfig>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                if let Some(font_path) = &overlay_config.font_path {
                    TextStyle {
                        font_size: overlay_config.font_size,
                        color: overlay_config.font_color,
                        font: asset_server.load(font_path),
                    }
                } else {
                    TextStyle {
                        font_size: overlay_config.font_size,
                        color: overlay_config.font_color,
                        ..Default::default()
                    }
                },
            ),
            TextSection::from_style(if let Some(font_path) = &overlay_config.font_path {
                TextStyle {
                    font_size: overlay_config.font_size,
                    color: overlay_config.font_color,
                    font: asset_server.load(font_path),
                }
            } else {
                TextStyle {
                    font_size: overlay_config.font_size,
                    color: overlay_config.font_color,
                    ..Default::default()
                }
            }),
        ]),
        FpsText,
    ));
}

fn update_text(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn customize_text(
    overlay_config: Res<FpsOverlayConfig>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if !overlay_config.is_changed() {
        return;
    }

    for mut text in &mut query {
        for section in text.sections.iter_mut() {
            section.style = if let Some(font_path) = &overlay_config.font_path {
                TextStyle {
                    font_size: overlay_config.font_size,
                    color: overlay_config.font_color,
                    font: asset_server.load(font_path),
                }
            } else {
                TextStyle {
                    font_size: overlay_config.font_size,
                    color: overlay_config.font_color,
                    ..Default::default()
                }
            }
        }
    }
}
