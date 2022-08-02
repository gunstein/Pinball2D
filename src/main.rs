use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

pub const PIXELS_PER_METER : f32 = 492.3;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pinball2d".to_string(),
            width: 360.0,
            height: 640.0,
            ..Default::default()
        })
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WallsPlugin)
        .add_plugin(LauncherPlugin) 
        .add_plugin(FlippersPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(PinsPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.label("main_setup"))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vector2::zeros();
    rapier_config.gravity = Vec2::new(0.0, -220.0);

    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default());  
}

