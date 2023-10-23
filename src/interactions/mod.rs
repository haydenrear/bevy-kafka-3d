use std::io::Cursor;
use std::marker::PhantomData;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::{Entity, Event, Interaction};
use bevy::window::CursorMoved;
use ndarray::s;
use crate::pickable_events::PickableEvent;


/// Convert all interactions to these events so that ui and 3d are comparable.
#[derive(Event)]
pub enum InteractionEvent<QueryFilterT: ReadOnlyWorldQuery> {
    UiComponentInteraction { event: Interaction, entity: Entity },
    RayCastInteraction { event: PickingEvent<QueryFilterT> },
    ScrollWheelEvent { event: MouseWheel },
    CursorEvent {event: CursorMoved }
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
pub enum PickingEvent<QueryFilterT: ReadOnlyWorldQuery> {
    Selection(SelectionEvent, PhantomData<QueryFilterT>),
    Hover(HoverEvent, PhantomData<QueryFilterT>),
    Clicked(Entity, PhantomData<QueryFilterT>),
}

impl<QueryFilterT: ReadOnlyWorldQuery> From<&PickableEvent> for PickingEvent<QueryFilterT> {
    fn from(value: &PickableEvent) -> Self {
        match value {
            PickableEvent::Selection(selected) => {
                match selected {
                    SelectionEvent::JustSelected(selected) => {
                        PickingEvent::Selection(SelectionEvent::JustSelected(*selected), PhantomData::default())
                    }
                    SelectionEvent::JustDeselected(deselected) => {
                        PickingEvent::Selection(SelectionEvent::JustDeselected(*deselected), PhantomData::default())
                    }
                }
            }
            PickableEvent::Hover(hover) => {
                match hover {
                    HoverEvent::JustEntered(entered) => {
                        PickingEvent::Hover(HoverEvent::JustEntered(*entered), PhantomData::default())
                    }
                    HoverEvent::JustLeft(left) => {
                        PickingEvent::Hover(HoverEvent::JustLeft(*left), PhantomData::default())
                    }
                }
            }
            PickableEvent::Clicked(clicked) => {
                return PickingEvent::Clicked(*clicked, PhantomData::default());
            }
        }
    }
}