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
    mut query: Query<(&mut KinematicCharacterController, &ActionState<PlayerAction>, &Transform, &Jumper, &mut Velocity), With<Player>>,
    time: Res<Time>,
    rapier: Res<RapierConfiguration>,
) {
    for (mut character_controller, action_state, transform, jumper, mut velocity) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
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
        direction *= 5.0;
        direction *= time.delta_seconds();

        if jumper.has_ground_contact {
            if action_state.just_pressed(PlayerAction::Jump) {
                velocity.linvel.y = 100.0;
            } else {
                velocity.linvel.y = 0.0;
            }
        } else {
            info!("No ground contact");
            velocity.linvel.y += rapier.gravity.y;
        }

        info!("Velocity: {:?}", velocity.linvel);

        direction += velocity.linvel * time.delta_seconds();

        character_controller.translation = Some(direction);
    }
}

pub fn player_jump(
    mut query: Query<(&Jumper, &ActionState<PlayerAction>, &mut ExternalForce)>,
    rapier: Res<RapierConfiguration>,
    time: Res<Time>,
) {
    return;
    for (jumper, action_state, mut ext_force) in query.iter_mut() {
        if jumper.has_ground_contact {
            if action_state.just_pressed(PlayerAction::Jump) {
                ext_force.force += -rapier.gravity * 1000.0;
            } else {
                ext_force.force = rapier.gravity * time.delta_seconds();
            }
        } else {
            ext_force.force = rapier.gravity * time.delta_seconds();
        }
    }
}
