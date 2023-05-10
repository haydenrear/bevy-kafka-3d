use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Button, Changed, Color, Display, Entity, Interaction, Query, Size, Style, With};
use bevy::log::info;
use crate::event::event_state::{StateChange, Update};
use crate::event::event_propagation::{ChangePropagation, StartingState};
use crate::menu::ui_menu_event::change_style::StyleNode;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiComponent, UiEventArgs};

/// Contains the state data needed in order to generate the UIEvents from the state change required.
#[derive(Clone, Debug)]
pub enum UiClickStateChange {
    ChangeColor {
        update_display: HashMap<Entity, Update<BackgroundColor>>,
    },
    ChangeDisplay {
        update_display: HashMap<Entity, Update<Display>>,
    },
    ChangeSize {
        update_display: HashMap<Entity, Update<Size>>,
    },
    None,
}


impl StateChange {

    pub fn get_ui_event(&self, args: &HashMap<Entity, StyleNode>, start_state: StartingState) -> Option<UiEventArgs> {
        if let StateChange::ChangeComponentStyle(change_style) = self {
            // here we translate to the UiComponentFilters, from the change_style, and then
            // pass the UiComponentFilter to a method that executes
            let starting = change_style.get_current_state(args, &start_state);
            let filtered = change_style.filter_entities(args);
            if filtered.is_empty() {
                info!("Filtered entities were none.");
            } else {
                info!("Doing state change with {:?}", filtered);
            }
            return change_style.do_change(starting, filtered);

        }
        None
    }

}

pub fn hover_event(
    mut query: Query<(&mut Style, &mut BackgroundColor, &Interaction), (With<UiComponent>, With<Button>, Changed<Interaction>)>,
) {
    for (_, mut color, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
            }
            Interaction::Hovered => {
                color.0 = Color::YELLOW;
            }
            Interaction::None => {
                color.0 = Color::GREEN;
            }
        }
    }
}
