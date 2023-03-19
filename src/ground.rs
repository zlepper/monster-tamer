use crate::jumping::JumpPoint;
use crate::prelude::*;

fn spawn_ground(asset_server: Res<AssetServer>, mut commands: Commands) {
    let ground = asset_server.load("world/world.glb#Scene0");

    commands
        .spawn(SceneBundle {
            scene: ground,
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

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ground.in_schedule(OnEnter(GameState::Playing)));
    }
}
