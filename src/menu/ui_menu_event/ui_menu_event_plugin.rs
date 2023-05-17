use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy::hierarchy::{Children, Parent};
use std::fmt::{Debug, Formatter};
use bevy::ecs::schedule::SystemSetConfig;
use bevy::ecs::system::SystemParam;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::time::TimerMode;
use bevy::utils::{HashMap, HashSet};
use crate::cursor_adapter::CursorResource;
use crate::event::event_actions::{ClickWriteEvents, InteractionEventReader};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::{ClickContext, Context, HoverStateChange, NextStateChange, StateChange, StateChangeFactory, StateUpdate, Update, UpdateStateInPlace};
use crate::event::event_state::StateChange::ChangeComponentStyle;
use crate::menu::{CollapsableMenu, ConfigurationOptionEnum, DraggableComponent, Dropdown, DropdownOption, MenuItemMetadata, Radial, RadialButton, RadialButtonSelection, ScrollableComponent, Slider, SliderKnob, ui_menu_event, UiBundled, UiComponent};
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::menu::ui_menu_event::interaction_ui_event_reader::UiEventReader;
use crate::menu::ui_menu_event::interaction_ui_event_writer::{DragEvents, GlobalState, ScrollEvents, StateChangeActionTypeStateRetriever};
use crate::menu::ui_menu_event::ui_menu_event_plugin::DisplayState::DisplayNone;
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::UiClickStateChange;
use crate::ui_components::state_transitions::insert_state_transitions;
use crate::ui_components::ui_components::BuildMenuResult;
use crate::ui_components::ui_menu_component::{create_menu, UiIdentifiableComponent};

pub struct UiEventPlugin;

pub type UiComponentStyleFilter = (With<UiComponent>, With<Style>);
pub type UiComponentStyleIxnFilter = (With<UiComponent>, With<Button>, Changed<Interaction>);
pub type DraggableUiComponentFilter = (With<UiComponent>, With<Style>, With<DraggableComponent>);
pub type DraggableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<DraggableComponent>);
pub type ScrollableUiComponentFilter = (With<UiComponent>, With<Style>, With<ScrollableComponent>);
pub type ScrollableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<ScrollableComponent>);
pub type UiComponentStateTransitionsQuery<'a> = (Entity, &'a UiComponent, &'a Style, &'a UiIdentifiableComponent, &'a UiEntityComponentStateTransitions);
pub type ScrollableIxnFilterQuery = (With<UiComponent>, With<Button>, With<ScrollableComponent>);
pub type PropagationQuery<'a> = (Entity, &'a Style, &'a UiIdentifiableComponent);
pub type PropagationQueryFilter<'a> = (With<Style>);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CreateMenu {
    #[default]
    AddResources,
    InsertStateTransitions,
}

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<CreateMenu>()
            .add_startup_system(create_menu)
            .add_system(insert_state_transitions
                .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            )
            .insert_resource(BuildMenuResult::default())
            .insert_resource(StateChangeActionTypeStateRetriever::<UiComponentStyleFilter, UiComponentStyleIxnFilter>::default())
            .insert_resource(StateChangeActionTypeStateRetriever::<DraggableUiComponentFilter, DraggableUiComponentIxnFilter>::default())
            .insert_resource(StateChangeActionTypeStateRetriever::<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter>::default())
            .add_system(StateChangeActionTypeStateRetriever::<UiComponentStyleFilter, UiComponentStyleIxnFilter>::click_write_events)
            .add_system(DragEvents::click_write_events)
            .add_system(ScrollEvents::click_write_events)
            .insert_resource(StyleContext::default())
            .add_system(UiEventReader::read_events)
            .add_system(ui_state_change::hover_event)
            .add_event::<UiEventArgs>()
            .add_event::<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>();
    }
}

#[derive(Resource, Default, Clone, Debug)]
pub struct StateChangeActionComponentStateFactory;

impl StateChangeFactory<StateChangeActionType, UiEventArgs, Style, Style, StyleContext, NextUiState>
for StateChangeActionComponentStateFactory {
    fn current_state(current: &EventDescriptor<StateChangeActionType, UiEventArgs, Style>, context: &mut ResMut<StyleContext>) -> Vec<NextStateChange<NextUiState, Style, StyleContext>> {
        if let UiEventArgs::Event(UiClickStateChange::ChangeSize { entity, update_display}) = &current.event_args {
            vec![NextStateChange {
                entity: *entity,
                next_state: NextUiState::ReplaceSize(update_display.clone()),
                phantom: Default::default(),
                phantom_ctx: Default::default(),
            }]
        } else if let UiEventArgs::Event(UiClickStateChange::ChangeDisplay { entity, update_display}) = &current.event_args {
            vec![NextStateChange {
                entity: *entity,
                next_state: NextUiState::ReplaceDisplay(update_display.clone()),
                phantom: Default::default(),
                phantom_ctx: Default::default(),
            }]
        } else if let UiEventArgs::Event(UiClickStateChange::Slider { update_scroll, entity }) = &current.event_args {
            vec![NextStateChange {
                entity: *entity,
                next_state: NextUiState::UpdatePosition(update_scroll.clone()),
                phantom: Default::default(),
                phantom_ctx: Default::default(),
            }]
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
pub enum NextUiState {
    ReplaceSize(Update<Size>),
    ReplaceDisplay(Update<Display>),
    UpdatePosition(Update<UiRect>)
}

#[derive(Resource, Default, Clone, Debug)]
pub struct StyleContext {
    pub(crate) visible: HashMap<Entity, Style>,
    pub(crate) is_dragging: bool,
    pub(crate) delta: Option<Vec2>,
    pub(crate) scroll_wheel: Option<Vec2>,
    pub(crate) scroll_wheel_units: Option<MouseScrollUnit>,
}

impl Context for StyleContext {}

impl ClickContext<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter> for StyleContext {
    fn clicked(&mut self) {
        self.is_dragging = true;
    }

    fn un_clicked(&mut self) {
        self.is_dragging = false;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.delta = Some(cursor_moved.cursor_delta.clone());
        self.scroll_wheel = Some(cursor_moved.scroll_wheel_delta.clone());
        self.scroll_wheel_units = cursor_moved.wheel_units.clone();
    }
}

impl ClickContext<UiComponentStyleFilter, UiComponentStyleIxnFilter> for StyleContext {
    fn clicked(&mut self) {
        self.is_dragging = true;
    }

    fn un_clicked(&mut self) {
        self.is_dragging = false;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.delta = Some(cursor_moved.cursor_delta.clone());
        self.scroll_wheel = Some(cursor_moved.scroll_wheel_delta.clone());
        self.scroll_wheel_units = cursor_moved.wheel_units.clone();
    }
}

impl ClickContext<DraggableUiComponentFilter, DraggableUiComponentIxnFilter> for StyleContext {
    fn clicked(&mut self) {
        self.is_dragging = true;
    }

    fn un_clicked(&mut self) {
        self.is_dragging = false;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.delta = Some(cursor_moved.cursor_delta.clone());
        self.scroll_wheel = Some(cursor_moved.scroll_wheel_delta.clone());
    }
}

impl UpdateStateInPlace<Style, StyleContext> for NextUiState {
    fn update_state(&self, commands: &mut Commands,  value: &mut Style, style_context: &mut ResMut<StyleContext>) {
        if let NextUiState::ReplaceSize(update) = &self {
            update.update_state(commands, &mut value.size, style_context);
        } else if let NextUiState::ReplaceDisplay(display) = &self {
            display.update_state(commands, &mut value.display, style_context);
        } else if let NextUiState::UpdatePosition(update) = &self {
            update.update_state(commands, &mut value.position ,style_context);
        }
    }
}

pub trait UiComponentStateFilter<T> {
    fn matches(&self, other: &T) -> bool;
}

#[derive(Debug)]
pub enum UiComponentState {
    StateDisplay(DisplayState),
    StateSize(SizeState),
    Selected,
    Deselected,
    Any
}

impl UiComponentState {
    pub(crate) fn matches(&self, style: &Style) -> bool {
        match self {
            UiComponentState::StateDisplay(display) => display.matches(&style.display),
            UiComponentState::StateSize(state) => state.matches(&style.size),
            UiComponentState::Any => true,
            other => {
                info!("Did not match: {:?}", other);
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct UiComponentStateTransition {
    pub(crate) filter_state: UiComponentState,
    pub(crate) state_change: Vec<StateChangeActionType>,
    pub(crate) propagation: ChangePropagation,
    /// Sometimes the event is driven from another component, and then propagated to other components.
    /// In this case, there needs to be a filter for whether or not to change the other component, the
    /// component for which the event is propagated to.
    pub(crate) current_state_filter: UiComponentState
}

#[derive(Debug)]
pub struct EntityComponentStateTransition {
    pub(crate) entity_to_change: EntitiesStateTypes,
    pub(crate) filter_state: UiComponentState,
    // filter for the component to be changed.
    pub(crate) current_state_filter: UiComponentState
}

#[derive(Debug)]
pub struct EntitiesStateTypes{
    /// In the update function, the EntityComponentStateTransitionComponent will iterate through
    /// each of the states and get the related components to calculate the value to be passed to
    /// the StateUpdate function.
    pub(crate) states: Vec<(Entity, Relationship, StateChangeActionType)>
}

#[derive(Component, Debug)]
pub struct UiEntityComponentStateTransitions {
    pub(crate) transitions: Vec<EntityComponentStateTransition>,
}

#[derive(Component, Debug)]
pub struct UiComponentStateTransitions {
    pub(crate) transitions: Vec<UiComponentStateTransition>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DisplayState {
    DisplayFlex,
    DisplayNone,
    DisplayAny,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SizeState {
    Expanded{
        height: u32,
        width: u32,
    }, Minimized {
        height: u32,
        width: u32,
    }
}

impl DisplayState {
    fn get_display(&self) -> Display {
        match self {
            DisplayState::DisplayFlex => {
                Display::Flex
            }
            DisplayState::DisplayNone => {
                Display::None
            }
            DisplayState::DisplayAny => {
                Display::Flex
            }
        }
    }
}

impl UiComponentStateFilter<Display> for DisplayState {
    fn matches(&self, other: &Display) -> bool {
        if let DisplayState::DisplayAny = self {
            return true;
        }
        if self.get_display() == *other  {
            return true;
        } else {
            return false;
        }
    }
}

impl SizeState {

    fn get_width_height(&self) -> (u32, u32) {
        match self  {
            SizeState::Expanded { height, width } => {
                (*height, *width)
            }
            SizeState::Minimized { height, width } => {
                (*height, *width)
            }
        }
    }

}

impl UiComponentStateFilter<Size> for SizeState {
    fn matches(&self, starting_state: &Size) -> bool {
        let (height_state, width_state) = self.get_width_height();
        info!("{} is height and {} is width, and {:?} is starting_state.", height_state, width_state, starting_state);
        if let Val::Percent(height) = starting_state.height {
            if let Val::Percent(width) = starting_state.width {
                info!("{} is match height and {} is match width.", height, width);
                if height as u32 == height_state && width as u32 == width_state {
                    info!("matched");
                    return true;
                }
                return false;
            }
        }
        if let Val::Px(height) = starting_state.height {
            if let Val::Px(width) = starting_state.width {
                info!("{} is match height and {} is match width.", height, width);
                info!("{} is match height and {} is match width.", height, width);
                if height as u32 == height_state && width as u32 == width_state {
                    info!("matched");
                    return true;
                }
                return false;
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum UiEventArgs {
    Event(UiClickStateChange)
}

impl EventArgs for UiEventArgs {}

#[derive(Debug)]
pub struct ChangeComponentColorUpdate {
    new_color: Color
}

impl UpdateStateInPlace<BackgroundColor, StyleContext> for ChangeComponentColorUpdate {
    fn update_state(&self, commands: &mut Commands, value: &mut BackgroundColor, ctx: &mut ResMut<StyleContext>) {
        value.0 = self.new_color;
    }
}

#[derive(Clone, Debug)]
pub enum ColorChange {
    ChangeColor(Color),
    SwapColor {
        color_1: Color,
        color_2: Color,
    },
}

impl ColorChange {
    fn change_color(&self, mut display: &mut BackgroundColor) {
        match &self {
            ColorChange::ChangeColor(color) => {
                display.0 = color.clone();
            }
            ColorChange::SwapColor { color_1, color_2 } => {
                if &display.0 == color_1 {
                    display.0 = color_2.clone();
                } else {
                    display.0 = color_1.clone();
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UiComponentFilters {
    pub(crate) exclude: Option<Vec<f32>>,
}

impl EventData for StateChangeActionType {}

#[derive(Clone, Debug)]
pub enum StateChangeActionType {
    Hover(StateChange),
    Clicked(StateChange),
    Dragged(StateChange)
}
