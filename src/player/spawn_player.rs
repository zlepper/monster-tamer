use crate::jumping::Jumper;
use crate::player::movement::PlayerAction;
use crate::player::roaming_camera::{CameraMovement, DEFAULT_CAMERA_VECTOR};
use crate::player::Player;
use crate::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "player/player.glb#Scene0")]
    model: Handle<Scene>,

    #[asset(path = "player/player.glb#Mesh0/Primitive0")]
    mesh: Handle<Mesh>,
}

pub fn spawn_player(
    player_assets: Res<PlayerAssets>,
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
) {
    info!("Spawning player");
    let player_mesh = meshes
        .get(&player_assets.mesh)
        .expect("Failed to load player mesh");

    let player_collider = Collider::from_bevy_mesh(player_mesh, &ComputedColliderShape::TriMesh)
        .expect("Failed to create collider for player");

    commands
        .spawn(SceneBundle {
            scene: player_assets.model.clone(),
            transform: Transform::from_xyz(2.0, 5.0, 2.0),
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(Jumper::default())
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_translation(DEFAULT_CAMERA_VECTOR),
                    ..default()
                })
                .insert(InputManagerBundle::<CameraMovement> {
                    input_map: InputMap::default()
                        .insert(DualAxis::mouse_motion(), CameraMovement::Rotate)
                        .build(),
                    ..default()
                })
                .insert(RigidBody::KinematicPositionBased)
                .insert(Collider::ball(0.05))
                .insert(KinematicCharacterController {
                    apply_impulse_to_dynamic_bodies: false,
                    slide: true,
                    autostep: None,
                    snap_to_ground: None,
                    offset: CharacterLength::Absolute(0.5),
                    ..default()
                })
                .insert(ActiveCollisionTypes::empty() | ActiveCollisionTypes::KINEMATIC_STATIC);
        })
        .insert(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (QwertyScanCode::W, PlayerAction::MoveForward),
                (QwertyScanCode::S, PlayerAction::MoveBackward),
                (QwertyScanCode::A, PlayerAction::MoveLeft),
                (QwertyScanCode::D, PlayerAction::MoveRight),
                (QwertyScanCode::Space, PlayerAction::Jump),
            ]),
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(player_collider)
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.05),
            ..default()
        })
        .insert(Velocity::default())
        .insert(ActiveEvents::COLLISION_EVENTS);
}
