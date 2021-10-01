use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;
use bevy_prototype_lyon::prelude::*;

mod ball;
use ball::*;

mod flippers;
use flippers::*;

mod walls;
use walls::*;

mod launcher;
use launcher::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pinball2d".to_string(),
            width: 360.0,
            height: 640.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WallsPlugin)
        .add_plugin(LauncherPlugin) 
        .add_plugin(FlippersPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system().label("main_setup"))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vector2::zeros();
    rapier_config.gravity = Vector2::new(0.0, -0.8);

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    rapier_config.scale = 640.0/1.3;    
}

