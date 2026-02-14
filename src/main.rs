use bevy::{asset::AssetMetaCheck, prelude::*, window::PresentMode};

use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod ball;
use ball::*;

mod flippers;
use flippers::*;

mod walls;
use walls::*;

mod launcher;
use launcher::*;

mod pins;
use pins::*;

pub const PIXELS_PER_METER: f32 = 492.3;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pinball2d".into(),
                        resolution: (360, 640).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        canvas: Some("#bevy-canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins((
            ShapePlugin,
            WallsPlugin,
            LauncherPlugin,
            FlippersPlugin,
            BallPlugin,
            PinsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: Single<&mut RapierConfiguration, With<DefaultRapierContext>>,
) {
    // Set gravity and spawn camera.
    rapier_config.gravity = Vec2::new(0.0, -520.0);

    commands.spawn(Camera2d);
}
