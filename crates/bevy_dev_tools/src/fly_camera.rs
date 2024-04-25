//! idk

use bevy_app::{App, Plugin, Startup};
use bevy_core_pipeline::core_2d::Camera2dBundle;
use bevy_ecs::system::{Commands, Resource};
use bevy_reflect::Reflect;
use bevy_text::TextStyle;
use bevy_ui::node_bundles::TextBundle;

use crate::{DevTool, DevToolsApp, ReflectDevTool};

pub struct DevFlyCameraPlugin;
impl Plugin for DevFlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_dev_tool::<DevFlyCamera>()
            .add_systems(Startup, setup);
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(DevTool)]
pub struct DevFlyCamera {
    pub enabled: bool,
    pub movement_speed: Option<f32>,
    pub turn_speed: Option<f32>,
}

impl Default for DevFlyCamera {
    fn default() -> Self {
        DevFlyCamera {
            enabled: true,
            movement_speed: Some(3.0),
            turn_speed: Some(10.0),
        }
    }
}

impl DevTool for DevFlyCamera {
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "on",
        TextStyle {
            font_size: 25.0,
            ..Default::default()
        },
    ));
}
