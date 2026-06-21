use crate::chunk_map::{ChunkMap, draw_chunks};
use crate::startup::startup;
use crate::update::update;
use avian2d::PhysicsPlugins;
use avian2d::debug_render::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::DefaultPlugins;
use bevy::app::{App, AppExit, Last, PluginGroup, Startup, Update};
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::color::Color;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::gizmos::AppGizmoBuilder;
use bevy::gizmos::config::GizmoConfig;
use bevy::image::{ImagePlugin, ImageSamplerDescriptor};
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::{MessageReader, NonSend, NonSendMut};
use bevy::window::{Window, WindowPlugin, WindowResized};
use bevy_framebuffer::PixelsPlugin;
use bevy_framebuffer::pixels_impl::{PixelsConfig, PixelsFrame};
use bevy_framebuffer::schedule::{RenderSchedule, SurfaceSchedule};
const BUFFER_WIDTH: u32 = 320;
const BUFFER_HEIGHT: u32 = 180;
pub fn app_run() -> AppExit {
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
        PixelsPlugin {
            config: PixelsConfig {
                width: BUFFER_WIDTH,
                height: BUFFER_HEIGHT,
                ..Default::default()
            },
        },
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
    .insert_resource(ChunkMap::default())
    .add_systems(Startup, startup)
    .add_systems(Update, update)
    .add_systems(Last, draw_chunks)
    .add_systems(SurfaceSchedule, resize_system)
    .add_systems(RenderSchedule, render_system);
    app.run()
}
pub fn resize_system(
    mut events: MessageReader<WindowResized>,
    mut buffer: NonSendMut<PixelsFrame>,
) {
    if let Some(event) = events.read().last() {
        buffer
            .resize_surface(event.width as u32, event.height as u32)
            .unwrap()
    }
}
pub fn render_system(buffer: NonSend<PixelsFrame>) {
    buffer.render().unwrap();
}
