use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode, core_pipeline::clear_color::ClearColorConfig};

#[derive(Component)]
struct Rotator;

fn main() {
    let plugins = DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "pendule".to_string(),
            width: 800.,
            height: 640.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    });
    App::new()
        .add_plugins(plugins)
        .add_startup_system(setup)
        .add_system(oscillate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });

    let pendulum_len = 300.;
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 100., 0.))),
            Rotator,
        ))
        .with_children(|child_builder| {
            // Line
            child_builder.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(18., pendulum_len)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                transform: Transform::from_translation(Vec3::new(0., -pendulum_len * 0.5, 100.)),
                ..default()
            });
            // Circle
            child_builder.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0., -pendulum_len, 500.)),
                ..default()
            });
        });
}

fn oscillate(mut rotators: Query<&mut Transform, With<Rotator>>, timer: Res<Time>) {
    for mut transform in &mut rotators {
        *transform = Transform::from_rotation(Quat::from_rotation_z((timer.elapsed().as_secs_f32() * 1.2).sin()));
        transform.translation.y = 200.;
    }
}
