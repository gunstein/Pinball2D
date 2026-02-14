use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

pub struct PinsPlugin;

impl Plugin for PinsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pins)
            .add_systems(Update, update_pins);
    }
}

#[derive(Component)]
struct Pin {
    timestamp_last_hit: f64,
    position: Vec2,
}

fn spawn_pins(mut commands: Commands) {
    let pins_pos: [Vec2; 3] = [
        Vec2::new(
            crate::PIXELS_PER_METER * -0.17,
            crate::PIXELS_PER_METER * 0.35,
        ),
        Vec2::new(
            crate::PIXELS_PER_METER * 0.17,
            crate::PIXELS_PER_METER * 0.35,
        ),
        Vec2::new(0.0, crate::PIXELS_PER_METER * 0.2),
    ];

    for pin_pos in pins_pos {
        spawn_single_pin(&mut commands, pin_pos, None);
    }
}

fn spawn_single_pin(commands: &mut Commands, position: Vec2, timestamp_last_hit: Option<f64>) {
    let shape_pin = shapes::Circle {
        radius: crate::PIXELS_PER_METER * 0.05,
        center: Vec2::ZERO,
    };

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = bevy::color::palettes::css::GREEN;
    if temp_timestamp_last_hit == 0.0 {
        color = bevy::color::palettes::css::TEAL;
    }

    commands
        .spawn((
            ShapeBuilder::with(&shape_pin)
                .fill(Color::BLACK)
                .stroke((color, 2.0))
                .build(),
            Transform::from_xyz(position.x, position.y, 0.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(shape_pin.radius))
        .insert(Restitution::coefficient(0.7))
        .insert(Pin {
            timestamp_last_hit: temp_timestamp_last_hit,
            position,
        });
}

fn update_pins(
    pins: Query<(Entity, &Pin)>,
    mut collision_events: MessageReader<CollisionEvent>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let now = time.elapsed_secs_f64();
    let mut hit_pins = HashSet::new();

    for collision_event in collision_events.read() {
        let CollisionEvent::Started(entity_a, entity_b, _event_flag) = collision_event else {
            continue;
        };

        if pins.get(*entity_a).is_ok() {
            hit_pins.insert(*entity_a);
        }

        if pins.get(*entity_b).is_ok() {
            hit_pins.insert(*entity_b);
        }
    }

    for entity in &hit_pins {
        let Ok((_, pin)) = pins.get(*entity) else {
            continue;
        };

        // Respawn to toggle color when hit.
        let pos = pin.position;
        commands.entity(*entity).despawn();
        spawn_single_pin(&mut commands, pos, Some(now));
    }

    for (entity, pin) in &pins {
        if hit_pins.contains(&entity) {
            continue;
        }

        let diff = now - pin.timestamp_last_hit;
        if pin.timestamp_last_hit > 0.0 && diff > 1.0 {
            // Color has been toggled for more than a second, so respawn.
            let pos = pin.position;
            commands.entity(entity).despawn();
            spawn_single_pin(&mut commands, pos, None);
        }
    }
}
