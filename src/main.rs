use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::Loading)
        .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
        .add_system(spawn_world.in_schedule(OnEnter(GameState::Playing)))
        .add_system(move_player)
        .add_system(allow_jumpers_to_jump)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    Jump,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "world/world.glb#Scene0")]
    world: Handle<Scene>,

    #[asset(path = "player/player.glb#Scene0")]
    player: Handle<Scene>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Component, Debug)]
struct JumpPoint;

fn spawn_world(my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn(SceneBundle {
            scene: my_assets.world.clone(),
            ..Default::default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(RigidBody::Fixed)
        .insert(JumpPoint);

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Component)]
struct Player;

#[derive(Component, Default, Debug)]
struct Jumper {
    has_ground_contact: bool,
}

fn spawn_player(my_assets: Res<MyAssets>, mut commands: Commands) {
    commands
        .spawn(SceneBundle {
            scene: my_assets.player.clone(),
            transform: Transform::from_xyz(2.0, 5.0, 2.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Jumper::default())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 3.0, 10.0),
                ..default()
            });
        })
        .insert(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (QwertyScanCode::W, Action::MoveForward),
                (QwertyScanCode::S, Action::MoveBackward),
                (QwertyScanCode::A, Action::MoveLeft),
                (QwertyScanCode::D, Action::MoveRight),
                (QwertyScanCode::Space, Action::Jump),
            ]),
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(0.6, 2.0, 0.8))
        .insert(KinematicCharacterController::default())
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
struct MovePlayerQuery {
    character_controller: &'static mut KinematicCharacterController,
    action_state: &'static ActionState<Action>,
    transform: &'static Transform,
    ext_force: &'static mut ExternalForce,
    jumper: &'static Jumper,
    _p: With<Player>,
}

fn move_player(
    mut query: Query<MovePlayerQuery>,
    time: Res<Time>,
    rapier: Res<RapierConfiguration>,
) {
    for mut player in query.iter_mut() {
        let mut direction = rapier.gravity * time.delta_seconds();
        if player.action_state.pressed(Action::MoveForward) {
            direction += player.transform.forward();
        }
        if player.action_state.pressed(Action::MoveBackward) {
            direction += player.transform.back();
        }
        if player.action_state.pressed(Action::MoveLeft) {
            direction += player.transform.left();
        }
        if player.action_state.pressed(Action::MoveRight) {
            direction += player.transform.right();
        }
        if player.jumper.has_ground_contact && player.action_state.just_pressed(Action::Jump) {
            player.ext_force.force += -rapier.gravity * 1000.0;
        } else {
            player.ext_force.force = Vec3::ZERO;
        }
        if direction != Vec3::ZERO {
            player.character_controller.translation = Some(direction * time.delta_seconds() * 5.0);
        }
    }
}

fn allow_jumpers_to_jump(
    rapier_context: Res<RapierContext>,
    mut jumpers: Query<(&mut Jumper, Entity)>,
    jump_points: Query<Entity, With<JumpPoint>>,
) {
    for (mut jumper, jumper_entity) in jumpers.iter_mut() {
        let mut has_ground_contact = false;
        for jump_point in jump_points.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(jumper_entity, jump_point) {
                has_ground_contact = contact_pair.has_any_active_contacts();
                break;
            }
        }
        jumper.has_ground_contact = has_ground_contact;
    }
}