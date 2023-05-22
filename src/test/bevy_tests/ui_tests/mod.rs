use bevy::prelude::*;
use bevy::prelude::KeyCode::A;
use crate::camera::raycast_select::BevyPickingState;
use crate::event::event_propagation::{component_propagation_system, SideEffectWriter};
use crate::interactions::InteractionEvent;
use crate::menu::menu_resource::MenuResource;
use crate::menu::ui_menu_event::ui_state_change::GlobalState;
use crate::menu::ui_menu_event::next_action::Matches;
use crate::menu::ui_menu_event::types::UiStyleEntityComponentStateTransitions;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiEventPlugin};
use crate::test::bevy_tests::default_plugins::NoRenderBevyIntegrationTestPlugin;
use crate::ui_components::menu_components::BuildMenuResult;

pub(crate) mod html_parsing_tests;

#[test]
pub(crate) fn test_state_transitions_added() {
    let mut app = App::new();
    let app = create_app(&mut app);
    let build_menu_result = app.world.resource::<BuildMenuResult>();

    assert_ne!(build_menu_result.base_menu_results.len(), 0);
    assert_ne!(build_menu_result.submenu_results.len(), 0);
    assert_ne!(build_menu_result.dropdown_menu_option_results.len(), 0);
    assert_ne!(build_menu_result.collapsable.len(), 0);
    assert_ne!(build_menu_result.dropdown.len(), 0);

    let collapsable = build_menu_result.collapsable
        .iter()
        .map(|(entity, result)| result.collapsable_menu_button)
        .collect::<Vec<Entity>>();

    assert_ne!(collapsable.len(), 0);

    for collapsable in collapsable.iter() {
        println!("Checking {:?}", collapsable);
        let state_transitions = app.world.get::<UiStyleEntityComponentStateTransitions>(*collapsable);
        assert!(state_transitions.is_some());
        let state_transitions = state_transitions.unwrap();
        let any_visibility_state_transitions = state_transitions.transitions.iter().any(|s| {
            let mut style = Style::default();
            style.display = Display::Flex;
            if s.current_state_filter.matches(&style) {
                println!("Checking: {:?}", s);
                return s.entity_to_change.states.len() != 0
            }
            false
        });
        assert!(any_visibility_state_transitions);
    }



}

fn create_app<'a>(app: &'a mut App) -> &'a mut App {


    let mut app = app
        .insert_resource(MenuResource::default())
        .insert_resource(GlobalState::default())
        .insert_resource(BevyPickingState::default())
        .add_event::<SideEffectWriter>()
        .add_event::<InteractionEvent<()>>()
        .add_plugins(NoRenderBevyIntegrationTestPlugin)
        .add_plugin(UiEventPlugin);

    app.update();
    app.update();
    app.update();
    app.update();
    app
}
