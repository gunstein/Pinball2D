use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::{Point2};
use bevy_prototype_lyon::prelude::*;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_launcher.system().after("walls").label("launcher"))
            .add_system(launcher_movement.system());
    }
}

#[derive(Component)]
struct Launcher{
    start_point : Point2<f32>,
 }

fn spawn_launcher(
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>,
) {
    //Spawn launcher
    let shape_launcher = shapes::Rectangle {
        extents: Vec2::new(0.05*rapier_config.scale, 0.05*rapier_config.scale),
        origin: shapes::RectangleOrigin::Center
    };

    let launcher_pos : Point2<f32> = Point2::new(0.3, -0.29);

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_launcher,
                DrawMode::Outlined{
                    fill_mode: FillMode::color(Color::BLACK),
                    outline_mode: StrokeMode::new(Color::TEAL, 2.0),
                },
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicPositionBased.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_launcher.extents.x/rapier_config.scale/2.0, shape_launcher.extents.y/rapier_config.scale/2.0).into(),
            position: launcher_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Launcher{start_point: launcher_pos});
    
}

fn launcher_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut launcher_info: Query<(&Launcher, &mut RigidBodyPositionComponent)>,
) {
    for (launcher, mut rbodypos) in launcher_info.iter_mut() {
        let mut next_ypos = rbodypos.position.translation.vector.y;
        
        if keyboard_input.pressed(KeyCode::Space)
        {
            next_ypos = next_ypos + 0.04;
        }
        else
        {
            next_ypos = next_ypos - 0.04;
        }   
        let clamped_ypos = next_ypos.clamp(launcher.start_point.y, launcher.start_point.y + 0.05);
        rbodypos.next_position.translation.vector.y = clamped_ypos;    
    }
}
