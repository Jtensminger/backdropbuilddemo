use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod helper;
use helper::HelperPlugin;


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::WHITE))

        .add_plugins(DefaultPlugins)
        .add_plugins(HelperPlugin)
        .add_plugins(ShapePlugin)

        .add_systems(Update, (
                setup_circle,
                setup_sink,
                setup_arrow,
                setup_interface,
                setup_circum_points,
        ))
        .run();
}

fn setup_circle(mut commands: Commands) {
        /* Draw a shape in the center of the screen */
        let shape = shapes::Circle {
                radius: 300.0,
                center: Vec2::new(0.0, 0.0),
        };
        
        commands.spawn((
                ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        ..default()
                },
                Fill::color(Color::ORANGE_RED),
                Stroke::new(Color::BLACK, 5.0),
        ));

        /* Circle Origin Point */
        commands.spawn((
                ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Circle {
                                radius: 2.0,
                                center: Vec2::new(0., 0.),
                        }),
                        spatial: SpatialBundle {
                                transform: Transform::from_xyz(0., 0., 4.),
                                ..default()
                        },
                        ..default()
                },
                Stroke::new(Color::BLACK, 1.0),
                Fill::color(Color::CYAN),
        ));
}

fn setup_sink(mut commands: Commands) {
        let (wall, basin) = (75.0, 150.0);
        
        let mut path_builder = PathBuilder::new();
        
        // start point
        path_builder.move_to(Vec2::new(-wall, -basin / 2.));
        // wall line
        path_builder.line_to(Vec2::new(0., -basin / 2.));
        // basin line
        path_builder.line_to(Vec2::new(0., basin / 2.));
        // wall line
        path_builder.line_to(Vec2::new(-wall, basin / 2.));

        let path = path_builder.build();
    
        commands.spawn((
            ShapeBundle {
                path,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(-500., 0., 3.),
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::BLACK, 5.0),
        ));
}

fn setup_arrow(mut commands: Commands) {
        let mut path_builder = PathBuilder::new();
        // line
        let from = Vec2::new(-500., 0.);
        let to   = Vec2::new(-315., 0.);
        let ctrl = Vec2::new(-400., 0.);
        
        path_builder.move_to(from);
        path_builder.quadratic_bezier_to(ctrl, to);

        // arrow
        let base_up   = Vec2::new(0., 10.);
        let base_down = Vec2::new(0., -10.);
        let tip       = Vec2::new(15., 0.);
        path_builder.line_to(to + base_up);
        path_builder.line_to(to + tip);
        path_builder.line_to(to + base_down);
        path_builder.line_to(to);

        let path = path_builder.build();
    
        commands.spawn((
            ShapeBundle {
                path,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::BLUE, 3.0),
        ));
}

fn setup_interface(mut commands: Commands) {
        
        let points = [
                Vec2::new(11., 5.), // top right
                Vec2::new(1.,  5.), // top left
                Vec2::new(1.,  1.), // bottom left
                Vec2::new(11., 1.), // bottom right
        ].map(|x| x * 10.);

        //             Top Right        Bottom Left
        let centre_x = (&points[0][0] + &points[2][0]) / 2.;
        let centre_y = (&points[0][1] + &points[2][1]) / 2.;
        let centroid = Vec2::new(centre_x, centre_y);
        //info!("Centre: {:?}", centroid);

        let shape = shapes::RoundedPolygon {
                points: points.into_iter().collect(),
                radius: 5.,
                closed: false,
        };

        commands.spawn((
                ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        spatial: SpatialBundle {
                                transform: Transform::from_xyz(0., 0., 4.),
                                ..default()
                        },
                        ..default()
                },
                Stroke::new(Color::BLACK, 2.0),
                Fill::color(Color::CYAN),
        ));

        let circle = shapes::Circle {
                radius: 2.0,
                center: centroid,
        };

        commands.spawn((
                ShapeBundle {
                        path: GeometryBuilder::build_as(&circle),
                        spatial: SpatialBundle {
                                transform: Transform::from_xyz(0., 0., 4.),
                                ..default()
                        },
                        ..default()
                },
                Stroke::new(Color::BLACK, 1.0),
                Fill::color(Color::CYAN),
        ));
}

fn setup_circum_points(mut commands: Commands) {
        let j = 0.;   // x-coordinate of the circle's origin
        let k = 0.;   // y-coordinate of the circle's origin
        let r = 300.; // radius of the circle
        let step = 2; // Angle step in degrees
    
        for t in (0..=360).step_by(step) {
                let radian = t as f32 * PI / 180.0; // Convert degree to radian
                let x = r * radian.cos() + j;       // Calculate x-coordinate
                let y = r * radian.sin() + k;       // Calculate y-coordinate
                let circum_point = shapes::Circle {
                        radius: 2.0,
                        center: Vec2::new(x, y),
                };
        
                commands.spawn((
                        ShapeBundle {
                                path: GeometryBuilder::build_as(&circum_point),
                                spatial: SpatialBundle {
                                        transform: Transform::from_xyz(0., 0., 4.),
                                        ..default()
                                },
                                ..default()
                        },
                        Stroke::new(Color::BLACK, 1.0),
                        Fill::color(Color::CYAN),
                ));
        }
}