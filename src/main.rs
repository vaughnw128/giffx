use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    prelude::*,
    render::RenderPlugin,
    time::TimeUpdateStrategy,
    winit::WinitPlugin,
};
use bevy_capture::{
    encoder::{gif},
    CameraTargetHeadless, Capture, CaptureBundle,
};
use std::{f32::consts::TAU, fs, time::Duration};

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {

    // Initialize the texture
    let texture_handle = asset_server.load("textures/image.png");

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // Spawn a cube to rotate.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: material_handle,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Rotatable { speed: 0.4 },
    ));

    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()}.target_headless(512, 512, &mut images),
           CaptureBundle::default())
        );

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default() });
}

fn update(
    mut app_exit: EventWriter<AppExit>,
    mut capture: Query<&mut Capture>,
    mut cubes: Query<(&mut Transform, &Rotatable)>,
    mut frame: Local<u32>,
) {
    let mut capture = capture.single_mut();
    if !capture.is_capturing() {
        capture.start(
            gif::GifEncoder::new(fs::File::create("simple.gif").unwrap())
                .with_repeat(gif::Repeat::Infinite)
        );
    }

    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(*frame as f32 / 60.0 * TAU * cube.speed)
    }

    println!("{:?}", frame);

    *frame += 1;
    if *frame >= 100 {
        app_exit.send(AppExit::Success);
    }
}

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: bevy::window::ExitCondition::DontExit,
                close_when_requested: false,
            })
            .set(RenderPlugin {
                synchronous_pipeline_compilation: true,
                ..default()
            }),
             ScheduleRunnerPlugin {
                run_mode: RunMode::Loop { wait: None },
            },
            bevy_capture::CapturePlugin,
        ));

    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
        1.0 / 60.0,
    )));

    app.add_systems(Startup, setup);

    app.add_systems(Update, update);

    app.run()
}