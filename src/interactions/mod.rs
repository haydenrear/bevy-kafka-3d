use std::marker::PhantomData;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::{Entity, Interaction};
use ndarray::s;

/// Convert all interactions to these events so that ui and 3d are comparable.
pub enum InteractionEvent<T: ReadOnlyWorldQuery> {
    BevyUiInteraction{event: Interaction},
    RaycastInteraction{event: PickingEvent<T>}
}

/// An event that triggers when the selection state of a [Selection] enabled [PickableMesh] changes.
#[derive(Debug, Clone)]
pub enum SelectionEvent {
    JustSelected(Entity),
    JustDeselected(Entity),
}

/// An event that triggers when the hover state of a [Hover] enabled [PickableMesh] changes.
#[derive(Debug, Clone)]
pub enum HoverEvent {
    JustEntered(Entity),
    JustLeft(Entity),
}

/// An event that wraps selection and hover events
#[derive(Debug, Clone)]
pub enum PickingEvent<Q: ReadOnlyWorldQuery> {
    Selection(SelectionEvent, PhantomData<Q>),
    Hover(HoverEvent, PhantomData<Q>),
    Clicked(Entity, PhantomData<Q>),
}

impl <Q: ReadOnlyWorldQuery> From<&bevy_mod_picking::PickingEvent> for PickingEvent<Q> {
    fn from(value: &bevy_mod_picking::PickingEvent) -> Self {
        match value {
            bevy_mod_picking::PickingEvent::Selection(selected) => {
                match selected {
                    bevy_mod_picking::SelectionEvent::JustSelected(selected) => {
                        PickingEvent::Selection(SelectionEvent::JustSelected(*selected), PhantomData::default())
                    }
                    bevy_mod_picking::SelectionEvent::JustDeselected(deselected) => {
                        PickingEvent::Selection(SelectionEvent::JustDeselected(*deselected), PhantomData::default())
                    }
                }
            }
            bevy_mod_picking::PickingEvent::Hover(hover) => {
                match hover {
                    bevy_mod_picking::HoverEvent::JustEntered(entered) => {
                        PickingEvent::Hover(HoverEvent::JustEntered(*entered), PhantomData::default())
                    }
                    bevy_mod_picking::HoverEvent::JustLeft(left) => {
                        PickingEvent::Hover(HoverEvent::JustLeft(*left), PhantomData::default())
                    }
                }
            }
            bevy_mod_picking::PickingEvent::Clicked(clicked) => {
                return PickingEvent::Clicked(*clicked, PhantomData::default());
            }
        }
    }
}