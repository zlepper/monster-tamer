use crate::jumping::JumpPoint;
use crate::prelude::*;



#[derive(AssetCollection, Resource)]
pub struct GroundAssets {
    #[asset(path = "world/world.glb#Scene0")]
    pub ground: Handle<Scene>,
    #[asset(path = "world/world.glb#Mesh0/Primitive0")]
    pub mesh: Handle<Mesh>,
}


fn spawn_ground(ground_assets: Res<GroundAssets>, mut commands: Commands, meshes: Res<Assets<Mesh>>) {
    println!("Spawning ground");

    let ground_mesh = meshes.get(&ground_assets.mesh).expect("Failed to load ground mesh");

    let collider = Collider::from_bevy_mesh(ground_mesh, &ComputedColliderShape::TriMesh).expect("Failed to create collider for world ground");

    commands
        .spawn(SceneBundle {
            scene: ground_assets.ground.clone(),
            ..Default::default()
        })
        .insert(Name::new("Ground"))
        .insert(collider)
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
        app
            .add_collection_to_loading_state::<_, GroundAssets>(GameState::LoadingFromDisk)
            .add_system(spawn_ground.in_schedule(OnEnter(GameState::Playing)));
    }
}
