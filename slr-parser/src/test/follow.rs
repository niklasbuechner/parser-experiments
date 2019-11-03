use super::Grammar;
use super::GrammarSymbol;
use super::Production;
use super::Token;

fn get_ll_grammar() -> Grammar {
    let mut grammar = Grammar::empty();
    grammar.set_starting_non_terminal("E");
    grammar.set_end_token("EndSymbol");

    grammar.add_production(Production::from_string(
        "E",
        vec![
            GrammarSymbol::non_terminal("T"),
            GrammarSymbol::non_terminal("E'"),
        ],
    ));

    grammar.add_production(Production::from_string(
        "E'",
        vec![
            GrammarSymbol::terminal(Token::new("Plus")),
            GrammarSymbol::non_terminal("T"),
            GrammarSymbol::non_terminal("E'"),
        ],
    ));
    grammar.add_production(Production::from_string("E'", Vec::new()));

    grammar.add_production(Production::from_string(
        "T",
        vec![
            GrammarSymbol::non_terminal("F"),
            GrammarSymbol::non_terminal("T'"),
        ],
    ));

    grammar.add_production(Production::from_string(
        "T'",
        vec![
            GrammarSymbol::terminal(Token::new("Times")),
            GrammarSymbol::non_terminal("F"),
            GrammarSymbol::non_terminal("T'"),
        ],
    ));
    grammar.add_production(Production::from_string("T'", Vec::new()));

    grammar.add_production(Production::from_string(
        "F",
        vec![
            GrammarSymbol::terminal(Token::new("OpeningBracket")),
            GrammarSymbol::non_terminal("E"),
            GrammarSymbol::terminal(Token::new("ClosingBracket")),
        ],
    ));
    grammar.add_production(Production::from_string(
        "F",
        vec![GrammarSymbol::terminal(Token::new("Id"))],
    ));

    grammar
}

#[test]
fn slr_follow_e_stroke() {
    let grammar = Grammar::new();
    let expected_follow = vec![Token::new("EndSymbol")];

    let follow = grammar.get_follow("E'");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn slr_follow_e() {
    let grammar = Grammar::new();
    let expected_follow = vec![
        Token::new("Plus"),
        Token::new("ClosingBracket"),
        Token::new("EndSymbol"),
    ];

    let follow = grammar.get_follow("E");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn slr_follow_t() {
    let grammar = Grammar::new();
    let expected_follow = vec![
        Token::new("Times"),
        Token::new("Plus"),
        Token::new("ClosingBracket"),
        Token::new("EndSymbol"),
    ];

    let follow = grammar.get_follow("T");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn slr_follow_f() {
    let grammar = Grammar::new();
    let expected_follow = vec![
        Token::new("Times"),
        Token::new("Plus"),
        Token::new("ClosingBracket"),
        Token::new("EndSymbol"),
    ];

    let follow = grammar.get_follow("F");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn ll_follow_e() {
    let grammar = get_ll_grammar();
    let expected_follow = vec![Token::new("ClosingBracket"), Token::new("EndSymbol")];

    let follow = grammar.get_follow("E");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn ll_follow_e_stroke() {
    let grammar = get_ll_grammar();
    let expected_follow = vec![Token::new("ClosingBracket"), Token::new("EndSymbol")];

    let follow = grammar.get_follow("E'");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn ll_follow_t() {
    let grammar = get_ll_grammar();
    let expected_follow = vec![
        Token::new("Plus"),
        Token::new("ClosingBracket"),
        Token::new("EndSymbol"),
    ];

    let follow = grammar.get_follow("T");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn ll_follow_t_stroke() {
    let grammar = get_ll_grammar();
    let expected_follow = vec![
        Token::new("Plus"),
        Token::new("ClosingBracket"),
        Token::new("EndSymbol"),
    ];

    let follow = grammar.get_follow("T'");

    assert_contents_only(expected_follow, follow);
}

#[test]
fn ll_follow_f() {
    let grammar = get_ll_grammar();
    let expected_follow = vec![
        Token::new("Times"),
        Token::new("Plus"),
        Token::new("ClosingBracket"),
        Token::new("EndSymbol"),
    ];

    let follow = grammar.get_follow("F");

    assert_contents_only(expected_follow, follow);
}

fn assert_contents_only<T: std::fmt::Debug + PartialEq>(expected: Vec<T>, actual: Vec<T>) {
    println!("Expected: {:#?}", expected);
    println!("Actual: {:#?}", actual);

    assert_eq!(expected.len(), actual.len());
    expected.iter().for_each(|element| {
        if !actual.contains(element) {
            panic!("Element does not exist in actual Vec: {:#?}", element);
        }
    })
}
