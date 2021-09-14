use crate::error::ParseError;
use crate::event::Event;
use crate::grammar;
use crate::marker::Marker;
use crate::source::Source;
use drip_lexer::{Token, TokenKind};
use drip_syntax::SyntaxKind;
use std::mem;

const RECOVERY_SET: [TokenKind; 1] = [TokenKind::Ident];

pub(crate) struct Parser<'l, 'input> {
    source: Source<'l, 'input>,
    pub(crate) events: Vec<Event>,
    expected_token_kinds: Vec<TokenKind>,
}

impl<'l, 'input> Parser<'l, 'input> {
    pub fn new(source: Source<'l, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_token_kinds: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub fn bump(&mut self) {
        self.expected_token_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub fn error(&mut self) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // end of last input => range of last token
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_token_kinds),
            found,
            range,
        }));

        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let marker = self.start();
            self.bump();
            marker.complete(self, SyntaxKind::Error);
        }
    }

    pub fn eat(&mut self, kinds: &[TokenKind]) {
        self.source.eat(kinds);
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_token_kinds.push(kind);
        self.peek() == Some(kind)
    }

    pub fn current(&mut self) -> Option<TokenKind> {
        self.peek()
    }

    pub fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub(crate) fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek()
    }

    pub(crate) fn peek_nth(&mut self, nth: isize) -> Option<TokenKind> {
        self.source.peek_nth_raw(nth)
    }
}
