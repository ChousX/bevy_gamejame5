use crate::{game::{GamePhase, Selected}, helpers::AnimationTimer, prelude::*};

mod soul;
mod body;

pub use soul::*;
pub use body::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Enter)

            )
            .add_systems(
                Update,
                player_builder.run_if(in_state(GamePhase::BodySelection)))
            .add_plugins((
                BodyPlugin,
                SoulPlugin,
            ));
    }
}

fn player_builder(
    mut commands: Commands,
    selected_soul: Query<(&SoulName, &Magnetism, &Presence, Entity), (With<Selected>, With<SoulRoot>)>,
    selected_body: Query<(&Speed, &HitPoints, &HitPointRegeneration, &Size, &BodyType, Entity), (With<Selected>, With<BodyRoot>)>,
    mut game_phase: ResMut<NextState<GamePhase>>,
    body_assets: Res<BodyTexture>,
){
    let (body, soul)  = if !selected_body.is_empty() && !selected_soul.is_empty() {
            (selected_body.single(), selected_soul.single())
    } else {
        return;
    };
    //Get Data to Build Player
    let (name, magnetism, presence, soul_entity) = soul;
    let (speed, hit_points, hp_regen, size, body_type, body_entity) = body;

    let soul = SoulBundle {
        root: SoulRoot,
        name: name.clone(),
        magnetism: magnetism.clone(),
        presence: presence.clone(),
    };

    let body = BodyBundle {
        root: BodyRoot,
        speed: speed.clone(),
        hit_points: hit_points.clone(),
        hit_points_regen: hp_regen.clone(),
        size: size.clone(),
        body_type: body_type.clone()
    };

    let pick_up_range = PickupRange(size.0 + magnetism.0);
    let aoe = AreaOfEffect(size.0 + presence.0);

    let (body_texture, texture_atlas) = match body_type {
        BodyType::Crab => (body_assets.crab.clone(), TextureAtlas::from(body_assets.crab_layout.clone())),
        _ => (body_assets.crab.clone(), TextureAtlas::from(body_assets.crab_layout.clone()))
    };

    //Build Player
    let _player = commands.spawn((
        PlayerBundle {
            root: PlayerRoot,
            pick_up_range,
            aoe,
            body,
            soul,
            sprite: SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                texture: body_texture,
                ..default()
            },
        },
        texture_atlas,
        AnimationTimer::default(),
    )).id();

    //Despawn selected 
    commands.entity(body_entity).despawn_recursive();
    commands.entity(soul_entity).despawn_recursive();

    //move to next phase
    game_phase.set(GamePhase::Prepration);
}

#[derive(Component, Default)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct PickupRange(pub f32);
impl Default for PickupRange {
    fn default() -> Self{
        Self(5.0)
    }
}

#[derive(Component)]
pub struct AreaOfEffect(pub f32);
impl Default for AreaOfEffect {
    fn default() -> Self{
        Self(1.0)
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub root: PlayerRoot,
    pub pick_up_range: PickupRange,
    pub aoe: AreaOfEffect,
    pub soul: SoulBundle,
    pub body: BodyBundle,
    pub sprite: SpriteBundle,
}


