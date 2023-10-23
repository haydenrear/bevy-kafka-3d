use bevy::prelude::{Entity, Event};
use crate::interactions::{HoverEvent, SelectionEvent};

#[derive(Event)]
pub enum PickableEvent {
    Selection(SelectionEvent),
    Hover(HoverEvent),
    Clicked(Entity)
}

pub enum PickableComponentState {
    Spawned(ComponentSpawned)
}

pub enum ComponentSpawned {
    ComponentSpawned,
    ComponentNotSpawned,
    Any
}
