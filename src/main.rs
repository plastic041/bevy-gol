use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

#[derive(Component)]
struct VerletObject {
    position_current: Vec2,
    position_previous: Vec2,
    acceleration: Vec2,
}

#[derive(Component)]
struct Constraint;

impl VerletObject {
    fn new(position: Vec2) -> Self {
        Self {
            position_current: position,
            position_previous: position,
            acceleration: Vec2::ZERO,
        }
    }

    fn update_position(&mut self, delta_time: f32) {
        let position_next = self.position_current
            + (self.position_current - self.position_previous)
            + (self.acceleration * delta_time * delta_time);
        self.position_previous = self.position_current;
        self.position_current = position_next;
        self.acceleration = Vec2::ZERO;
    }

    fn accelerate(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }
}

const BALL_RADIUS: f32 = 25.0;
const CONSTRAINT_POSITION: Vec2 = Vec2::new(0.0, 0.0);
const CONSTRAINT_RADIUS: f32 = 150.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a verlet object
    commands.spawn((
        VerletObject::new(Vec2::new(20.0, 0.0)),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform {
                translation: Vec3::new(20.0, 0.0, 100.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));

    // Spawn a constraint
    commands.spawn((
        Constraint,
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(CONSTRAINT_RADIUS).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::ANTIQUE_WHITE)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
}

const GRAVITY: Vec2 = Vec2::new(0.0, -1000.0);

fn update_verlet(mut query: Query<(&mut VerletObject, &mut Transform)>) {
    for (mut verlet_object, mut transform) in query.iter_mut() {
        // apply accelerate
        verlet_object.accelerate(GRAVITY);

        // apply constraint
        let to_obj = verlet_object.position_current - CONSTRAINT_POSITION;
        let dist = to_obj.length();
        if dist > CONSTRAINT_RADIUS - BALL_RADIUS {
            let n = to_obj / dist;
            verlet_object.position_current =
                CONSTRAINT_POSITION + n * (CONSTRAINT_RADIUS - BALL_RADIUS);
        }

        // update position
        verlet_object.update_position(1.0 / 60.0);
        transform.translation = Vec3::new(
            verlet_object.position_current.x,
            verlet_object.position_current.y,
            100.0,
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: {
                WindowDescriptor {
                    title: "Interesting Title".to_string(),
                    width: 600.0,
                    height: 400.0,
                    ..default()
                }
            },
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(update_verlet)
        .run();
}
