use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::{Point2};
use bevy_prototype_lyon::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_ball.system().after("launcher").label("ball"))
            .add_system(handle_ball_events.system());
    }
}

struct Ball;

fn spawn_ball(    
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>,
)
{
    let ball_pos : Point2<f32> = Point2::new(0.3, -0.2);

    let shape_ball = shapes::Circle{
        radius: 0.03 * rapier_config.scale,
        center: Vec2::ZERO,
    };

    commands.spawn()
    .insert_bundle(
        GeometryBuilder::build_as(
            &shape_ball,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
            Transform::default(),
        )
    )
    .insert_bundle(RigidBodyBundle {
        ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default() },
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::ball(shape_ball.radius/rapier_config.scale),
        collider_type: ColliderType::Solid,
        flags: (ActiveEvents::INTERSECTION_EVENTS).into(),
        position: ball_pos.into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..ColliderBundle::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(Ball);
}


fn handle_ball_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    query: Query<Entity, With<Ball>>,
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>
) {
    
    let mut should_spawn_ball = false;
    for intersection_event in intersection_events.iter() {
        for entity in query.iter() {
            if intersection_event.collider1.entity() == entity{
                commands.entity(entity).despawn();
                should_spawn_ball = true;
            }
            else if intersection_event.collider2.entity() == entity{
                commands.entity(entity).despawn();
                should_spawn_ball = true;
            }
        }
    }

    if should_spawn_ball
    {
        spawn_ball(commands, rapier_config);
    }
}