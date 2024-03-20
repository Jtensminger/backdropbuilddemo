use bevy::window::PrimaryWindow;
use bevy::render::deterministic::DeterministicRenderingConfig;
use bevy::prelude::*;

pub struct HelperPlugin;
impl Plugin for HelperPlugin {
        fn build(&self, app: &mut App) {
                app.init_resource::<MyWorldCoords>()
                        .init_state::<CursorHelperState>()
                        
                        .add_systems(Startup, 
                                (
                                        setup_cameras,
                                        setup_cursor_world_position
                                ).chain()
                        )
                        .add_systems(OnEnter(CursorHelperState::Enabled), (
                                setup_cursor_position_text,
                        ))
                        .add_systems(OnExit(CursorHelperState::Enabled), (
                                despawn_cursor_position_text,
                        ))
                        .add_systems(Update, (
                                zoom_control_system,
                                toggle_cursor_helper,
                                (
                                        update_cursor_position,
                                        update_cursor_position_text
                                ).chain().run_if(in_state(CursorHelperState::Enabled)),
                        ));
        }
}
/// We will store the world position of the mouse cursor here.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum CursorHelperState {
        Enabled,
        #[default]
        Disabled,
}

fn toggle_cursor_helper(
        state: Res<State<CursorHelperState>>,
        mut next_state: ResMut<NextState<CursorHelperState>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
) {
        if keyboard_input.just_pressed(KeyCode::KeyC) {
                match state.get() {
                        CursorHelperState::Enabled  => next_state.set(CursorHelperState::Disabled),
                        CursorHelperState::Disabled => next_state.set(CursorHelperState::Enabled),
                }
        }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;


fn setup_cameras(
        mut commands: Commands,
        mut deterministic_rendering_config: ResMut<DeterministicRenderingConfig>,
) {
        deterministic_rendering_config.stable_sort_z_fighting = true;
        commands.spawn((Camera2dBundle::default(),MainCamera));
}

#[derive(Component)]
pub struct CursorPositionText;

fn setup_cursor_world_position(
        mut mycoords: ResMut<MyWorldCoords>,
        // query to get the window (so we can read the current cursor position)
        q_window: Query<&Window, With<PrimaryWindow>>,
        // query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();
    
        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();
    
        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
                mycoords.0 = world_position;
        }
}

fn update_cursor_position(
        mut mycoords: ResMut<MyWorldCoords>,
        mut cursor_evr: EventReader<CursorMoved>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
        // iter CursorMove events
        for ev in cursor_evr.read() {
                // check if cursor is outside thew window
                if let Some(delta) = ev.delta {
                        // check if cursor has moved
                        if delta.x != 0.0 && delta.y != 0.0 {
                                let (camera, camera_transform) = q_camera.single();

                                // update the cursor world coordinates resource
                                if let Some(world_position) = camera.viewport_to_world(camera_transform, ev.position)
                                        .map(|ray| ray.origin.truncate())
                                {
                                        mycoords.0 = world_position;
                                }
                        }
                }
        }
}

fn setup_cursor_position_text(
        mut commands: Commands
) {
                // Text with one section
                commands.spawn((
                        // Create a TextBundle that has a Text with a single section.
                        TextBundle::from_section(
                                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                                // format!("({:.2}, {:.2})", world_position.x, world_position.y),
                                "".to_string(),
                                TextStyle {
                                        // This font is loaded and will be used instead of the default font.
                                        font: default(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                },
                        ) // Set the justification of the Text
                        .with_text_justify(JustifyText::Center)
                        // Set the style of the TextBundle itself.
                        .with_style(Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(20.0),
                                right: Val::Px(20.0),
                                ..default()
                        }),
                        CursorPositionText,
                ));
}

fn update_cursor_position_text(
        mycoords: ResMut<MyWorldCoords>,
        mut cursor_evr: EventReader<CursorMoved>,
        mut query: Query<&mut Text, With<CursorPositionText>>,
) {
        // iter CursorMove events
        for ev in cursor_evr.read() {
                // check if cursor is outside thew window
                if let Some(delta) = ev.delta {
                        // check if cursor has moved
                        if delta.x != 0.0 && delta.y != 0.0 {
                                // update the compontent text w/ new cursor position
                                for mut text in &mut query {
                                        let world_position = mycoords.0.clone();
                                        text.sections[0].value = format!("({:.2}, {:.2})", world_position.x, world_position.y);
                                }  
                        }
                }
        }
}

fn despawn_cursor_position_text(
        mut commands: Commands,
        query: Query<Entity, With<CursorPositionText>>,
) {
        for entity in query.iter() {
                commands.entity(entity).despawn();
        }        
}

fn zoom_control_system(
        input: Res<ButtonInput<KeyCode>>,
        mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
        let mut projection = camera_query.single_mut();
    
        if input.pressed(KeyCode::Minus) {
            projection.scale += 0.2;
        }
    
        if input.pressed(KeyCode::Equal) {
            projection.scale -= 0.2;
        }
    
        projection.scale = projection.scale.clamp(0.2, 5.);
}
    