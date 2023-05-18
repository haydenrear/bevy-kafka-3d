use bevy::prelude::{Commands, Component, Entity, error, EventReader, info, Visibility};
use bevy::ui::{Display, Style};
use crate::event::event_state::StyleStateChangeEventData;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

// macro_rules! component_propagation {
//
//     ($($component_ident:ident, $component_ty:ty),*) => {
//
//         pub enum ComponentPropagation
//         {
//             $(
//                 $component_ident(Entity, $component_ty)
//             )*
//         }
//
//         pub fn component_propagation_system(
//             mut commands: Commands,
//             mut propagation_reader: EventReader<ComponentPropagation>
//         ) {
//             for event in propagation_reader.iter() {
//                 match event {
//                     $(
//                         ComponentPropagation::$component_ident(e, c) => {
//                             add_component(&mut commands, e, *c);
//                         }
//                     )*
//                 }
//             }
//         }
//     }
// }
//
// component_propagation!(
//     Visible, Visibility
// );

#[derive(Debug)]
pub enum PropagateComponentEvent {
    ChangeVisible(Entity, Visibility),
    ChangeStyle(Entity, Style)
}

pub fn component_propagation_system(
    mut commands: Commands,
    mut propagation_reader: EventReader<PropagateComponentEvent>
) {
    for event in propagation_reader.into_iter() {
        match event {
            PropagateComponentEvent::ChangeVisible(entity, component) => {
                info!("Adding propagation visibility event: {:?} with entity {:?}", component, entity);
                add_component(&mut commands, entity, *component)
            }
            PropagateComponentEvent::ChangeStyle(entity, component) => {
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

#[derive(Clone, Debug)]
pub enum ChangePropagation {
    // Include self as parent and any children of parent
    ParentToChild(Relationship),
    // Include self as child and parent of self
    ChildToParent(Relationship),
    // Include self only
    SelfChange(Relationship),
    // Include children only
    Children(Relationship),
    // includes siblings
    Siblings(Relationship),
    // include self and siblings
    SelfToSiblings(Relationship),
    // Include parent only
    Parent(Relationship),
    // propagate event to siblings children
    SiblingsChildren(Relationship),
    // Propagate event to specific Id's
    SiblingsChildrenRecursive(Relationship),
    // Propagate event to specific Id's
    CustomPropagation {
        to: Vec<f32>,
        // starting state
        from: Relationship
    },
}

impl ChangePropagation {
    pub(crate) fn get_starting_state(&self) -> &Relationship {
        match self {
            ChangePropagation::ParentToChild(starting) => {
                starting
            }
            ChangePropagation::ChildToParent(starting) => {
                starting
            }
            ChangePropagation::SelfChange(starting) => {
                starting
            }
            ChangePropagation::Children(starting) => {
                starting
            }
            ChangePropagation::Parent(starting) => {
                starting
            }
            ChangePropagation::CustomPropagation { to , from} => {
                from
            }
            ChangePropagation::Siblings(starting) => {
                starting
            }
            ChangePropagation::SelfToSiblings(starting) => {
                starting
            }
            ChangePropagation::SiblingsChildren(starting) => {
               starting
            }
            ChangePropagation::SiblingsChildrenRecursive(starting) => {
                starting
            }
        }
    }
}

impl Relationship {
    pub(crate) fn includes_parent(&self) -> bool {
        match self {
            Relationship::Parent => {
                true
            }
            _ => {
                false
            }
        }
    }

    pub(crate) fn includes_self(&self) -> bool {
        match self {
            Relationship::SelfState => {
                true
            }
            _ => {
                false
            }
        }
    }

    pub(crate) fn includes_children(&self) -> bool {
        match self {
            Relationship::Child => {
                true
            }
            _ => {
                false
            }
        }
    }

    pub(crate) fn includes_sibling(&self) -> bool {
        match self {
            Relationship::Sibling => {
                true
            }
            _ => {false}
        }
    }

    pub(crate) fn includes_siblings_children(&self) -> bool {
        match self {
            Relationship::SiblingChild => {
                true
            }
            _ => {
                false
            }
        }
    }

}

impl ChangePropagation {

    pub(crate) fn includes_parent(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(state) => {
                true
            }
            ChangePropagation::ChildToParent(state) => {
                true
            }
            ChangePropagation::SelfChange(state) => {
                state.includes_parent()
            }
            ChangePropagation::Children(state) => {
                state.includes_parent()
            }
            ChangePropagation::Siblings(state) => {
                state.includes_parent()
            }
            ChangePropagation::SelfToSiblings(state) => {
                state.includes_parent()
            }
            ChangePropagation::Parent(state) => {
                true
            }
            ChangePropagation::SiblingsChildren(state) => {
                state.includes_parent()
            }
            ChangePropagation::CustomPropagation { to, from } => {
                from.includes_parent()
            }
            ChangePropagation::SiblingsChildrenRecursive(state) => {
                state.includes_parent()
            }
        }
    }

    pub(crate) fn includes_self(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(state) => {
                state.includes_self()
            }
            ChangePropagation::ChildToParent(state) => {
                state.includes_self()
            }
            ChangePropagation::SelfChange(state) => {
                true
            }
            ChangePropagation::Children(state) => {
                state.includes_self()
            }
            ChangePropagation::Siblings(state) => {
                state.includes_self()
            }
            ChangePropagation::SelfToSiblings(state) => {
                true
            }
            ChangePropagation::Parent(state) => {
                state.includes_self()
            }
            ChangePropagation::SiblingsChildren(state) => {
                state.includes_self()
            }
            ChangePropagation::CustomPropagation { to, from } => {
                from.includes_self()
            }
            ChangePropagation::SiblingsChildrenRecursive(state) => {
                state.includes_self()
            }
        }
    }

    pub(crate) fn includes_children(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(state) => {
                true
            }
            ChangePropagation::ChildToParent(state) => {
                true
            }
            ChangePropagation::SelfChange(state) => {
                state.includes_children()
            }
            ChangePropagation::Children(state) => {
                true
            }
            ChangePropagation::Siblings(state) => {
                state.includes_children()
            }
            ChangePropagation::SelfToSiblings(state) => {
                state.includes_children()
            }
            ChangePropagation::Parent(state) => {
                state.includes_children()
            }
            ChangePropagation::SiblingsChildren(state) => {
                state.includes_children()
            }
            ChangePropagation::CustomPropagation { to, from } => {
                from.includes_children()
            }
            ChangePropagation::SiblingsChildrenRecursive(state) => {
                state.includes_children()
            }
        }
    }

    pub(crate) fn includes_sibling(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(state) => {
                state.includes_sibling()
            }
            ChangePropagation::ChildToParent(state) => {
                state.includes_sibling()
            }
            ChangePropagation::SelfChange(state) => {
                state.includes_sibling()
            }
            ChangePropagation::Children(state) => {
                state.includes_sibling()
            }
            ChangePropagation::Siblings(state) => {
                true
            }
            ChangePropagation::SelfToSiblings(state) => {
                true
            }
            ChangePropagation::Parent(state) => {
                state.includes_sibling()
            }
            ChangePropagation::SiblingsChildren(state) => {
                state.includes_sibling()
            }
            ChangePropagation::CustomPropagation { to, from } => {
                from.includes_sibling()
            }
            ChangePropagation::SiblingsChildrenRecursive(state) => {
                state.includes_sibling()
            }
        }
    }

    pub(crate) fn includes_siblings_children(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::ChildToParent(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::SelfChange(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::Children(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::Siblings(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::SelfToSiblings(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::Parent(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::SiblingsChildren(state) => {
                true
            }
            ChangePropagation::CustomPropagation { to, from  } => {
                from.includes_siblings_children()
            }
            ChangePropagation::SiblingsChildrenRecursive(state) => {
                true
            }
        }
    }

    pub(crate) fn includes_siblings_children_recursive(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::ChildToParent(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::SelfChange(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::Children(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::Siblings(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::SelfToSiblings(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::Parent(state) => {
                state.includes_siblings_children()
            }
            ChangePropagation::SiblingsChildren(state) => {
                true
            }
            ChangePropagation::CustomPropagation { to, from  } => {
                from.includes_siblings_children()
            }
            ChangePropagation::SiblingsChildrenRecursive(state) => {
                true
            }
        }
    }

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

