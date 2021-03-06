use drip_lexer::{Token, TokenKind};
use text_size::TextRange;

pub struct Source<'l, 'input> {
    tokens: &'l [Token<'input>],
    cursor: usize,
}

impl<'l, 'input> Source<'l, 'input> {
    pub fn new(tokens: &'l [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn peek(&mut self) -> Option<TokenKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub fn peek_nth_raw(&mut self, nth: isize) -> Option<TokenKind> {
        let mut counter = 0;
        let mut nth_counter = 0;
        while nth_counter <= nth {
            let token = self.tokens.get(self.cursor + counter);
            if token.is_none() {
                return None;
            }
            if !token.unwrap().is_trivia() {
                nth_counter += 1;
            }
            counter += 1;
        }
        self.tokens
            .get(self.cursor + counter - 1)
            .map(|Token { kind, .. }| *kind)
    }

    pub fn next_token(&mut self) -> Option<&'l Token<'input>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub fn last_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|Token { range, .. }| *range)
    }

    fn peek_kind_raw(&self) -> Option<TokenKind> {
        self.tokens.get(self.cursor).map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    pub fn eat(&mut self, kinds: &[TokenKind]) {
        while self.at_ignore(kinds) {
            self.cursor += 1;
        }
    }

    fn at_ignore(&self, kinds: &[TokenKind]) -> bool {
        let token = self.peek_kind_raw();
        if token.is_none() {
            return false;
        }
        let token = token.unwrap();
        for kind in kinds {
            if *kind == token {
                return true;
            }
        }
        false
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, TokenKind::is_trivia)
    }
}
