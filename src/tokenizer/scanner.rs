use std::collections::VecDeque;
use crate::error::YamlError;
use crate::tokenizer::reader::{Reader, StrReader};
use crate::tokenizer::scanner::State::StreamStart;
use crate::tokenizer::{StrIterator};

#[derive(Clone, Default)]
pub struct Scanner {
    state: State,
    tokens: VecDeque<SpanToken>,
    pub(crate) eof: bool,
}

#[derive(Copy, Clone)]
pub enum State {
    StreamStart,
    DocStart,
    Post,
}

impl Default for State {
    fn default() -> Self {
        StreamStart
    }
}

pub enum Control {
    Continue,
    Eof,
    Err(YamlError),
}


impl Scanner {
    pub fn from_str_reader(string: &str) -> StrIterator<'_> {
        StrIterator {
            state: Default::default(),
            reader: StrReader::new(string),
        }
    }

    pub(crate) fn emit_end_of_stream(&mut self) {
        self.tokens.push_back(SpanToken::StreamEnd);
    }

    pub(crate) fn pop_token(&mut self) -> Option<SpanToken> {
        self.tokens.pop_front()
    }

    pub(crate) fn next_state<R: Reader>(&mut self, reader: &mut R) -> Control {
        match self.state {
            StreamStart => self.read_start_stream(reader),
            _ => return Control::Eof,
        };
        Control::Continue
    }

    pub(crate) fn read_start_stream<T: Reader>(&mut self, reader: &mut T)  {
        self.try_read_comments(reader);
        self.state = State::DocStart;
        self.tokens.push_back(SpanToken::StreamStart);
    }

    fn try_read_comments<T: Reader>(&self, reader: &mut T)  {
        reader.skip_space_tab();

        if reader.peek_byte_is(b'#') {
            reader.read_line();
        }

    }
}

#[derive(Copy, Clone)]
pub enum SpanToken {
    Scalar(usize, usize),
    StreamStart,
    StreamEnd,
}