use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WINDOWHEIGHT: f32 = 900.0;
const WINDOWWIDTH: f32 = 500.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "The Block Game".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(add_player.system())
        .add_startup_system(add_camera.system())
        .add_startup_system(spawn_walls.system())
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

//THE WALLS
fn spawn_walls(mut commands: Commands,
               mut materials: ResMut<Assets<ColorMaterial>>,
               rapier_config: ResMut<RapierConfiguration>)
{
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());

    // The floor
    // TODO
    // alright figure out what the hell is going on with this bullshit
    // you have got to line up all the different positions for all these components
    // NOTE
    // according to docs, x and y positions here automatically multiplied by raper_config.scale
    let floor_position: Vec2 = Vec2::new(0.0, -10.0);
    // println!("{}", rapier_config.scale); //is 20
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            // This is the origin point.
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            // And I suspect this is pixel units
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            position: floor_position.into(),
            // I suspect this is "physics units"
            shape: ColliderShape::cuboid(5.0, 5.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        // TODO
        // Hopefully you can figure out how to use the debugger lol
        .insert(ColliderDebugRender {
            color: Color::GOLD
        });

    // The Left Wall
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(0.0 - WINDOWWIDTH / 2.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(50.0, WINDOWHEIGHT)),
    //         ..Default::default()
    //     });

    // The Right Wall
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(0.0 + WINDOWWIDTH / 2.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(50.0, WINDOWHEIGHT)),
    //         ..Default::default()
    //     });
}


// The float value is the player movement speed in 'pixels/second'.
struct Player(f32);
fn add_player(mut commands: Commands,
              mut materials: ResMut<Assets<ColorMaterial>>,
              mut rapier_config: ResMut<RapierConfiguration>)
{
    let sprite_size_x = 40.0;
    let sprite_size_y = 40.0;

    // While we want our sprite to look ~40 px square, we want to keep the physics units smaller
    // to prevent float rounding problems. To do this, we set the scale factor in RapierConfiguration
    // and divide our sprite_size by the scale.
    rapier_config.scale = 20.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle(ColliderBundle {
            position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Player(300.0));
}
