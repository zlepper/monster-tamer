use crate::prelude::*;

#[derive(Component, Debug)]
pub struct JumpPoint;


#[derive(Component, Default, Debug)]
pub struct Jumper {
    pub has_ground_contact: bool,
}


fn allow_jumpers_to_jump(
    rapier_context: Res<RapierContext>,
    mut jumpers: Query<(&mut Jumper, Entity), Without<KinematicCharacterControllerOutput>>,
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

fn allow_kinematic_jumpers_to_jump(
    mut jumpers: Query<(&mut Jumper, &KinematicCharacterControllerOutput)>,
) {
    for (mut jumper, kinematic_output) in jumpers.iter_mut() {
        jumper.has_ground_contact = kinematic_output.grounded;
    }
}

pub struct JumpingPlugin;

impl Plugin for JumpingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((allow_jumpers_to_jump, allow_kinematic_jumpers_to_jump));
    }
}
