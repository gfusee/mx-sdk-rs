use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::ValueSubTree,
    value_interpreter::{interpret_string, interpret_subtree},
};

use std::fmt;

#[derive(Clone, Debug)]
pub struct BytesValue {
    pub value: Vec<u8>,
    pub original: ValueSubTree,
}

impl BytesValue {
    pub fn empty() -> Self {
        BytesValue {
            value: Vec::new(),
            original: ValueSubTree::Str(String::default()),
        }
    }
}

impl From<Vec<u8>> for BytesValue {
    fn from(v: Vec<u8>) -> Self {
        BytesValue {
            value: v,
            original: ValueSubTree::Str(String::default()),
        }
    }
}

impl InterpretableFrom<ValueSubTree> for BytesValue {
    fn interpret_from(from: ValueSubTree, context: &InterpreterContext) -> Self {
        BytesValue {
            value: interpret_subtree(&from, context),
            original: from,
        }
    }
}

impl InterpretableFrom<&str> for BytesValue {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        BytesValue {
            value: interpret_string(from, context),
            original: ValueSubTree::Str(from.to_string()),
        }
    }
}

impl InterpretableFrom<String> for BytesValue {
    fn interpret_from(from: String, context: &InterpreterContext) -> Self {
        BytesValue {
            value: interpret_string(from.as_str(), context),
            original: ValueSubTree::Str(from),
        }
    }
}

impl Default for BytesValue {
    fn default() -> Self {
        Self {
            value: Vec::new(),
            original: ValueSubTree::Str(String::new()),
        }
    }
}

impl fmt::Display for BytesValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}
