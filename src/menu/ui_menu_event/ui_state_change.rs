use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Button, Changed, Color, Display, Entity, Interaction, Query, ResMut, Size, Style, With};
use bevy::log::info;
use crate::event::event_state::{StateChange, Update};
use crate::event::event_propagation::{ChangePropagation, StartingState};
use crate::menu::ui_menu_event::change_style::{ChangeStyleTypes, StyleNode};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{DisplayState, StyleContext, UiComponent, UiComponentState, UiComponentStateFilter, UiEventArgs};

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

    pub fn get_ui_event(
        &self,
        args: &HashMap<Entity, StyleNode>,
        start_state: StartingState,
        current_state_filter: &UiComponentState,
        style_context: &mut ResMut<StyleContext>) -> Option<UiEventArgs> {
        if let StateChange::ChangeComponentStyle(change_style) = self {
            let starting = change_style.get_current_state(args, &start_state);
            info!("{} is size of starting and {} is size of args..", starting.len(), args.len());

            let (starting, filtered) =
                Self::filter_entities(args, current_state_filter, change_style, starting);
            info!("{} is size of starting and {} is size of filtered..", starting.len(), filtered.len());
            if filtered.is_empty() {
                info!("Filtered entities were none.");
            } else {
                info!("Doing state change with {:?}", filtered);
            }
            return change_style.do_change(starting, filtered);

        }
        None
    }


    fn filter_entities<'a>(
        args: &'a HashMap<Entity, StyleNode>,
        current_state_filter: &UiComponentState,
        change_style: &'a ChangeStyleTypes,
        starting: HashMap<Entity, Style>
    ) -> (HashMap<Entity, Style>, HashMap<&'a Entity, &'a StyleNode>) {
        let starting = starting.into_iter()
            .filter(|(entity, state)| {
                match current_state_filter {
                    UiComponentState::StateDisplay(display) => display.matches(&state.display),
                    UiComponentState::StateSize(size) => size.matches(&state.size)
                }
            })
            .collect::<HashMap<Entity, Style>>();

        let filtered = change_style.filter_entities(args);
        (starting, filtered)
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
