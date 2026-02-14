use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_launcher)
            .add_systems(Update, launcher_movement);
    }
}

#[derive(Component)]
struct Launcher {
    start_point: Vec2,
}

const LAUNCHER_PULL_DISTANCE: f32 = crate::PIXELS_PER_METER * 0.05;
const LAUNCHER_SPEED_PER_SEC: f32 = crate::PIXELS_PER_METER * 2.4;

fn spawn_launcher(mut commands: Commands) {
    //Spawn launcher
    let shape_launcher = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.05,
            crate::PIXELS_PER_METER * 0.05,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    let launcher_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.3,
        crate::PIXELS_PER_METER * -0.58,
    );

    commands
        .spawn((
            ShapeBuilder::with(&shape_launcher)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            Transform::from_xyz(launcher_pos.x, launcher_pos.y, 0.0),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            shape_launcher.extents.x / 2.0,
            shape_launcher.extents.y / 2.0,
        ))
        .insert(Launcher {
            start_point: launcher_pos,
        });
}

fn launcher_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut launchers: Query<(&mut Launcher, &mut Transform)>,
) {
    for (launcher, mut launcher_transform) in launchers.iter_mut() {
        let next_ypos = next_launcher_y(
            launcher_transform.translation.y,
            keyboard_input.pressed(KeyCode::Space),
            launcher.start_point.y,
            time.delta_secs(),
        );
        launcher_transform.translation.y = next_ypos;
    }
}

fn next_launcher_y(current_y: f32, is_pressed: bool, start_y: f32, delta_secs: f32) -> f32 {
    let y_delta = LAUNCHER_SPEED_PER_SEC * delta_secs;
    let mut next_y = current_y;

    if is_pressed {
        next_y += y_delta;
    } else {
        next_y -= y_delta;
    }

    next_y.clamp(start_y, start_y + LAUNCHER_PULL_DISTANCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_respects_bounds() {
        let max_y = next_launcher_y(0.0, true, 0.0, 10.0);
        let min_y = next_launcher_y(0.0, false, 0.0, 10.0);

        assert_eq!(max_y, LAUNCHER_PULL_DISTANCE);
        assert_eq!(min_y, 0.0);
    }

    #[test]
    fn launcher_motion_is_time_based() {
        let small_steps = (0..10).fold(0.0, |y, _| next_launcher_y(y, true, 0.0, 1.0 / 600.0));
        let one_step = next_launcher_y(0.0, true, 0.0, 1.0 / 60.0);

        assert!((small_steps - one_step).abs() < 0.0001);
    }
}
