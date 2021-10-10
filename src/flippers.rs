use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::{Isometry2, UnitComplex, Point2};
use bevy_prototype_lyon::prelude::*;

pub struct FlippersPlugin;

impl Plugin for FlippersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_flippers.system().after("walls").label("flippers"))
            .add_system(left_flipper_movement.system())
            .add_system(right_flipper_movement.system());
    }
}

struct LeftFlipper{
    point_of_rotation : Point2<f32>,
 }
 
 struct RightFlipper{
     point_of_rotation : Point2<f32>,
  }

fn spawn_flippers(
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>,
) {
    //Spawn flippers
    let shape_flipper = shapes::Rectangle {
        width: 0.25*rapier_config.scale,
        height: 0.05*rapier_config.scale,
        origin: shapes::RectangleOrigin::Center
    };

    //Spawn left flipper
    let left_flipper_pos : Point2<f32> = Point2::new(-0.2, -0.4);

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_flipper,
                ShapeColors::outlined(Color::TEAL, Color::BLACK),
                DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicPositionBased,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_flipper.width/rapier_config.scale/2.0, shape_flipper.height/rapier_config.scale/2.0),
            position: left_flipper_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(LeftFlipper{point_of_rotation: Point2::new(left_flipper_pos.x -((shape_flipper.width/rapier_config.scale)/2.0), left_flipper_pos.y +(shape_flipper.height/rapier_config.scale)/2.0)});

        //Spawn right flipper
        let right_flipper_pos : Point2<f32> = Point2::new(0.1, -0.4);

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_flipper,
                ShapeColors::outlined(Color::TEAL, Color::BLACK),
                DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicPositionBased,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_flipper.width/rapier_config.scale/2.0, shape_flipper.height/rapier_config.scale/2.0),
            position: right_flipper_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(RightFlipper{point_of_rotation: Point2::new(right_flipper_pos.x + ((shape_flipper.width/rapier_config.scale)/2.0), right_flipper_pos.y +(shape_flipper.height/rapier_config.scale)/2.0)});
    
}

fn left_flipper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut flipper_info: Query<(&LeftFlipper, &mut RigidBodyPosition)>,
) {
    for (flipper, mut rbodypos) in flipper_info.iter_mut() {

        let mut next_rotation_angle = rbodypos.position.rotation.angle();
        if keyboard_input.pressed(KeyCode::Left)
        {
            next_rotation_angle += 0.09;
        }
        else
        {
            next_rotation_angle -= 0.07;
        }

        let clamped_angle = next_rotation_angle.clamp(-0.3, 0.3);

        rbodypos.next_position = Isometry2::rotation_wrt_point(UnitComplex::new(clamped_angle), flipper.point_of_rotation);
    }
}

fn right_flipper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut flipper_info: Query<(&RightFlipper, &mut RigidBodyPosition)>,
) {
    for (flipper, mut rbodypos) in flipper_info.iter_mut() {

        let mut next_rotation_angle = rbodypos.position.rotation.angle();
        if keyboard_input.pressed(KeyCode::Right)
        {
            next_rotation_angle -= 0.09;
        }
        else
        {
            next_rotation_angle += 0.07;
        }

        let clamped_angle = next_rotation_angle.clamp(-0.3, 0.3);

        rbodypos.next_position = Isometry2::rotation_wrt_point(UnitComplex::new(clamped_angle), flipper.point_of_rotation);
    }
}