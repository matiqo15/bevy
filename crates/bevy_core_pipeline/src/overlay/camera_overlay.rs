use bevy_ecs::{
    entity::Entity,
    prelude::{Bundle, Component, With},
    query::QueryItem,
    system::{Commands, Query},
};
use bevy_render::{
    camera::{Camera, CameraRenderGraph, OrthographicProjection},
    extract_component::ExtractComponent,
    view::VisibleEntities,
    Extract,
};
use bevy_transform::prelude::GlobalTransform;

use super::overlay_node::graph;

pub(crate) fn extract_overlay_camera_phases(
    mut commands: Commands,
    cameras_overlay: Extract<Query<(Entity, &Camera), With<CameraOverlay>>>,
) {
    for (entity, camera) in cameras_overlay.iter() {
        if camera.is_active {
            commands.get_or_spawn(entity);
        }
    }
}

/// Marker component for the camera used to display the FPS overlay.
///
/// See [`CameraOverlayBundle`] for the full list of components needed to display the overlay.
#[derive(Component, Default, Clone)]
pub struct CameraOverlay;

impl ExtractComponent for CameraOverlay {
    type Data = &'static Self;
    type Filter = With<Camera>;
    type Out = Self;

    fn extract_component(item: QueryItem<Self::Data>) -> Option<Self::Out> {
        Some(item.clone())
    }
}

/// Bundle of components needed to display the FPS overlay.
///
/// See [`OverlayPlugin`](super::OverlayPlugin) on how to enable the overlay.
#[derive(Bundle)]
pub struct CameraOverlayBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub global_transform: GlobalTransform,
    pub camera_overlay: CameraOverlay,
}

impl Default for CameraOverlayBundle {
    fn default() -> Self {
        Self {
            camera: Camera {
                order: isize::MAX,
                ..Default::default()
            },
            camera_render_graph: CameraRenderGraph::new(graph::NAME),
            visible_entities: Default::default(),
            projection: Default::default(),
            global_transform: Default::default(),
            camera_overlay: Default::default(),
        }
    }
}
