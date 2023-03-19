use crate::jumping::JumpPoint;
use crate::prelude::*;

#[derive(AssetCollection, Resource)]
struct GroundAssets {
    #[asset(path = "world/world.glb#Scene0")]
    world: Handle<Scene>,
}

fn spawn_ground(my_assets: Res<GroundAssets>, mut commands: Commands) {
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

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, GroundAssets>(GameState::Loading)
            .add_system(spawn_ground.in_schedule(OnEnter(GameState::Playing)));
    }
}
