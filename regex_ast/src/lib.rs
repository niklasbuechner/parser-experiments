mod regex;

pub use regex::get_regex_syntax_tree;

#[derive(Debug, PartialEq)]
pub enum RegexAstElements {
    Alternation(Box<RegexAstElements>, Box<RegexAstElements>),
    Concatenation(Box<RegexAstElements>, Box<RegexAstElements>),
    Leaf(MatchingGroup),
    None,
    ZeroOrMore(Box<RegexAstElements>),
    ZeroOrOne(Box<RegexAstElements>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingGroup {
    Character(char),
    Group(Vec<MatchingGroupElements>),
    NegativeGroup(Vec<MatchingGroupElements>),
    AcceptedState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingGroupElements {
    Character(char),
    Range(char, char),
}
