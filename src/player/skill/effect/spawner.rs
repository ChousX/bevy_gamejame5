use crate::prelude::*;
use super::{SkillEffectType, SkillEffectDuration, SkillTarget};

pub struct SkillEffectSpawnerPlugin;
impl Plugin for SkillEffectSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnSkillEffect>()
            .add_systems(
                Update, 
                spawn_skill_effect.run_if(on_event::<SpawnSkillEffect>()))
            .add_systems(
                OnEnter(GamePhase::Tribulation), 
                spawn_boom_test)
    ;}
}

#[derive(Event)]
pub struct SpawnSkillEffect(pub SkillEffectType, pub SkillTarget);

//this can be split up into lots of systems
pub fn spawn_skill_effect(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnSkillEffect>,
    textures: Res<SkillEffectTexture>,
    transforms: Query<&Transform>,
) {
    use SkillEffectType::*;
    for SpawnSkillEffect(skill_type, target) in spawn_events.read(){
        let transform = match *target {
            SkillTarget::Entity(id) => transforms.get(id)
                            .expect("skill effect target entity did not exist").clone(),
            SkillTarget::Position(pos) => Transform::from_translation(pos.extend(0.91)),
        };

        match *skill_type {
            Boom => {
                commands.spawn((
                    SpriteBundle {
                        texture: textures.boom_texture.clone(),
                        transform,
                        ..default()
                    },
                    TextureAtlas::from(textures.boom_layout.clone()),
                    SkillEffectDuration::new(0.05, 17),
                ));
            },
        }
    }
}

fn spawn_boom_test(mut events: EventWriter<SpawnSkillEffect>){
    events.send(
        SpawnSkillEffect(
            SkillEffectType::Boom, 
            SkillTarget::Position(Vec2::new(0.0, 0.0))));
}
