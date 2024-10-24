use std::f32::consts::{FRAC_PI_3, FRAC_PI_4, FRAC_PI_6};

use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::PickableBundle;
use sokudo_io::{read::{collider::ParsedShape, ParsedWorld}, write::ReadWorldStateHistory};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ColliderEntities>()
            .init_resource::<WorldStateIndex>()
            .init_resource::<DeltaTime>()
            .init_resource::<PlaybackTime>()
            .init_state::<PlayerState>()
            .add_systems(Startup, (setup_lights, setup_initial_state))
            .add_systems(
                PreUpdate,
                (
                    set_player_state_playing.run_if(in_state(PlayerState::Paused)),
                    set_player_state_paused.run_if(in_state(PlayerState::Playing)),
                    update_world_state.after(set_player_state_playing).run_if(in_state(PlayerState::Playing)),
                )
            )
            .add_systems(
                Update,
                (
                    update_colliders.run_if(any_with_component::<Collider>),
                ).run_if(in_state(PlayerState::Playing))
            );
    }
}

#[derive(Resource)]
pub struct InitialWorld {
    pub world: ParsedWorld,
}

#[derive(Resource, Default)]
pub struct DeltaTime {
    pub dt: f32,
}

#[derive(Resource, Default)]
pub struct PlaybackTime {
    pub time: f32,
}

#[derive(Resource)]
pub struct WorldStateHistory {
    pub history: ReadWorldStateHistory,
}

#[derive(Resource, Default)]
pub struct WorldStateIndex {
    pub step: usize,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum PlayerState {
    #[default]
    Paused,
    Playing
}

#[derive(Resource, Default)]
struct ColliderEntities {
    map: HashMap<u32, Entity>,
}

#[derive(Component)]
struct Collider;

fn setup_lights(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.brightness = 200.0;

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            2.0 * FRAC_PI_3,
            FRAC_PI_6,
            0.0,
        )),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 2000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            4.0 * FRAC_PI_3,
            FRAC_PI_3,
            0.0,
        )),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 500.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            0.0,
            FRAC_PI_4,
            0.0,
        )),
        ..default()
    });
}

fn setup_initial_state(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_entities: ResMut<ColliderEntities>,
    mut delta_time: ResMut<DeltaTime>,
    world: Res<InitialWorld>,
) {
    delta_time.dt = world.world.dt;

    for collider in world.world.colliders.iter() {
        let mesh: Mesh = match collider.shape {
            ParsedShape::Cuboid => Cuboid::new(1.0, 1.0, 1.0).into(),
        };

        let material = StandardMaterial::from_color(Color::srgb(1.0, 0.0, 0.0));

        let entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(material),
                transform: Transform {
                    translation: Vec3::new(
                        collider.transform.translate.x,
                        collider.transform.translate.y,
                        collider.transform.translate.z,
                    ),
                    rotation: Quat::from_xyzw(
                        collider.transform.rotate.x,
                        collider.transform.rotate.y,
                        collider.transform.rotate.z,
                        collider.transform.rotate.w,
                    ),
                    scale: Vec3::new(
                        collider.transform.scale.x,
                        collider.transform.scale.y,
                        collider.transform.scale.z,
                    ),
                },
                ..default()
            },
            Collider,
            PickableBundle::default(),
        )).id();

        collider_entities.map.insert(collider.id, entity);
    }
}

fn set_player_state_playing(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PlayerState>>,
    mut playback_time: ResMut<PlaybackTime>,
    time: Res<Time>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(PlayerState::Playing);
        playback_time.time = time.elapsed_seconds();
    }
}

fn set_player_state_paused(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(PlayerState::Paused)
    }
}

fn update_world_state(
    mut index: ResMut<WorldStateIndex>,
    mut next_state: ResMut<NextState<PlayerState>>,
    history: Res<WorldStateHistory>,
    delta_time: Res<DeltaTime>,
    mut playback_time: ResMut<PlaybackTime>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() - playback_time.time <= delta_time.dt {
        return;
    }

    playback_time.time += delta_time.dt;

    index.step += 1;

    if history.history.len() <= index.step {
        next_state.set(PlayerState::Paused);
        index.step = 0;
    }
}

fn update_colliders(
    collider_entities: Res<ColliderEntities>,
    mut colliders: Query<&mut Transform, With<Collider>>,
    index: Res<WorldStateIndex>,
    history: Res<WorldStateHistory>,
) {
    let world_state = history.history.get(index.step);

    for collider in world_state.colliders.iter() {
        let Some(&entity) = collider_entities.map.get(&collider.id) else {
            continue;
        };

        let Ok(mut transform) = colliders.get_mut(entity) else {
            continue;
        };

        transform.translation = Vec3::new(
            collider.transform.translate.x,
            collider.transform.translate.y,
            collider.transform.translate.z,
        );

        transform.rotation = Quat::from_xyzw(
            collider.transform.rotate.x,
            collider.transform.rotate.y,
            collider.transform.rotate.z,
            collider.transform.rotate.w,
        );
    }
}