use crate::{game::{GamePhase, Selected}, helpers::AnimationTimer, prelude::*};

mod soul;
mod body;
mod skill;

pub use soul::*;
pub use body::*;
pub use skill::*;

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
                SkillPlugin,
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
        BodyType::Chicken => (body_assets.chicken.clone(), TextureAtlas::from(body_assets.chicken_layout.clone())),
        BodyType::Crab => (body_assets.crab.clone(), TextureAtlas::from(body_assets.crab_layout.clone())),
        BodyType::Toad => (body_assets.toad.clone(), TextureAtlas::from(body_assets.toad_layout.clone())),
        BodyType::Monkey => (body_assets.monkey.clone(), TextureAtlas::from(body_assets.monkey_layout.clone())),
        BodyType::Pig => (body_assets.pig.clone(), TextureAtlas::from(body_assets.pig_layout.clone())),
        BodyType::Dog => (body_assets.dog.clone(), TextureAtlas::from(body_assets.dog_layout.clone())),
        BodyType::Cow => (body_assets.cow.clone(), TextureAtlas::from(body_assets.cow_layout.clone())),
        BodyType::Goose => (body_assets.goose.clone(), TextureAtlas::from(body_assets.goose_layout.clone())),
        BodyType::Gorilla => (body_assets.gorilla.clone(), TextureAtlas::from(body_assets.gorilla_layout.clone())),
        BodyType::Frog => (body_assets.frog.clone(), TextureAtlas::from(body_assets.frog_layout.clone())),
        BodyType::Boar => (body_assets.boar.clone(), TextureAtlas::from(body_assets.boar_layout.clone())),
        BodyType::Moose => (body_assets.moose.clone(), TextureAtlas::from(body_assets.moose_layout.clone())),
        BodyType::Cat => (body_assets.cat.clone(), TextureAtlas::from(body_assets.cat_layout.clone())),
        BodyType::Goat => (body_assets.goat.clone(), TextureAtlas::from(body_assets.goat_layout.clone())),
        BodyType::Sheep => (body_assets.sheep.clone(), TextureAtlas::from(body_assets.sheep_layout.clone())),
        BodyType::Turtle => (body_assets.turtle.clone(), TextureAtlas::from(body_assets.turtle_layout.clone())),
        BodyType::SnowFox => (body_assets.snow_fox.clone(), TextureAtlas::from(body_assets.snow_fox_layout.clone())),
        BodyType::Porcupine => (body_assets.porcupine.clone(), TextureAtlas::from(body_assets.porcupine_layout.clone())),
        BodyType::Skunk => (body_assets.skunk.clone(), TextureAtlas::from(body_assets.skunk_layout.clone())),
        BodyType::Wolf => (body_assets.wolf.clone(), TextureAtlas::from(body_assets.wolf_layout.clone())),
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


