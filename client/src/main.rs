use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{Cursor, CursorGrabMode},
};
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::{DebugRenderMode, RapierDebugRenderPlugin},
};
use bevy_sprite3d::Sprite3dPlugin;
use menu::MenuPlugin;
use networking::client_bullet_receiver::NetworkingPlugin;
use states::StatePlugin;

pub mod lifetime;
pub mod menu;
pub mod networking;
pub mod physics;
pub mod player;
pub mod setup;
pub mod sprites;
pub mod states;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    let mut app = App::new();

    setup::init(&mut app);
    player::init(&mut app);
    sprites::init(&mut app);
    lifetime::init(&mut app);
    let mut cursor = Cursor::default();
    cursor.visible = true; // false;
    cursor.grab_mode = CursorGrabMode::None; //CursorGrabMode::Locked;

    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Realm of the OctoSurvivors!".into(),
            resolution: (WIDTH, HEIGHT).into(),
            // present_mode: window::PresentMode::AutoVsync,
            resizable: false,
            // Tells wasm to resize the window according to the available canvas
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            cursor,
            ..default()
        }),
        ..default()
    };

    app
        // bevy_sprite3d
        // Background Color
        .insert_resource(ClearColor(Color::hex("212121").unwrap()))
        // Load Assets
        .add_plugins((
            Sprite3dPlugin,
            FlyCameraPlugin,
            FrameTimeDiagnosticsPlugin,
            StatePlugin,
            NetworkingPlugin,
            // Default Plugins
            DefaultPlugins
                .set(window)
                .set(ImagePlugin::default_nearest()),
            MenuPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                mode: DebugRenderMode::all(),
                ..default()
            },
            WorldInspectorPlugin::new(),
        ))
        .run();
}
