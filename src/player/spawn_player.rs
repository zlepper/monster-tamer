use crate::jumping::Jumper;
use crate::player::movement::PlayerAction;
use crate::player::roaming_camera::{CameraMovement, DEFAULT_CAMERA_VECTOR};
use crate::player::Player;
use crate::prelude::*;

pub fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    let player_model = asset_server.load("player/player.glb#Scene0");

    commands
        .spawn(SceneBundle {
            scene: player_model,
            transform: Transform::from_xyz(2.0, 5.0, 2.0),
            ..Default::default()
        })
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
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(0.6, 2.0, 0.8))
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.05),
            ..default()
        })
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ActiveEvents::COLLISION_EVENTS);
}
