use bevy::prelude::{Commands, Component, Entity, error, EventReader, info, Visibility};
use bevy::ui::{Display, Style};

#[derive(Debug)]
pub enum SideEffectWriter {
    ChangeVisible(Entity, Visibility),
    ChangeStyle(Entity, Style)
}

pub fn component_propagation_system(
    mut commands: Commands,
    mut propagation_reader: EventReader<SideEffectWriter>
) {
    for event in propagation_reader.into_iter() {
        match event {
            SideEffectWriter::ChangeVisible(entity, component) => {
                info!("Adding propagation visibility event: {:?} with entity {:?}", component, entity);
                add_component(&mut commands, entity, *component)
            }
            SideEffectWriter::ChangeStyle(entity, component) => {
                info!("Adding propagation style event: {:?} with entity {:?}", component, entity);
                add_component(&mut commands, entity, component.clone())
            }
        }
    }
}

fn add_component<T>(commands: &mut Commands, e: &Entity, to_add: T)
where
    T: Component
{
    let _ = commands.get_entity(*e)
        .as_mut()
        .map(|entity| {
            entity.insert(to_add);
        })
        .or_else(|| {
            error!("Could not find entity to make hidden.");
            None
        });
}

/// Determines where to get the starting state from, which determines the next state. For instance,
/// if a child is swapping from visible to invisible, and the parent is swapping, then in order so
/// that they won't swap out of sync, you use starting state of one to determine next state of both.
#[derive(Clone, Debug)]
pub enum Relationship {
    Child,
    Parent,
    SelfState,
    EachSelfState,
    Sibling,
    SiblingChild,
    Other(f32),
    VisibleState(Display)
}

