use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use std::fmt;

const WINDOWHEIGHT: f32 = 900.0;
const WINDOWWIDTH: f32 = 500.0;
const JUMPVELOCITY: f32 = 300.0;
const GRAVITY: f32 = -150.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "The Block Game".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_player.system())
        .add_startup_system(add_camera.system())
        .add_startup_system(add_block.system())
        .add_startup_system(spawn_walls.system())
        .add_system(block_physics_system.system())
        .add_system(block_collisions_system.system())
        .add_system(player_collision_system.system())
        .add_system(player_movement_system.system())
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

//THE WALLS

fn spawn_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());

    // The Floor
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            // This is the origin point. It is in the middle of the screen.
            transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0 + 15.0, 0.0),
            // This is the width/length of wall
            // Starts expanding from the origin point in both directions
            sprite: Sprite::new(Vec2::new(500.0, 100.0)),
            ..Default::default()
        })
        .insert(Collidable);

    // The Left Wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0 - WINDOWWIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(50.0, WINDOWHEIGHT)),
            ..Default::default()
        })
        .insert(Collidable);

    // The Right Wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0 + WINDOWWIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(50.0, WINDOWHEIGHT)),
            ..Default::default()
        })
        .insert(Collidable);
}

// THE BLOCK

struct BlockPhysics {
    fall_speed: f32,
    is_falling: bool,
}

struct Collidable;

fn add_block(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());

    // test block
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(80.0, 50.0)),
            ..Default::default()
        })
        .insert(BlockPhysics {
            fall_speed: 100.0,
            is_falling: true,
        });

    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-80.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(60.0, 50.0)),
            ..Default::default()
        })
        .insert(BlockPhysics {
            fall_speed: 100.0,
            is_falling: true,
        });

    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-80.0, -50.0, 0.0),
            sprite: Sprite::new(Vec2::new(60.0, 50.0)),
            ..Default::default()
        })
        .insert(BlockPhysics {
            fall_speed: 100.0,
            is_falling: true,
        });
}

fn block_physics_system(time: Res<Time>, mut query: Query<(&BlockPhysics, &mut Transform)>) {
    for (block_physics, mut transform) in query.iter_mut() {
        if block_physics.is_falling == true {
            transform.translation.y -= block_physics.fall_speed * time.delta_seconds();
        }
    }
}

// Note* I really don't like the Collidable addition to this.
// It gets the queries to work but I wish there was a more elegant way to go about this
// Now falling blocks are not collidable :(
// So the player cannot touch them
fn block_collisions_system(
    mut commands: Commands,
    mut block_query: Query<
        (Entity, &mut Transform, &mut BlockPhysics, &Sprite),
        Without<Collidable>,
    >,
    collision_query: Query<(Entity, &Transform, &Sprite), With<Collidable>>,
) {
    for (block_entity, mut block_transform, mut block_physics, block_sprite) in
        block_query.iter_mut()
    {
        let block_size: Vec2 = block_sprite.size;

        for (_other_entity, transform, sprite) in collision_query.iter() {
            let possible_collision = collide(
                transform.translation,
                sprite.size,
                block_transform.translation,
                block_size,
            );

            if let Some(collision) = possible_collision {
                let block_position: &mut Vec3 = &mut block_transform.translation;
                match collision {
                    Collision::Bottom => {
                        // stop the block from falling
                        block_physics.is_falling = false;

                        // Make the block grounded so that it is collidable
                        commands.entity(block_entity).insert(Collidable);

                        // put the block right above the current ground
                        let collided_y_position: f32 = transform.translation.y;
                        block_position.y =
                            collided_y_position + block_size.y / 2.0 + sprite.size.y / 2.0;

                        //TODO eventually deal with player death here
                    }

                    // blocks should only collide at the bottom
                    _ => {}
                }
            }
        }
    }
}

// THE PLAYER

struct PlayerPhysics {
    jump_velocity: f32,
    gravity: f32,
    movement_speed: f32,
    is_grounded: IsGrounded,
}

enum IsGrounded {
    Grounded,
    Jumping,
    Falling
}

impl fmt::Display for IsGrounded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IsGrounded::Grounded => write!(f, "grounded"),
            IsGrounded::Jumping => write!(f, "jumping"),
            IsGrounded::Falling => write!(f, "falling"),
        }
    }
}

fn add_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0 + 180.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(PlayerPhysics {
            is_grounded: IsGrounded::Grounded,
            movement_speed: 100.0,
            jump_velocity: JUMPVELOCITY,
            gravity: GRAVITY,
        });
}

fn player_collision_system(
    mut player_query: Query<(&mut PlayerPhysics, &mut Transform, &Sprite)>,
    collision_query: Query<(&Transform, &Sprite), Without<PlayerPhysics>>,
) {
    match player_query.single_mut() {
        Ok((mut player_physics, mut player_transform, sprite)) => {
            let player_size = sprite.size;

            for (transform, sprite) in collision_query.iter() {
                // Note* Order of args matters
                let possible_collision = collide(
                    transform.translation,
                    sprite.size,
                    player_transform.translation,
                    player_size,
                );

                match possible_collision {
                    Some(collision) => {
                        let player_position: &mut Vec3 = &mut player_transform.translation;
                        match collision {
                            Collision::Bottom => {
                                // Reset jumping for the player
                                player_physics.jump_velocity = JUMPVELOCITY;
                                player_physics.is_grounded = IsGrounded::Grounded;

                                // Make sure player does not endlessly collide with the ground
                                let collided_y_position: f32 = transform.translation.y;
                                player_position.y =
                                    collided_y_position + player_size.y / 2.0 + sprite.size.y / 2.0;

                                // NOTE
                                // alright so here is the thing
                                // we are colliding with the ground and not colliding at the same time
                                // Or rather, I think that the ground collisions are occuring on different
                                // frames than the no collisions
                                //
                                // Honestly who knows whats going on here
                                println!("colliding with ground");
                            }
                            Collision::Left => {
                                let collided_x_position: f32 = transform.translation.x;
                                player_position.x =
                                    collided_x_position + player_size.x / 2.0 + sprite.size.x / 2.0;
                            }
                            Collision::Right => {
                                let collided_x_position: f32 = transform.translation.x;
                                player_position.x =
                                    collided_x_position - player_size.x / 2.0 - sprite.size.x / 2.0;
                            }
                            // come back to this when you can jump
                            Collision::Top => println!("Top"),
                        }
                    },
                    None => {
                        player_physics.is_grounded = IsGrounded::Falling;
                        println!("falling");
                    }
                }
            }
        }
        Err(msg) => println!("{}", msg),
    }
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerPhysics, &mut Transform)>,
) {
    match query.single_mut() {
        Ok((mut player_physics, mut transform)) => {
            let player_position: &mut Vec3 = &mut transform.translation;
            let mut direction: f32 = 0.0;

            // Apply gravity
            player_position.y += player_physics.gravity * time.delta_seconds();

            if keyboard_input.pressed(KeyCode::Left) {
                direction = -1.0;
            }

            if keyboard_input.pressed(KeyCode::Right) {
                direction = 1.0;
            }


            match player_physics.is_grounded {
                IsGrounded::Grounded => {
                    if keyboard_input.pressed(KeyCode::Up) {
                        player_position.y += 5.0;
                        player_physics.is_grounded = IsGrounded::Jumping;
                    }
                },
                IsGrounded::Jumping => {
                    // apply jump physics over time
                    // Note* this needs to be tuned up to make it more fun
                    // Currently jumping is like a standard parabola, both sides are equal and obeys mathematical properties
                    // But this is not fun
                    player_position.y += player_physics.jump_velocity * time.delta_seconds()
                        + 0.5
                            * player_physics.gravity
                            * time.delta_seconds()
                            * time.delta_seconds();
                    player_physics.jump_velocity += player_physics.gravity * time.delta_seconds();
                },
                IsGrounded::Falling => {
                }
            }

            player_position.x += direction * player_physics.movement_speed * time.delta_seconds();
        }
        Err(msg) => println!("{}", msg),
    }
}
