use std::f32::consts::PI;
use bevy::{input::mouse::{self, MouseButtonInput}, prelude::*};
use bevy_prototype_lyon::prelude::*;

mod helper;
use helper::{HelperPlugin, MyWorldCoords};
mod toolbar_menu;
use toolbar_menu::ToolbarMenuPlugin;


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(CircumPoints::default())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        
        .add_plugins(DefaultPlugins)
        .add_plugins(ToolbarMenuPlugin)
        .add_plugins(HelperPlugin)
        .add_plugins(ShapePlugin)

        .add_systems(Startup, (
                setup_circle,
                setup_circum_points,
                // setup_sink,
                //setup_arrow,
                //setup_interface,
        ).chain())
        .add_systems(Update, (
                on_mouse_input,
                update_interface,
        ).chain())
        .run();
}

// temporary function for demo purposes
fn on_mouse_input(
        mut commands: Commands,
        mouse_button_input: ResMut<'_, ButtonInput<MouseButton>>,
        mut cp: ResMut<CircumPoints>
) { 
        // create sink & arrow
        if mouse_button_input.just_pressed(MouseButton::Left) {
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
                Stroke::new(Color::BLACK, 3.0),
                ));

        }
        // create interface
        if mouse_button_input.just_pressed(MouseButton::Right) {                          
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
                let interface_depth = 4.;
                commands.spawn((
                        ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                spatial: SpatialBundle {
                                        transform: Transform::from_xyz(-312., 60., interface_depth),
                                        ..default()
                                },
                                ..default()
                        },
                        Stroke::new(Color::BLACK, 3.0),
                        Fill::color(Color::WHITE),
                        InterfaceMarker,
                ));

        }

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
            Stroke::new(Color::BLACK, 3.0),
        ));
}

#[derive(Component)]
struct InterfaceMarker;

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
        let interface_depth = 4.;
        commands.spawn((
                ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        spatial: SpatialBundle {
                                transform: Transform::from_xyz(-312., 60., interface_depth),
                                ..default()
                        },
                        ..default()
                },
                Stroke::new(Color::BLACK, 3.0),
                Fill::color(Color::WHITE),
                InterfaceMarker,
        ));

        // let circle = shapes::Circle {
        //         radius: 2.0,
        //         center: centroid,
        // };

        // commands.spawn((
        //         ShapeBundle {
        //                 path: GeometryBuilder::build_as(&circle),
        //                 spatial: SpatialBundle {
        //                         transform: Transform::from_xyz(0., 0., interface_depth + 1.),
        //                         ..default()
        //                 },
        //                 ..default()
        //         },
        //         Stroke::new(Color::BLACK, 1.0),
        //         Fill::color(Color::BLACk),
        // ));
}

#[derive(Resource, Default)]
struct CircumPoints(Vec<Vec2>);
fn setup_circum_points(
        mut commands: Commands,
        mut cp: ResMut<CircumPoints>,
        cursor_pos: Res<MyWorldCoords>
) {
        let j = 0.;   // x-coordinate of the circle's origin
        let k = 0.;   // y-coordinate of the circle's origin
        let r = 300.; // radius of the circle
        let step = 2; // Angle step in degrees
        cp.0.clear(); // remove default values
        for t in (0..=360).step_by(step) {
                let radian = t as f32 * PI / 180.0; // Convert degree to radian
                let x = r * radian.cos() + j;       // Calculate x-coordinate
                let y = r * radian.sin() + k;       // Calculate y-coordinate
                cp.0.push(Vec2::new(x, y));
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

fn update_interface(
        mut query: Query<&mut Transform, With<InterfaceMarker>>,
        cp: Res<CircumPoints>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        time: Res<Time>,
) {        
        // let circum_point_target = Vec3::new(0., 0., 0.);
        // // rotate the interface to the circum_point
        // //t.look_at(circum_point_target, Vec3::Y);
        // t.rotate_around(point, rotation);
        let mut movement_factor = 0.;
        let mut rotation_factor = 0.;
        let rotation_speed = f32::to_radians(45.);

        // update transform to new coordinates and angle based on the circum_point
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                movement_factor = 10.;
        } 
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                movement_factor = -10.;
        } 
        // if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        //         rotation_factor += 10.0;
        // }
        // if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        //         rotation_factor -= 10.0;
        // }
        if query.is_empty() {
                return;
        }

        // interface transform
        let mut t = query.single_mut();

        // find the closest circum_point to the Inteface
        let closest_cp = find_closest_point(cp.0.as_ref(), &t.translation.xy()).unwrap();

        // // update the rotation around the Z axis (perpendicular to the 2D plane of the screen)
        t.rotate_z(rotation_factor);
        // get the forward vector by applying the current rotation to the interfaces initial facing vector
        // let movement_direction = t.rotation * Vec3::Y;
        // // get the distance the ship will move based on direction, the ship's movement speed and delta time
        // let movement_distance = movement_factor;
        // // create the change in translation using the new movement direction and distance
        // let translation_delta = movement_direction * movement_distance;
        // // update the ship translation with our new translation delta
        // t.translation += translation_delta;
        // // get the circum_point's translation in 2D
        let cp_translation = Transform::from_xyz(1., 60., 0.).translation.xy();
        // // get the vector from the circum_point to the interface in 2D and normalize it.
        let to_cp = (cp_translation - t.translation.xy()).normalize();
        // // get the quaternion to rotate from the initial interface facing direction to the direction facing the cp
        let rotate_to_cp = Quat::from_rotation_arc(Vec3::Y, to_cp.extend(0.));
        // // rotate the enemy to face the player
        t.rotation = rotate_to_cp;

        // // get the dot product between the interface forward vector and the direction to the circum_point.
        // let forward_dot_circum = movement_direction.xy().dot(to_cp);

        // // get the right vector of the enemy ship in 2D (already unit length)
        // let interface_right = (t.rotation * Vec3::X).xy();
        
        // // get the dot product of the enemy right vector and the direction to the player ship.
        // // if the dot product is negative them we need to rotate counter clockwise, if it is
        // // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
        // // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
        // // with the right vector).
        // let right_dot_cp = interface_right.dot(to_cp);

        // // determine the sign of rotation from the right dot player. We need to negate the sign
        // // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
        // // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
        // // negative is clockwise.
        // let rotation_sign = -f32::copysign(1.0, right_dot_cp);

        // // limit rotation so we don't overshoot the target. We need to convert our dot product to
        // // an angle here so we can get an angle of rotation to clamp against.
        // let max_angle = forward_dot_circum.clamp(-1.0, 1.0).acos(); // clamp acos for safety

        // // calculate angle of rotation with limit
        // let rotation_angle = rotation_sign * (rotation_speed * time.delta_seconds()).min(max_angle);
        
        // // rotate the interface to face the circum_point
        // t.rotate_z(rotation_angle);
}

fn find_closest_point(points: &Vec<Vec2>, comparison_point: &Vec2) -> Option<Vec2> {
        points
            .iter()
            .min_by(|a, b| {
                // Calculate the squared distance to avoid sqrt for comparison
                let distance_a = (comparison_point.x - a.x).powi(2) + (comparison_point.y - a.y).powi(2);
                let distance_b = (comparison_point.x - b.x).powi(2) + (comparison_point.y - b.y).powi(2);
    
                distance_a.partial_cmp(&distance_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied() // Copy the value to return it, since iter() returns references
}