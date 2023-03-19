use crate::jumping::Jumper;
use crate::player::movement::PlayerAction;
use crate::player::roaming_camera::{CameraMovement, DEFAULT_CAMERA_DISTANCE};
use crate::player::Player;
use crate::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "player/player.glb#Scene0")]
    player: Handle<Scene>,
}

pub fn spawn_player(my_assets: Res<PlayerAssets>, mut commands: Commands) {
    commands
        .spawn(SceneBundle {
            scene: my_assets.player.clone(),
            transform: Transform::from_xyz(2.0, 5.0, 2.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Jumper::default())
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 3.0, DEFAULT_CAMERA_DISTANCE),
                    ..default()
                })
                .insert(InputManagerBundle::<CameraMovement> {
                    input_map: InputMap::default()
                        .insert(DualAxis::mouse_motion(), CameraMovement::Rotate)
                        .build(),
                    ..default()
                });
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
        .insert(KinematicCharacterController::default())
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ActiveEvents::COLLISION_EVENTS);
}
