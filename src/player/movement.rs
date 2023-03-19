use crate::prelude::*;

use crate::jumping::Jumper;
use crate::player::Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    Jump,
}

pub fn move_player(
    mut query: Query<(&mut KinematicCharacterController, &ActionState<PlayerAction>, &Transform), With<Player>>,
    time: Res<Time>,
    rapier: Res<RapierConfiguration>,
) {
    for (mut character_controller, action_state, transform) in query.iter_mut() {
        let mut direction = rapier.gravity * time.delta_seconds();
        if action_state.pressed(PlayerAction::MoveForward) {
            direction += transform.forward();
        }
        if action_state.pressed(PlayerAction::MoveBackward) {
            direction += transform.back();
        }
        if action_state.pressed(PlayerAction::MoveLeft) {
            direction += transform.left();
        }
        if action_state.pressed(PlayerAction::MoveRight) {
            direction += transform.right();
        }
        if direction != Vec3::ZERO {
            character_controller.translation = Some(direction * time.delta_seconds() * 5.0);
        }
    }
}

pub fn player_jump(
    mut query: Query<(&Jumper, &ActionState<PlayerAction>, &mut ExternalForce)>,
    rapier: Res<RapierConfiguration>,
) {
    for (jumper, action_state, mut ext_force) in query.iter_mut() {
        if jumper.has_ground_contact && action_state.just_pressed(PlayerAction::Jump) {
            ext_force.force += -rapier.gravity * 1000.0;
        } else {
            ext_force.force = Vec3::ZERO;
        }
    }
}
