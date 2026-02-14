use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct FlippersPlugin;

impl Plugin for FlippersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flippers)
            .add_systems(Update, (left_flipper_movement, right_flipper_movement));
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

const FLIPPER_MIN_ANGLE: f32 = -0.3;
const FLIPPER_MAX_ANGLE: f32 = 0.3;
const FLIPPER_RAISE_SPEED_RAD_PER_SEC: f32 = 5.4;
const FLIPPER_RETURN_SPEED_RAD_PER_SEC: f32 = 4.2;

fn spawn_flippers(mut commands: Commands) {
    //Spawn flippers
    let shape_flipper: shapes::Rectangle = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.25,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    //Spawn left flipper
    let left_flipper_pos = Vec2::new(
        crate::PIXELS_PER_METER * -0.2,
        crate::PIXELS_PER_METER * -0.4,
    );

    commands
        .spawn((
            ShapeBuilder::with(&shape_flipper)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            Transform::from_xyz(left_flipper_pos.x, left_flipper_pos.y, 0.0),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            shape_flipper.extents.x / 2.0,
            shape_flipper.extents.y / 2.0,
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
            ShapeBuilder::with(&shape_flipper)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            Transform::from_xyz(right_flipper_pos.x, right_flipper_pos.y, 0.0),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            shape_flipper.extents.x / 2.0,
            shape_flipper.extents.y / 2.0,
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut left_flippers: Query<(&mut LeftFlipper, &mut Transform)>,
) {
    for (mut left_flipper, mut left_flipper_transform) in left_flippers.iter_mut() {
        let new_clamped_angle = next_left_flipper_angle(
            left_flipper.curr_angle,
            keyboard_input.pressed(KeyCode::ArrowLeft),
            time.delta_secs(),
        );
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - left_flipper.curr_angle);
        left_flipper_transform.rotate_around(left_flipper.point_of_rotation, pivot_rotation);
        left_flipper.curr_angle = new_clamped_angle;
    }
}

fn right_flipper_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut right_flippers: Query<(&mut RightFlipper, &mut Transform)>,
) {
    for (mut right_flipper, mut right_flipper_transform) in right_flippers.iter_mut() {
        let new_clamped_angle = next_right_flipper_angle(
            right_flipper.curr_angle,
            keyboard_input.pressed(KeyCode::ArrowRight),
            time.delta_secs(),
        );
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - right_flipper.curr_angle);
        right_flipper_transform.rotate_around(right_flipper.point_of_rotation, pivot_rotation);
        right_flipper.curr_angle = new_clamped_angle;
    }
}

fn next_left_flipper_angle(curr_angle: f32, is_pressed: bool, delta_secs: f32) -> f32 {
    let angle_delta = if is_pressed {
        FLIPPER_RAISE_SPEED_RAD_PER_SEC * delta_secs
    } else {
        -FLIPPER_RETURN_SPEED_RAD_PER_SEC * delta_secs
    };

    (curr_angle + angle_delta).clamp(FLIPPER_MIN_ANGLE, FLIPPER_MAX_ANGLE)
}

fn next_right_flipper_angle(curr_angle: f32, is_pressed: bool, delta_secs: f32) -> f32 {
    let angle_delta = if is_pressed {
        -FLIPPER_RAISE_SPEED_RAD_PER_SEC * delta_secs
    } else {
        FLIPPER_RETURN_SPEED_RAD_PER_SEC * delta_secs
    };

    (curr_angle + angle_delta).clamp(FLIPPER_MIN_ANGLE, FLIPPER_MAX_ANGLE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_flipper_angle_clamps_to_range() {
        let angle = next_left_flipper_angle(0.29, true, 1.0);
        assert_eq!(angle, FLIPPER_MAX_ANGLE);
    }

    #[test]
    fn right_flipper_angle_clamps_to_range() {
        let angle = next_right_flipper_angle(-0.29, true, 1.0);
        assert_eq!(angle, FLIPPER_MIN_ANGLE);
    }

    #[test]
    fn flipper_motion_is_time_based() {
        let small_steps = (0..10).fold(0.0, |angle, _| {
            next_left_flipper_angle(angle, true, 1.0 / 600.0)
        });
        let one_step = next_left_flipper_angle(0.0, true, 1.0 / 60.0);

        assert!((small_steps - one_step).abs() < 0.0001);
    }
}
