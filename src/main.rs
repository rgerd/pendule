use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
    time::FixedTimestep, window::PresentMode,
};

const GRAVITY: f32 = 9.8 * 1000.; // 1 meter = 1000 units
const PENDULUM_LEN: f32 = 300.;
const MU: f32 = 1.;
const TIME_STEP: f32 = 1. / 60.;

#[derive(Component, Default)]
struct Pendulum {
    theta: f32,
    d_theta: f32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

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
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(TIME_STEP.into()))
                .with_system(oscillate),
        )
        .add_system(input)
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

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 100., 0.))),
            Pendulum::default(),
        ))
        .with_children(|child_builder| {
            // Line
            child_builder.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(18., PENDULUM_LEN)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                transform: Transform::from_translation(Vec3::new(0., -PENDULUM_LEN * 0.5, 100.)),
                ..default()
            });
            // Circle
            child_builder.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0., -PENDULUM_LEN, 500.)),
                ..default()
            });
        });
}

fn oscillate(mut pendulums: Query<(&mut Transform, &mut Pendulum)>) {
    for (mut transform, mut pendulum) in &mut pendulums {
        let dd_theta = -MU * pendulum.d_theta - (GRAVITY / PENDULUM_LEN) * pendulum.theta.sin();
        pendulum.d_theta += dd_theta * TIME_STEP;
        pendulum.theta += pendulum.d_theta * TIME_STEP;
        *transform = Transform::from_rotation(Quat::from_rotation_z(pendulum.theta));
        transform.translation.y = 200.;
    }
}

fn input(keys: Res<Input<KeyCode>>, mut pendulums: Query<&mut Pendulum>) {
    let sign: f32 = if keys.just_pressed(KeyCode::Right) {
        1.
    } else if keys.just_pressed(KeyCode::Left) {
        -1.
    } else {
        0.
    };
    for mut pendulum in &mut pendulums {
        pendulum.d_theta += 5. * sign;
    }
}
