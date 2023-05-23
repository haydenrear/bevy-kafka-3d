use std::collections::HashSet;
use std::marker::PhantomData;
use bevy::prelude::{Changed, Commands, Component, Entity, Query};

pub type ReactQuery<'a, ComponentChangeT> = (Entity, &'a ComponentChangeT);
pub type ReactQueryComponentChangedFilter<ComponentChangeT> = (Changed<ComponentChangeT>);

pub trait DownstreamReaction<ComponentChangedT: Component> {
    fn do_reaction(commands: &mut Commands, changed_component: &ComponentChangedT);
}

pub trait DownstreamReactor<ComponentChangedT: Component, Rxn: DownstreamReaction<ComponentChangedT>> {
    fn react(
        commands: Commands,
        reaction_query: Query<ReactQuery<ComponentChangedT>, ReactQueryComponentChangedFilter<ComponentChangedT>>,
        reaction: PhantomData<Rxn>
    );
}

#[derive(Component)]
pub struct InsertGraphMenuComponent {
    indices: HashSet<String>
}

pub struct InsertGraphMenuDownstreamReaction;

impl InsertGraphMenuDownstreamReaction {
    fn do_reaction(
        commands: &mut Commands,
        changed_component: &InsertGraphMenuComponent,
    )
    {
        todo!()
    }
}