use crate::seml::cst::{SemlAtom, SemlToken};

use super::cst;

#[derive(Clone, Copy)]
pub struct Parser<'a> {
    pub state: ParserState,
    pub data: &'a str,
}

impl<'a> Parser<'a> {
    const NIL: &'static str = "nil";

    pub fn new(data: &'a str) -> Self {
        Self {
            state: ParserState::new(),
            data,
        }
    }

    pub fn parse_nil(&mut self) -> Result<SemlToken> {
        if self.data.starts_with(NIL) {
            let res = SemlToken::Atom(SemlAtom::Nil(self.state));
            self.state.begin_item();
            self.state.advance(NIL.len());
        } else {
            ParseError::err(self.state, "expected nil")
        }
    }
}

#[derive(Clone, Copy)]
pub struct ParserState {
    pub line: usize,
    pub column: usize,
    pub indentation: usize,
    pub inline: bool,
    pub fresh: bool,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            line: 0,
            column: 0,
            indentation: 0,
            inline: false,
            fresh: true,
        }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
        if !self.inline {
            self.fresh = true;
        }
    }

    pub fn advance(&mut self, n: usize) {
        self.column += n;
        if !self.inline {
            self.indentation += n
        }
    }

    pub fn begin_item(&mut self) {
        self.inline = true;
        self.fresh = false;
    }

    pub fn finish_item(&mut self) {
        self.inline = false;
    }
}

#[derive(Clone, Copy)]
pub struct ParseError {
    pub state: ParserState,
    pub reason: Cow<'static, str,
}

impl ParseError {
    pub fn err<T>(state: ParserState, reason: &'static str) -> Result<T> {
        Err(Self { state, reason })
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
