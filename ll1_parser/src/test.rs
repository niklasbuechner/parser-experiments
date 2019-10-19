use super::Grammar;
use super::Token;

#[test]
fn first_e() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::OpenBracket, Token::Id];

    let first_set = grammar.get_first("E");

    assert_eq!(expected_set, first_set);
}

#[test]
fn first_e_stroke() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::Plus, Token::EmptySet];

    let first_set = grammar.get_first("E'");

    assert_eq!(expected_set, first_set);
}

#[test]
fn first_t() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::OpenBracket, Token::Id];

    let first_set = grammar.get_first("T");

    assert_eq!(expected_set, first_set);
}

#[test]
fn first_t_stroke() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::Times, Token::EmptySet];

    let first_set = grammar.get_first("T'");

    assert_eq!(expected_set, first_set);
}

#[test]
fn first_f() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::OpenBracket, Token::Id];

    let first_set = grammar.get_first("F");

    assert_eq!(expected_set, first_set);
}
