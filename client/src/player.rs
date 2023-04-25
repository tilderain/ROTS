use crate::{
    setup::CameraFollow,
    sprites::AnimationTimer,
    states::{FreeCamState, GameState},
};
use bevy::{
    input::mouse::{MouseWheel, MouseMotion}, prelude::*,
};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, ExternalImpulse, GravityScale, LockedAxes, RigidBody,
};
use bevy_sprite3d::{AtlasSprite3d, Sprite3dParams};

pub fn init(app: &mut App) -> &mut App {
    app.add_system(spawn_player_sprite.run_if(in_state(GameState::Ready).and_then(run_once())))
        .add_systems(
            (player_movement, camera_follow_system)
                .distributive_run_if(in_state(FreeCamState::Locked)),
        )
}

#[derive(Component)]
pub struct Jumper {
    pub cooldown: f32,
    pub timer: Timer,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerSpriteAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32.))]
    #[asset(texture_atlas(columns = 3, rows = 1))]
    #[asset(path = "brownSheet.png")]
    pub run: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct FaceCamera; // tag entity to make it always face the camera
#[derive(Reflect, Component)]
pub struct PlayerMovable;

#[derive(Reflect, Component)]
pub struct Player {
    pub looking_at: Vec3,
    pub facing_vel: f32,
    pub velocity: Vec3,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            // Look at camera
            looking_at: Vec3::new(10., 10., 10.),
            facing_vel: 0.,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn spawn_player_sprite(
    mut commands: Commands,
    images: Res<PlayerSpriteAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let starting_location = Vec3::new(-3., 0.5, 2.);
    let sprite = AtlasSprite3d {
        atlas: images.run.clone(),

        pixels_per_metre: 32.,
        partial_alpha: true,
        unlit: true,

        index: 1,

        transform: Transform::from_translation(starting_location).looking_at(Vec3::new(10., 10., 10.), Vec3::Y),
        // pivot: Some(Vec2::new(0.5, 0.5)),
        ..default()
    }
    .bundle(&mut sprite_params);

    commands
        .spawn(sprite) 
        .with_children(|parent| {
            parent.spawn(RigidBody::Dynamic)
            .insert(Collider::cuboid(0.3, 1., 1.))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(GravityScale(1.))
            .insert(PlayerMovable)
            .insert(Transform::from_translation(starting_location))
            .insert(Jumper{
                cooldown: 0.5,
                timer: Timer::from_seconds(1., TimerMode::Once),
            })
            .insert(ColliderMassProperties::Density(12.0));
        })
        .insert(Name::new("PlayerSprite"))
        .insert(Player::default())
        .insert(PlayerMovable)
        .insert(FaceCamera)
        .insert(Jumper{
            cooldown: 0.5,
            timer: Timer::from_seconds(1., TimerMode::Once),
        })
        .insert(Name::new("PlayerBody"))
        .insert(AnimationTimer(Timer::from_seconds(
            0.4,
            TimerMode::Repeating,
        )));
}

pub const PLAYER_SPEED: f32 = 5.;
pub fn player_movement(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, Entity, &mut Jumper), With<PlayerMovable>>,
    camera_query: Query<&CameraFollow>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let rotation= Vec2::new(
        f32::to_radians(camera_query.single().degrees).sin(),
        f32::to_radians(camera_query.single().degrees).cos()
    );
    dbg!(rotation);
for(mut transform, player_ent, mut jumper) in player_query.iter_mut() {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        // direction += rotation.xyy() * Vec3::new(1., 0., 1.);
        direction += Vec3::new(-rotation.x, 0., -rotation.y )
    }
    if keyboard_input.pressed(KeyCode::S) {
        // direction += rotation.xyy() * Vec3::new(-1., 0., -1.);
        direction += Vec3::new(rotation.x, 0., rotation.y )
    }
    if keyboard_input.pressed(KeyCode::A) {
        // direction = direction.mul_add(rotation.perp().xyy() * Vec3::new(1., 0., 1.), direction);
        direction += Vec3::new(rotation.perp().x, 0., rotation.perp().y )
    }
    if keyboard_input.pressed(KeyCode::D) {
        // direction = direction.mul_add(rotation.perp().xyy() * Vec3::new(-1., 0., -1.), direction);
        direction += Vec3::new(-rotation.perp().x, 0., -rotation.perp().y)
    }
    if keyboard_input.pressed(KeyCode::Space) {
        jumper.timer.tick(time.delta());
        if jumper.timer.just_finished() {
            commands.entity(player_ent).insert(ExternalImpulse {
                    impulse: Vec3::new(0., 400., 0.),
                    torque_impulse: Vec3::new(0., 0., 0.),
                });
                jumper.timer.reset();
            }
        }
        if direction.length() > 0. {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn camera_follow_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_input: Res<Input<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut CameraFollow), With<Camera3d>>,
    player_query: Query<&Transform, (With<Player>, Without<CameraFollow>)>,
) {
    for (_, mut camera_follow) in camera_query.iter_mut() {
        for event in mouse_wheel_events.iter() {
            camera_follow.distance = match event.y {
                y if y < 0. => (camera_follow.distance + 1.).abs(),
                y if y > 0. => (camera_follow.distance - 1.).abs(),
                _ => camera_follow.distance,
            };
            if camera_follow.distance < camera_follow.min_distance {
                camera_follow.distance = camera_follow.min_distance;
            } else if camera_follow.distance > camera_follow.max_distance {
                camera_follow.distance = camera_follow.max_distance;
            }
        }
        if mouse_input.pressed(MouseButton::Right) {
            for event in mouse_events.iter() {
                camera_follow.degrees += event.delta.x;
            }
        }
    }
    if let Ok(player_transform) = player_query.get_single() {
        for (mut transform, camera_follow) in camera_query.iter_mut() {
        let new_transform= Transform::from_translation(
                Vec3::new(
                    f32::to_radians(camera_follow.degrees).sin(),
                    1.,
                    f32::to_radians(camera_follow.degrees).cos()
                    )
                 * camera_follow.distance + player_transform.translation
             ).looking_at(player_transform.translation, Vec3::Y);
        transform.translation = new_transform.translation;
        transform.rotation = new_transform.rotation;
        }
    }
}
