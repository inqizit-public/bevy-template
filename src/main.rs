///! This example illustrates how to resize windows, and how to respond to a window being resized.
use bevy::{prelude::*, window::WindowResized};

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                title: "Game of Life".to_string(),
                // canvas: Some("#bevy".to_string()),
                ..default()
            },
            ..default()
        }))
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ui)
        .add_system(on_resize_system)
        .add_system(toggle_resolution)
        // .add_system(update_marker.after(on_resize_system))
        .run();
}

/// Marker component for the text that displays the current reslution.
#[derive(Component)]
struct ResolutionText;

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Spawns the UI
fn setup_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::YELLOW),
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ),
            ResolutionText,
            // Transform::IDENTITY,
        ));
    });
    // .with_children(|root| {
    //     // Text where we display current resolution
    //     root.spawn((
    //         SpriteBundle {
    //             texture: asset_server.load("icon.png"),
    //             transform: Transform::from_xyz(100.0, 100.0, 100.0),
    //             ..default()
    //         },
    //         Marker,
    //         // Transform::IDENTITY,
    //     ));
    // });
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    resolution: Res<ResolutionSettings>,
) {
    let window = windows.primary_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution.small;
        window.set_resolution(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution.medium;
        window.set_resolution(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let res = resolution.large;
        window.set_resolution(res.x, res.y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.iter() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
        // text.sections[0].
    }
}

// //Component to tag a marker
// #[derive(Component)]
// struct Marker;
// //system that updates the position of the marker
// fn update_marker(mut query: Query<&mut Transform, With<Marker>>, mut windows: ResMut<Windows>) {
//     //window resource has changed, so lets update the position
//     let window = windows.primary_mut();
//     for mut transform in &mut query {
//         transform.translation = find_bottom_right(&window);
//     }
// }
// //help function that finds the bottom right coordinates of the window
// fn find_bottom_right(window: &Window) -> Vec3 {
//     Vec3::new(window.width() / 2.0, window.height() / -2.0, 0.0)
// }
