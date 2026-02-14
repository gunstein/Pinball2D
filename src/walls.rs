use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

#[derive(Component)]
pub struct BottomWall;

fn spawn_walls(mut commands: Commands) {
    //Spawn outer wall
    //Spawn top and bottom wall
    let shape_top_and_bottom_wall = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.73,
            crate::PIXELS_PER_METER * 0.03,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    //Spawn bottom wall
    let bottom_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * -0.64);
    commands
        .spawn((
            ShapeBuilder::with(&shape_top_and_bottom_wall)
                .fill(bevy::color::palettes::css::TEAL)
                .build(),
            Transform::from_xyz(bottom_wall_pos.x, bottom_wall_pos.y, 0.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            shape_top_and_bottom_wall.extents.x / 2.0,
            shape_top_and_bottom_wall.extents.y / 2.0,
        ))
        .insert(Sensor)
        .insert(BottomWall);

    //Spawn top wall
    let top_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * 0.64);
    commands
        .spawn((
            ShapeBuilder::with(&shape_top_and_bottom_wall)
                .fill(bevy::color::palettes::css::TEAL)
                .build(),
            Transform::from_xyz(top_wall_pos.x, top_wall_pos.y, 0.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            shape_top_and_bottom_wall.extents.x / 2.0,
            shape_top_and_bottom_wall.extents.y / 2.0,
        ));

    //Spawn left and right wall
    let shape_left_and_right_wall = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.03,
            crate::PIXELS_PER_METER * 1.3,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    //Spawn left wall
    let left_wall_pos = Vec2::new(crate::PIXELS_PER_METER * -0.35, 0.0);
    commands
        .spawn((
            ShapeBuilder::with(&shape_left_and_right_wall)
                .fill(bevy::color::palettes::css::TEAL)
                .build(),
            Transform::from_xyz(left_wall_pos.x, left_wall_pos.y, 0.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            shape_left_and_right_wall.extents.x / 2.0,
            shape_left_and_right_wall.extents.y / 2.0,
        ));

    //Spawn right wall
    let right_wall_pos = Vec2::new(crate::PIXELS_PER_METER * 0.35, 0.0);
    commands
        .spawn((
            ShapeBuilder::with(&shape_left_and_right_wall)
                .fill(bevy::color::palettes::css::TEAL)
                .build(),
            Transform::from_xyz(right_wall_pos.x, right_wall_pos.y, 0.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            shape_left_and_right_wall.extents.x / 2.0,
            shape_left_and_right_wall.extents.y / 2.0,
        ));

    //Spawn launcher wall
    let shape_launcher_wall = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.03,
            crate::PIXELS_PER_METER * 0.5,
        ),
        origin: shapes::RectangleOrigin::Center,
        radii: None,
    };

    let launcher_wall_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.25,
        crate::PIXELS_PER_METER * -0.36,
    );
    commands
        .spawn((
            ShapeBuilder::with(&shape_launcher_wall)
                .fill(bevy::color::palettes::css::TEAL)
                .build(),
            Transform::from_xyz(launcher_wall_pos.x, launcher_wall_pos.y, 0.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            shape_launcher_wall.extents.x / 2.0,
            shape_launcher_wall.extents.y / 2.0,
        ));

    //Spawn upper right obstruction
    let shape_upper_right_obstruction = shapes::Polygon {
        points: vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, crate::PIXELS_PER_METER * 0.25),
            Vec2::new(
                crate::PIXELS_PER_METER * -0.2,
                crate::PIXELS_PER_METER * 0.25,
            ),
        ],
        closed: true,
    };

    let upper_right_obstruction_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.37,
        crate::PIXELS_PER_METER * 0.4,
    );

    commands
        .spawn((
            ShapeBuilder::with(&shape_upper_right_obstruction)
                .fill(bevy::color::palettes::css::TEAL)
                .build(),
            Transform::from_xyz(
                upper_right_obstruction_pos.x,
                upper_right_obstruction_pos.y,
                0.0,
            ),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::polyline(
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, crate::PIXELS_PER_METER * 0.25),
                Vec2::new(
                    crate::PIXELS_PER_METER * -0.2,
                    crate::PIXELS_PER_METER * 0.25,
                ),
                Vec2::new(0.0, 0.0),
            ],
            None,
        ));
}
