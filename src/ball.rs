use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use super::BottomWall;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, handle_ball_intersections_with_bottom_wall);
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(mut commands: Commands) {
    spawn_ball_entity(&mut commands);
}

fn spawn_ball_entity(commands: &mut Commands) {
    let ball_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.3,
        crate::PIXELS_PER_METER * -0.2,
    );

    let shape_ball = shapes::Circle {
        radius: crate::PIXELS_PER_METER * 0.03,
        center: Vec2::ZERO,
    };

    commands
        .spawn((
            ShapeBuilder::with(&shape_ball)
                .fill(Color::BLACK)
                .stroke((bevy::color::palettes::css::TEAL, 2.0))
                .build(),
            Transform::from_xyz(ball_pos.x, ball_pos.y, 0.0),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::ball(shape_ball.radius))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(Ball);
}

fn handle_ball_intersections_with_bottom_wall(
    mut collision_events: MessageReader<CollisionEvent>,
    query_ball: Query<(), With<Ball>>,
    query_bottom_wall: Query<(), With<BottomWall>>,
    mut commands: Commands,
) {
    let mut should_spawn_ball = false;

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = collision_event {
            let entity_ball = if query_ball.get(*entity_a).is_ok() {
                *entity_a
            } else if query_ball.get(*entity_b).is_ok() {
                *entity_b
            } else {
                continue;
            };

            let is_bottom_collision = query_bottom_wall.get(*entity_a).is_ok()
                || query_bottom_wall.get(*entity_b).is_ok();

            if is_bottom_collision {
                commands.entity(entity_ball).despawn();
                should_spawn_ball = true;
            }
        }
    }

    if should_spawn_ball {
        spawn_ball_entity(&mut commands);
    }
}
