#![allow(missing_docs)]
//! Shim module for running tests in virtue.
//! These methods should all be compatible in API as `proc-macro`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    Brace,
    Bracket,
    Parenthesis,
}

#[derive(Debug, Clone)]
pub struct Group {
    delim: Delimiter,
    body: TokenStream,
}

impl Group {
    pub fn new(delim: Delimiter, body: TokenStream) -> Self {
        Self { delim, body }
    }

    pub fn delimiter(&self) -> Delimiter {
        self.delim
    }

    pub fn span(&self) -> Span {
        Span {}
    }

    pub fn stream(&self) -> TokenStream {
        self.body.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    USize(usize),
    I64(i64),
}

impl Literal {
    pub fn string(s: &str) -> Self {
        Self::String(s.to_owned())
    }
    pub fn usize_unsuffixed(val: usize) -> Self {
        Self::USize(val)
    }
    pub fn i64_unsuffixed(val: i64) -> Self {
        Self::I64(val)
    }
    pub fn span(&self) -> Span {
        Span {}
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::USize(val) => val.to_string(),
            Self::I64(val) => val.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Punct {
    char: char,
    spacing: Spacing,
}

impl Punct {
    pub fn new(char: char, spacing: Spacing) -> Self {
        Self { char, spacing }
    }

    pub fn as_char(&self) -> char {
        self.char
    }

    pub fn span(&self) -> Span {
        Span {}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Spacing {
    Alone,
    Joint,
}

#[derive(Default, Debug, Clone)]
pub struct TokenStream {
    pub trees: Vec<TokenTree>,
}

impl std::str::FromStr for TokenStream {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("{s}")
    }
}

impl TokenStream {
    pub fn new() -> TokenStream {
        TokenStream::default()
    }

    pub fn extend(&mut self, item: impl IntoIterator<Item = TokenTree>) {
        self.trees.extend(item);
    }

    pub fn into_iter(self) -> TokenStreamIter {
        TokenStreamIter {
            trees: self.trees.into_iter(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.trees)
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        Self {
            trees: iter.into_iter().collect(),
        }
    }
}

impl Iterator for TokenStream {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct TokenStreamIter {
    trees: std::vec::IntoIter<TokenTree>,
}

impl Iterator for TokenStreamIter {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.trees.next()
    }
}

#[derive(Debug)]
pub enum LexError {}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Literal(Literal),
    Ident(Ident),
    Group(Group),
    Punct(Punct),
}

impl TokenTree {
    pub fn to_string(&self) -> String {
        todo!()
    }

    pub fn set_span(&mut self, span: Span) {}

    pub fn span(&self) -> Span {
        Span {}
    }
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        Self {
            name: name.to_owned(),
            span,
        }
    }

    pub fn span(&self) -> Span {
        Span {}
    }
}

impl<'a> PartialEq<&'a str> for Ident {
    fn eq(&self, other: &&'a str) -> bool {
        self.name == *other
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Span {}

impl Span {
    pub fn call_site() -> Span {
        Span {}
    }
}
