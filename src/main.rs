use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(keyboard_input_system.system())
        .run();
}

#[derive(Bundle, Copy, Clone)]
struct PlayerAnimation {
    current_action: Action,
    idle_animation: Animation,
    run_animation: Animation,
    attack_animation: Animation,
    animation_locked: bool,
    current_index: u32,
}

#[derive(Bundle, Copy, Clone)]
struct Player {
    position: f32,
    speed: f32,
    direction: Direction
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Copy, Clone)]
enum Action {
    Attack,
    Run,
    Idle,
}

#[derive(Copy, Clone)]
struct Animation {
    start: u32,
    end: u32,
}

impl PlayerAnimation {
    fn new() -> Self {
        PlayerAnimation {
            current_action: Action::Idle,
            idle_animation: Animation {
                start: 4,
                end: 6,
            },
            run_animation: Animation {
                start: 7,
                end: 12,
            },
            attack_animation: Animation {
                start: 0,
                end: 3,
            },
            animation_locked: false,
            current_index: 0
        }
    }
    fn next_sprite(self, current_index: u32) -> u32 {
        // self.animation_locked = true;

        match self.current_action {
            Action::Idle => {
                if current_index >= self.idle_animation.end || current_index < self.idle_animation.start {
                    return self.idle_animation.start
                }
            },
            Action::Run => {
                if current_index >= self.run_animation.end || current_index < self.run_animation.start {
                    return self.run_animation.start
                }
            },
            Action::Attack => {
                if current_index >= self.attack_animation.end || current_index < self.attack_animation.start {

                    return self.attack_animation.start
                }
            }
        }
        return current_index + 1
    }

    fn animation_finished(self) -> bool {
        return match self.current_action {
            Action::Idle => {
                self.current_index == self.idle_animation.end
            },
            Action::Run => {
                self.current_index == self.run_animation.end
            },
            Action::Attack => {
                self.current_index == self.attack_animation.end
            }
        }
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut PlayerAnimation, &Player)>,
) {
    for (mut timer, mut sprite, mut player_anim, player) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = player_anim.next_sprite(sprite.index);
            player_anim.current_index = sprite.index;
            match player.direction {
                Direction::Right => {
                    sprite.flip_x = false;
                },
                Direction::Left => {
                    sprite.flip_x = true;
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let idle_texture_handle = asset_server.load("textures/adventurer.png");
    let idle_texture_atlas = TextureAtlas::from_grid(idle_texture_handle, Vec2::new(50.0, 37.0), 4, 4);
    let idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);


    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: idle_texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(PlayerAnimation::new())
        .insert(Player {
            position: 0.0,
            speed: 0.0,
            direction: Direction::Right
        });
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut PlayerAnimation, &mut Player)>) {
    let (mut player_animation, mut player) = query.iter_mut().next().unwrap();
    if !player_animation.animation_locked {
        if keyboard_input.pressed(KeyCode::Space) {
            player_animation.current_action = Action::Attack;
            player_animation.animation_locked = true;
        } else if keyboard_input.pressed(KeyCode::Right) {
            player.direction = Direction::Right;
            player_animation.current_action = Action::Run;
        } else if keyboard_input.pressed(KeyCode::Left) {
            player.direction = Direction::Left;
            player_animation.current_action = Action::Run;
        } else {
            player_animation.current_action = Action::Idle;
        }
        // println!("direction: {:#?}", player.direction);
    } else {
        if player_animation.animation_locked && player_animation.animation_finished() {
            player_animation.animation_locked = false;
        }
    }
}
