use crate::PIXEL_LENGTH;
use crate::camera::{align_camera, move_camera, zoom_camera};
use crate::chunk_map::ChunkMap;
use crate::startup::startup;
use crate::update::update;
use crate::world_image::{PixelLength, display_world, on_resize_world};
use avian2d::PhysicsPlugins;
use avian2d::debug_render::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::DefaultPlugins;
use bevy::app::{App, AppExit, FixedUpdate, PluginGroup, Startup, Update};
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::color::Color;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::gizmos::AppGizmoBuilder;
use bevy::gizmos::config::GizmoConfig;
use bevy::image::ImagePlugin;
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::window::{PresentMode, Window, WindowPlugin};
pub fn app_run() -> AppExit {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "noiter".into(),
                    resizable: true,
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::Immediate,
                    ..Window::default()
                }),
                ..WindowPlugin::default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..AssetPlugin::default()
            })
            .set(ImagePlugin::default_nearest()),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin,
        MeshPickingPlugin,
        FpsOverlayPlugin::default(),
    ));
    app.insert_gizmo_config(
        PhysicsGizmos {
            axis_lengths: None,
            collider_color: Some(Color::srgba_u8(0, 0, 0, 127)),
            sleeping_color_multiplier: None,
            ..PhysicsGizmos::default()
        },
        GizmoConfig::default(),
    );
    app.insert_resource(ChunkMap::default());
    app.add_systems(Startup, startup);
    app.add_systems(
        Update,
        (
            update,
            align_camera,
            (on_resize_world, display_world).chain(),
        ),
    );
    app.add_systems(FixedUpdate, (move_camera, zoom_camera));
    app.insert_resource(PixelLength(PIXEL_LENGTH));
    app.run()
}
