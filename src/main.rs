use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;

use std::time::Duration;

#[derive(Component)]
struct DinoAnimation {
    timer: Timer,
}

/// DinoState is the state of the dino.
enum DinoState {
    /// Idle is set when the dino is not in moviment.
    Idle,
    /// Running is set when the dino is in moviment on horizontal axis.
    Running,
    /// Jumping is set when the dino is in moviment on vertical axis.
    Jumping,
    /// Dead is set when the dino is dead.
    Dead,
}

#[derive(Component)]
struct Dino {
    state: DinoState,
    sprite: Handle<TextureAtlas>,
}

/// Setup loads the game assets and set it to game world.
fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let dino_background = assets_server.load("background.png");

    let dino_spritesheet: Handle<Image> = assets_server.load("dino.png");

    let dino_running_atlas = atlases.add(TextureAtlas::from_grid(
        dino_spritesheet.clone(),
        Vec2::new(88.0, 96.0),
        2,
        1,
        None,
        Some(Vec2::new(1854.0, 0.0)),
    ));

    commands.spawn(SpriteBundle { // background.
        texture: dino_background,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands // ground.
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(2402.0 / 2.0, 4.0),
            ActiveEvents::CONTACT_FORCE_EVENTS,
        ))
        .insert(SpriteSheetBundle {
            texture_atlas: dino_ground_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        });

    commands // dino.
        .spawn((
            RigidBody::KinematicVelocityBased,
            KinematicCharacterController::default(),
            Collider::cuboid(88.0 / 2.0, 96.0 / 2.0),
            ColliderMassProperties::Density(1.0),
            ActiveEvents::CONTACT_FORCE_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
        ))
        .insert((
            SpriteSheetBundle {
                texture_atlas: dino_running_atlas.clone(),
                transform: Transform::from_xyz(0.0, 48.0, 2.0),
                ..default()
            },
            DinoAnimation {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
        ));
}

fn animate(time: Res<Time>, mut query: Query<(&mut DinoAnimation, &mut TextureAtlasSprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation
            .timer
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        if animation.timer.just_finished() {
            sprite.index = if sprite.index == 1 {
                0
            } else {
                sprite.index + 1
            };
        }
    }
}

fn movement(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut Transform,
            &mut ExternalImpulse,
            &mut KinematicCharacterController,
        ),
        With<DinoAnimation>,
    >,
) {
    let (mut velocity, mut transform, mut impulse, mut controller) = query.single_mut();

    if keyboard.just_pressed(KeyCode::Up) {
        velocity.linvel = Vec2::new(0.0, 400.0);
    }

    if keyboard.pressed(KeyCode::Left) {
        velocity.linvel = Vec2::new(-400.0, 0.0);
    }

    if keyboard.pressed(KeyCode::Right) {
        velocity.linvel = Vec2::new(400.0, 0.0);
    }

    if keyboard.just_released(KeyCode::Up) {
        velocity.linvel = Vec2::new(0.0, 0.0);
    }

    if keyboard.just_released(KeyCode::Left) {
        velocity.linvel = Vec2::new(0.0, 0.0);
    }

    if keyboard.just_released(KeyCode::Right) {
        velocity.linvel = Vec2::new(0.0, 0.0);
    }
}

fn read_result_system(controllers: Query<&KinematicCharacterControllerOutput>) {
    for output in controllers.iter() {
        //println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
        //         entity, output.effective_translation, output.grounded);
        // println!("{:?}", output);
    }
}

/* A system that displays the events. */
fn display_events(
    mut controller: Query<&mut KinematicCharacterController>,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        // println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn main() {
    App::new()
        // Essential plugins.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dino".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Development plugins.
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DebugLinesPlugin::default())
        // Game plugins.
        .add_startup_system(setup)
        .add_system(animate)
        .add_system(movement)
        .add_system(read_result_system)
        .add_system(display_events)
        .run();
}
