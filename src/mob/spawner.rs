use crate::helpers::AnimationTimer;

use super::*;

pub struct MobSpawnerPlugin;
impl Plugin for MobSpawnerPlugin{
    fn name(&self) -> &str {
        "Mob Spawner Plugin"
    }
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnMob>()
            .add_systems(
                Update, 
                mob_spawner.run_if(on_event::<SpawnMob>()))
    ;}
}

#[derive(Event)]
pub enum SpawnMob {
    Single{
        mob_type: MobType,
        pos: Vec2,
    },

    Rectangle {
        mob_type: MobType,
        pos: Vec2,
        size: UVec2,
    },

    CirclePerimeter{
        mob_type: MobType,
        centrum: Vec2,
        raidas: f32,
    }
}

fn mob_spawner(
    mut events: EventReader<SpawnMob>,
    mut commands: Commands,
    textures: Res<MobTexture>,
){
    use SpawnMob::*;
    const Z: f32 = 0.9;
    for event in events.read(){

        fn additional_bundle(mob: &MobType, id: Entity, commands: &mut Commands){
            use MobType::*;
            let mut entity = commands.entity(id);
            match *mob{
                Grimlock => {
                },
                Eye => {
 
                },
                Ogre => {},
                CrimsonSlaad => {},
                Cyclops => {},
                Slime => {},
                Myconid => {},
                Ettin => {},
                MurkySlaad => {},
                Jelly => {},
                Watcher => {},
                RedCap => {},
                Mushroom => {},
                StoneTroll => {},
                SwampTroll => {},
            }; 
            entity.insert(
                (
                    Melee{ damage: 1.0 },
                    MobSpeed(10.0)
                    )
            );
        }

        match event{
            Single { mob_type, pos } => {
                let (texture, layout) = textures.get(&mob_type);
                let id =commands.spawn((
                    MobBundle {
                        sprite: SpriteBundle{
                            texture,
                            transform: Transform::from_translation(pos.extend(Z)),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas::from(layout),
                    AnimationTimer::default()
                )).id();
                additional_bundle(mob_type, id, &mut commands);
            },
            Rectangle { mob_type, pos, size } => {
                let mut entity_to_spawn = Vec::new();
                let (texture, layout) = textures.get(&mob_type);
                const SPACING: f32 = 40.0;
                let start = *pos - ( SPACING * Vec2::new(size.x as f32 * 0.5, -(size.y as f32 * 0.5)));
                for y in 0..size.y {
                    for x in 0..size.x {
                        let cp = (Vec2::new(x as f32 , y as f32) * SPACING) + start;
                        entity_to_spawn.push((
                            MobBundle{
                                sprite: SpriteBundle {
                                    texture: texture.clone(),
                                    transform: Transform::from_translation(cp.extend(Z)),
                                    ..default()
                                },
                                ..default()
                            },
                            layout.clone(),
                            AnimationTimer::default(),
                        ));
                    }
                }
                commands.spawn_batch(entity_to_spawn);
            },
            CirclePerimeter { mob_type, centrum, raidas } => {
                let (texture, layout) = textures.get(&mob_type);
                todo!()
            }
        }
    }
}
