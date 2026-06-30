use crate::camera::{align_camera, move_camera, zoom_camera};
use crate::chunk_map::{ChunkMap, ChunkMapModified, VoxelChunkMap};
use crate::collider_world::update_colliders;
use crate::load_chunks::load_chunks;
use crate::pointer::spawn_cells;
use crate::simulate_world::simulate_world;
use crate::startup::startup;
use crate::world_image::{PixelLength, display_world, on_resize_world};
use crate::{APP_NAME, PIXEL_LENGTH};
use avian2d::PhysicsPlugins;
use bevy::DefaultPlugins;
use bevy::app::{App, AppExit, FixedUpdate, PluginGroup as _, Startup, Update};
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
#[cfg(feature = "colliders")]
use bevy::gizmos::AppGizmoBuilder as _;
use bevy::image::ImagePlugin;
use bevy::settings::SettingsPlugin;
use bevy::window::{PresentMode, Window, WindowPlugin};
pub fn app_run() -> AppExit {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "noiter".to_owned(),
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
        SettingsPlugin::new(APP_NAME),
        #[cfg(feature = "colliders")]
        avian2d::debug_render::PhysicsDebugPlugin,
        #[cfg(feature = "fps")]
        bevy::dev_tools::fps_overlay::FpsOverlayPlugin::default(),
    ));
    #[cfg(feature = "colliders")]
    app.insert_gizmo_config(
        avian2d::debug_render::PhysicsGizmos {
            axis_lengths: None,
            collider_color: Some(bevy::color::Color::srgba_u8(0, 0, 0, 127)),
            sleeping_color_multiplier: None,
            ..avian2d::debug_render::PhysicsGizmos::default()
        },
        bevy::gizmos::config::GizmoConfig::default(),
    );
    app.insert_resource(ChunkMap::default());
    app.insert_resource(VoxelChunkMap::default());
    app.insert_resource(ChunkMapModified::default());
    app.insert_resource(ClearColor(Color::srgba_u32(0x96b7_ddff)));
    app.add_systems(Startup, startup);
    app.add_systems(
        Update,
        (update_colliders, (on_resize_world, display_world).chain()),
    );
    app.add_systems(
        FixedUpdate,
        (
            spawn_cells,
            (
                (align_camera, move_camera, zoom_camera),
                load_chunks,
                simulate_world,
            )
                .chain(),
        ),
    );
    app.insert_resource(PixelLength(PIXEL_LENGTH));
    app.run()
}
