use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::{Point2};
use bevy_prototype_lyon::prelude::*;

pub struct PinsPlugin;

impl Plugin for PinsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_pins.system().after("launcher").label("pins"))
            .add_system(handle_pin_events.system())
            .add_system(respawn_pin_to_toggle_color.system());
    }
}

struct Pin{
    timestamp_last_hit : f64,
    position : Point2<f32>,
}

fn spawn_pins(    
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
)
{
    let pins_pos : [Point2<f32>;3] = [
        Point2::new(-0.17, 0.35),
        Point2::new(0.17, 0.35),
        Point2::new(0.0, 0.2),
    ];

    for i in 0..pins_pos.len() {
        let pin_pos = pins_pos[i];

        spawn_single_pin(&mut commands, &rapier_config, pin_pos, None);
    }
}


fn spawn_single_pin(    
    commands: &mut Commands,
    rapier_config: &Res<RapierConfiguration>,
    position: Point2<f32>,
    timestamp_last_hit: Option<f64>
)
{
    let shape_pin = shapes::Circle{
        radius: 0.05 * rapier_config.scale,
        center: Vec2::ZERO,
    };

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = Color::GREEN;
    if temp_timestamp_last_hit == 0.0{
        color = Color::TEAL;
    }

    commands.spawn()
    .insert_bundle(
        GeometryBuilder::build_as(
            &shape_pin,
            ShapeColors::outlined(color, Color::BLACK),
            DrawMode::Stroke(StrokeOptions::default().with_line_width(2.0)),
            Transform::default(),
        )
    )
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::ball(shape_pin.radius/rapier_config.scale),
        collider_type: ColliderType::Solid,
        flags: (ActiveEvents::CONTACT_EVENTS).into(),
        position: position.into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..ColliderBundle::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(ColliderDebugRender::with_id(0))
    .insert(Pin{timestamp_last_hit: temp_timestamp_last_hit, position: position });
}

/*
//This is how I would like to change color of pin when hit, but it's not working.
//Seems to be fixed in a not released version of lyon
fn change_draw_mode_of_pin(mut query: Query<(&Pin, &mut ShapeColors), With<Pin>>, time: Res<Time>) {
    for (pin, mut shape_colors) in query.iter_mut() {
        let diff = time.seconds_since_startup() - pin.timestamp_last_hit;
        if diff < 1.0{
            println!("test0");
            *shape_colors = 
                ShapeColors {
                    main: Color::GREEN,
                    outline: Color::GREEN,
                };
            //shape_colors.main = Color::GREEN;
            //shape_colors.outline = Color::GREEN;
        }
        else{
            println!("test1");
            *shape_colors = 
                ShapeColors {
                    main: Color::TEAL,
                    outline: Color::GREEN,
                };
            //shape_colors.main = Color::TEAL;
            //shape_colors.outline = Color::BLACK;
        }
    }
}
*/

fn respawn_pin_to_toggle_color(mut query: Query<(Entity, &Pin), With<Pin>>, 
        time: Res<Time>,
        mut commands: Commands,
        rapier_config: Res<RapierConfiguration>) {
    for (entity, pin) in query.iter_mut() {
        let diff = time.seconds_since_startup() - pin.timestamp_last_hit;
        if pin.timestamp_last_hit > 0.0 && diff > 1.0{
            //Color have been toggled for more than a second so respawn
            let pos = pin.position;
            commands.entity(entity).despawn();
            spawn_single_pin(&mut commands, &rapier_config, pos, None);
        }
    }
}

fn handle_pin_events(
    query: Query<(Entity, &Pin), With<Pin>>,
    time: Res<Time>,
    mut contact_events: EventReader<ContactEvent>,
    mut commands: Commands,
    rapier_config: Res<RapierConfiguration>
) {
    for contact_event in contact_events.iter() {
        for (entity, pin) in query.iter() {
            if let ContactEvent::Started(h1, h2) = contact_event {
                if h1.entity() == entity || h2.entity() == entity {
                    //Respawn to change color
                    let pos = pin.position;
                    let timestamp_last_hit = time.seconds_since_startup();
                    commands.entity(entity).despawn();
                    spawn_single_pin(&mut commands, &rapier_config, pos, Some(timestamp_last_hit));
                }
            }
        }
    }
}