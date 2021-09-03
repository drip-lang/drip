use crate::error::ParseError;
use crate::event::Event;
use crate::Parse;
use drip_lexer::Token;
use drip_syntax::Drip;
use rowan::{GreenNodeBuilder, Language};
use std::mem;

pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'l [Token<'input>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<ParseError>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(tokens: &'l [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::new(),
        }
    }

    pub fn finish(mut self) -> Parse {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode { kind, offset } => {
                    let mut kinds = vec![kind];
                    let mut idx = idx;
                    let mut offset = offset;

                    while let Some(fp) = offset {
                        idx += fp;

                        offset = if let Event::StartNode { kind, offset } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            offset
                        } else {
                            unreachable!();
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(Drip::kind_to_raw(kind))
                    }
                }
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(error) => self.errors.push(error),
                Event::Placeholder => {}
            }

            self.eat_trivia();
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivia() {
                break;
            }

            self.token();
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];

        self.builder.token(Drip::kind_to_raw(kind.into()), text);
        self.cursor += 1;
    }
}
