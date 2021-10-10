use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::{Point2};
use bevy_prototype_lyon::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_walls.system().after("main_setup").label("walls"));
    }
}


fn spawn_walls(
    mut commands: Commands,
    rapier_config: ResMut<RapierConfiguration>,
) {
    //Spawn outer wall
    //Spawn top and bottom wall
    let shape_top_and_bottom_wall = shapes::Rectangle {
        width: 0.73*rapier_config.scale,
        height: 0.03*rapier_config.scale,
        origin: shapes::RectangleOrigin::Center
    };

    //Spawn bottom wall
    let bottom_wall_pos : Point2<f32> = Point2::new(0.0, -0.64);
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_top_and_bottom_wall,
                ShapeColors::outlined(Color::TEAL, Color::TEAL),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default(),
                },
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Sensor,
            shape: ColliderShape::cuboid(shape_top_and_bottom_wall.width/rapier_config.scale/2.0, shape_top_and_bottom_wall.height/rapier_config.scale/2.0),
            position: bottom_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
     
    //Spawn top wall
    let top_wall_pos : Point2<f32> = Point2::new(0.0, 0.64);
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_top_and_bottom_wall,
                ShapeColors::outlined(Color::TEAL, Color::TEAL),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default(),
                },
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_top_and_bottom_wall.width/rapier_config.scale/2.0, shape_top_and_bottom_wall.height/rapier_config.scale/2.0),
            position: top_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);        

    //Spawn left and right wall
    let shape_left_and_right_wall = shapes::Rectangle {
        width: 0.03*rapier_config.scale,
        height: 1.3*rapier_config.scale,
        origin: shapes::RectangleOrigin::Center
    };

    //Spawn left wall
    let left_wall_pos : Point2<f32> = Point2::new(-0.35, 0.0);
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_left_and_right_wall,
                ShapeColors::outlined(Color::TEAL, Color::TEAL),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default(),
                },
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_left_and_right_wall.width/rapier_config.scale/2.0, shape_left_and_right_wall.height/rapier_config.scale/2.0),
            position: left_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
     
    //Spawn right wall
    let right_wall_pos : Point2<f32> = Point2::new(0.35, 0.0);
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_left_and_right_wall,
                ShapeColors::outlined(Color::TEAL, Color::TEAL),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default(),
                },
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_left_and_right_wall.width/rapier_config.scale/2.0, shape_left_and_right_wall.height/rapier_config.scale/2.0),
            position: right_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);        

    //Spawn launcher wall
    let shape_launcher_wall = shapes::Rectangle {
        width: 0.03*rapier_config.scale,
        height: 0.5*rapier_config.scale,
        origin: shapes::RectangleOrigin::Center
    };

    let launcher_wall_pos : Point2<f32> = Point2::new(0.25, -0.36);
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &shape_launcher_wall,
                ShapeColors::outlined(Color::TEAL, Color::TEAL),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default(),
                },
                Transform::default(),
            )
        )
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(shape_launcher_wall.width/rapier_config.scale/2.0, shape_launcher_wall.height/rapier_config.scale/2.0),
            position: launcher_wall_pos.into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete); 


    //Spawn upper right obstruction
    let shape_upper_right_obstruction = shapes::Polygon{
        points: vec!(
            Vec2::new(0.0, 0.0), 
            Vec2::new(0.0, 0.25*rapier_config.scale),
            Vec2::new(-0.2*rapier_config.scale, 0.25*rapier_config.scale),
            ),
        closed: true
    };
    
    let upper_right_obstruction_pos : Point2<f32> = Point2::new(0.37, 0.4);

    commands
    .spawn()
    .insert_bundle(
        GeometryBuilder::build_as(
            &shape_upper_right_obstruction,
            ShapeColors::outlined(Color::TEAL, Color::TEAL),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default(),
            },
            Transform::default(),
        )
    )
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static,
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::polyline(vec!(
            Point2::new(0.0, 0.0), 
            Point2::new(0.0, 0.25),
            Point2::new(-0.2, 0.25),
            Point2::new(0.0, 0.0),
            ), None),
        position: upper_right_obstruction_pos.into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete); 
}