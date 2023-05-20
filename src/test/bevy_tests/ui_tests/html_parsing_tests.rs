use bevy::ecs::system::CommandQueue;
use bevy::prelude::{Commands, World};
use pulldown_cmark::{html, Parser};
use crate::render_html::render_html;

#[test]
pub(crate) fn test_html() {
    let mut queue = CommandQueue::default();
    let world = World::default();
    let mut commands: Commands = Commands::new(&mut queue, &world);
    let mut parser = Parser::new(&"###little header");
    let out = render_html(&mut commands, parser);

    assert!(out.is_ok());
    assert!(out.as_ref().unwrap().html_tree.is_some());

}