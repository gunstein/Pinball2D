use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct FlippersPlugin;

impl Plugin for FlippersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_flippers)
            .add_system(left_flipper_movement)
            .add_system(right_flipper_movement);
    }
}

#[derive(Component)]
struct LeftFlipper {
    point_of_rotation: Vec3,
    curr_angle: f32,
}

#[derive(Component)]
struct RightFlipper {
    point_of_rotation: Vec3,
    curr_angle: f32,
}

fn spawn_flippers(mut commands: Commands) {
    //Spawn flippers
    let shape_flipper: shapes::Rectangle = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.25,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
    }
    .into();

    //Spawn left flipper
    let left_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * -0.2,
        crate::PIXELS_PER_METER * -0.4,
    );

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_flipper),
                ..default()
            },
            Fill::color(Color::BLACK),
            Stroke::new(Color::TEAL, 2.0),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            shape_flipper.extents.x / 2.0,
            shape_flipper.extents.y / 2.0,
        ))
        .insert(Transform::from_xyz(
            left_flipper_pos.x,
            left_flipper_pos.y,
            0.0,
        ))
        .insert(LeftFlipper {
            point_of_rotation: Vec3::new(
                left_flipper_pos.x - (shape_flipper.extents.x / 2.0),
                left_flipper_pos.y + (shape_flipper.extents.y) / 2.0,
                0.0,
            ),
            curr_angle: 0.0,
        });

    //Spawn right flipper
    let right_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.1,
        crate::PIXELS_PER_METER * -0.4,
    );

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_flipper),
                ..default()
            },
            Fill::color(Color::BLACK),
            Stroke::new(Color::TEAL, 2.0),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            shape_flipper.extents.x / 2.0,
            shape_flipper.extents.y / 2.0,
        ))
        .insert(Transform::from_xyz(
            right_flipper_pos.x,
            right_flipper_pos.y,
            0.0,
        ))
        .insert(RightFlipper {
            point_of_rotation: Vec3::new(
                right_flipper_pos.x + (shape_flipper.extents.x / 2.0),
                right_flipper_pos.y + (shape_flipper.extents.y) / 2.0,
                0.0,
            ),
            curr_angle: 0.0,
        });
}

fn left_flipper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut left_flippers: Query<(&mut LeftFlipper, &mut Transform), With<LeftFlipper>>,
) {
    for (mut left_flipper, mut left_flipper_transform) in left_flippers.iter_mut() {
        let mut new_angle = left_flipper.curr_angle;
        let change_angle: f32;

        if keyboard_input.pressed(KeyCode::Left) {
            change_angle = 0.09;
        } else {
            change_angle = -0.07;
        }

        new_angle += change_angle;
        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - left_flipper.curr_angle);
        left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);
        left_flipper.curr_angle = new_clamped_angle;
    }
}

fn right_flipper_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut right_flippers: Query<(&mut RightFlipper, &mut Transform), With<RightFlipper>>,
) {
    for (mut right_flipper, mut right_flipper_transform) in right_flippers.iter_mut() {
        let mut new_angle = right_flipper.curr_angle;
        let change_angle: f32;
        if keyboard_input.pressed(KeyCode::Right) {
            change_angle = -0.09;
        } else {
            change_angle = 0.07;
        }

        new_angle += change_angle;
        let new_clamped_angle = new_angle.clamp(-0.3, 0.3);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - right_flipper.curr_angle);
        right_flipper_transform.rotate_around(right_flipper.point_of_rotation, pivot_rotation);
        right_flipper.curr_angle = new_clamped_angle;
    }
}
