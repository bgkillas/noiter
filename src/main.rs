use avian2d::PhysicsPlugins;
use avian2d::debug_render::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::DefaultPlugins;
use bevy::app::{App, AppExit, PluginGroup, Startup, Update};
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::camera::Camera2d;
use bevy::color::Color;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::ecs::system::Commands;
use bevy::gizmos::AppGizmoBuilder;
use bevy::gizmos::config::GizmoConfig;
use bevy::image::{ImagePlugin, ImageSamplerDescriptor};
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::window::{Window, WindowPlugin};
fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "noiter".into(),
                    resizable: true,
                    fit_canvas_to_parent: true,
                    ..Window::default()
                }),
                ..WindowPlugin::default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..AssetPlugin::default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            }),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin,
        MeshPickingPlugin,
        FpsOverlayPlugin::default(),
    ))
    .insert_gizmo_config(
        PhysicsGizmos {
            axis_lengths: None,
            collider_color: Some(Color::srgba_u8(0, 0, 0, 127)),
            sleeping_color_multiplier: None,
            ..PhysicsGizmos::default()
        },
        GizmoConfig::default(),
    )
    .add_systems(Startup, startup)
    .add_systems(Update, update);
    app.run()
}
fn update() {}
fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
