use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude as lyon;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_walls.after("main_setup").label("walls"));
    }
}

#[derive(Component)]
pub struct BottomWall;

fn spawn_walls(
    mut commands: Commands
) {
    //Spawn outer wall
    //Spawn top and bottom wall
    let shape_top_and_bottom_wall = lyon::shapes::Rectangle {
        extents: Vec2::new(crate::PIXELS_PER_METER * 0.73, crate::PIXELS_PER_METER * 0.03),
        origin: lyon::shapes::RectangleOrigin::Center
    };

    //Spawn bottom wall
    let bottom_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * -0.64);
    commands
        .spawn()
        .insert_bundle(
            lyon::GeometryBuilder::build_as(
                &shape_top_and_bottom_wall,
                lyon::DrawMode::Outlined{
                    fill_mode: lyon::FillMode::color(Color::TEAL),
                    outline_mode: lyon::StrokeMode::color(Color::TEAL),
                },                
                Transform::default(),
            )
        )
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(shape_top_and_bottom_wall.extents.x/2.0, shape_top_and_bottom_wall.extents.y/2.0))
        .insert(Sensor)
        .insert(Transform::from_xyz(bottom_wall_pos.x, bottom_wall_pos.y, 0.0))
        .insert(BottomWall);
     
    //Spawn top wall
    let top_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * 0.64);
    commands
        .spawn()
        .insert_bundle(
            lyon::GeometryBuilder::build_as(
                &shape_top_and_bottom_wall,
                lyon::DrawMode::Outlined{
                    fill_mode: lyon::FillMode::color(Color::TEAL),
                    outline_mode: lyon::StrokeMode::color(Color::TEAL),
                },
                Transform::default(),
            )
        )
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(shape_top_and_bottom_wall.extents.x/2.0, shape_top_and_bottom_wall.extents.y/2.0))
        .insert(Transform::from_xyz(top_wall_pos.x, top_wall_pos.y, 0.0));        

    //Spawn left and right wall
    let shape_left_and_right_wall = lyon::shapes::Rectangle {
        extents: Vec2::new(crate::PIXELS_PER_METER * 0.03, crate::PIXELS_PER_METER * 1.3),
        origin: lyon::shapes::RectangleOrigin::Center
    };

    //Spawn left wall
    let left_wall_pos = Vec2::new(crate::PIXELS_PER_METER * -0.35, 0.0);
    commands
        .spawn()
        .insert_bundle(
            lyon::GeometryBuilder::build_as(
                &shape_left_and_right_wall,
                lyon::DrawMode::Outlined{
                    fill_mode: lyon::FillMode::color(Color::TEAL),
                    outline_mode: lyon::StrokeMode::color(Color::TEAL),
                },
                Transform::default(),
            )
        )
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(shape_left_and_right_wall.extents.x/2.0, shape_left_and_right_wall.extents.y/2.0))
        .insert(Transform::from_xyz(left_wall_pos.x, left_wall_pos.y, 0.0));
     
    //Spawn right wall
    let right_wall_pos = Vec2::new(crate::PIXELS_PER_METER * 0.35, 0.0);
    commands
        .spawn()
        .insert_bundle(
            lyon::GeometryBuilder::build_as(
                &shape_left_and_right_wall,
                lyon::DrawMode::Outlined{
                    fill_mode: lyon::FillMode::color(Color::TEAL),
                    outline_mode: lyon::StrokeMode::color(Color::TEAL),
                },
                Transform::default(),
            )
        )
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(shape_left_and_right_wall.extents.x/2.0, shape_left_and_right_wall.extents.y/2.0))
        .insert(Transform::from_xyz(right_wall_pos.x, right_wall_pos.y, 0.0));
        
    //Spawn launcher wall
    let shape_launcher_wall = lyon::shapes::Rectangle {
        extents: Vec2::new(crate::PIXELS_PER_METER * 0.03, crate::PIXELS_PER_METER * 0.5),
        origin: lyon::shapes::RectangleOrigin::Center
    };

    let launcher_wall_pos = Vec2::new(crate::PIXELS_PER_METER * 0.25, crate::PIXELS_PER_METER * -0.36);
    commands
        .spawn()
        .insert_bundle(
            lyon::GeometryBuilder::build_as(
                &shape_launcher_wall,
                lyon::DrawMode::Outlined{
                    fill_mode: lyon::FillMode::color(Color::TEAL),
                    outline_mode: lyon::StrokeMode::color(Color::TEAL),
                },
                Transform::default(),
            )
        )
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(shape_launcher_wall.extents.x/2.0, shape_launcher_wall.extents.y/2.0))
        .insert(Transform::from_xyz(launcher_wall_pos.x, launcher_wall_pos.y, 0.0));

    //Spawn upper right obstruction
    let shape_upper_right_obstruction = lyon::shapes::Polygon{
        points: vec!(
            Vec2::new(0.0, 0.0), 
            Vec2::new(0.0, crate::PIXELS_PER_METER * 0.25),
            Vec2::new(crate::PIXELS_PER_METER * -0.2, crate::PIXELS_PER_METER * 0.25),
            ),
        closed: true
    };
    
    let upper_right_obstruction_pos = Vec2::new(crate::PIXELS_PER_METER * 0.37, crate::PIXELS_PER_METER * 0.4);

    commands
    .spawn()
    .insert_bundle(
        lyon::GeometryBuilder::build_as(
            &shape_upper_right_obstruction,
            lyon::DrawMode::Outlined{
                fill_mode: lyon::FillMode::color(Color::TEAL),
                outline_mode: lyon::StrokeMode::color(Color::TEAL),
            },
            Transform::default(),
        )
    )
    .insert(RigidBody::Fixed)
    .insert(Collider::polyline(vec!(
        Vec2::new(0.0, 0.0), 
        Vec2::new(0.0, crate::PIXELS_PER_METER * 0.25),
        Vec2::new(crate::PIXELS_PER_METER * -0.2, crate::PIXELS_PER_METER * 0.25),
        Vec2::new(0.0, 0.0),
        ), None))
    .insert(Transform::from_xyz(upper_right_obstruction_pos.x, upper_right_obstruction_pos.y, 0.0));
}