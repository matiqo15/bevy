#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://bevyengine.org/assets/icon.png",
    html_favicon_url = "https://bevyengine.org/assets/icon.png"
)]

//! This crate provides additional utilities for the [Bevy game engine](https://bevyengine.org),
//! focused on improving developer experience.

use bevy_app::prelude::*;
use bevy_ecs::{
    system::Resource,
    world::{Command, World},
};
use bevy_reflect::{FromReflect, Reflect, TypePath};
use bevy_reflect::{FromType, GetTypeRegistration, TypeInfo};
use std::{any::TypeId, borrow::BorrowMut, fmt::Debug};

#[cfg(feature = "bevy_ci_testing")]
pub mod ci_testing;

pub mod fps_overlay;

pub mod fly_camera;

#[cfg(feature = "bevy_ui_debug")]
pub mod ui_debug_overlay;

/// Enables developer tools in an [`App`]. This plugin is added automatically with `bevy_dev_tools`
/// feature.
///
/// Warning: It is not recommended to enable this in final shipped games or applications.
/// Dev tools provide a high level of access to the internals of your application,
/// and may interfere with ordinary use and gameplay.
///
/// To enable developer tools, you can either:
///
/// - Create a custom crate feature (e.g "`dev_mode`"), which enables the `bevy_dev_tools` feature
/// along with any other development tools you might be using:
///
/// ```toml
/// [feature]
/// dev_mode = ["bevy/bevy_dev_tools", "other_dev_tools"]
/// ```
///
/// - Use `--feature bevy/bevy_dev_tools` flag when using the `cargo run` command:
///
/// `cargo run --features bevy/bevy_dev_tools`
///
/// - Add the `bevy_dev_tools` feature to the bevy dependency in your `Cargo.toml` file:
///
/// `features = ["bevy_dev_tools"]`
///
///  Note: The third method is not recommended, as it requires you to remove the feature before
///  creating a build for release to the public.
pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(feature = "bevy_ci_testing")]
        {
            ci_testing::setup_app(_app);
        }
    }
}

pub trait DevTool: Resource + Reflect + Debug {
    fn name(&self) -> &str {
        self.reflect_short_type_path()
    }

    /// Turns this dev tool on (true) or off (false).
    fn set_enabled(&mut self, enabled: bool);

    /// Is this dev tool currently enabled?
    fn is_enabled(&self) -> bool;
}

#[derive(Clone)]
pub struct ReflectDevTool {
    get_reflect: fn(&World) -> Option<&dyn Reflect>,
    get: fn(&World) -> Option<&dyn DevTool>,
    from_reflect: fn(&dyn Reflect) -> Option<Box<dyn DevTool>>,
}

impl ReflectDevTool {
    pub fn get_reflect<'a>(&self, world: &'a World) -> Option<&'a dyn Reflect> {
        (self.get_reflect)(world)
    }

    pub fn get<'a>(&self, world: &'a World) -> Option<&'a dyn DevTool> {
        (self.get)(world)
    }

    pub fn from_reflect(&self, reflect: &dyn Reflect) -> Option<Box<dyn DevTool>> {
        (self.from_reflect)(reflect)
    }
}

impl<D: DevTool + Reflect + FromReflect> FromType<D> for ReflectDevTool {
    fn from_type() -> Self {
        Self {
            get_reflect: |world| world.get_resource::<D>().map(|d| d as &dyn Reflect),
            get: |world| world.get_resource::<D>().map(|d| d as &dyn DevTool),
            from_reflect: |reflect| {
                D::from_reflect(reflect).map(|d| {
                    let d: Box<dyn DevTool> = Box::new(d);
                    d
                })
            },
        }
    }
}

pub trait DevCommand: Command + Reflect + Debug {
    /// The name of this tool, as might be supplied by a command line interface.
    fn name(&self) -> &str {
        self.reflect_short_type_path()
    }

    fn apply(&self, world: &mut World) {}
}

#[derive(Clone)]
pub struct ReflectDevCommand {
    from_reflect: fn(&dyn Reflect) -> Option<Box<dyn DevCommand>>,
}

impl ReflectDevCommand {
    pub fn from_reflect(&self, reflect: &dyn Reflect) -> Option<Box<dyn DevCommand>> {
        (self.from_reflect)(reflect)
    }
}

impl<D: DevCommand + Reflect + FromReflect> FromType<D> for ReflectDevCommand {
    fn from_type() -> Self {
        Self {
            from_reflect: |reflect| {
                D::from_reflect(reflect).map(|d| {
                    let d: Box<dyn DevCommand> = Box::new(d);
                    d
                })
            },
        }
    }
}

#[derive(Reflect, Debug, Default)]
#[reflect(DevCommand)]
pub struct Enable<T: DevTool + GetTypeRegistration + Default> {
    dev_tool: T,
}

impl<T: DevTool + GetTypeRegistration + Default> Command for Enable<T> {
    fn apply(mut self, _world: &mut World) {
        self.dev_tool.set_enabled(true);
    }
}
impl<T: DevTool + Default + TypePath + GetTypeRegistration + FromReflect> DevCommand for Enable<T> {}

#[derive(Reflect, Debug, Default)]
#[reflect(DevCommand)]
pub struct Disable<T: DevTool + GetTypeRegistration + Default> {
    dev_tool: T,
}

impl<T: DevTool + GetTypeRegistration + Default> Command for Disable<T> {
    fn apply(mut self, _world: &mut World) {
        self.dev_tool.set_enabled(false);
    }
}
impl<T: DevTool + Default + GetTypeRegistration + FromReflect + TypePath> DevCommand
    for Disable<T>
{
}

#[derive(Reflect, Debug, Default)]
#[reflect(DevCommand)]
pub struct Toggle<T: DevTool + GetTypeRegistration + Default> {
    dev_tool: T,
}

impl<T: DevTool + GetTypeRegistration + Default> Command for Toggle<T> {
    fn apply(mut self, _world: &mut World) {
        self.dev_tool.set_enabled(!self.dev_tool.is_enabled());
    }
}
impl<T: DevTool + Default + GetTypeRegistration + FromReflect + TypePath> DevCommand for Toggle<T> {}

pub trait DevToolsApp {
    fn init_dev_tool<T: DevTool + TypePath + GetTypeRegistration + FromReflect + Default>(
        &mut self,
    ) -> &mut Self;
    fn insert_dev_tool<T: DevTool + TypePath + GetTypeRegistration + FromReflect + Default>(
        &mut self,
        tool: T,
    ) -> &mut Self;
    fn register_dev_command<
        C: DevCommand + Default + TypePath + GetTypeRegistration + FromReflect,
    >(
        &mut self,
    ) -> &mut Self;
}

impl DevToolsApp for App {
    fn init_dev_tool<T: DevTool + TypePath + GetTypeRegistration + FromReflect + Default>(
        &mut self,
    ) -> &mut Self {
        self.register_type::<T>();
        self.init_resource::<T>();
        self.register_dev_command::<Enable<T>>()
            .register_dev_command::<Disable<T>>()
            .register_dev_command::<Toggle<T>>()
    }

    fn insert_dev_tool<T: DevTool + TypePath + GetTypeRegistration + FromReflect + Default>(
        &mut self,
        tool: T,
    ) -> &mut Self {
        self.register_type::<T>();
        self.insert_resource(tool);
        self.register_dev_command::<Enable<T>>()
            .register_dev_command::<Disable<T>>()
            .register_dev_command::<Toggle<T>>()
    }

    fn register_dev_command<C: DevCommand + TypePath + GetTypeRegistration + FromReflect>(
        &mut self,
    ) -> &mut Self {
        self.register_type::<C>()
    }
}
