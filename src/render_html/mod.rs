use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use pulldown_cmark::{CowStr, Event, HeadingLevel, Parser, Tag};

#[derive(Component)]
pub enum HtmlTags {
    Paragraph,
    H1, H2, H3, H4, H5, H6,
    Code
}

#[derive(Clone)]
pub struct HtmlParsingResult {
    pub(crate) tree: HtmlTree
}

#[derive(Clone)]
pub struct HtmlTree {
    pub(crate) html_tree: Option<Box<HtmlTree>>,
    pub(crate) self_entity: Entity
}

#[derive(Debug)]
pub struct HtmlParsingError {
}

/// Will divide into sections to render
pub(crate) fn render_html(mut commands: &mut Commands, mut parser: Parser) -> Result<HtmlTree, HtmlParsingError> {
    let mut next_value = parser.next();

    if next_value.as_ref().is_some() {
        match next_value.unwrap() {
            Event::Start(start) => {
                let mut entity = commands.spawn(NodeBundle {
                    ..default()
                });
                let mut html_tree = HtmlTree {
                    html_tree: None,
                    self_entity: entity.id()
                };
                html_child(&mut entity, &mut parser, &start, &mut html_tree);
                let next = parser.next();
                if !matches!(next, Some(Event::End(_))) {
                    return Err(HtmlParsingError{});
                }
                return Ok(html_tree);
            }
            val => {
            }
        }
    }

    Err(HtmlParsingError{})
}

pub(crate) fn html_child(mut commands: &mut EntityCommands, mut parser: &mut Parser, tag: &Tag, mut html_tree: &mut HtmlTree) {
    let mut next = parser.next();
    if next.is_some() {
        commands.with_children(|children| {
            match next.unwrap() {
                Event::Start(start) => {
                    let mut commands = children.spawn((
                        NodeBundle {
                            ..default()
                        }
                    ));
                    let mut entity = commands.id();
                    let mut next_html = HtmlTree {
                        html_tree: None,
                        self_entity: entity,
                    };
                    html_child(&mut commands, parser, &start, &mut next_html);
                    html_tree.html_tree = Some(next_html.into());
                }
                Event::End(_) => {
                    return;
                }
                Event::Text(txt) => {
                    match tag {
                        Tag::Paragraph => {
                            let next = spawn_text_value(children, txt, HtmlTags::Paragraph);
                            html_tree.html_tree = Some(HtmlTree {
                                html_tree: None,
                                self_entity: next,
                            }.into());
                        }
                        Tag::Heading(heading, _, _) => {
                            match heading {
                                HeadingLevel::H1 => {
                                    spawn_text_value(children, txt, HtmlTags::H1);
                                },
                                HeadingLevel::H2 => {
                                    spawn_text_value(children, txt, HtmlTags::H2);
                                },
                                HeadingLevel::H3 => {
                                    spawn_text_value(children, txt, HtmlTags::H3);
                                },
                                HeadingLevel::H4 => {
                                    spawn_text_value(children, txt, HtmlTags::H4);
                                },
                                HeadingLevel::H5 => {
                                    spawn_text_value(children, txt, HtmlTags::H5);
                                },
                                HeadingLevel::H6 => {
                                    spawn_text_value(children, txt, HtmlTags::H6);
                                },
                            }
                        }
                        Tag::BlockQuote => {}
                        Tag::CodeBlock(_) => {}
                        Tag::List(_) => {}
                        Tag::Item => {}
                        Tag::FootnoteDefinition(_) => {}
                        Tag::Table(_) => {}
                        Tag::TableHead => {}
                        Tag::TableRow => {}
                        Tag::TableCell => {}
                        Tag::Emphasis => {}
                        Tag::Strong => {}
                        Tag::Strikethrough => {}
                        Tag::Link(_, _, _) => {}
                        Tag::Image(_, _, _) => {}
                    }
                }
                Event::Code(code) => {
                }
                Event::Html(html) => {
                }
                Event::FootnoteReference(_) => {}
                Event::SoftBreak => {
                }
                Event::HardBreak => {
                }
                Event::Rule => {
                }
                Event::TaskListMarker(_) => {
                }
            }
        });
    }
}

fn spawn_text_value(children: &mut ChildBuilder, txt: CowStr, tags: HtmlTags) -> Entity {
    let spawned = children.spawn((
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(txt.into_string(), TextStyle {
                        ..default()
                    })
                ],
                ..default()
            },
            ..default()
        },
        tags
    ));

    spawned.id()

}