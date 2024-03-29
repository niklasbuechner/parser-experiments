use super::Grammar;
use super::GrammarSymbol;
use super::Token;
use std::collections::HashMap;

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

#[test]
fn follow_e() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::EndSymbol, Token::ClosingBracket];

    let follow_set = grammar.get_follow("E");

    assert_eq!(expected_set, follow_set);
}

#[test]
fn follow_e_stroke() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::EndSymbol, Token::ClosingBracket];

    let follow_set = grammar.get_follow("E'");

    assert_eq!(expected_set, follow_set);
}

#[test]
fn follow_t() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::Plus, Token::EndSymbol, Token::ClosingBracket];

    let follow_set = grammar.get_follow("T");

    assert_eq!(expected_set, follow_set);
}

#[test]
fn follow_t_stroke() {
    let grammar = Grammar::new();
    let expected_set = vec![Token::Plus, Token::EndSymbol, Token::ClosingBracket];

    let follow_set = grammar.get_follow("T'");

    assert_eq!(expected_set, follow_set);
}

#[test]
fn follow_f() {
    let grammar = Grammar::new();
    let expected_set = vec![
        Token::Times,
        Token::Plus,
        Token::EndSymbol,
        Token::ClosingBracket,
    ];

    let follow_set = grammar.get_follow("F");

    assert_eq!(expected_set, follow_set);
}

#[test]
fn get_parser_table() {
    let grammar = Grammar::new();
    let parser_table = grammar.get_parse_table();
    let expected_table = get_expected_parser_table();

    // Loop through the table one by one in order to see where the error is.
    for key in parser_table.keys() {
        let parser_row = parser_table.get(key);
        let expected_row = expected_table.get(key);

        assert_eq!(expected_row, parser_row);
    }

    assert_eq!(expected_table, parser_table);
}

fn get_expected_parser_table() -> HashMap<String, HashMap<Token, Vec<GrammarSymbol>>> {
    let e_opening_bracket_and_id_production = vec![
        GrammarSymbol::NonTerminal("T".to_string()),
        GrammarSymbol::NonTerminal("E'".to_string()),
    ];

    let e_stroke_plus_production = vec![
        GrammarSymbol::Terminal(Token::Plus),
        GrammarSymbol::NonTerminal("T".to_string()),
        GrammarSymbol::NonTerminal("E'".to_string()),
    ];

    let t_opening_bracket_and_id_production = vec![
        GrammarSymbol::NonTerminal("F".to_string()),
        GrammarSymbol::NonTerminal("T'".to_string()),
    ];

    let t_stroke_times_production = vec![
        GrammarSymbol::Terminal(Token::Times),
        GrammarSymbol::NonTerminal("F".to_string()),
        GrammarSymbol::NonTerminal("T'".to_string()),
    ];

    let f_id_production = vec![GrammarSymbol::Terminal(Token::Id)];
    let f_opening_bracket_production = vec![
        GrammarSymbol::Terminal(Token::OpenBracket),
        GrammarSymbol::NonTerminal("E".to_string()),
        GrammarSymbol::Terminal(Token::ClosingBracket),
    ];

    let mut e_line = HashMap::new();
    e_line.insert(
        Token::OpenBracket,
        e_opening_bracket_and_id_production.clone(),
    );
    e_line.insert(Token::Id, e_opening_bracket_and_id_production);

    let mut e_stroke_line = HashMap::new();
    e_stroke_line.insert(Token::ClosingBracket, Vec::new());
    e_stroke_line.insert(Token::Plus, e_stroke_plus_production);
    e_stroke_line.insert(Token::EndSymbol, Vec::new());

    let mut t_line = HashMap::new();
    t_line.insert(
        Token::OpenBracket,
        t_opening_bracket_and_id_production.clone(),
    );
    t_line.insert(Token::Id, t_opening_bracket_and_id_production);

    let mut t_stroke_line = HashMap::new();
    t_stroke_line.insert(Token::ClosingBracket, Vec::new());
    t_stroke_line.insert(Token::Plus, Vec::new());
    t_stroke_line.insert(Token::Times, t_stroke_times_production);
    t_stroke_line.insert(Token::EndSymbol, Vec::new());

    let mut f_line = HashMap::new();
    f_line.insert(Token::Id, f_id_production);
    f_line.insert(Token::OpenBracket, f_opening_bracket_production);

    let mut table = HashMap::new();
    table.insert("E".to_string(), e_line);
    table.insert("E'".to_string(), e_stroke_line);
    table.insert("T".to_string(), t_line);
    table.insert("T'".to_string(), t_stroke_line);
    table.insert("F".to_string(), f_line);

    table
}
