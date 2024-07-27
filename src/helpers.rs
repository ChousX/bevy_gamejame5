use crate::prelude::*;

pub fn despawn_all<T: Component>(
    query: Query<Entity, With<T>>,
    mut commands: Commands
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
impl Default for AnimationTimer{
    fn default() -> Self{
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

